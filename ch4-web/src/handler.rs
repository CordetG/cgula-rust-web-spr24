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
