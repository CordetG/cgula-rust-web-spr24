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

use crate::question::*;

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
    questions: HashMap<QuestionId, Question>,
}

impl Store {
    /// The function `new` initializes a `Store` struct with questions initialized using the `init` method.
    ///
    /// Returns:
    ///
    /// An instance of the `Store` struct is being returned.
    pub fn new() -> Self {
        Store {
            questions: Self::init(),
        }
    }

    /// The `init` function reads questions from a JSON file and returns them as a HashMap.
    ///
    /// Returns:
    ///
    /// A `HashMap` containing `QuestionId` as keys and `Question` as values is being returned.
    fn init() -> HashMap<QuestionId, Question> {
        let file: &str = include_str!("../questions.json");
        serde_json::from_str(file).expect("can't read questions.json")
    }
}

impl Default for Store {
    fn default() -> Self {
        Self::new()
    }
}
