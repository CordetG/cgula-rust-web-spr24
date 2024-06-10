use crate::*;
use appstate::AppState;
use bytes::Bytes;
use http::{header::USER_AGENT, HeaderValue, Request};
use http_body_util::Full;
use hyper_util::{client::legacy::Client, rt::TokioExecutor};
use serde_urlencoded::ser;
use serde_wasm_bindgen::Error;
use sqlx::error;
use std::fmt;
use std::sync::Arc;
use tower::{Service, ServiceBuilder, ServiceExt};
use tower_http::{
    classify::StatusInRangeAsFailures, decompression::DecompressionLayer,
    set_header::SetRequestHeaderLayer, trace::TraceLayer,
};

/*// ChatGPT help
enum CustomSerError {
    SerdeDocError(serde::__private::doc::Error),
    SerdeDeValueError(serde::de::value::Error),
    SerdeJsonError(serde_json::Error),
    SerdeUrlencodedError(serde_urlencoded::ser::Error),
    SerdeWasmBindgenError(serde_wasm_bindgen::Error),
    BincodeError(Box<error::ErrorKind>),
    FmtError(std::fmt::Error),
}

impl fmt::Display for CustomSerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Implement formatting logic here
        match self {
            CustomSerError::SerdeDocError(err) => write!(f, "Serde Doc Error: {}", err),
            CustomSerError::SerdeDeValueError(err) => write!(f, "Serde De Value Error: {}", err),
            CustomSerError::SerdeJsonError(err) => write!(f, "Serde JSON Error: {}", err),
            CustomSerError::SerdeUrlencodedError(err) => {
                write!(f, "Serde URL Encoded Error: {}", err)
            }
            CustomSerError::SerdeWasmBindgenError(err) => {
                write!(f, "Serde Wasm Bindgen Error: {}", err)
            }
            CustomSerError::BincodeError(err) => write!(f, "Bincode Error: {:?}", err),
            CustomSerError::FmtError(err) => write!(f, "Format Error: {}", err),
        }
    }
}

// Implement serde::ser::Error for CustomSerError
impl serde::ser::Error for CustomSerError {
    fn custom<T: fmt::Display>(msg: T) -> Self {
        CustomSerError::FmtError(fmt::Error::new(ErrorKind::Other, format!("{}", msg)))
    }
}*/

pub async fn startup() -> Result<(), Box<dyn std::error::Error>> {
    //yew::Renderer::<App>::new().render();

    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "question=debug,info".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();
    // https://carlosmv.hashnode.dev/adding-logging-and-tracing-to-an-axum-app-rust
    let trace_layer: trace::TraceLayer<
        tower_http::classify::SharedClassifier<tower_http::classify::ServerErrorsAsFailures>,
    > = trace::TraceLayer::new_for_http()
        .make_span_with(trace::DefaultMakeSpan::new().level(tracing::Level::INFO))
        .on_response(trace::DefaultOnResponse::new().level(tracing::Level::INFO));

    let session_store: MemoryStore = MemoryStore::default();
    let session_layer: SessionManagerLayer<MemoryStore> = SessionManagerLayer::new(session_store)
        .with_secure(false)
        .with_expiry(Expiry::OnSessionEnd);

    /* let store: Store = Store::new()
    .await
    .unwrap_or_else(|err: Box<dyn Error>| -> Store {
        tracing::error!("store: {}", err);
        std::process::exit(1);
    });*/

    let store = Store::new();
    let store: Arc<_> = Arc::new(store);

    // Create a new Axum router
    let app = Router::new()
        // Define GET /questions route
        .route(
            "/questions",
            get(Store::get_questions.clone().into_service()),
        )
        // Define POST /questions route
        .route("/questions", post(add_question.clone().into_service()))
        // Define PUT /questions/:id route
        .route(
            "/questions/:id",
            put(update_question.clone().into_service()),
        )
        // Add error recovery middleware
        .recover(return_error);

    let service = ServiceBuilder::new()
        .layer(AddExtension::new(store))
        .service(app);

    let store: Store = Store::new();
    let store_clone: Store = store.clone();
    let store_arc: Arc<Store> = Arc::new(store);

    let store_filter = axum::extract::Extension(store_arc);

    let localhost: Ipv4Addr = Ipv4Addr::new(127, 0, 0, 1);
    let socket_addr: SocketAddrV4 = SocketAddrV4::new(localhost, 3060);

    let cors = cors::CorsLayer::new()
        .allow_methods([Method::GET])
        .allow_origin(cors::Any);

    let http_server: Router = Router::new()
        .route("/questions", get(Store::get_questions))
        .layer(
            CorsLayer::new()
                .allow_origin("http://localhost:3060".parse::<HeaderValue>().unwrap())
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
