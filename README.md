_Preparation in progress..._

A hands on tutorial to make a simple wiki with Rust.

# How to create a own wiki from scrarch

## Prerequisties

Install Cargo.
See <https://www.rust-lang.org/tools/install> to setup rustup and cargo.

### Update rust

Do not forget to update.

```sh
rustup update
```

## Implement Https server

[Here are the sample source](https://github.com/sano-jin/rust-hands-on-wiki/tree/master/https-server)

In this section, we implement a simple https server that returns a constant string message.

TODO: Modify this to firstly implement a http server
(not implementing https from the start)

### Create a new project

```sh
cargo new wiki-rs # create a new rust project named wiki-rs
cd wiki-rs
```

### Enable CA

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

### Add dependency

See <https://github.com/actix/examples/tree/master/https-tls/openssl>
and add dependency

`Cargo.toml`

```toml
# Cargo.toml
[package]
name = "wiki-rs" # The name of the project
version = "0.1.0" # The version of the project
edition = "2021" # The version of the rust we will be using

# Add the following dependency
[dependencies]
actix-web = { version = "4", features = ["openssl"]  } # Use actix-web to implement a backend server
env_logger = "0.9" # for logging
openssl = "0.10" # for TLS
```

### Implement with actix-web

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

### Run the backend and access

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

## Static file server

[Here are the sample source](https://github.com/sano-jin/rust-hands-on-wiki/tree/master/static-file-server)

In this section, we let the server to desplay the files at `/public/<filename>`
if the user access `/files/<filename>`.
i.e. static server.

See <https://actix.rs/docs/static-files/>

### Add dependencies

Add

```toml
actix-files = "0.6.0"
```

in the dependency list in the `Cargo.toml`.

### Add routing to `/public` directory

Add

```rust
use actix_files;
```

and add

```rust
    HttpServer::new(|| {
        App::new()
            // enable logger
            .wrap(middleware::Logger::default())

            // with path parameters
            // **Newly added here**
            // GET /files/**/*.html and return the file /public/**/*.html
            .service(fs::Files::new("/files", "public").show_files_listing())

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

## Contents management server

[Here are the sample source](https://github.com/sano-jin/rust-hands-on-wiki/tree/master/contents-management-server)

In this section, we will extend the static server to contents management server.
We will add post, delete method to enable CRUD (Create, Read, Update and Delete).

### API design

- Read: GET `/files/xxxxxx`
  - html ページのレスポンスを返す
  - サーバ上のファイルから読み込む
  - We have already implemented this in the previous section.
- Create and Update: POST /edit
  - body: `{path:"Path to the page", body: "The updated file"}`
  - markdown を投げ，それで /xxxxxx.html を更新する
  - そのページがもともと存在しない場合は新しく作る．
  - サーバ上のファイルに書き出しておく
- Delete: DELETE `/edit?path=<Path to the page>`
  - /xxxxxx.html を消去する
  - サーバ上のファイルは消去する

### Add dependencies

Add some dependencies to handle json.

```toml
json = "0.12"
serde = { version = "1.0", features = ["derive"] } # to serialize/deserialize
serde_json = "1.0"
urlencoding = "2.1.0" # For encoding the filename
```

### Encoding filename

To make it easy to handle files, we encode their names with `urlencodings::encode`

Add

```rust
// src/main.rs

use urlencoding;

/// Get the new path <root_dir>/<encoded filename>
fn get_path(root_dir: &str, filename: &str) -> PathBuf {
    let encoded = urlencoding::encode(&filename); // encode the filename
    Path::new(&root_dir).join(Path::new(&encoded.into_owned()))
}
```

to the `src/main.rs`

### Handle POST method

When we get POST request, then we need to

1. obtain the file name and the contents to update from the body of the request and
2. update the file with the contents.

We will be using JSON to send the request with the file name and the new contents as

```json
{
  "path": "<the file name of to update>",
  "body": "<the contents to be replaced>"
}
```

Therefore, we firstly add `struct` deriving `Serialize` and `Deserialize` to/from JSON from/to String (html body).

```rust
#[derive(Debug, Serialize, Deserialize)]
struct NewPageObj {
    path: String,
    body: String,
}
```

The add the function handles POST method.

```rust
/// Create and Update the file with POST method
async fn post(item: web::Json<NewPageObj>) -> Result<HttpResponse, Error> {
    println!("post {:?}", item);

    // Get the file path
    let path: PathBuf = get_path("public", &item.path);
    println!("path: {:?}", path);

    // Update the file with the given contents
    let mut file = File::create(&path)?;
    file.write_all(item.body.as_bytes())?;

    // TODO: navigate to the new page created
    Ok(HttpResponse::Ok().json("created")) // <- send json response
}
```

Finally, add the function to the routing.

```rust
  // **Newly added here**
  // POST the new contents to update the file
  .service(web::resource("/edit").route(web::post().to(post)))
```

#### Test

Run

```sh
cargo run
```

Test it with

```sh
curl -H "content-type: application/json" -kX POST -d \
    "{\"path\": \"filename.html\", \"body\": \"new contents\"}" \
        https://localhost:8443/edit
```

and then check with

```sh
curl -kX GET https://127.0.0.1:8443/files/filename
```

### Handle DELETE method

We will be handling DELETE method as well as the POST method.
When we get POST request, then we need to

1. obtain the file name to delete from the the request and
2. delete the file.

http DELETE method basically does not have it's body.
Thus, we need to use the other way other than obtaining the file name information from the body.

We are going to use query parameters this time.
The request is as follows:

```
https://127.0.0.1:8443/edit?path=<filename>
```

Therefore, we firstly add `struct` deriving `Serialize` and `Deserialize` to/from JSON from/to String (query parameters).

```rust
#[derive(Debug, Serialize, Deserialize)]
struct QueryPath {
    path: String,
}
```

The add the function handles POST method.

```rust
/// Delete the file with DELETE method
async fn delete(item: web::Query<QueryPath>) -> Result<HttpResponse, Error> {
    println!("delete ? {:?}", item);

    let path: PathBuf = get_path("public", &item.path);
    println!("path: {:?}", path);

    std::fs::remove_file(&path)?;

    // TODO: navigate to the root page
    Ok(HttpResponse::Ok().json("deleted"))
}
```

Finally, add the function to the routing.

```rust
  // POST the new contents to update the file
  .service(web::resource("/edit")
    .route(web::post().to(post))
    .route(web::delete().to(delete)) // Newly added here
  )
```

#### Test

Run

```sh
cargo run
```

Test it with

```sh
curl -kX DELETE "https://localhost:8443/edit?path=filename"
```

and then check with

```sh
curl -kX GET https://127.0.0.1:8443/files/filename
```

which should not work (file not found)

## Markdown parsing and generating html

[Here are the sample source](https://github.com/sano-jin/rust-hands-on-wiki/tree/master/simple-wiki-backend)

In this section, we parse the posted markdown and convert it to a html file.

We will be saving the markdown file in `/public/edit` directory
and html file in `/public/pages` directory.

### Add dependencies

Add dependency to `Cargo.toml`.
We will be using `pulldown_cmark` to convert markdown to html.

```toml
pulldown-cmark = { version = "0.9.1", default-features = false }
```

and denote to use it in `src/main.rs`.

```rust
// src/main.rs

// Newly added pulldown_cmark
use pulldown_cmark::{html, Options, Parser};
```

### Create `public/edit` and `public/pages`

Since we are adding the new markdown files in the `public/edit` directory and
the newly generated html files in the `public/pages` directory,
we need to create the both directory.
Generating the files without the directories will cause OS error `No such file or directory`.

```sh
cd public
mkdir edit
mkdir pages
```

The directory structure of the `public` directory is now as the follows:

```
public
├── edit
├── pages
└── test.html
```

### Convert markdown to html at the POST request

Add the converter form markdown to html to the `post` function.

```rust
/// Create and Update the file with POST method
async fn post(item: web::Json<NewPageObj>) -> Result<HttpResponse, Error> {
    println!("post {:?}", item);

    // Update the file with the given contents
    let path: PathBuf = get_path("public/edit", &item.path);
    let mut file = File::create(&path)?;
    file.write_all(item.body.as_bytes())?;

    // Set parser options
    let mut options = Options::empty();
    options.insert(Options::ENABLE_STRIKETHROUGH);
    options.insert(Options::ENABLE_TABLES);
    options.insert(Options::ENABLE_FOOTNOTES);
    options.insert(Options::ENABLE_TASKLISTS);
    options.insert(Options::ENABLE_SMART_PUNCTUATION);
    options.insert(Options::ENABLE_HEADING_ATTRIBUTES);

    // Parse the given markdown with the pulldown_cmark parser
    println!("parsing the given markdown with the pulldown_cmark parser");
    let parser = Parser::new(&item.body);
    let mut html_buf = String::new();
    html::push_html(&mut html_buf, parser);
    println!("parsed: {}", html_buf);

    // Update the file with the given contents
    let path: PathBuf = get_path("public/pages", &item.path);
    let mut file = File::create(&path)?;
    file.write_all(html_buf.as_bytes())?;

    // TODO: navigate to the new page created
    Ok(HttpResponse::Ok().json("created")) // <- send json response
}
```

#### Test the POST request

Run

```sh
cargo run
```

ant test it with

```sh
curl -H "content-type: application/json" -kX POST -d \
    "{\"path\": \"filename\", \"body\": \"# This is a title\"}" \
        https://localhost:8443/edit
```

This will generate (or update) the new 2 files `public/edit/filename` and `public/pages/filename`.
Here are the new directory structure:

```
public
├── edit
│   └── filename
├── pages
│   └── filename
└── test.html
```

Then check with

```sh
curl -kX GET https://127.0.0.1:8443/files/pages/filename
```

### Delete both the markdown and the html files

Delete both the markdown and the html file at the DELETE request.

```rust
/// Delete the file with DELETE method
async fn delete(item: web::Query<QueryPath>) -> Result<HttpResponse, Error> {
    println!("delete ? {:?}", item);

    // delete the markdown file
    let path: PathBuf = get_path("public/edit", &item.path);
    std::fs::remove_file(&path)?;

    // delete the html file
    let path: PathBuf = get_path("public/pages", &item.path);
    std::fs::remove_file(&path)?;

    // TODO: navigate to the root page
    Ok(HttpResponse::Ok().json("deleted"))
}
```

#### Test the DELETE request

Run

```sh
cargo run
```

ant test it with

```sh
curl -kX DELETE "https://localhost:8443/edit?path=filename"
```

This will delete the 2 files `public/edit/filename` and `public/pages/filename`.
Here are the updated directory structure:

```
public
├── edit
├── pages
└── test.html
```

You may want to check that the GET request to the deleted file fails

```sh
curl -kX GET https://127.0.0.1:8443/files/pages/filename
```

### Viewing html at the GET request

We used actix-files to show/download files but we are going to implement the other our own new function to
enable viewing html files to deal with more complecated tasks later
(such as displaying the access date and so on).

- Read: GET `/pages?path=xxxxxx`
  - html ページのレスポンスを返す
  - サーバ上のファイルから読み込む

```rust
// main.rs

/// GET the page
async fn get_page(item: web::Query<QueryPath>) -> Result<HttpResponse, Error> {
    println!("get_page ? {:?}", item);

    // Load the file
    let path = get_path("public/pages", &item.path);
    let contents = std::fs::read_to_string(&path)?;

    // Return the response and display the html file on the browser
    Ok(HttpResponse::Ok().content_type("text/html").body(contents))
}
```

and add the routing to the function

```rust
// main.rs

  .service(
      web::resource("/pages").route(web::get().to(get_page)), // GET the page
  )
```

#### Test with the GET request to the pages

Run

```sh
cargo run
```

Add files with the POST method we have tested before.

and then open <https://127.0.0.1:8443/pages?path=filename> on your browser,
which should display the updated html (with proper rendering).

## Client-side integration

Add JavaScript to jump to the editor and to update the edited page.

Using fetch API.

## Some improvements

Add a list of recent updated pages.

Store `recent_updates` the list of the title of the recent updated files.

# Memo

![Demo](/docs/wiki-rs-demo.png)

# API design

## Front

- 普通にアクセスして見る．
- 今見ているページの markdown を編集して，それでページを更新する．
  - edit button
- 新しいページの markdown を編集して，それでページを更新する．
  - create button

## Backend API

- GET `/page/xxxxxx`
  - html ページのレスポンスを返す
  - サーバ上のファイルから読み込む
- GET `/edit?path=<Path to the page">`
  - 編集用の markdown を返す
  - サーバ上のファイルから読み込む
- POST /edit
  - body: `{path:"Path to the page", body: "The updated markdown"}`
  - markdown を投げ，それで /xxxxxx.html を更新する
  - そのページがもともと存在しない場合は新しく作る．
  - サーバ上のファイルに書き出しておく
- DELETE `/edit?path=<Path to the page>`
  - /xxxxxx.html を消去する
  - サーバ上のファイルは消去する
