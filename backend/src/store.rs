#![allow(unused_imports, dead_code, unused_must_use, unused_variables)]
use axum::extract::{self, path, Extension, Path, State};
use axum::{
    http::{HeaderValue, Method, StatusCode},
    response::{IntoResponse, Response},
    routing::{any, get, post},
    Json, Router,
};
use axum_macros::debug_handler;
use headers::ContentType;
use serde::{Deserialize, Serialize, Serializer};
use tokio::sync::mpsc::error;
use tokio::sync::RwLock;
use tower_http::compression::CompressionLayer;
use tower_http::cors::CorsLayer;
use tower_http::follow_redirect::policy::PolicyExt;
use tower_http::services::{ServeDir, ServeFile};
extern crate tracing;
use axum::handler::Handler;
use core::num::ParseIntError;
use serde::ser::{Error, SerializeStruct};
use sqlx::postgres::{PgPool, PgPoolOptions, PgRow};
use sqlx::{PgConnection, Pool, Postgres, Row};
use std::collections::{HashMap, HashSet};
use std::convert::Infallible;
use std::io::ErrorKind;
use std::net::{Ipv4Addr, SocketAddrV4, TcpStream};
use std::str::FromStr;
use std::sync::Arc;
use utoipa::{
    openapi::{ObjectBuilder, RefOr, Schema, SchemaType},
    ToSchema,
};

use crate::question::*;
/*use crate::sqlx::types::{
    answer::Answer,
    question::{Question, QuestionId},
};*/
use sqlx::error::Error as SqlxError;
use tracing::{event, instrument, Level};

// Implementing Axum 'IntoResponse' from shuttle.rs but with the Serialized Question
pub enum ApiResponse {
    OK,
    Created,
    JsonData(Question),
}

// To return a result, implement an error type
pub enum ApiError {
    NotFound,
    NotImplemented,
    Failed,
}

/// The `impl IntoResponse for ApiResponse` block is implementing the `IntoResponse` trait for the
/// `ApiResponse` enum. This trait allows instances of the `ApiResponse` enum to be converted into an
/// HTTP response.
impl IntoResponse for ApiResponse {
    /// The function `into_response` converts an enum variant into a corresponding HTTP response.
    ///
    /// Returns:
    ///
    /// The `into_response` function is returning a `Response` object based on the variant of the enum
    /// `Self`. Depending on the variant, it will create and return a response with the corresponding
    /// status code and data.
    fn into_response(self) -> Response {
        match self {
            Self::OK => (StatusCode::OK).into_response(),
            Self::Created => (StatusCode::CREATED).into_response(),
            Self::JsonData(data) => (StatusCode::OK, Json(data)).into_response(),
        }
    }
}

/// The `impl IntoResponse for ApiError` block is implementing the `IntoResponse` trait for the
/// `ApiError` enum. This trait allows instances of the `ApiError` enum to be converted into an HTTP
/// response.
impl IntoResponse for ApiError {
    /// The function `into_response` converts enum variants into corresponding HTTP responses.
    ///
    /// Returns:
    ///
    /// A `Response` object is being returned based on the variant of the enum `self`. The
    /// `into_response` method is being called on the tuple `(StatusCode, &str)` to convert it into a
    /// `Response` object.
    fn into_response(self) -> Response {
        match self {
            Self::NotFound => (StatusCode::NOT_FOUND, "404 Not Found").into_response(),
            Self::NotImplemented => {
                (StatusCode::NOT_IMPLEMENTED, "501 Not Implemented").into_response()
            }
            Self::Failed => {
                (StatusCode::EXPECTATION_FAILED, "417 Expectation Failed").into_response()
            }
        }
    }
}

// Reference from jokebase class repo
#[derive(Debug, thiserror::Error, ToSchema, Serialize)]
pub enum StoreErr {
    #[error("Cannot parse parameter")]
    ParseError(String),
    #[error{"Missing Parameter"}]
    MissingParameters(String),
    #[error("Query could not be executed")]
    DatabaseQueryError(String),
    #[error("Question doesn't exist")]
    QuestionNotFound(String),
}

impl From<std::num::ParseIntError> for StoreErr {
    fn from(e: std::num::ParseIntError) -> Self {
        StoreErr::ParseError(e.to_string())
    }
}

