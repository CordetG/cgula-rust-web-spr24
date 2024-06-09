#![allow(unused_imports, dead_code, unused_must_use, unused_variables)]
use axum::extract::{self, path, Extension, Path, State};
use axum::{
    http::{HeaderValue, Method, StatusCode},
    response::{IntoResponse, Response},
    routing::{any, get, post},
    Json, Router,
};

use axum_macros::debug_handler;
use tokio::sync::RwLock;
use tower_http::compression::CompressionLayer;
use tower_http::cors::CorsLayer;
use tower_http::follow_redirect::policy::PolicyExt;
use tower_http::services::{ServeDir, ServeFile};

use headers::ContentType;
use serde::{Deserialize, Serialize, Serializer};
extern crate tracing;

use crate::PgRow;
use axum::handler::Handler;
use serde::ser::{Error, SerializeStruct};
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

// Section 4.2 -- Creating a 'store' for the questions
/// The `Store` struct contains a collection of questions stored in a HashMap with `QuestionId` keys and
/// `Question` values.
///
/// Properties:
///
/// * `questions`: The `questions` property in the `Store` struct is a HashMap that stores `Question`
/// objects with a key of type `QuestionId`. This allows you to efficiently store and retrieve questions
/// based on their unique identifiers.
#[derive(Clone)]
pub struct Store {
    pub questions: Arc<RwLock<HashMap<QuestionId, Question>>>,
}

impl Store {
    /// The function `new` initializes a `Store` struct with questions initialized using the `init` method.
    ///
    /// Returns:
    ///
    /// An instance of the `Store` struct is being returned.
    pub fn new() -> Self {
        Store {
            questions: Arc::new(RwLock::new(Self::init())),
        }
    }

    /// The `init` function reads questions from a JSON file and returns them as a HashMap.
    ///
    /// Returns:
    ///
    /// A `HashMap` containing `QuestionId` as keys and `Question` as values is being returned.
    fn init() -> HashMap<QuestionId, Question> {
        let file = include_str!("../questions.json");
        serde_json::from_str(file).expect("can't read questions.json")
    }

    /// The function `get_questions` asynchronously retrieves a question and returns a result indicating
    /// success or failure.
    ///
    /// Returns:
    ///
    /// The function `get_questions()` returns a `Result` enum with either an `ApiResponse` or an
    /// `ApiError`. In this specific case, if the parsing of the question ID to an `i32` is successful, it
    /// will return `Ok(ApiResponse::JsonData(question))`, where `question` is an instance of the `Question`
    /// struct. If the parsing fails, it will return an INvalidInput ApiError.
    #[debug_handler]
    pub async fn get_questions() -> Result<ApiResponse, ApiError> {
        /*match params.get("start") {
            Some(start) => println!("{}", start),
            None => println!("No start value"),
        }*/

        let question: Question = Question::new(
            QuestionId::from_str("1").expect("No id provided"),
            "First Question",
            "Content of question",
            &["faq"],
        );
        match question.id.0.parse::<i32>() {
            Err(_) => Err(ApiError::NotFound),
            Ok(_) => Ok(ApiResponse::JsonData(question)),
        }
    }
}

impl Default for Store {
    fn default() -> Self {
        Self::new()
    }
}

