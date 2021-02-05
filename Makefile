.PHONY: deploy native wasm wasmdeps npmdepshelp

SHELL        = /bin/bash
export PATH := bin:$(PATH)

native:
	cargo run --features="sdl"

npmdeps:
	cd www && \
	npm install

wasmdeps:
	cargo install wasm-pack

alldeps: npmdeps wasmdeps

dev: wasm
	cd www && \
	npm run start

deploy: wasm
	rm -rf docs && \
	cp -r www/dist docs

wasm:
	wasm-pack build --release -- --features="wasm" && \
	cd www                                         && \
	npm run build

help:
    @echo "Usage: make {deploy|native|help|wasm}" 1>&2 && false
