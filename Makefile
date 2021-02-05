.PHONY: deploy native wasm help

SHELL        = /bin/bash
export PATH := bin:$(PATH)

native:
	cargo run --features="sdl"

deploy: wasm
	rm -rf docs && \
	cp -r www/dist docs

wasm:
	wasm-pack build --release -- --features="wasm" && \
	cd www                                         && \
	npm run build

help:
    @echo "Usage: make {deploy|native|help|wasm}" 1>&2 && false
