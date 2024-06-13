#![allow(unused_imports, dead_code, unused_must_use, unused_variables)]
use axum::extract::{self, path, Extension, Path, State};
use axum::{
    http::{HeaderValue, Method, StatusCode},
    response::{IntoResponse, Response},
    routing::{any, get, post},
    Json, Router,
};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

use crate::types::{
    answer::{Answer, AnswerId},
    question::{Question, QuestionId},
};
use axum_macros::debug_handler;
use headers::ContentType;
use serde::{Deserialize, Serialize, Serializer};
use std::collections::HashSet;
use tokio::sync::mpsc::error;
use tower_http::compression::CompressionLayer;
use tower_http::cors::CorsLayer;
use tower_http::follow_redirect::policy::PolicyExt;
use tower_http::services::{ServeDir, ServeFile};
extern crate tracing;
use crate::error::StoreErr;
use axum::handler::Handler;
use core::num::ParseIntError;
use serde::ser::{Error, SerializeStruct};
use serde_json::json;
use serde_json::Value;
use sqlx::postgres::{PgPool, PgPoolOptions, PgRow};
use sqlx::FromRow;
use sqlx::{PgConnection, Pool, Postgres, Row};
use std::convert::Infallible;
use std::io::ErrorKind;
use std::net::{Ipv4Addr, SocketAddrV4, TcpStream};
use std::str::FromStr;
use utoipa::{
    openapi::{ObjectBuilder, RefOr, Schema, SchemaType},
    ToSchema,
};

use crate::routes::question::get_questions;
use sqlx::error::Error as SqlxError;
use tracing::{event, instrument, Level};

#[derive(Debug, Clone)]
pub struct Store {
    pub connection: Pool<Postgres>,
}