impl From<SqlxError> for StoreErr {
    fn from(e: SqlxError) -> Self {
        StoreErr::DatabaseQueryError(e.to_string())
    }
}

#[derive(Debug)]
pub struct StoreError {
    pub status: StatusCode,
    pub error: StoreErr,
}

pub fn error_schema(name: &str, example: serde_json::Value) -> (&str, RefOr<Schema>) {
    let sch: RefOr<Schema> = ObjectBuilder::new()
        .property(
            "status",
            ObjectBuilder::new().schema_type(SchemaType::String),
        )
        .property(
            "error",
            ObjectBuilder::new().schema_type(SchemaType::String),
        )
        .example(Some(example))
        .into();
    (name, sch)
}

impl<'s> ToSchema<'s> for StoreError {
    fn schema() -> (&'s str, RefOr<Schema>) {
        let example = serde_json::json!({
            "status":"404","error":"no question"
        });
        error_schema("StoreError", example)
    }
}

impl Serialize for StoreError {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let status: String = self.status.to_string();
        let mut state = serializer.serialize_struct("StoreError", 2)?;
        state.serialize_field("status", &status)?;
        state.serialize_field("error", &self.error)?;
        state.end()
    }
}

impl StoreError {
    pub fn response(status: StatusCode, error: StoreErr) -> Response {
        let error: StoreError = StoreError { status, error };
        (status, Json(error)).into_response()
    }
}

/// Pagination struct which is getting extract
/// from query params
#[derive(Default, Debug)]
pub struct Pagination {
    /// The index of the last item which has to be returned
    pub limit: Option<u32>,
    /// The index of the first item which has to be returned
    pub offset: u32,
}

/// Extract query parameters from the `/questions` route
///
/// # Example query
///
/// GET requests to this route can have a pagination attached so we just
/// return the questions we need
/// `/questions?start=1&end=10`
///
/// # Example usage
///
/// ```rust
/// let mut query = HashMap::new();
/// query.insert("start".to_string(), "1".to_string());
/// query.insert("end".to_string(), "10".to_string());
/// let p = types::pagination::extract_pagination(query).unwrap();
/// assert_eq!(p.start, 1);
/// assert_eq!(p.end, 10);
/// ```
pub fn extract_pagination(params: HashMap<String, String>) -> Result<Pagination, StoreErr> {
    // Could be improved in the future
    if params.contains_key("limit") && params.contains_key("offset") {
        return Ok(Pagination {
            // Takes the "limit" parameter in the query
            // and tries to convert it to a number
            limit: Some(params.get("limit").unwrap().parse::<u32>().map_err(
                |error: std::num::ParseIntError| -> ParseIntError {
                    std::num::ParseIntError::into(error)
                },
            )?),
            // Takes the "offset" parameter in the query
            // and tries to convert it to a number
            offset: params.get("offset").unwrap().parse::<u32>().map_err(
                |error: std::num::ParseIntError| -> ParseIntError {
                    std::num::ParseIntError::into(error)
                },
            )?,
        });
    }

    Err(StoreErr::MissingParameters(
        "Missing Parameters".to_string(),
    ))
}

#[derive(Debug, Clone)]
pub struct Store {
    pub connection: PgPool,
}

