# chip8

Yet another Rust/WASM Chip8.  Targets SDL2 (working) and/or an HTML5 canvas via WebAssembly (partially working for now).

## Usage

To run the native renderer, use `cargo run --features="sdl"`.  By default it will run [corax89/chip8-test-rom](https://github.com/corax89/chip8-test-rom).  `make` or `make native` will run the test in SDL.  Use `--rom/-r` to pass a game name: `cargo run --features="sdl" -- -r brix`.  Game ROMs are compiled in to the library.

To build the WebAssembly frontend, first run `make alldeps`.  Then issue `make wasm` in the crate root, then `npm run start` in `www/`for the dev server.  Point your browser to `localhost:8080`.  To deploy the compiled site to `docs/`, run `make deploy`.

The source includes the [Chip8 Games Pack](https://www.zophar.net/pdroms/chip8/chip-8-games-pack.html) as well as the above tester.

## Acknowledgements

* This [awesome blog post](http://www.multigesture.net/articles/how-to-write-an-emulator-chip-8-interpreter/) by [Laurence Muller](http://www.multigesture.net/about/).
* The [Chip8](https://en.wikipedia.org/wiki/CHIP-8) Wikipedia article.
* The [Binary-coded decimal](https://en.wikipedia.org/wiki/Binary-coded_decimal) Wikipedia article.
