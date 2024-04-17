use std::todo;
// For empty ID checking
#[allow(unused_imports)]
use std::{fmt::Error, io::ErrorKind, process::id};

fn main() {
    println!("Hello, world!");
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
