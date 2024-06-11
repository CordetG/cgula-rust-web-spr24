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

## Object Safe

I kept getting errors similar to this:

```sh
the trait `question::_::_serde::ser::Error` cannot be made into an object
the trait cannot be made into an object because it requires `Self: Sized`
for a trait to be "object safe" it needs to allow building a vtable to allow the call to be resolvable dynamically; for more information visit <https://doc.rust-lang.org/reference/items/traits.html#object-safety>
the following types implement the trait, consider defining an enum where each variant holds one of these types, implementing `question::_::_serde::ser::Error` for this new enum and using it instead:
  question::_::_serde::__private::doc::Error
  question::_::_serde::de::value::Error
  serde_json::Error
  serde_urlencoded::ser::Error
  serde_wasm_bindgen::error::Error
  std::boxed::Box<bincode::error::ErrorKind>
  std::fmt::Error
```

I had included a code snippet -- asking chatpgt for help -- which I found completely useless, but no surprise there. I realized that reading this error discussing building a `vtable` for object-safety actually helped with understanding how the knock-knock repo was utilizing tables. I can't say -- even though they are rather frustrating -- that gettings errors aren't helpful with learning. I often seem to learn more in the process of failing than doing something successfully by accident.

~~Solution:~~ Interestingly, apparently the multitudes of this form of error was a result of missing a crate feature. Once I added `tower_http -- features = add_extension` to the `Cargo.toml` All the errors disappeared. It's safe to say that understanding the libraries you are working with a huge factor in the coding process.

Well, nevermind. running `cargo check` -- clippy mentioned it is not a feature. ugh. But adding it to the file did not fix an error saying it needed to be added. Lovely. I guess I'm just gunna `cargo clean` and keep working through it.

```sh
error: failed to select a version for `tower-http`.
    ... required by package `questions v0.1.2 (/git_local/rust_web/questions)`
versions that meet the requirements `^0.5.2` (locked to 0.5.2) are: 0.5.2

the package `questions` depends on `tower-http`, with features: `add_extension` but `tower-http` does not have these features.


failed to select a version for `tower-http` which could resolve this conflict
```

<!-- trunk-ignore(markdownlint/MD034) -->
So, I did the logical thing and looked at the docs again https://docs.rs/tower-http/latest/tower_http/. I then added some of the recommended crates in the examples. Thank goodness for `cargo add <crate>` -- it makes things a lot easy to add to `Cargo.toml`.

## Trait Bounds

I am so tired of the `trait-bounds` errors, her is one of many:

```sh
the trait bound `std::collections::HashSet<std::string::String>: sqlx::Decode<'_, sqlx::Postgres>` is not satisfied
the following other types implement trait `sqlx::Decode<'r, DB>`:
  <bool as sqlx::Decode<'r, sqlx::Any>>
  <bool as sqlx::Decode<'_, sqlx::Postgres>>
  <i8 as sqlx::Decode<'_, sqlx::Postgres>>
  <i16 as sqlx::Decode<'r, sqlx::Any>>
  <i16 as sqlx::Decode<'_, sqlx::Postgres>>
  <i32 as sqlx::Decode<'r, sqlx::Any>>
  <i32 as sqlx::Decode<'_, sqlx::Postgres>>
  <i64 as sqlx::Decode<'r, sqlx::Any>>
and 41 others
required for `std::option::Option<std::collections::HashSet<std::string::String>>` to implement `sqlx::Decode<'_, sqlx::Postgres>`
```

It just seems like a rabbit-hole of `"implement everything for everything"`. So I am ignoring them for now and hope that as I continue to got through my code, it ceases to be an issue. Otherwise, I will look more into it.

## Formatting

For the `format_tags` function, I kept getting an error when calling it from within `map` with advice to wrap in a closure due to mismatched types:

`consider wrapping the function in a closure:`|arg0: &std::vec::Vec<std::string::String>| `,`(/*&std::collections::HashSet<std::string::String>*/)`

