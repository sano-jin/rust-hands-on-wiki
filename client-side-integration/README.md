_Preparation in progress..._

A hands on tutorial to make a simple wiki with Rust.

# How to create your own wiki from scrarch

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

Add the following html file `public/layouts/edit.html`

```html
<!-- public/layouts/edit.html -->

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
MARKDOWN</textarea
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
// scripts in the public/layouts/page.html

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

### Improvement in the editor

Currently we are just displaying the raw file with the initial contents
but we want a improvement in it such as presetting the previous markdown contents.
We will replace the preset contents with the previous markdown
at the Http GET request to the editor.
The newly added API is follows:

- GET `/edit?path=xxxxxx`
  - Get the editor with the previous markdown of the xxxxxx as the preset contents
  - サーバ上のファイルから読み込む

Notice that the path given here may be the path to a non-existing files
since this can be used to newly create a file.
Thus, we firstly implement a new auxiliary function that read the contents of the file
with the default string which will be returned if the file does not exist.

```rust
// src/main.rs

/// Read a file
/// If the file doesn not exists, then return the default string
fn read_with_default(path: &str, default: &str) -> String {
    let contents = std::fs::read_to_string(&path);
    match contents {
        Ok(contents) => contents,
        Err(error) => match error.kind() {
            io::ErrorKind::NotFound => String::from(default),
            other_error => panic!("Problem opening the file: {:?}", other_error),
        },
    }
}
```

Then we are adding a new function handles the GET request to the editor

```rust
// src/main.rs

/// This handler uses json extractor with limit
/// GET the page for editing the page
async fn get_editor(item: web::Query<QueryPath>) -> Result<HttpResponse, Error> {
    println!("get_edit_page ? {:?}", item);
    let path: PathBuf = get_path("public/edit", &item.path);
    let contents = read_with_default(&path.to_string_lossy(), "");

    // decode the path to obtain the title
    let title = urlencoding::decode(&item.path).expect("cannot decode");

    // Open the file for editing
    let editor = std::fs::read_to_string("public/layouts/edit.html")?;
    // Replace the contents
    let editor = editor
        .replace("TITLE", &title.into_owned())
        .replace("MARKDOWN", &contents);

    Ok(HttpResponse::Ok().content_type("text/html").body(editor))
}
```

and add the routing to the editor

```rust
// src/main.rs

  .service(
      web::resource("/edit")
          .route(web::get().to(get_editor)) // **Newly added here** GET the editor
          .route(web::post().to(post)) // POST the new contents to update the file
          .route(web::delete().to(delete)), // Delete the file
  )
```

#### Test the editor page

Open <https://127.0.0.1:8443/edit?path=filename> on your browser,
Check that the previous contents are there.
fill the contents and
press the `Update` button.
Then you will be redirected to the newly created page.

### Improvement in page layout (Add a link to the editor)

Currently we are just displaying the raw html file generated by markdown
but we want a improvement in it such as adding the link to the editor

Therefore, we use a template html file `public/layouts/page.html` and
replace the contents with the generated html
at the Http GET request to the pages.

```html
<!-- public/layouts/page.html -->

<!DOCTYPE html>
<html lang="en">
  <head>
    <title>TITLE</title>
  </head>
  <body>
    <div>
      <!-- A link to the editor -->
      <a href="/edit?path=PATH">Update</a>
    </div>
    <div>BODY</div>
  </body>
</html>
```

We update the function to update the html file to use the template layout
after parsing the given markdown:

```rust
// src/main.rs
// in the `post` function

    // decode the path to obtain the title
    let title = urlencoding::decode(&item.path).expect("cannot decode");

    // Open the default file
    let default_page = std::fs::read_to_string("public/layouts/page.html")?;
    // Replace the title, path, contents
    let page = default_page
        .replace("TITLE", &title.into_owned())
        .replace("PATH", &item.path)
        .replace("BODY", &html_buf);

    // Update the file with the given contents
    let path: PathBuf = get_path("public/pages", &item.path);
    println!("writing to the file {:?}", path);
    let mut file = File::create(&path)?;
    file.write_all(page.as_bytes())?;
```

#### Test

Open <https://127.0.0.1:8443/edit?path=filename> on your browser,
fill the contents and
press the `Update` button.
Then you will be redirected to the newly created page with the proper title and
a link to the editor.

### Improvement in page layout (Add a delete button)

We will be adding a delete button which deletes the page.
The delete button can be implemented as same as the Update button we have already created.
The new html is the following:

```html
<!-- public/layouts/page.html -->

<!DOCTYPE html>
<html lang="en">
  <head>
    <title>TITLE</title>
  </head>
  <body>
    <div>
      <a href="/">HOME</a>
      <a href="/edit?path=PATH">Update</a>
      <form action="/edit" method="DELETE">
        <input type="submit" id="btn_submit" name="btn_submit" value="Delete" />
      </form>
    </div>
    <div>BODY</div>
  </body>

  <script>
    window.addEventListener("DOMContentLoaded", () => {
      // 送信ボタンのHTMLを取得
      const btn_submit = document.getElementById("btn_submit");

      btn_submit.addEventListener("click", async (e) => {
        e.preventDefault();

        // フォームの入力値を送信
        const response = await fetch(
          "/edit?" + new URLSearchParams({ path: "TITLE" }),
          {
            method: "DELETE", // *GET, POST, PUT, DELETE, etc.
          }
        );

        // redirect to the home page
        window.location = "/";
      });
    });
  </script>
</html>
```

Test it with clicking the delete button.
