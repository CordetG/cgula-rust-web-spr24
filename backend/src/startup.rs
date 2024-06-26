use crate::auth::make_jwt_keys;
use crate::auth::read_secret;
use crate::store::Store;
use crate::*;
use appstate::AppState;
use axum::extract::FromRequest;
use bytes::Bytes;
use core::convert::Infallible;
use http::{header::USER_AGENT, HeaderValue, Request};
use http_body_util::Full;
use hyper_util::{client::legacy::Client, rt::TokioExecutor};
use serde_urlencoded::ser;
use serde_wasm_bindgen::Error;
use sqlx::error;
use std::fmt;
use std::sync::Arc;
use tower::{Service, ServiceBuilder, ServiceExt};
use tower_http::add_extension::{AddExtension, AddExtensionLayer};
use tower_http::{
    classify::StatusInRangeAsFailures, decompression::DecompressionLayer,
    set_header::SetRequestHeaderLayer, trace::TraceLayer,
};
use utoipa::OpenApi;
use utoipa_rapidoc::RapiDoc;
use utoipa_redoc::Redoc;
use utoipa_redoc::Servable;
use utoipa_swagger_ui::SwaggerUi;

// Define an async handler function for Axum
async fn handler_404() -> Response {
    (StatusCode::NOT_FOUND, "404 Not Found").into_response()
}

pub const SESSION_ERROR_KEY: &str = "session_error";

pub async fn startup(ip: String) {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "question=debug,info".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();
    // https://carlosmv.hashnode.dev/adding-logging-and-tracing-to-an-axum-app-rust
    let trace_layer = trace::TraceLayer::new_for_http()
        .make_span_with(trace::DefaultMakeSpan::new().level(tracing::Level::INFO))
        .on_response(trace::DefaultOnResponse::new().level(tracing::Level::INFO));

    let session_store = MemoryStore::default();
    let session_layer = SessionManagerLayer::new(session_store)
        .with_secure(false)
        .with_expiry(Expiry::OnSessionEnd);

    use std::env::var;

    //let password = read_secret("PG_PASSWORDFILE").await?;

    let url = "localhost:5432";

    let jokebase: Store = Store::new(url).await;
    /*{
        tracing::error!("jokebase: {}", e);
        std::process::exit(1);
    };*/

    let jwt_keys = make_jwt_keys().await.unwrap_or_else(|_| {
        tracing::error!("jwt keys");
        std::process::exit(1);
    });

    let reg_key = read_secret("REG_PASSWORD").await.unwrap_or_else(|_| {
        tracing::error!("reg password");
        std::process::exit(1);
    });

    let state = Arc::new(RwLock::new(AppState::new(jokebase, jwt_keys, reg_key)));

    let cors = cors::CorsLayer::new()
        .allow_methods([Method::GET])
        .allow_origin(cors::Any);

    let mime_type = core::str::FromStr::from_str("image/vnd.microsoft.icon").unwrap();
    let favicon = services::ServeFile::new_with_mime("assets/static/favicon.ico", &mime_type);

    let mime_type = core::str::FromStr::from_str("text/css").unwrap();
    let stylesheet = services::ServeFile::new_with_mime(STYLESHEET, &mime_type);

    let apis = Router::new()
        .route("/questions", get(questions))
        .route("/question", get(question))
        .route("/question/:id", get(get_question))
        .route("/question/add", post(post_question))
        .route("/question/:id", delete(delete_question))
        .route("/question/:id", put(update_question))
        .route("/register", get(register));

    let swagger_ui = SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi());
    let redoc_ui = Redoc::with_url("/redoc", ApiDoc::openapi());
    let rapidoc_ui = RapiDoc::new("/api-docs/openapi.json").path("/rapidoc");

    let app = Router::new()
        //.route("/", get(handler_index))
        //.route("/index.html", get(handler_index))
        //.route("/tell", get(handler_tell))
        //.route("/add", get(handler_add))
        .route_service("/index.css", stylesheet)
        //.route_service("/favicon.ico", favicon)
        .merge(swagger_ui)
        .merge(redoc_ui)
        .merge(rapidoc_ui)
        .nest("/api/v1", apis)
        .fallback(handler_404)
        .layer(cors)
        .layer(session_layer)
        .layer(trace_layer)
        .with_state(state);

    let listener = tokio::net::TcpListener::bind(ip).await.unwrap();
    tracing::debug!("serving {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}
