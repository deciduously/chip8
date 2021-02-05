.PHONY: deploy native wasm wasmdeps npmdeps deps dev help

SHELL        = /bin/bash
export PATH := bin:$(PATH)

native:
	cargo run --features="sdl"

npmdeps:
	cd www && \
	npm install

wasmdeps:
	cargo install wasm-pack

deps: npmdeps wasmdeps

dev: wasm
	cd www && \
	npm run start

deploy: deps wasm
	cd www              && \
	npm run build       && \
	cd ..               && \
	rm -rf docs         && \
	cp -r www/dist docs

wasm:
	wasm-pack build --release -- --features="wasm"

help:
    @echo "Usage: make {deploy|deps|native|help|wasm}" 1>&2 && false
