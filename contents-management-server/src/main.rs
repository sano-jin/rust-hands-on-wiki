use std::io;

use actix_web::{middleware, web, App, Error, HttpRequest, HttpResponse, HttpServer};
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};

use actix_files;

// Newly added here
use std::fs::File;
use std::io::prelude::*;

use serde::{Deserialize, Serialize};

use std::path::{Path, PathBuf};
use urlencoding;

/// Get the new path <root_dir>/<encoded filename>
fn get_path(root_dir: &str, filename: &str) -> PathBuf {
    let encoded = urlencoding::encode(&filename); // encode the filename
    Path::new(&root_dir).join(Path::new(&encoded.into_owned()))
}

#[derive(Debug, Serialize, Deserialize)]
struct NewPageObj {
    path: String,
    body: String,
}

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

#[derive(Debug, Serialize, Deserialize)]
struct ReqObj {
    path: String,
}

/// Delete the file with DELETE method
async fn delete(item: web::Query<ReqObj>) -> Result<HttpResponse, Error> {
    println!("delete ? {:?}", item);

    let path: PathBuf = get_path("public", &item.path);
    println!("path: {:?}", path);

    std::fs::remove_file(&path)?;

    // TODO: navigate to the root page
    Ok(HttpResponse::Ok().json("deleted"))
}

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

    HttpServer::new(|| {
        App::new()
            // enable logger
            .wrap(middleware::Logger::default())
            // with path parameters
            // **Newly added here**
            .service(
                web::resource("/edit")
                    .route(web::post().to(post)) // POST the new contents to update the file
                    .route(web::delete().to(delete)), // Delete the file
            )
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
}
