use crate::*;

pub async fn startup() -> Result<(), Box<dyn std::error::Error>> {
    //yew::Renderer::<App>::new().render();

    let store: Store = Store::new();
    let store: Arc<Store> = Arc::new(store);

    let store_filter: Extension<Arc<Store>> = axum::extract::Extension(store.clone());

    let localhost: Ipv4Addr = Ipv4Addr::new(127, 0, 0, 1);
    let socket_addr: SocketAddrV4 = SocketAddrV4::new(localhost, 3080);

    let http_server: Router = Router::new()
        .route("/questions", get(Store::get_questions))
        .layer(
            CorsLayer::new()
                .allow_origin("http://localhost:3080".parse::<HeaderValue>().unwrap())
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
