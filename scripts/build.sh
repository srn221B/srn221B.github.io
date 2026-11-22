#!/usr/bin/env bash
# ホーム・全記事・一覧・タグページを Leptos で静的 HTML にし、Tailwind(standalone) + daisyUI で CSS を作る。
# Node は使わない。
#
#   ./scripts/build.sh                # dist に出力（ホームは dist/index.html、記事は dist/post/）
#   OUT=/tmp/out ./scripts/build.sh   # 出力先を変更
set -euo pipefail
cd "$(dirname "$0")/.."

OUT="${OUT:-dist}"
TOOLS=.tools

# 取得する外部ファイルはバージョンと SHA256 を固定する。更新するときは両方を書き換える
TAILWIND_VERSION=4.3.5
DAISYUI_VERSION=5.8.2

sha256() {
  if command -v sha256sum >/dev/null; then sha256sum "$1" | cut -d' ' -f1; else shasum -a 256 "$1" | cut -d' ' -f1; fi
}

# fetch <url> <dest> <expected sha256>: 手元のファイルが一致すれば何もしない。不一致なら取り直して再検証する
fetch() {
  local url="$1" dest="$2" want="$3"
  if [[ -f "$dest" && "$(sha256 "$dest")" == "$want" ]]; then
    return
  fi
  echo "downloading $(basename "$dest") ..."
  mkdir -p "$(dirname "$dest")"
  curl -fsSL -o "$dest" "$url"
  local got
  got="$(sha256 "$dest")"
  if [[ "$got" != "$want" ]]; then
    rm -f "$dest"
    echo "checksum mismatch: $url" >&2
    echo "  expected $want" >&2
    echo "  actual   $got" >&2
    exit 1
  fi
}

# --- Tailwind CSS standalone CLI ---
case "$(uname -s)-$(uname -m)" in
  Darwin-arm64)  asset=tailwindcss-macos-arm64;  tw_sha=f3f4f5d1727e2b693b8f1c9a1492b5dd87008c86ea286bee1589cb458e6235fc ;;
  Darwin-x86_64) asset=tailwindcss-macos-x64;    tw_sha=a63702686a3ed048b3135d84a73be49ea9d47f880c9b20e54b3ef60934ad8cdf ;;
  Linux-x86_64)  asset=tailwindcss-linux-x64;    tw_sha=e47695cee3f0d90623a91f3939b08bdee610eb5ad99340adff9d4145b284d6cc ;;
  Linux-aarch64) asset=tailwindcss-linux-arm64;  tw_sha=ffcd0d21dbd7c605dc12743ff0c0b47debc963c86b399674084c1c8f2c96ecad ;;
  *) echo "unsupported platform: $(uname -s)-$(uname -m)" >&2; exit 1 ;;
esac
TW="$TOOLS/tailwindcss-$TAILWIND_VERSION"
fetch "https://github.com/tailwindlabs/tailwindcss/releases/download/v$TAILWIND_VERSION/$asset" "$TW" "$tw_sha"
chmod +x "$TW"

# --- daisyUI のプラグインファイル ---
daisy="https://github.com/saadeghi/daisyui/releases/download/v$DAISYUI_VERSION"
fetch "$daisy/daisyui.mjs" style/daisyui.mjs fe2365a53ac9e27b61fb47da216353adedd07b48f409d833f405f0be22ff4ee5
fetch "$daisy/daisyui-theme.mjs" style/daisyui-theme.mjs 675978cf3ebed8110559f9997b14f26dbb52be1fb7c5854e99e01a70dd45468b

# --- HTML 生成 → CSS 生成 ---
cargo run --release -- "$OUT"
cp -R public/. "$OUT/"   # favicon・robots.txt など
"$TW" -i style/input.css -o "$OUT/style.css" --minify

echo
echo "done. プレビュー:"
echo "  python3 -m http.server -d dist 8000"
echo "  open http://localhost:8000/"
