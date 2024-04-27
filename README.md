# Rust Web Examples

Cordet Gula  
CS510 Rust Web Dev SPR 2024  
MCECS  
Professor: Bart Massey

## About

Rust Web Example is a course repo dedicated to Rust Web Development. The focus of this repo will be demonstrating web dev skills [*in rust*], such as providing a web service, REST API, backend, frontend, and more. 

## Assignments

To make access to the specific assignments easier, I made this guide to point to the corresponding package in repo.

Chapter 4: Implement a RESTful API  
['Get The REST Up' package](./ch4-web)  
[Assignment README documented section](#chapter-4)  

To see the server working from chapter 3 for a question: see [ch3-web](#chapter-3)

## <!-- End of Get-the-REST-Up -->

## Setup

```zsh
# Setup Chapter as Package
$ Cargo new ch*-web

# Setting up tokio:
$ cargo add tokio --features full

# Setting up header
$ cargo add axum-extra -F typed-header  
```

### Repo

```text
.
└── hello/ # Chapter 1
    └── hello v0.1.0
        ├── axum v0.7.5
        │   [build-dependencies]
        ├── reqwest v0.12.4
        └── tokio v1.37.0
├── ch2-web/
    └── ch2-web v0.1.0
        ├── axum v0.7.5
        │   [build-dependencies]
        └── tokio v1.37.0
├── ch3-web/
    └── ch3-web v0.1.0
        ├── axum v0.7.5
        │   [build-dependencies]
        ├── reqwest v0.12.4
        ├── serde v1.0.198
        └── tokio v1.37.0
├── ch4-web/
    └── ch4-web v0.1.0
├── assets/
├── .gitignore
├── error-notes.md
├── README
└── LICENSE
```

Each chapter is set up as a separate rust binary package. The reason why I implemented the repo this way was because it helps me conceptualize how the chapters from the book build on each other. It also allows me to reference prior chapters' examples. 

I could have probably set it up differently, such as a single package with multiple binaries and/or modules, but I went about it as a multi-package project instead. 

To make sure the rust-analyzer server could build my project(s) properly, I set the workspace ```settings.json``` to link the specific packages:

```json
{
    "docwriter.custom.author": "Cordet Gula",
    "docwriter.style": "RustDoc",
    "rust-analyzer.linkedProjects": [
        "./hello/Cargo.toml",
        "./ch2-web/Cargo.toml",
        "./ch3-web/Cargo.toml",
        "./ch4-web/Cargo.toml"
    ]
}
```

<!-- Code Snippets -->
## Chapters

### Chapter 1

Basic server up and running from chapter 1:

![Image of server](assets/hello-server.png)

<!-- Checking -->
Passes cargo clippy

```zsh
$ cargo clippy
    Checking hello v0.1.0 (<path>)
    Finished dev [unoptimized + debuginfo] target(s) in 0.11s
```

## Chapter 2

<!--Checking Cargo clippy-->
Note: I updated my zsh format.

Passes cargo clippy

```zsh
┌─(~/Desktop/git_local/rust_web/ch2-web)
└─(01:41:15 on main ✹)──> cargo clippy
    Finished dev [unoptimized + debuginfo] target(s) in 0.09s
```

## Chapter 3

Passes `cargo clippy`

```zsh
┌─(~/Desktop/git_local/rust_web/ch3-web)
└─(19:27:49 on main)──> cargo clippy
    Finished dev [unoptimized + debuginfo] target(s) in 0.11s
```

The web-server on port 3080 is working as expected!

![Screenshot of web-server](./assets/ch3-json-response.png)

## Chapter 4

Passes `cargo clippy`

```zsh
┌─(~/Desktop/git_local/rust_web/ch4-web)
└─(23:13:18 on main)──> cargo clippy
    Finished dev [unoptimized + debuginfo] target(s) in 0.11s
```

TODO:

Having issues understanding how to convert warp::filter to axum. From what I understand, axum uses a service instead of a filter.

But I'm still working on this...

<!-- Current -->

## Error Notes

To better help with my personal understanding of the content, I noted errors I came across in [the error-notes.md file](./error-notes.md).

## Acknowledgments

Thanks to Bart Massey for teaching the course material.

Thanks to the developers of tokio and axum.

Thanks to the developers of mintlify for making doc comments easier to write.

## License

This project is licensed with the [MIT license](./LICENSE).

## References  

Gruber, Bastian. Rust Web Development

https://github.com/Rust-Web-Development/code

https://github.com/tokio-rs/axum

https://github.com/pdx-cs-rust-web

https://www.shuttle.rs/blog/2023/12/06/using-axum-rust

https://docs.rs/axum/latest/axum/

[Mintlify Doc Writer](https://marketplace.visualstudio.com/items?itemName=mintlify.document)
