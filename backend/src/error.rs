use crate::store;
use axum::{
    http::{HeaderValue, Method, StatusCode},
    response::{IntoResponse, Response},
    routing::{any, get, post},
    Json, Router,
};
use serde::ser::{Error, SerializeStruct, Serializer};
use serde::Serialize;
use sqlx::postgres::{PgPool, PgPoolOptions, PgRow};
use sqlx::{error::Error as SqlxError, Postgres};
use std::collections::HashSet;
use tower_http::cors::CorsLayer;
use utoipa::openapi::{ObjectBuilder, RefOr, Schema, SchemaType};
use utoipa::ToSchema;

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
        let example: serde_json::Value = serde_json::json!({
            "status":"404","error":"no question"
        });
        error_schema("StoreError", example)
    }
}

impl Serialize for StoreError {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let status: String = self.status.to_string();
        let mut state: <S as Serializer>::SerializeStruct =
            serializer.serialize_struct("StoreError", 2)?;
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