/*
// Reference from jokebase class repo
#[derive(Debug, thiserror::Error, ToSchema, Serialize)]
// XXX Fixme!
#[allow(dead_code)]
pub enum StoreErr {
    #[error("Store io failed: {0}")]
    StoreIoError(String),
    #[error("no question")]
    NoQuestion,
    #[error("question {0} doesn't exist")]
    QuestionDoesNotExist(String),
    #[error("database error: {0}")]
    DatabaseError(String),
}

impl From<std::io::Error> for StoreErr {
    fn from(err: std::io::Error) -> Self {
        StoreErr::StoreIoError(err.to_string())
    }
}

impl From<sqlx::Error> for StoreErr {
    fn from(err: sqlx::Error) -> Self {
        StoreErr::DatabaseError(err.to_string())
    }
}

#[derive(Debug)]
pub struct StoreError {
    pub status: StatusCode,
    pub error: StoreErr,
}

pub fn error_schema(name: &str, example: serde_json::Value) -> (&str, RefOr<Schema>) {
    let sch = ObjectBuilder::new()
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
        let error = StoreError { status, error };
        (status, Json(error)).into_response()
    }
}

#[derive(Debug, Clone)]
pub struct Store(pub Pool<Postgres>);

impl Store {
    async fn to_question(&self, row: &PgRow) -> Result<Question, sqlx::Error> {
        let id: QuestionId = row.get("id");
        let tags: Vec<_> = sqlx::query(r#"SELECT tag FROM tags WHERE id = $1"#)
            .bind(&id)
            .fetch_all(&self.0)
            .await?;
        let tags: HashSet<String> = tags.iter().map(|row| row.get("tag")).collect();
        let tags: Option<HashSet<String>> = if tags.is_empty() { None } else { Some(tags) };
        Ok(Question {
            id,
            title,
            content,
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

    pub async fn new() -> Result<Self, Box<dyn Error>> {
        use std::env::var;

        let password = read_secret("PG_PASSWORDFILE").await?;
        let url = format!(
            "postgres://{}:{}@{}:5432/{}",
            var("PG_USER")?,
            password.trim(),
            var("PG_HOST")?,
            var("PG_DBNAME")?,
        );
        let pool: Pool<Postgres> = PgPool::connect(&url).await?;
        sqlx::migrate!().run(&pool).await?;
        Ok(Store(pool))
    }

    pub async fn get<'a>(&self, index: &str) -> Result<Question, StoreErr> {
        let row = sqlx::query(r#"SELECT * FROM questions WHERE id = $1;"#)
            .bind(index)
            .fetch_one(&self.0)
            .await?;

        let question = self.to_question(&row).await?;
        Ok(question)
    }

    pub async fn get_questions() -> Result<ApiResponse, ApiError> {
        /*match params.get("start") {
            Some(start) => println!("{}", start),
            None => println!("No start value"),
        }*/

        let question: Question = Question::new(
            QuestionId::from_str("1").expect("No id provided"),
            "First Question",
            "Content of question",
            &["faq"],
        );
        match question.id.0.parse::<i32>() {
            Err(_) => Err(ApiError::NotFound),
            Ok(_) => Ok(ApiResponse::JsonData(question)),
        }
    }

    /*pub async fn get_questions<'a>(&self) -> Result<Vec<Question>, StoreErr> {
        let rows: Vec<PgRow> = sqlx::query(r#"SELECT * FROM questions;"#)
            .fetch_all(&self.0)
            .await?;
        let mut questions: Vec<Question> = Vec::with_capacity(rows.len());
        for j in rows.iter() {
            questions.push(self.to_question(j).await?);
        }
        Ok(questions)
    }*/

    pub async fn add(&mut self, question: Question, id: QuestionId) -> Result<(), StoreErr> {
        let mut tx: sqlx::Transaction<'_, _> = Pool::begin(&self.0).await?;
        let result: Result<!, sqlx::Error> = sqlx::query(
            r#"INSERT INTO questions
            (id, whos_there, answer_who, source)
            VALUES ($1, $2, $3, $4);"#,
        )
        .bind(&id)
        .bind(&question.title)
        .bind(&question.content)
        .execute(&mut *tx)
        .await;
        result.map_err(|err| {
            if let sqlx::Error::Database(ref dbe) = err {
                if let Some("23505") = dbe.code().as_deref() {
                    return StoreErr::QuestionExists(question.id.to_string());
                }
            }
            StoreErr::DatabaseError(e.to_string())
        })?;
        Self::insert_tags(&mut tx, &question.id, &question.tags).await?;
        Ok(tx.commit().await?)
    }

    pub async fn delete(&mut self, index: &str) -> Result<(), StoreErr> {
        let mut tx = Pool::begin(&self.0).await?;
        sqlx::query(r#"DELETE FROM tags WHERE id = $1;"#)
            .bind(index)
            .execute(&mut *tx)
            .await?;
        let result = sqlx::query(r#"DELETE FROM questions WHERE id = $1 RETURNING questions.id;"#)
            .bind(index)
            .fetch_all(&mut *tx)
            .await?;
        if result.len() == 0 {
            return Err(StoreErr::QuestionDoesNotExist(index.to_string()));
        }
        Ok(tx.commit().await?)
    }

    pub async fn update(&mut self, index: &str, question: Question) -> Result<(), StoreErr> {
        let mut tx = Pool::begin(&self.0).await?;
        let q = sqlx::query(
            r#"UPDATE questions
            SET (whos_there, answer_who, source) = ($2, $3, $4)
            WHERE questions.id = $1
            RETURNING questions.id;"#,
        );
        let result = q
            .bind(&question.id)
            .bind(&question.title)
            .bind(&question.content)
            .fetch_all(&mut *tx)
            .await?;
        if result.len() == 0 {
            return Err(StoreErr::QuestionDoesNotExist(index.to_string()));
        }
        sqlx::query(r#"DELETE FROM tags WHERE id = $1;"#)
            .bind(index)
            .execute(&mut *tx)
            .await?;
        Self::insert_tags(&mut tx, &question.id, &question.tags).await?;
        Ok(tx.commit().await?)
    }
}*/
