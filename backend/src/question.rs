// Implementation for ch4 continued from ch3/ to set up RESTful API

#![allow(unused_imports, dead_code, unused_must_use, unused_variables)]
use crate::store::*;
use axum::extract::{self, path, Extension, Path, State};
use axum::{
    http::{HeaderValue, Method, StatusCode},
    response::{IntoResponse, Response},
    routing::{any, get, post},
    Json, Router,
};
use core::convert::Infallible;
use tracing::{event, instrument, Level};

use axum_macros::debug_handler;
use tower_http::compression::CompressionLayer;
use tower_http::cors::CorsLayer;
use tower_http::follow_redirect::policy::PolicyExt;
use tower_http::services::{ServeDir, ServeFile};

use headers::ContentType;
use serde::{Deserialize, Serialize};
use utoipa::{
    openapi::{ObjectBuilder, RefOr, Schema, SchemaType},
    ToSchema,
};
extern crate tracing;

use std::collections::{HashMap, HashSet};
use std::io::{Error, ErrorKind};
use std::net::{Ipv4Addr, SocketAddrV4, TcpStream};
use std::str::FromStr;
use std::sync::Arc;

/*/// The `Question` struct represents a question with an ID, title, content, and optional tags.
///
/// Properties:
///
/// * `id`: The `id` field in the `Question` struct appears to be of type `QuestionId`.
/// * `title`: The `title` property in the `Question` struct represents the title of the question. It is
/// of type `String` and stores the title of the question being asked.
/// * `content`: The `content` property in the `Question` struct represents the main text of the
/// question being asked. It typically contains the details, description, or context related to the
/// question being posed.
/// * `tags`: The `tags` field in the `Question` struct is an `Option` that contains a vector of
/// strings. This means that the `tags` field can either be `Some` with a vector of strings or `None`.
/// It allows for flexibility in cases where a question may or may not have a value.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Question {
    pub id: QuestionId,
    pub title: String,
    pub content: String,
    pub tags: Option<HashSet<String>>,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct QuestionId(pub String);

/// The `impl Question { ... }` block is implementing a method named `new` for the
/// `Question` struct. This method serves as a constructor function for creating new instances of the
/// `Question` struct.
impl Question {
    pub fn new(id: QuestionId, title: &str, content: &str, tags: &[&str]) -> Self {
        let id: QuestionId = id;
        let title: String = title.into();
        let content: String = content.into();
        let tags: Option<HashSet<String>> = if tags.is_empty() {
            None
        } else {
            Some(tags.iter().copied().map(String::from).collect())
        };
        Self {
            id,
            title,
            content,
            tags,
        }
    }
}

/// The `impl std::fmt::Display for Question { ... }` block is implementing the `Display` trait for the
/// `Question` struct. By implementing this trait, you are specifying how instances of the `Question`
/// struct should be formatted when using formatting macros like `println!` or `format!`.
impl std::fmt::Display for Question {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}, title: {}, content: {}, tags: {:?}",
            self.id, self.title, self.content, self.tags
        )
    }
}

/// The `impl std::fmt::Display for QuestionId { ... }` block is implementing the `Display` trait for
/// the `QuestionId` struct. By implementing this trait, you are specifying how instances of the
/// `QuestionId` struct should be formatted when using formatting macros like `println!` or `format!`.
impl std::fmt::Display for QuestionId {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        write!(f, "id: {}", self.0)
    }
}

/// The `impl FromStr for QuestionId { ... }` block is implementing the
/// `FromStr` trait for the `QuestionId` struct. This trait allows for parsing a string into an instance
/// of the specified type, in this case, `QuestionId`.
impl FromStr for QuestionId {
    type Err = std::io::Error;

    fn from_str(id: &str) -> Result<Self, Self::Err> {
        match id.is_empty() {
            false => Ok(QuestionId(id.to_string())),
            true => Err(Error::new(ErrorKind::InvalidInput, "No id provided")),
        }
    }
}*/

/// The function `format_tags` takes a HashSet of strings and returns a formatted string with the tags
/// separated by commas.
///
/// Arguments:
///
/// * `tags`: The `format_tags` function takes a reference to a `HashSet` of `String` values as input.
/// It then converts the `HashSet` into a vector of string references and joins them together with a
/// comma and space to create a single formatted string.
///
/// Returns:
///
/// A formatted string containing the tags from the HashSet, separated by commas.
pub fn format_tags(tags: &HashSet<String>) -> String {
    let taglist: Vec<&str> = tags.iter().map(String::as_ref).collect();
    taglist.join(", ")
}

/*impl From<&Question> for String {
    fn from(question: &Question) -> Self {
        let mut text: String = "Question\n".into();
        text += &format!("{}.\n", question.title);
        text += &format!("\"{}\" who?\n", question.content);
        text += "\n";

        let mut annote: Vec<String> = vec![format!("id: {}", question.id)];
        if let Some(tags) = &question.tags {
            annote.push(format!("tags: {}", format_tags(tags)));
        }
        let annote: String = annote.join("; ");
        text += &format!("[{}]\n", annote);
        text
    }
}

impl IntoResponse for &Question {
    fn into_response(self) -> Response {
        (StatusCode::OK, Json(&self)).into_response()
    }
}*/

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct Question {
    #[schema(example = "1")]
    pub id: String,
    #[schema(example = "How?")]
    pub title: String,
    #[schema(example = "Please help!")]
    pub content: String,
    #[schema(example = r#"["general"]"#)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<HashSet<String>>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct NewQuestion {
    pub title: String,
    pub content: String,
    pub tags: Option<Vec<String>>,
}

impl Question {
    pub fn new(id: &str, title: &str, content: &str, tags: &[&str], source: Option<&str>) -> Self {
        let id: String = id.into();
        let title: String = title.into();
        let content: String = content.into();
        let tags: Option<HashSet<String>> = if tags.is_empty() {
            None
        } else {
            Some(tags.iter().copied().map(String::from).collect())
        };
        Self {
            id,
            title,
            content,
            tags,
        }
    }
}

impl From<&Question> for String {
    fn from(question: &Question) -> Self {
        let mut text: String = "Question:\n".into();
        text += &format!("{}.\n", question.title);
        text += &format!("{}\n", question.content);
        text += "\n";

        let mut annote: Vec<String> = vec![format!("id: {}", question.id)];
        if let Some(tags) = &question.tags {
            annote.push(format!("tags: {}", format_tags(tags)));
        }
        let annote: String = annote.join("; ");
        text += &format!("[{}]\n", annote);
        text
    }
}

impl IntoResponse for &Question {
    fn into_response(self) -> Response {
        (StatusCode::OK, Json(&self)).into_response()
    }
}

#[instrument]
pub async fn get_questions(
    params: HashMap<String, String>,
    store: Store,
) -> Result<Json<Vec<Question>>, Infallible> {
    event!(target: "questions", Level::INFO, "querying questions");
    let mut pagination: Pagination = Pagination::default();

    if !params.is_empty() {
        event!(Level::INFO, pagination = true);
        pagination = extract_pagination(params)?;
        let res: Vec<Question> = store.questions.read().await.values().cloned().collect();
        let res: &[Question] = &res[pagination.start..pagination.end];
        Ok(axum::response::Json(res))
    } else {
        event!(Level::INFO, pagination = false);
        let res: Vec<Question> = match store
            .get_questions(pagination.limit, pagination.offset)
            .await
        {
            Ok(res) => res,
            Err(e) => {
                return Ok(axum::response::Json(warp::reply::json(
                    &Error::DatabaseQueryError(e),
                )));
            }
        };
        Ok(axum::response::Json(res))
    }
}
