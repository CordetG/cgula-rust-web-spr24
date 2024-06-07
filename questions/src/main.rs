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

use std::collections::{HashMap, HashSet};
use std::io::{Error, ErrorKind};
use std::net::{Ipv4Addr, SocketAddrV4, TcpStream};
use std::str::FromStr;
use std::sync::Arc;

use yew::prelude::*;

mod api;
mod appstate;
mod auth;
mod handler;
pub mod question;
mod startup;
pub mod store;
mod web;
use crate::handler::*;
use crate::question::*;
use store::*;

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
    let socket_addr: SocketAddrV4 = SocketAddrV4::new(localhost, 3040);

    let http_server: Router = Router::new().route("/questions", get(get_questions));

    // run with hyper, listening globally on port 3040
    let listener: tokio::net::TcpListener =
        tokio::net::TcpListener::bind(socket_addr).await.unwrap();
    tracing::debug!("serving {}", listener.local_addr().unwrap());
    axum::serve(listener, http_server).await.unwrap();

    // reqwest with async/await
    let resp: HashMap<String, String> = reqwest::get("https://httpbin.org/ip")
        .await?
        .json::<HashMap<String, String>>()
        .await?;
    println!("{:#?}", resp);
    Ok(())
}

// testing out yew from tutorial
#[function_component(App)]
fn app() -> Html {
    html! {
        <h1>{ "Hello World" }</h1>
    }
}

// main function
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    yew::Renderer::<App>::new().render();

    let store = Store::new();
    let store: Arc<Store> = Arc::new(store);

    let store_filter: Extension<Arc<Store>> = axum::extract::Extension(store.clone());

    let localhost: Ipv4Addr = Ipv4Addr::new(127, 0, 0, 1);
    let socket_addr: SocketAddrV4 = SocketAddrV4::new(localhost, 3040);

    let http_server: Router = Router::new().route("/questions", get(get_questions)).layer(
        CorsLayer::new()
            .allow_origin("http://localhost:3040".parse::<HeaderValue>().unwrap())
            .allow_methods([Method::GET]),
    );

    // run with hyper, listening globally on port 3040
    let listener: tokio::net::TcpListener =
        tokio::net::TcpListener::bind(socket_addr).await.unwrap();
    tracing::debug!("serving {}", listener.local_addr().unwrap());
    axum::serve(listener, http_server).await.unwrap();

    // reqwest with async/await
    let resp: HashMap<String, String> = reqwest::get("https://httpbin.org/ip")
        .await?
        .json::<HashMap<String, String>>()
        .await?;
    println!("{:#?}", resp);
    Ok(())
}