I originally attempted to create the closure, but it just made even more issues. So I worked with changing the types I was working with. Using the joke-repo as a reference, I realized that part of my confusion was with the naming scheme. A `tag` in one module vs. a different module are seperate entities, but may reference each-other --> so, I lost track of the types being used. Eventually, I was able to track through it. 

But, on the positive side, this is a good argument for why comments are important. It's also pretty magical having errors yell at you and then when the issue is fixed how they all disappear --> This is a good feeling.

I will have to say I disagree with the rust formatting to shadow the same names -- this often causes me to get my "wires-crossed" so-to-speak and mixing them up is one of my biggest issues, especially the more code that is added.

## Panic

```zsh
thread 'main' panicked at .cargo/registry/src/index.crates.io-6f17d22bba15001f/js-sys-0.3.69/src/lib.rs:6013:9:
cannot call wasm-bindgen imported functions on non-wasm targets
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
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

```zsh
Ensure that HEALTHCHECK instructions have been added to container images (Trunk)
```

Trunk is getting on my nerves. Half the errors are from trunk alone -- talk about micro-managing.

---

### Terminal

When I officially switched from `bash` to `zsh` I kept getting errors with cargo and rustup commands. I noticed my path in bash included the following command:

```vim
export PATH="$HOME/.cargo/bin:$PATH"
```

To confirm: [This Stack Overflow issue](https://stackoverflow.com/questions/67656028/rustup-gives-command-not-found-error-with-zsh-even-after-installing-with-brew) confirmed. Once I updated the path, everything worked fine.

### Docker

I kept getting a pop-up window for entering a passphrase for the gpg key. Problem is, I don't recall ever entering a passphrase and I'm usually pretty good at keeping track of those kinds of things. Apparently I am not the only one: [See the forum.](https://forums.docker.com/t/enter-the-passphrase-to-unlock-the-openpgp-secret-key/134700/7)

So, per the last comment in the forum I went to the page for [gpg-keys for linux users](https://docs.docker.com/desktop/get-started/#credentials-management-for-linux-users). I deleted my current keys and started from scratch with the aformentioned page.

Apparently, there were still problems so I completely unistalled docker using a couple references to do so:
[completely uninstall docker](https://www.benjaminrancourt.ca/how-to-completely-uninstall-docker/) -- then went ahead and followed the installation process.. again.

Because I was annoyed, I made a script with all the commands because I didn't want to do everything by hand again. 

I recieved the following error + output:

```zsh
Failed to start docker-desktop.service: Process org.freedesktop.systemd1 exited with status 1
See user logs and 'systemctl --user status docker-desktop.service' for details.

docker versions:
docker compose:
    Docker Compose version v2.27.0-desktop.2
docker --version:
    Docker version 26.1.4, build 5650f9b
docker version:
    Cannot connect to the Docker daemon at unix:///var/run/docker.sock. Is the docker daemon running?
    Client: Docker Engine - Community
    Cloud integration: v1.0.35+desktop.13
    Version:           26.1.4
    API version:       1.45
    Go version:        go1.21.11
    ...
```

So it installed successfully, but wouldn't start. I believe I had an issue starting docker desktop from the terminal previously as well, so I am going to start it from the GUI.

Well -- after all that, docker did not fully uninstall and now I can't even generate a new gpg key and trying to sign in results in an error with a ticket to support. So -- I give up on docker.

### Technical Difficulties

There are pros and cons of using an IDE. Pros is that there are feature that can assist with development. Cons is that it can break things.
I utilize mintlify doc writer to help with writing decent doc-comments, but everytime vscode has an update the extension breaks.

The following error was from the rust-analyzer where I added both the frontend and backend as dependencies to each other. Turns out the I just needed to add one as a dependency to the other. I wasn't sure which I should utilize, but it seems that adding just one as a dependency allows the other to use the other crate as well.

```sh
error: cyclic package dependency: package `backend v0.1.2 (.../git_local/rust_web/backend)` depends on itself.
```
