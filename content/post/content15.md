---
title: "Astroへの移行 and 467tn.comドメイン停止"
publishedAt: 2025-04-29
description: ""
slug: "content15"
isPublish: true
tags: ["Others"]
---

# Astroへの移行

長らくHugoでこちらのブログを管理していましたが、
ローカルで管理していたtemplateデザインやらのスクリプトが復元できなくなってしまった＆gitがよくわからない頃から運用していたりでごちゃごちゃになっていた、
などの理由からリファクタリングを行うことに決めました。

これまで通りHugoを使い続けるという手段もあったのですが、ブログを書くのが不定期ということもあり、毎回`hugo ...`をググっていたりしたので、
nodeで管理できる(hugoもデザイン選べばnodeでできそうだが)・シンプルだが拡張しやすそう、という理由から`Astro`への移行を決めました。

Hugoに比べてデメリットは、templateデザインの種類が豊富ではなかったぐらいで、他現状問題なく開発できています。

AstroのtemplateデザインはどれもStar数が少ないものが多く、
現在使っているデザインに関しては[MaeWolff/astro-portfolio-template](https://github.com/MaeWolff/astro-portfolio-template)というStar数300未満のデザイン(Astroの中で多い方)で、選んだ時に問題なく開発できるのか心配だったのですが杞憂でした。

またTailwindCSSを使い始めたのですが、こちらもかなり体験が良いです。

## 大変だったことなど

### 11つのmarkdownファイル(記事)のFrontMatter部分の移植

移植対象のmarkdownファイル(記事)は11つしかなかったのですが、FrontMatterの部分の移植が大変でした。

具体的には記事ごとのFrontMatterを移植後に合わせた形でリファクタするのが大変でした。

- 以前：[front matter](https://github.com/srn221B/blog/blob/7fc5ffc13ab735d43ef5f0bf8188cc02066964df/content/post/content11.md?plain=1#L2-L36)
- 移植後：[front matter](https://github.com/srn221B/blog/blob/master/post/content11.md?plain=1#L2-L7)

`title`,`description`など、そのままcopyしてcodeの方のpropsの定義を変える手もあったのですが、
`date`->`publishAt`などの型変換や`slug`追加のことを考えるとそうもいかず、手作業で移植しました。

### github Actionsの確認漏れ

（完全に確認不足な話）

[https://docs.astro.build/ja/guides/deploy/github/](https://docs.astro.build/ja/guides/deploy/github/)通りにci/cdを設定したのですが、
自分のDefault branchが`master`であることを忘却しており`on.push.branches: [main]`のままで一生mergeしており、時間を溶かしました。

# 467tn.comドメイン停止

`467tn.com`のドメインで公開していましたが、一時停止します。

カスタムドメインでgithub pagesを公開するとなると、大きく以下の手順になるのですが、

- 1.githubのSettings>Pagesからカスタムドメイン(`467tn.com`)の設定

- 2.ドメインに`CNAME`と`A`レコードを追加

上記移植の作業中に一時的(昨夜)に1の設定を外していました。

移植が終わったので、今日1の設定をしようとしたら以下のようなエラーとともに設定ができなくなり、

> The custom domain `467tn.com` is already taken. If you are the owner of this domain, check out https://docs.github.com/pages/configuring-a-custom-domain-for-your-github-pages-site/verifying-your-custom-domain-for-github-pages for information about how to verify and release this domain.

`467tn.com`にアクセスしたところ悪質なページにホスティングされていたので、ドメインのレコードを全て削除しました。

[The custom domain is already taken. Except it's not #23569
](https://github.com/orgs/community/discussions/23569)を見た限りですが、よくあることのようで、サポートへ問い合わせる手もあるのですが、もう少し様子を見てからにしてみようかなと思います。

ドメイン更新したばかりで勿体無い気持ちしかないですが、一旦停止ということで、今後は気をつけたいです。

