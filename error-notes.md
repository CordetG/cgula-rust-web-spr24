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

Once I added the macro `#[debug_handler]` -- and added the proper package version in the Cargo.toml -- I finally got a readable error. 

Of course, it was something obvious that I overlooked -- I forgot to impl IntoResponse for the ApiError. Heh!

Thank goodness for good documentation. 

After implementing ApiError:

```zsh
thread 'main' panicked at src/main.rs:190:58:
called `Result::unwrap()` on an `Err` value: Os { code: 98, kind: AddrInUse, message: "Address already in use" }
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

I realized that my hello/ package used port 3000, so I changed the port to 3080 and that fixed the issue.

## Chapter 4

```zsh
# Error occured when implimenting fn init() -> HashMap {...} from the book
missing generics for struct `std::collections::HashMap`
expected at least 2 generic arguments
```

Fix: Changed the return type to include the generic arguments.

```rust
    fn init() -> HashMap<QuestionId, Question> {
        let file: &str = include_str!("../questions.json");
        serde_json::from_str(file).expect("can't read questions.json")
    }
```

## Miscellaneous Errors

### Rust-analyzer

I kept having issues with rust-analyzer. I figured out that because I set up the repo with separate chapters as separate packages, I had to update the workspace linked projects to point to each project's Cargo.toml file.

---

### Trunk

Trunk is a rather handy extension for VS Code that includes lints (such as rustfmt) and various other tools all in one place. I used it briefly, but I think it had issues with some of my other settings. For example, 'Prettier' would override `rustfmt` and even after disabling it, I would get `rustfmt` errors that I couldn't pin down or ever resolve.

I uninstalled it for now, but I think I need to configure `rust-analyzer` and `trunk` settings to play nicely with each other.

-- update --

Well, now we are using trunk so I have to re-install it *heh*.

Upon reinstalling, I kept getting a trunk error. The solution: an improper setting. I was trying to restrict trunk to only rust settings, but misunderstood what I was changing. When I removed that setting, trunk connected successfully.

---

### Terminal

When I officially switched from `bash` to `zsh` I kept getting errors with cargo and rustup commands. I noticed my path in bash included the following command:

```vim
export PATH="$HOME/.cargo/bin:$PATH"
```

To confirm: [This Stack Overflow issue](https://stackoverflow.com/questions/67656028/rustup-gives-command-not-found-error-with-zsh-even-after-installing-with-brew) confirmed. Once I updated the path, everything worked fine.
