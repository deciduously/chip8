# chip8

Yet another Rust/WASM Chip8.  Targets SDL2 and/or an HTML5 canvas via WebAssembly.

## Usage

To run the native renderer, use `cargo run --features="sdl"`.  By default it will run [corax89/chip8-test-rom](https://github.com/corax89/chip8-test-rom)  The SDL renderer is not compiled in by default and must be specified.  Use `--rom/-r` to pass a game name: `cargo run --features="sdl" -- -r brix`.  Game ROMs are compiled in to the library.

To build the WebAssembly frontend, issue `wasm-pack build -- --features="wasm"` in the crate root, then `npm install && npm run start` in `www/`.  Point your browser to `localhost:8080`.

The source includes the [Chip8 Games Pack](https://www.zophar.net/pdroms/chip8/chip-8-games-pack.html) as well as the above tester.

## Acknowledgements

* This [awesome blog post](http://www.multigesture.net/articles/how-to-write-an-emulator-chip-8-interpreter/) by [Laurence Muller](http://www.multigesture.net/about/).
* The [Chip8](https://en.wikipedia.org/wiki/CHIP-8) Wikipedia article.
* The [Binary-coded decimal](https://en.wikipedia.org/wiki/Binary-coded_decimal) Wikipedia article.