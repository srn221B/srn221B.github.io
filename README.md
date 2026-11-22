# srn221b.github.io

Leptos (Rust) でビルド時に静的 HTML を生成する個人サイトです（試作中）。
スタイルは Tailwind CSS v4 (standalone CLI) + `@tailwindcss/typography` + daisyUI 5 で、Node は使いません。

## 必要なもの

- Rust（`rustup` で stable。依存の comrak が 1.89 以上を要求します）
- curl（初回に Tailwind の実行ファイルと daisyUI のファイルを取得します）

## 使い方

```sh
make build            # ホーム・一覧・全記事・タグページを dist/ に出力
make serve            # dist を http://localhost:8000 で配信