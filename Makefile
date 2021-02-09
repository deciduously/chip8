.PHONY: clean site deploy native wasm wasmdeps npmdeps deps dev help

SHELL        = /bin/bash
export PATH := bin:$(PATH)

native:
	cargo run --features="sdl"

npmdeps:
	cd www      && \
	npm install

wasmdeps:
	cargo install wasm-pack

deps: npmdeps wasmdeps

clean:
	rm -rf docs             && \
	rm -rf www/dist

dev: wasm
	cd www        && \
	npm run start

deploy: clean site
	cp -r www/dist docs

wasm:
	wasm-pack build --release -- --features="wasm"

site: wasm
	cd www                             && \
	NODE_ENV=production npm run build

help:
    @echo "Usage: make {clean|site|deploy|deps|native|help|wasm}" 1>&2 && false
