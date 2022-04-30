# A simple wiki backend -- Markdown parsing and generating html

[Here are the sample source](https://github.com/sano-jin/rust-hands-on-wiki/tree/master/simple-wiki-backend)

In this section, we parse the posted markdown and convert it to a html file.
We save the markdown file in the `/public/edit` directory and the html file in the `/public/pages` directory.

## Add dependencies

Add dependency to Cargo.toml. We will be using pulldown_cmark to convert markdown to html.

```toml
# Cargo.toml

pulldown-cmark = { version = "0.9.1", default-features = false }
```

and denote to use it in src/main.rs.

```rust
// src/main.rs

// Newly added pulldown_cmark
use pulldown_cmark::{html, Options, Parser};
```

Since we are adding the new markdown files in the `public/edit` directory
and the newly generated html files in the `public/pages` directory,
we need to create the both directory beforehand.
Generating the files without the directories will cause OS error `No such file or directory`.

```sh
cd public
mkdir edit
mkdir pages
```

The directory structure of the public directory is now as the follows:

```
public
├── edit
├── pages
└── test.html
```

## Convert markdown to html at the POST request

Add the converter form markdown to html in the `post` function.

```rust
// src/main.rs

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
    let url = format!("/pages?path={}", &item.path);
    Ok(HttpResponse::Ok().json(url))
}
```

### Test the POST request

Run

```sh
cargo run
```

ant test it with

```sh
curl -H 'content-type: application/json' -kX POST -d \
    '{"path": "filename", "body": "# This is a title"}' \
        https://localhost:8443/edit
```

This will generate (or update) the new 2 files `public/edit/filename` and `public/pages/filename`.
Here are the new directory structure:

```
public/
├── edit/
│   └── filename
├── pages/
│   └── filename
└── test.html
```

Then check with

```sh
curl -kX GET https://127.0.0.1:8443/files/pages/filename
```

## Delete both the markdown and the html files

Delete both the markdown and the html file at the DELETE request.

```rust
// src/main.rs

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

### Test the DELETE request

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
public/
├── edit/
├── pages/
└── test.html
```

You may want to check that the GET request to the deleted file fails

```sh
curl -kX GET https://127.0.0.1:8443/files/pages/filename
```

## Viewing html at the GET request

We used actix-files to show/download files but we are going to implement the other our own new function to
enable viewing html files to deal with more complecated tasks later
(such as displaying the access date and so on).

- Read: GET `/pages?path=xxxxxx`
  - html ページのレスポンスを返す
  - サーバ上のファイルから読み込む

```rust
// src/main.rs

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
// src/main.rs

  .service(
      web::resource("/pages").route(web::get().to(get_page)), // GET the page
  )
```

### Test with the GET request to the pages

Run

```sh
cargo run
```

Add files with the POST method we have tested before.

and then open <https://127.0.0.1:8443/pages?path=filename> on your browser,
which should display the updated html (with proper rendering).
