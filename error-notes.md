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

## Chapter 3

Kept getting Handler errors when trying to use `.get(get_questions)`. Luckily the axum crate mentioned this in the [Debugging handler type errors Section](https://docs.rs/axum/latest/axum/handler/index.html).

The crate mentions the following:

```text
This error doesn’t tell you why your function doesn’t implement Handler. It’s possible to improve the error with the debug_handler proc-macro from the axum-macros crate.
```

Once I added the macro `#[debug_handler]` it resolved the issue! Thank goodness for good documentation. 

## Miscellaneous Errors

### Rust-analyzer

I kept having issues with rust-analyzer. I figured out that because I set up the repo with separate chapters as separate packages, I had to update the workspace linked projects to point to each project's Cargo.toml file.

---

### Trunk

Trunk is a rather handy extension for VS Code that includes lints (such as rustfmt) and various other tools all in one place. I used it briefly, but I think it had issues with some of my other settings. For example, 'Prettier' would override `rustfmt` and even after disabling it, I would get `rustfmt` errors that I couldn't pin down or ever resolve.

I uninstalled it for now, but I think I need to configure `rust-analyzer` and `trunk` settings to play nicely with each other.

---

### Terminal

When I officially switched from `bash` to `zsh` I kept getting errors with cargo and rustup commands. I noticed my path in bash included the following command:

```vim
export PATH="$HOME/.cargo/bin:$PATH"
```

To confirm: [This Stack Overflow issue](https://stackoverflow.com/questions/67656028/rustup-gives-command-not-found-error-with-zsh-even-after-installing-with-brew) confirmed. Once I updated the path, everything worked fine.
