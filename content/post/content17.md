---
title: "npmの卒業。Leptosを使ったRust->HTMLへの生成へ。"
publishedAt: 2026-11-22
description: "さよなら依存パッケージの脆弱性対応"
slug: "content17"
isPublish: true
tags: ["Rust"]
---

## 背景

長らくAstroやらHugoやらのフレームワークでこのサイトを作ってきたが、近今の情勢により、depandabotによる依存パッケージの脆弱性更新が多すぎる。脆弱性対応は仕事だけで十分で、こんな個人サイトに大切なプライベートの時間を避けられないよ。ということで、npmを卒業することにしました。

## 仕組み

* Markdownファイルにおいての記事生成は維持したい。
* styleもできれば今と同じがよい。
* Rustを書きたい。

なのでClaudeCodeと相談しながら以下のような仕組みになっています。

**Markdown -> HTML**

ブログ記事(Markdown)からHTML生成の仕組み

| 仕組み | 使っているツール |
| --- | --- |
| MarkdownをStructに変換/Validate | [acatton/serde-yaml-ng](https://github.com/acatton/serde-yaml-ng) |
| MarkdownをHTMLに変換 | [kivikakk/comrak](https://github.com/kivikakk/comrak/tree/main) |

**Rust -> HTML**

ブログ記事以外(Rust)からHTML生成の仕組み

| 仕組み | 使っているツール |
| --- | --- |
| RustをHTMLに変換 | [leptos-rs/Leptos](https://github.com/leptos-rs/leptos) |

RustをHTMLに変換するツールとして`Yew`もありましたが、SSRをするのにランタイムが必要とのことでLaptosの方を選びました。

CSSは前回から変わらず、Tailwind + daisyUI を使っています。

## そのほか変更したことなど

* DuckDB-WASMでtwitterデータをSQLで検索できるページを作っていたがまだ未実装です。
* 私ですら記憶を思い出すために参照もしなさそうな記事はgithub io上では非公開にしました。

## 所感

`npm run dev`が使えなくなり、都度buildが必要になったのは少し面倒かも。

まだ作ったばかりなので運用がどうなるのかはわかりません。ただ従来扱っていたReact/TypeScriptなどよりはRustの方がまだ知識に明るいので読みやすくなったことは確かです。

去年の5月に[Astroから移行したばかり](https://srn221b.github.io/post/content15/)で、インターネットに存在するサグラダファミリア（そういえばもうサグラダファミリアは完成したんだっけ）になっていてワロタですが、少しは運用が楽になると良いな、なんて思っています。
