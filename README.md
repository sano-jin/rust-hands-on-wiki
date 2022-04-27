_Preparation in progress..._

A hands on tutorial to make a simple wiki with Rust.

[View on GitHub](https://github.com/sano-jin/rust-hands-on-wiki)

Please send me a pull-request to improve!

# How to create your own wiki from scratch

## Prerequisites

Install Cargo.
See <https://www.rust-lang.org/tools/install> to setup rustup and cargo.

### Update rust

Do not forget to update.

```sh
rustup update
```

## [Https server](./https-server)

[Here are the sample source](https://github.com/sano-jin/rust-hands-on-wiki/tree/master/https-server)

In this section, we implement a simple https server that returns a constant string message.

TODO: Modify this to firstly implement a http server
(not implementing https from the start)

## [Static file server](./static-file-server)

[Here are the sample source](https://github.com/sano-jin/rust-hands-on-wiki/tree/master/static-file-server)

In this section, we let the server to display the files at `/public/<filename>`
if the user accesses `/files/<filename>`.
i.e. static server.

## [Contents management server](./contents-management-server)

[Here are the sample source](https://github.com/sano-jin/rust-hands-on-wiki/tree/master/contents-management-server)

In this section, we extend the static file server to a contents management server.
We will add post, delete method to enable CRUD (Create, Read, Update and Delete).

## [A simple wiki backend](./simple-wiki-backend)

[Here are the sample source](https://github.com/sano-jin/rust-hands-on-wiki/tree/master/simple-wiki-backend)

In this section, we parse the posted markdown and convert it to a html file.
We save the markdown file in the `/public/edit` directory and the html file in the `/public/pages` directory.

## [Client-side integration](./client-side-integration)

[Here are the sample source](https://github.com/sano-jin/rust-hands-on-wiki/tree/master/client-side-intergration)

In this section we add JavaScript to jump to the editor and to update the edited page
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
