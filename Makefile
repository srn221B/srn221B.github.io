OUT  ?= dist
PORT ?= 8000

.PHONY: build serve open dev clean help check ci

help:
	@echo "make build                  ホーム・一覧・全記事・タグページを HTML + CSS にビルド"
	@echo "make serve                  dist を http://localhost:$(PORT) で配信"
	@echo "make dev                    build してから serve"
	@echo "make open                   ブラウザでホームを開く"
	@echo "make clean                  dist を削除"
	@echo "make check                  fix / clippy / fmt を適用（手元用）"
	@echo "make ci                     fmt --check / clippy -D warnings / test（CI 用）"

build:
	chmod 755 ./build.sh
	OUT=$(OUT) ./build.sh

serve:
	python3 -m http.server -d dist $(PORT)

dev: build serve

open:
	open http://localhost:$(PORT)/

clean:
	rm -rf dist

check:
	cargo fix --allow-dirty
	cargo clippy
	cargo fmt
	cargo check

ci:
	cargo fmt --check
	cargo clippy --locked --all-targets -- -D warnings
	cargo test --locked
