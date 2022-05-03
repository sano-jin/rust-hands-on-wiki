# Https server

[Here are the sample source](https://github.com/sano-jin/rust-hands-on-wiki/tree/master/https-server)

In this section, we implement a simple https server that returns a constant string message.

TODO: Modify this to firstly implement a http server
(not implementing https from the start)

HTTP is an acronym for Hyper Text Transfer Protocol.
It defines the way how files transfer through the internet from/to server to/from client (your browser).
We use this daily since every web site use this protocol to display the contents on its server. For example, you can see the hidden **http** word in Google's site <https://www.google.com>.

However, HTTP does not provide (defines) the way to authorize the server and secure the file transportation network: you may be accessing to a fake server without leagal authentication and it is easy for others to see what you are getting/posting through the internet, which may lead to severe security issues.
Therefore, we usually use HTTPS, Secure HTTP, which authorize the server
and encrypt the file transportation.
Notice that Google's site <https://www.google.com> also uses HTTPS instead of HTTP without S (Google also provides <http://www.google.com> but you will be redirected to the HTTPS site if you access this).

To enable HTTPS, we firstly need to authenticate the server.
It is common to use letsencrypt in the production level (you may use this if you want in this hands-on).
However since we are going to develop software locally, it is enough to authorize locally.
To do this, we use mkcert.

## Create a new project

```sh
cargo new wiki-rs  create a new rust project named wiki-rs
cd wiki-rs
```

## Enable CA

See <https://github.com/actix/examples/tree/master/https-tls/openssl> and
follow the instructions on README.md to enable CA.
We will be using [mkcert](https://github.com/FiloSottile/mkcert).

1. use local CA

   ```sh
   mkcert -install
   ```

2. generate own cert/private key

   ```sh
   mkcert 127.0.0.1
   ```

   rename the `127.0.0.1-key.pem` file with `key.pem` and
   the `127.0.0.1.pem` file with `cert.pem`.

## Add dependency

See <https://github.com/actix/examples/tree/master/https-tls/openssl>
and add dependency

`Cargo.toml`

```toml
# Cargo.toml

[package]
name = "wiki-rs"  # The name of the project
version = "0.1.0"  # The version of the project
edition = "2021"  # The version of the rust we will be using

# Add the following dependency
[dependencies]
actix-web = { version = "4", features = ["openssl"] } # Use actix-web to implement a backend server
env_logger = "0.9" # for logging
openssl = "0.10" # for TLS
```

## Implement with actix-web

Implement `src/main.rs`
following <https://github.com/actix/examples/blob/master/https-tls/openssl/src/main.rs>.

```rust
// src/main.rs

use std::io;

use actix_web::{middleware, web, App, Error, HttpRequest, HttpResponse, HttpServer};
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};

/// simple handle
async fn index(req: HttpRequest) -> Result<HttpResponse, Error> {
    println!("{:?}", req);
    Ok(HttpResponse::Ok()
        .content_type("text/plain")
        .body("Welcome!"))
}

#[actix_web::main]
async fn main() -> io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=debug");
    env_logger::init();

    println!("Started http server: 127.0.0.1:8443");

    // load TLS keys
    let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
    builder
        .set_private_key_file("key.pem", SslFiletype::PEM)
        .unwrap();
    builder.set_certificate_chain_file("cert.pem").unwrap();

    // Start http(s) server
    HttpServer::new(|| {
        App::new()
            // enable logger
            .wrap(middleware::Logger::default())
            // register simple handler, handle all methods
            .service(web::resource("/index.html").to(index))
            // with path parameters
            .service(web::resource("/").route(web::get().to(|| async {
                HttpResponse::Found()
                    .append_header(("LOCATION", "/index.html"))
                    .finish()
            })))
    })
    .bind_openssl("127.0.0.1:8443", builder)?
    .run()
    .await
}
```

## Run the backend and access

```sh
cargo run
```

and

```sh
curl https://127.0.0.1:8443/index.html
```

with the other terminal.

or access <https://127.0.0.1:8443/index.html> on browser.

You will get `Welcome!` if it goes fine.
