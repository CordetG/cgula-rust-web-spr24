use std::todo;
#[allow(unused_imports)]
use std::net::{TcpListener, TcpStream};
// For empty ID checking
#[allow(unused_imports)]
use std::{fmt::Error, io::ErrorKind, process::id};
use std::collections::HashMap;

#[allow(unused_variables)]
#[allow(dead_code)]
fn handle_client(stream: TcpStream) {
    todo!();
}

// ch_01/minimal_reqwest/src/main.rs
// https://github.com/Rust-Web-Development/code/tree/main/ch_01/minimal_reqwest
 
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // http example
    /* let listener = TcpListener::bind("127.0.0.1:80")?;
    for stream in listener.incoming() {
        handle_client(stream?);
    }
    Ok(())*/

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
