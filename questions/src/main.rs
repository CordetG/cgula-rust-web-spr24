// Implementation for ch4 continued from ch3/ to set up RESTful API

#![allow(unused_imports, dead_code, unused_must_use, unused_variables)]
use axum::extract::{self, path, Extension, Path, State};
use axum::{
    async_trait,
    extract::FromRequestParts,
    http::HeaderValue,
    http::{request::Parts, Method, StatusCode},
    response::Response,
    response::{IntoResponse, Redirect},
    routing::{any, get, post},
    routing::{delete, put},
    Json, RequestPartsExt, Router,
};

use sqlx::{
    self,
    postgres::{PgConnection, PgPool, PgRow, Postgres},
    Pool, Row,
};

use axum_macros::debug_handler;
use tower_http::compression::CompressionLayer;
use tower_http::cors::CorsLayer;
use tower_http::follow_redirect::policy::PolicyExt;
use tower_http::services::{ServeDir, ServeFile};
use tower_http::{cors, services, trace};
use tower_sessions::{Expiry, MemoryStore, Session, SessionManagerLayer};
extern crate tracing;
use tokio::{self, sync::RwLock};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use headers::ContentType;
use serde::{Deserialize, Serialize};

use std::collections::{HashMap, HashSet};
use std::io::{Error, ErrorKind};
use std::net::{Ipv4Addr, SocketAddrV4, TcpStream};
use std::str::FromStr;
use std::sync::Arc;

use yew::prelude::*;

mod api;
mod appstate;
mod auth;
pub mod question;
mod startup;
pub mod store;
mod web;
use crate::question::*;
use store::*;

/// The line `const STYLESHEET: &str = "css/question.css";` is declaring a constant named `STYLESHEET`
/// with a value of the string `"css/question.css"`. This constant is of type `&str`, which is a string
/// slice that points to a sequence of UTF-8 bytes in memory.
const STYLESHEET: &str = "css/question.css";

// testing out yew from tutorial
#[function_component(App)]
fn app() -> Html {
    html! {
        <h1>{ "Hello World" }</h1>
    }
}

// main function
#[tokio::main]
async fn main() {
    startup::startup();
}
