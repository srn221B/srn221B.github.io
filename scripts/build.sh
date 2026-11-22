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
TW="$TOOLS/tailwindcss"

# --- Tailwind CSS standalone CLI を取得（初回のみ）---
if [[ ! -x "$TW" ]]; then
  case "$(uname -s)-$(uname -m)" in
    Darwin-arm64)  asset=tailwindcss-macos-arm64 ;;
    Darwin-x86_64) asset=tailwindcss-macos-x64 ;;
    Linux-x86_64)  asset=tailwindcss-linux-x64 ;;
    Linux-aarch64) asset=tailwindcss-linux-arm64 ;;
    *) echo "unsupported platform: $(uname -s)-$(uname -m)" >&2; exit 1 ;;
  esac
  mkdir -p "$TOOLS"
  echo "downloading $asset ..."
  curl -fsSL -o "$TW" "https://github.com/tailwindlabs/tailwindcss/releases/latest/download/$asset"
  chmod +x "$TW"
fi

# --- daisyUI のプラグインファイルを取得（初回のみ）---
for f in daisyui.mjs daisyui-theme.mjs; do
  if [[ ! -f "style/$f" ]]; then
    echo "downloading $f ..."
    curl -fsSL -o "style/$f" "https://github.com/saadeghi/daisyui/releases/latest/download/$f"
  fi
done

# --- HTML 生成 → CSS 生成 ---
cargo run --release -- "$OUT"
cp -R public/. "$OUT/"   # favicon・robots.txt など
"$TW" -i style/input.css -o "$OUT/style.css" --minify

echo
echo "done. プレビュー:"
echo "  python3 -m http.server -d dist 8000"
echo "  open http://localhost:8000/"
