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

## [Implement Https server](./https-server)

[Here are the sample source](https://github.com/sano-jin/rust-hands-on-wiki/tree/master/https-server)

In this section, we implement a simple https server that returns a constant string message.

TODO: Modify this to firstly implement a http server
(not implementing https from the start)

## [Static file server](./static-file-server)

[Here are the sample source](https://github.com/sano-jin/rust-hands-on-wiki/tree/master/static-file-server)

In this section, we let the server to desplay the files at `/public/<filename>`
if the user access `/files/<filename>`.
i.e. static server.

## [Contents management server](./contents-management-server)

[Here are the sample source](https://github.com/sano-jin/rust-hands-on-wiki/tree/master/contents-management-server)

In this section, we will extend the static server to contents management server.
We will add post, delete method to enable CRUD (Create, Read, Update and Delete).

## [A simple wiki backend](./simple-wiki-backend)

[Here are the sample source](https://github.com/sano-jin/rust-hands-on-wiki/tree/master/simple-wiki-backend)

In this section, we parse the posted markdown and convert it to a html file.
We will be saving the markdown file in /public/edit directory and html file in /public/pages directory.

## [Client-side integration](./client-side-integration)

[Here are the sample source](https://github.com/sano-jin/rust-hands-on-wiki/tree/master/client-side-intergration)

In this section we will be adding JavaScript to jump to the editor and to update the edited page
using fetch API.

## TBD: Some improvements

Add

1. styling
2. a list of recent updated pages.

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
