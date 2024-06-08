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
use serde::{Deserialize, Serialize};
extern crate tracing;

use std::collections::HashMap;
use std::io::{Error, ErrorKind};
use std::net::{Ipv4Addr, SocketAddrV4, TcpStream};
use std::str::FromStr;
use std::sync::Arc;

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
