# Error Notes

This file is to log error output that occurs for personal understanding, debugging, and development.

## Chapter 1

The code snippet on ch. 1 page 80 had a typo that appeared to be fixed in the [minimal reqwest example repo](https://github.com/Rust-Web-Development/code/blob/main/ch_01/minimal_reqwest/src/main.rs).

Code snippet: Listing 1.12 Sending HTTP GET requests asynchronously in Rust

```rust
// ch_01/minimal_reqwest/src/main.rs
// https://github.com/Rust-Web-Development/code/tree/main/ch_01/minimal_reqwest
 
use std::collections::HashMap;
 
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let resp = reqwest::get("https://httpbin.org/ip")
        .await?; // Error with ';' here.
        .json::<HashMap<String, String>>()
        .await?;
    println!("{:#?}", resp);
    Ok(())
}

```

```zsh
# This error occured from the typo in the book

$ cargo build --release

error: expected expression, found `.`
  --> src/main.rs:28:9
   |
28 |         .json::<HashMap<String, String>>()
   |         ^ expected expression

```
