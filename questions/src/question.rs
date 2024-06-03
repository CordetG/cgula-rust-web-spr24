// Implementation for ch4 continued from ch3/ to set up RESTful API

#![allow(unused_imports, dead_code, unused_must_use, unused_variables)]
use axum::extract::{self, path, Extension, Path, State};
use axum::{
    http::{HeaderValue, Method, StatusCode},
    response::{IntoResponse, Response},
    routing::{any, get, post},
    Json, Router,
};

use axum_macros::debug_handler;
use tower_http::compression::CompressionLayer;
use tower_http::cors::CorsLayer;
use tower_http::follow_redirect::policy::PolicyExt;
use tower_http::services::{ServeDir, ServeFile};

use headers::ContentType;
use serde::{Deserialize, Serialize};
extern crate tracing;

use std::collections::HashMap;
use std::io::{Error, ErrorKind};
use std::net::{Ipv4Addr, SocketAddrV4, TcpStream};
use std::str::FromStr;
use std::sync::Arc;

/// The `Question` struct represents a question with an ID, title, content, and optional tags.
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
    pub tags: Option<Vec<String>>,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct QuestionId(pub String);

/// The `impl Question { ... }` block is implementing a method named `new` for the
/// `Question` struct. This method serves as a constructor function for creating new instances of the
/// `Question` struct.
impl Question {
    pub fn new(id: QuestionId, title: String, content: String, tags: Option<Vec<String>>) -> Self {
        Question {
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
}
