# Contents management server

[Here are the sample source](https://github.com/sano-jin/rust-hands-on-wiki/tree/master/contents-management-server)

In this section, we extend the static file server to a contents management server.
We will add POST and DELETE method to enable CRUD (Create, Read, Update and Delete).

### API design

- Read: GET `/files/xxxxxx`
  - Serve the file with the given name on the server.
  - We have already implemented this in the previous section.
- Create and Update: `POST /edit`
  - body: `{path:"Path to the page", body: "The updated file"}`
  - Post the filename and the path and create or update the file on the server.
- Delete: DELETE `/edit?path=<filename>`
  - Delete the file `<filename>` on the server

## Add dependencies

Add some dependencies to handle json.

```toml
# Cargo.toml

json = "0.12"
serde = { version = "1.0", features = ["derive"] } # to serialize/deserialize
serde_json = "1.0"
urlencoding = "2.1.0" # For encoding the filename
```

## Encoding filename

To make it easy to handle files, we encode their names with `urlencodings::encode`

Add

```rust
// src/main.rs

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
```

to the `src/main.rs`

## Handle POST method

When we get POST request, then we need to

1. obtain the file name and the contents to update from the body of the request and
2. update the file with the contents.

We use JSON to send the request with the file name and the new contents as

```json
{
  "path": "<the file name of to update>",
  "body": "<the contents to be replaced>"
}
```

Therefore, we firstly add `struct` deriving `Serialize` and `Deserialize` to/from JSON from/to String (html body).

```rust
// src/main.rs

#[derive(Debug, Serialize, Deserialize)]
struct NewPageObj {
    path: String,
    body: String,
}
```

Then add the function handles POST method.

```rust
// src/main.rs

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

Finally, add the function for routing.

```rust
// src/main.rs

  // **Newly added here**
  // POST the new contents to update the file
  .service(web::resource("/edit").route(web::post().to(post)))
```

### Test

Run

```sh
cargo run
```

Test it with

```sh
curl -H 'content-type: application/json' -kX POST -d \
    '{"path": "filename", "body": "new contents"}' \
        https://localhost:8443/edit
```

and then check with

```sh
curl -kX GET https://127.0.0.1:8443/files/filename
```

## Handle DELETE method

We handle DELETE method as well as the POST method.
When we get POST request, then we need to

1. obtain the file name to delete from the the request and
2. delete the file.

Http DELETE method (basically) does not have it's body.
Thus, we need to use the other way other than obtaining the file name information from the body.

We are going to use query parameters this time.
The request is as follows:

```
https://127.0.0.1:8443/edit?path=<filename>
```

Therefore, we firstly add `struct` deriving `Serialize` and `Deserialize` to/from JSON from/to String (query parameters).

```rust
// src/main.rs

#[derive(Debug, Serialize, Deserialize)]
struct QueryPath {
    path: String,
}
```

Then add the function handles POST method.

```rust
// src/main.rs

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

Finally, add the function for routing.

```rust
// src/main.rs

  // POST the new contents to update the file
  .service(web::resource("/edit")
    .route(web::post().to(post))
    .route(web::delete().to(delete)) // Newly added here
  )
```

### Test

Run

```sh
cargo run
```

Test it with

```sh
curl -kX DELETE https://localhost:8443/edit?path=filename
```

and then check with

```sh
curl -kX GET https://127.0.0.1:8443/files/filename
```

which should not work (file not found)