impl Store {
    async fn insert_tags(
        tx: &mut PgConnection,
        id: &QuestionId,
        tags: &Option<HashSet<String>>,
    ) -> Result<(), sqlx::Error> {
        if let Some(tags) = tags {
            for tag in tags {
                sqlx::query(r#"INSERT INTO tags (id, tag) VALUES ($1, $2);"#)
                    .bind(id.0.as_str())
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
            Err(e) => panic!("Couldn't establish DB connection:{}", e),
        };

        Store {
            connection: db_pool,
        }
    }

    pub async fn to_question(&self, row: &PgRow) -> Result<Question, sqlx::Error> {
        let id: String = row.get("id");
        let tags: Vec<_> = sqlx::query(r#"SELECT tag FROM tags WHERE id = $1"#)
            .bind(&id)
            .fetch_all(&self.connection)
            .await?;
        let tags: HashSet<String> = tags.iter().map(|row: &PgRow| row.get("tag")).collect();
        let tags: Option<HashSet<String>> = if tags.is_empty() { None } else { Some(tags) };
        Ok(Question {
            id: QuestionId(row.get("id")),
            title: row.get("title"),
            content: row.get("content"),
            tags,
        })
    }

    pub async fn get<'a>(&self, index: &str) -> Result<Question, StoreErr> {
        let row: PgRow = sqlx::query(r#"SELECT * FROM questions WHERE id = $1;"#)
            .bind(index)
            .fetch_one(&self.connection)
            .await?;

        let question: Question = self.to_question(&row).await?;
        Ok(question)
    }

    pub async fn get_random(&self) -> Result<Question, StoreErr> {
        let row: PgRow = sqlx::query(r#"SELECT * FROM questions ORDER BY RANDOM () LIMIT 1;"#)
            .fetch_one(&self.connection)
            .await?;

        let question: Question = self.to_question(&row).await?;
        Ok(question)
    }

    pub async fn get_questions<'a>(&self) -> Result<Vec<Question>, StoreErr> {
        let rows = sqlx::query(r#"SELECT * FROM jokes;"#)
            .fetch_all(&self.connection)
            .await?;
        let mut questions: Vec<Question> = Vec::with_capacity(rows.len());
        for q in rows.iter() {
            questions.push(self.to_question(q).await?);
        }
        Ok(questions)
    }

    // Define an async handler function for Axum

    pub async fn add_question(
        &self,
        new_question: Question,
        question_id: i32,
    ) -> Result<(), StoreErr> {
        let mut tx: sqlx::Transaction<'_, Postgres> = Pool::begin(&self.connection).await?;
        sqlx::query("INSERT INTO questions (title, content, tags) VALUES ($1, $2, $3)")
            .bind(question_id)
            .bind(new_question.title)
            .bind(new_question.content)
            .execute(&mut *tx)
            .await;
        Self::insert_tags(&mut tx, &new_question.id, &new_question.tags).await?;
        Ok(tx.commit().await?)
    }

    pub async fn delete_question(&mut self, index: &str) -> Result<(), StoreErr> {
        let mut tx: sqlx::Transaction<'_, Postgres> = Pool::begin(&self.connection).await?;
        sqlx::query(r#"DELETE FROM tags WHERE id = $1;"#)
            .bind(index)
            .execute(&mut *tx)
            .await?;
        let result: Vec<PgRow> =
            sqlx::query(r#"DELETE FROM questions WHERE id = $1 RETURNING questions.id;"#)
                .bind(index)
                .fetch_all(&mut *tx)
                .await?;
        #[allow(clippy::len_zero)]
        if result.len() == 0 {
            return Err(StoreErr::QuestionNotFound(index.to_string()));
        }
        Ok(tx.commit().await?)
    }

    pub async fn update_question(
        &mut self,
        index: &str,
        question: Question,
        question_id: i32,
    ) -> Result<(), StoreErr> {
        let mut tx: sqlx::Transaction<'_, Postgres> = Pool::begin(&self.connection).await?;
        let q: sqlx::query::Query<Postgres, sqlx::postgres::PgArguments> = sqlx::query(
            r#"UPDATE questions
        SET (title, content) = ($2, $3)
        WHERE id = $1
        RETURNING id;"#,
        );
        let result: Vec<PgRow> = q
            .bind(question_id)
            .bind(&question.title)
            .bind(&question.content)
            .fetch_all(&mut *tx)
            .await?;
        if result.is_empty() {
            return Err(StoreErr::QuestionNotFound(index.to_string()));
        }
        sqlx::query(r#"DELETE FROM tags WHERE id = $1;"#)
            .bind(index)
            .execute(&mut *tx)
            .await?;
        Self::insert_tags(&mut tx, &question.id, &question.tags).await?;
        Ok(tx.commit().await?)
    }

    pub async fn add_answer(&self, new_answer: Answer) -> Result<(), sqlx::Error> {
        let mut tx: sqlx::Transaction<'_, Postgres> = Pool::begin(&self.connection).await?;
        sqlx::query("INSERT INTO questions (title, content, tags) VALUES ($1, $2, $3)")
            .bind(new_answer.question_id.0)
            .bind(new_answer.content)
            .bind(new_answer.id.0)
            .execute(&mut *tx)
            .await;
        tx.commit().await
    }
}

/*impl Store {
    pub async fn new(db_url: &str) -> Self {
        let db_pool = match PgPoolOptions::new()
            .max_connections(5)
            .connect(db_url)
            .await
        {
            Ok(pool) => pool,
            Err(e) => panic!("Couldn't establish DB connection: {}", e),
        };

        Store {
            connection: db_pool,
        }
    }

    pub async fn get_questions(
        &self,
        limit: Option<u32>,
        offset: u32,
    ) -> Result<Vec<Question>, Error> {
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

    pub async fn add_question(&self, new_question: NewQuestion) -> Result<Question, Error> {
        match sqlx::query(
            "INSERT INTO questions (title, content, tags)
                 VALUES ($1, $2, $3)
                 RETURNING id, title, content, tags",
        )
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
            Err(e) => {
                tracing::event!(tracing::Level::ERROR, "{:?}", e);
                Err(Error::DatabaseQueryError)
            }
        }
    }

    pub async fn update_question(
        &self,
        question: Question,
        question_id: i32,
    ) -> Result<Question, Error> {
        match sqlx::query(
            "UPDATE questions SET title = $1, content = $2, tags = $3
        WHERE id = $4
        RETURNING id, title, content, tags",
        )
        .bind(question.title)
        .bind(question.content)
        .bind(question.tags)
        .bind(question_id)
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
            Err(e) => {
                tracing::event!(tracing::Level::ERROR, "{:?}", e);
                Err(Error::DatabaseQueryError)
            }
        }
    }

    pub async fn delete_question(&self, question_id: i32) -> Result<bool, Error> {
        match sqlx::query("DELETE FROM questions WHERE id = $1")
            .bind(question_id)
            .execute(&self.connection)
            .await
        {
            Ok(_) => Ok(true),
            Err(e) => {
                tracing::event!(tracing::Level::ERROR, "{:?}", e);
                Err(Error::DatabaseQueryError)
            }
        }
    }

    pub async fn add_answer(&self, new_answer: NewAnswer) -> Result<Answer, Error> {
        match sqlx::query("INSERT INTO answers (content, question_id) VALUES ($1, $2)")
            .bind(new_answer.content)
            .bind(new_answer.question_id.0)
            .map(|row: PgRow| Answer {
                id: AnswerId(row.get("id")),
                content: row.get("content"),
                question_id: QuestionId(row.get("question_id")),
            })
            .fetch_one(&self.connection)
            .await
        {
            Ok(answer) => Ok(answer),
            Err(e) => {
                tracing::event!(tracing::Level::ERROR, "{:?}", e);
                Err(Error::DatabaseQueryError)
            }
        }
    }
}*/
