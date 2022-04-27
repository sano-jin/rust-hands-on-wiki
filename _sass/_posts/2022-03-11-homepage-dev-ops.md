---
layout: post
title: "Generating and Maintaining Homepage"
description: "Introducing Jekyll"
image: "https://www.csrhymes.com/img/landing-page.jpg"
hide_hero: true
show_sidebar: false
menubar_toc: true
author: sano
tags: webdev showdev webdesign
---

# Generating and Maintaining Homepage

ホームページの整備を現代風に行う

1. HTML を直書きするのではなく，markdown で楽に書きたい
2. サーバ上で行う作業は最小化したい

## Jekyll の紹介

2020 年以降にもなって，html を直書きするのはちょっと．．．

- 別に悪くはないのだけど，単純に面倒くさい

[Jekyll](https://jekyllrb.com/) は

- markdown から html を生成することができるアプリケーション
  - html を置いておいても問題ない（markdown 以外のものとも共存可能）
- 全てのページに共通な部品を用意することもできる
  - ページのヘッダの部分とか
- ソースコードの **syntax-highliting もできる**
- [色々なテーマが提供されている](https://jekyllthemes.io/)
  - 自分で作ることもできる（と思われるけどやったことはない）
- Ruby 製（Gem という ruby の package-manager を用いる）

## Jekyll の Quickstart

詳しくは， <https://jekyllrb.com/docs/> を参照されたし．

### Prerequisites

- Ruby version 2.5.0 or higher
- RubyGems
- GCC and Make

### Instructions

1. Install all prerequisites
2. Install the jekyll and bundler using gems

   ```sh
   gem install jekyll bundler
   ```

3. Create a new Jekyll site at `./homepage`（もちろん違う名前のディレクトリでも良い）.

   ```sh
   jekyll new homepage
   ```

4. Change into your new directory.

   ```sh
   cd homepage
   ```

5. （上記ページの注意書きにもあるのだが）webrick を依存関係に付与する必要がある．
   `Gemfile` の末尾（どこでも良いけど）に，

   ```
   gem "webrick"
   ```

   を付加する

6. Build the site and make it available on a local server.

   ```sh
   bundle exec jekyll serve --livereload
   ```

7. Browse to <http://localhost:4000>

   これで，ローカルのブラウザ上で見れるようになる．

## ページのビルド

U 研では，自分のページは `public_html`
ディレクトリに置き， `<U研のurl>/~ユーザ名` でアクセスできる．

1. `_config.yml` の `baseurl` を設定．
   これをすることで，deploy したときに，
   リンクや css のロードが正しく動くようになる（jekyll serve の時には影響してこないことに注意）．

   ```yml
   baseurl: "/~sano"
   ```

2. markdown を html へ変換する．

   ```sh
   bundle exec jekyll build
   ```

   これで，`_site` ディレクトリに html ファイルが生成される

   - ただし，url はローカルでは正しく設定されていないはずなので，
     `open _site**/index.html` などではうまく表示できない（css の styling が適用されていない）はず

## Deploy

`_site` 以下のファイルをすべて `dXXXXX` の自分のディレクトリの `public_html` 内に配置する．
