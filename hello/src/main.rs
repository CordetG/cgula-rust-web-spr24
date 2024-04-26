use std::net::{Ipv4Addr, SocketAddrV4, TcpStream};
use std::todo;
// For empty ID checking
use axum::{routing::get, Router};
use std::collections::HashMap;
#[allow(unused_imports)]
use std::{fmt::Error, io::ErrorKind, process::id};

#[allow(unused_variables)]
#[allow(dead_code)]
fn handle_client(stream: TcpStream) {
    todo!();
}

// ch_01/minimal_reqwest/src/main.rs
// https://github.com/Rust-Web-Development/code/tree/main/ch_01/minimal_reqwest

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let localhost = Ipv4Addr::new(127, 0, 0, 1);
    let socket_addr = SocketAddrV4::new(localhost, 3000);
    // http example
    /*let listener = TcpListener::bind("127.0.0.1:80")?;
    for stream in listener.incoming() {
        handle_client(stream?);
    }
    Ok(())*/

    // axum crate example -- administered to ch.1 example
    let http_server = Router::new().route("/", get(|| async { "Hello, World!" }));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind(socket_addr).await.unwrap();
    axum::serve(listener, http_server).await.unwrap();

    // reqwest with async/await
    let resp = reqwest::get("https://httpbin.org/ip")
        .await?
        .json::<HashMap<String, String>>()
        .await?;
    println!("{:#?}", resp);
    Ok(())
}

#[allow(dead_code)]
fn err_id_check() {
    todo!("Check for empty ID not fully implemented");
    /* match id.is_empty() {
        false => Ok(QuestionId(id.to_string())),
        true => Err(Error::new(ErrorKind::InvalidInput, "No id provided")),
    }
    */
}
