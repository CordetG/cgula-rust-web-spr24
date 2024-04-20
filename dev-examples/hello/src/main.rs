use std::todo;
// For empty ID checking
use std::net::{TcpListener, TcpStream};
#[allow(unused_imports)]
use std::{fmt::Error, io::ErrorKind, process::id};

#[allow(unused_variables)]
fn handle_client(stream: TcpStream) {
    todo!();
}

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:80")?;
    for stream in listener.incoming() {
        handle_client(stream?);
    }
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
