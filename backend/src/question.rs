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
use std::ops::Add;
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

use sqlx::{FromRow, Row};
use std::collections::{HashMap, HashSet};
use std::io::{Error, ErrorKind};
use std::net::{Ipv4Addr, SocketAddrV4, TcpStream};
use std::str::FromStr;
use std::sync::Arc;

use std::fmt;

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct QuestionId(pub i32);

impl fmt::Display for QuestionId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct Question {
    #[schema(example = "1")]
    pub id: QuestionId,
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

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct AnswerId(pub i32);

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Answer {
    id: AnswerId,
    content: String,
    question_id: QuestionId,
}

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

impl Question {
    pub fn new(
        id: QuestionId,
        title: &str,
        content: &str,
        tags: &[&str],
        source: Option<&str>,
    ) -> Self {
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


