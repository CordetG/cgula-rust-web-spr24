// Chapter 3 - Setting up Questions

#![allow(unused_imports, dead_code, unused_must_use)]
use axum::extract::{self, path, Extension, Path, State};
use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::{get, post},
    Json, Router,
};
use axum_macros::debug_handler;
use headers::ContentType;
use serde::Serialize;
use std::collections::HashMap;
use std::io::{Error, ErrorKind};
use std::net::{Ipv4Addr, SocketAddrV4, TcpStream};
use std::str::FromStr;
use std::todo;

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
#[derive(Debug, Clone, Serialize)]
pub struct Question {
    id: QuestionId,
    title: String,
    content: String,
    tags: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
struct QuestionId(String);

/// The `impl Question { ... }` block is implementing a method named `new` for the
/// `Question` struct. This method serves as a constructor function for creating new instances of the
/// `Question` struct.
impl Question {
    fn new(id: QuestionId, title: String, content: String, tags: Vec<String>) -> Self {
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

// Implementing Axum 'IntoResponse' from shuttle.rs but with the Serialized Question
enum ApiResponse {
    OK,
    Created,
    JsonData(Question),
}

// To return a result, implement an error type
enum ApiError {
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

impl IntoResponse for ApiError {
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
async fn get_questions() -> Result<ApiResponse, ApiError> {
    let question: Question = Question::new(
        QuestionId::from_str("1").expect("No id provided"),
        "First Question".to_string(),
        "Content of question".to_string(),
        vec!["faq".to_string()],
    );
    match question.id.0.parse::<i32>() {
        Err(_) => Err(ApiError::NotFound),
        Ok(_) => Ok(ApiResponse::JsonData(question)),
    }
}

/// The function `init_router` sets up a web server using Axum in Rust, listens on port 3000, and makes
/// an async request to https://httpbin.org/ip using reqwest.
///
/// Returns:
///
/// The `init_router` function returns a `Result<(), Box<dyn std::error::Error>>`. This means that it
/// can either return `Ok(())` indicating that the function executed successfully without any errors, or
/// it can return an `Err` containing a boxed error trait object that implements the `std::error::Error`
/// trait in case of any errors occurring during the execution of the function.
async fn init_router() -> Result<(), Box<dyn std::error::Error>> {
    let localhost: Ipv4Addr = Ipv4Addr::new(127, 0, 0, 1);
    let socket_addr: SocketAddrV4 = SocketAddrV4::new(localhost, 3000);

    let http_server: Router = Router::new().route("/", get(get_questions));
    // run with hyper, listening globally on port 3000
    let listener: tokio::net::TcpListener =
        tokio::net::TcpListener::bind(socket_addr).await.unwrap();

    axum::serve(listener, http_server).await.unwrap();

    // reqwest with async/await
    let resp: HashMap<String, String> = reqwest::get("https://httpbin.org/ip")
        .await?
        .json::<HashMap<String, String>>()
        .await?;
    println!("{:#?}", resp);
    Ok(())
}

#[tokio::main]
async fn main() {
    init_router().await;
}
