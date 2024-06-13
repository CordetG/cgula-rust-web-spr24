// Implementation for ch4 continued from ch3/ to set up RESTful API

#![allow(unused_imports, dead_code, unused_must_use, unused_variables)]
use crate::api::*;
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
use tower::ServiceBuilder;

use tower::ServiceExt;

use axum_macros::debug_handler;
use tower_http::add_extension::AddExtensionLayer;
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

use axum::handler::Handler;
use core::net::SocketAddr;
use std::collections::{HashMap, HashSet};
use std::io::{Error, ErrorKind};
use std::net::{Ipv4Addr, SocketAddrV4, TcpStream};
use std::str::FromStr;
use std::sync::Arc;
use utoipa::openapi::Server;
use yew::prelude::*;

mod api;
mod appstate;
mod auth;
mod error;
mod routes;
mod startup;
mod store;
mod types;
mod web;
use crate::routes::question::get_questions;
use crate::store::*;

use log::{debug, error, info, warn};

/*info!("User {} logged in", user.id);
warn!("User {} logged in {} times", user.id, login_count);
err!("Failed to load User {} from DB", user.id);
debug!(
    "User {} access controls: {}, {}",
    user.id, user.admin, user.supervisor
);*/

/// The line `const STYLESHEET: &str = "css/question.css";` is declaring a constant named `STYLESHEET`
/// with a value of the string `"css/question.css"`. This constant is of type `&str`, which is a string
/// slice that points to a sequence of UTF-8 bytes in memory.
const STYLESHEET: &str = "../../frontend/index.css";

// testing out yew from tutorial
#[function_component(App)]
fn app() -> Html {
    html! {
        <h1>{ "Hello World" }</h1>
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let log_filter: String = std::env::var("RUST_LOG")
        .unwrap_or_else(|_| "handle_errors=warn,backend=warn,axum=warn".to_owned());
    let store: Store = Store::new();
    let store_clone: Store = store.clone();
    let store_arc: Arc<Store> = Arc::new(store);

    let store_filter: Extension<Arc<Store>> = axum::extract::Extension(store_arc);

    let localhost: Ipv4Addr = Ipv4Addr::new(127, 0, 0, 1);
    let socket_addr: SocketAddrV4 = SocketAddrV4::new(localhost, 3060);

    let cors: CorsLayer = cors::CorsLayer::new()
        .allow_methods([Method::GET])
        .allow_origin(cors::Any);

    let http_server: Router = Router::new()
        .route(
            "/backend",
            get(get_questions)
                .route("/questions", post(add_question))
                .route("/questions/:id", put(update_question))
                .route("/questions/:id", delete(delete_question)),
        )
        .layer(
            CorsLayer::new()
                .allow_origin("http://localhost:3060".parse::<HeaderValue>().unwrap())
                .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE]),
        );

    // run with hyper, listening globally on port 3060
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

    /*let pool: Pool = Pool::new("mysql://guest:123@localhost:3306/postgres");

    let mut conn: mysql_async::Conn = pool.get_conn().await.unwrap();
    let result: () = conn
        .query_drop("CREATE TABLE users (id INT, name TEXT)")
        .await
        .unwrap();*/
    //startup::startup();
}
