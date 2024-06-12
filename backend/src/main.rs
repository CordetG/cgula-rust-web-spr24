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

use core::net::SocketAddr;
use std::collections::{HashMap, HashSet};
use std::io::{Error, ErrorKind};
use std::net::{Ipv4Addr, SocketAddrV4, TcpStream};
use std::str::FromStr;
use std::sync::Arc;
use tower::ServiceBuilder;
use utoipa::openapi::Server;

use yew::prelude::*;

mod api;
mod appstate;
mod auth;
pub mod question;
mod startup;
pub mod store;
mod web;
use crate::question::*;
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

// main function
#[tokio::main]
async fn main() {
    let log_filter: String = std::env::var("RUST_LOG")
        .unwrap_or_else(|_| "handle_errors=warn,practical_rust_book=warn,warp=warn".to_owned());
    let store: Store = store::Store::new("postgres://localhost:3060/guest").await;
    use mysql_async::{prelude::Queryable, Pool};

    // Assuming `store` is your shared state
    let store_layer: AddExtensionLayer<_> = AddExtensionLayer::new(store);

    let app = Router::new()
        .route(
            "/questions/:id",
            put(question::update_question).delete(question::delete_question),
        )
        .layer(ServiceBuilder::new().layer(store_layer))
        .boxed();

    // Then you can run your app with hyper
    let addr = SocketAddr::from(([127, 0, 0, 1], 3060));
    hyper::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();

    let pool: Pool = Pool::new("mysql://guest:123@localhost:3306/postgres");

    let mut conn: mysql_async::Conn = pool.get_conn().await.unwrap();
    let result: () = conn
        .query_drop("CREATE TABLE users (id INT, name TEXT)")
        .await
        .unwrap();
    startup::startup();
}
