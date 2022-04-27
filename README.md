_Preparation in progress..._

A hands on tutorial to make a simple wiki with Rust.

[View on GitHub](https://github.com/sano-jin/rust-hands-on-wiki)

# How to create your own wiki from scrarch

## Prerequisites

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
3. ...?

# What's next!

Please contribute to the [Wiki.rs](https://github.com/sano-jin/wiki-rs) project.
Send me a pull request!

![Demo](/docs/wiki-rs-demo.png)