impl Store {
    async fn to_question(&self, row: &PgRow) -> Result<Question, sqlx::Error> {
        let id: String = row.get("id");
        let tags: Vec<_> = sqlx::query(r#"SELECT tag FROM tags WHERE id = $1"#)
            .bind(&id)
            .fetch_all(&self.0)
            .await?;
        let tags: HashSet<String> = tags.iter().map(|row| row.get("tag")).collect();
        let tags: Option<HashSet<String>> = if tags.is_empty() { None } else { Some(tags) };
        Ok(Question {
            id,
            title: row.get("title"),
            content: row.get("content"),
            tags,
        })
    }

    async fn insert_tags(
        tx: &mut PgConnection,
        id: &str,
        tags: &Option<HashSet<String>>,
    ) -> Result<(), sqlx::Error> {
        if let Some(tags) = tags {
            for tag in tags {
                sqlx::query(r#"INSERT INTO tags (id, tag) VALUES ($1, $2);"#)
                    .bind(id)
                    .bind(tag)
                    .execute(&mut *tx)
                    .await?;
            }
        }
        Ok(())
    }

    pub async fn new(db_url: &str) -> Self {
        let db_pool: Pool<Postgres> = match PgPoolOptions::new()
            .max_connections(5)
            .connect(db_url)
            .await
        {
            Ok(pool) => pool,
            Err(e) => panic!("Couldn't establish DB connection:[]", e),
        };

        Store {
            connection: db_pool,
        }
    }

    pub async fn get<'a>(&self, index: &str) -> Result<Question, StoreErr> {
        let row: PgRow = sqlx::query(r#"SELECT * FROM questions WHERE id = $1;"#)
            .bind(index)
            .fetch_one(&self.0)
            .await?;

        let question: Question = self.to_question(&row).await?;
        Ok(question)
    }

    pub async fn get_questions(
        &self,
        limit: Option<u32>,
        offset: u32,
    ) -> Result<Vec<Question>, sqlx::Error> {
        match sqlx::query("SELECT * from questions LIMIT $1 OFFSET $2")
            .bind(limit)
            .bind(offset)
            .map(|row: PgRow| Question {
                id: QuestionId(row.get("id")),
                title: row.get("title"),
                content: row.get("content"),
                tags: row.get("tags"),
            })
            .fetch_all(&self.connection)
            .await
        {
            Ok(questions) => Ok(questions),
            Err(e) => {
                tracing::event!(tracing::Level::ERROR, "{:?}", e);
                Err(Error::DatabaseQueryError)
            }
        }
    }

    async fn add_answer(
        Extension(store): Extension<Store>,
        Json(params): Json<HashMap<String, String>>,
    ) -> impl IntoResponse {
        let answer = Answer {
            id: AnswerId("1".to_string()),
            content: params.get("content").unwrap().to_string(),
            question_id: QuestionId(params.get("questionId").unwrap().to_string()),
        };

        store
            .answers
            .write()
            .await
            .insert(answer.id.clone(), answer);

        (
            StatusCode::OK,
            AxumJson(json!({ "message": "Answer added" })),
        )
    }

    // Define an async handler function for Axum

    pub async fn add_question(&self, new_question: NewQuestion) -> Result<Question, sqlx::Error> {
        match sqlx::query("INSERT INTO questions (title, content, tags) VALUES ($1, $2, $3)")
            .bind(new_question.title)
            .bind(new_question.content)
            .bind(new_question.tags)
            .map(|row: PgRow| Question {
                id: QuestionId(row.get("id")),
                title: row.get("title"),
                content: row.get("content"),
                tags: row.get("tags"),
            })
            .fetch_one(&self.connection)
            .await
        {
            Ok(question) => Ok(question),
            Err(e) => Err(e),
        }
    }

    pub async fn delete(&mut self, index: &str) -> Result<(), StoreErr> {
        let mut tx: sqlx::Transaction<'_, Postgres> = Pool::begin(&self.questions).await?;
        sqlx::query(r#"DELETE FROM tags WHERE id = $1;"#)
            .bind(index)
            .execute(&mut *tx)
            .await?;
        let result = sqlx::query(r#"DELETE FROM questions WHERE id = $1 RETURNING questions.id;"#)
            .bind(index)
            .fetch_all(&mut *tx)
            .await?;
        if result.len() == 0 {
            return Err(StoreErr::QuestionNotFound(index.to_string()));
        }
        Ok(tx.commit().await?)
    }

    pub async fn update(&mut self, index: &str, question: Question) -> Result<(), StoreErr> {
        let mut tx: sqlx::Transaction<'_, Postgres> = Pool::begin(&self.questions).await?;
        let q: sqlx::query::Query<Postgres, sqlx::postgres::PgArguments> = sqlx::query(
            r#"UPDATE questions
            SET (whos_there, answer_who, source) = ($2, $3, $4)
            WHERE questions.id = $1
            RETURNING questions.id;"#,
        );
        let result: Vec<PgRow> = q
            .bind(&question.id)
            .bind(&question.title)
            .bind(&question.content)
            .fetch_all(&mut *tx)
            .await?;
        if result.len() == 0 {
            return Err(StoreErr::QuestionNotFound(index.to_string()));
        }
        sqlx::query(r#"DELETE FROM tags WHERE id = $1;"#)
            .bind(index)
            .execute(&mut *tx)
            .await?;
        Self::insert_tags(&mut tx, &question.id, &question.tags).await?;
        Ok(tx.commit().await?)
    }
}
