# A hands-on tutorial of web development in Rust for absolute beginners

# How to create your own wiki from scrarch

## Static file server

[Here are the sample source](https://github.com/sano-jin/rust-hands-on-wiki/tree/master/static-file-server)

In this section, we let the server to desplay the files at `/public/<filename>`
if the user access `/files/<filename>`.
i.e. static file server.

See <https://actix.rs/docs/static-files/>

### Add dependencies

Add

```toml
# Cargo.toml

actix-files = "0.6.0"
```

in the dependency list in the `Cargo.toml`.

### Add routing to `/public` directory

Add

```rust
// src/main.rs

use actix_files;
```

and add

```rust
// src/main.rs

    HttpServer::new(|| {
        App::new()
            // enable logger
            .wrap(middleware::Logger::default())

            // with path parameters
            // **Newly added here**
            // GET /files/**/*.html and return the file /public/**/*.html
            .service(actix_files::Files::new("/files", "public").show_files_listing())

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
```

### Add some test files and test

create and add some test files in `/public` directory

```sh
mkdir public
cd public
echo "This is a test" > test.html
```

The directory structure will become as follows:

```
public/
└── test.html
```

### Run the backend and access

```sh
cargo run
```

and

```sh
curl https://127.0.0.1:8443/files/test.html
```

with the other terminal.

or access <https://127.0.0.1:8443/files/test.html> on browser.

You will get `This is a test` if it goes fine.
