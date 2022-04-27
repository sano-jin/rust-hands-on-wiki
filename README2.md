_Preparation in progress..._

A hands on tutorial to make a simple wiki with Rust.

# How to create a own wiki from scrarch

### Convert markdown to html at the POST request

Add the converter form markdown to html to the `post` function.

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

### Delete both the markdown and the html files

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
public/
├── edit/
├── pages/
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

[Here are the sample source](https://github.com/sano-jin/rust-hands-on-wiki/tree/master/client-side-intergration)

In this section we will be adding JavaScript to jump to the editor and to update the edited page
using fetch API.
We are going to add the html and JavaScript code of the editor to the `public/layouts/` directory.
Therefore, we firstly create the directory:

```sh
mkdir public/layouts
```

Then the directory structure will become as follows:

```
public/
├── edit/
├── layouts/
├── pages/
└── test.html
```

### File editor

Add the followins html file

```html
<!DOCTYPE html>
<html lang="en">
  <head>
    <title>Editing</title>
  </head>
  <body>
    <form action="/edit" method="POST">
      <input
        type="text"
        id="input_path"
        name="input_path"
        required
        minlength="20"
        maxlength="100"
        size="100"
        value="TITLE"
      />
      <br />
      <textarea name="input_content" id="input_content" rows="40" cols="50">
Initial markdown code comes here...</textarea
      >
      <br />
      <input type="submit" id="btn_submit" name="btn_submit" value="Update" />
    </form>
  </body>
  <script></script>
</html>
```

Here are the scripts added in the script tags `<script></script>`.

```javascript
window.addEventListener("DOMContentLoaded", () => {
  // 送信ボタンのHTMLを取得
  const btn_submit = document.getElementById("btn_submit");

  btn_submit.addEventListener("click", async (e) => {
    e.preventDefault();

    // (3) フォーム入力欄のHTMLを取得
    const input_path = document.querySelector("input[name=input_path]");
    const path = input_path.value;

    // (3) フォーム入力欄のHTMLを取得
    const input_content = document.querySelector(
      "textarea[name=input_content]"
    );

    // (4) FormDataオブジェクトにデータをセット
    const body = input_content.value;

    // (5) フォームの入力値を送信
    const response = await fetch("/edit", {
      method: "POST", // *GET, POST, PUT, DELETE, etc.
      mode: "cors", // no-cors, *cors, same-origin
      cache: "no-cache", // *default, no-cache, reload, force-cache, only-if-cached
      credentials: "same-origin", // include, *same-origin, omit
      headers: {
        "Content-Type": "application/json",
        // 'Content-Type': 'application/x-www-form-urlencoded',
      },
      redirect: "follow", // manual, *follow, error
      referrerPolicy: "no-referrer", // no-referrer, *no-referrer-when-downgrade, origin, origin-when-cross-origin, same-origin, strict-origin, strict-origin-when-cross-origin, unsafe-url
      body: JSON.stringify({ path: path, body: body }), // body data type must match "Content-Type" header
    });

    const location = await response.json();
    console.log("location: ", location);

    // redirect to the returned location
    window.location = location;
  });
});
```

#### Test the edit page

Open <https://127.0.0.1:8443/files/layouts/edit.html> on your browser,
fill the contents and
press the `Update` button.
Then you will be redirected to the newly created page.

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
