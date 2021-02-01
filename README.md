# chip8

Yes, another Rust/WASM Chip8.  Targets SDL2 and/or an HTML5 canvas via WebAssembly.

## Usage

To run the native renderer, use `cargo run --features="sdl"`.  The SDL renderer is not compiled in by default and must be specified.

I tested with the [Chip8 Games Pack](https://www.zophar.net/pdroms/chip8/chip-8-games-pack.html) as well as [corax89/chip8-test-rom](https://github.com/corax89/chip8-test-rom).

Create a folder called `games/` in the root of the repo and add the games there.  It expects each rom to be called, e.g. `GAME.ch8`, for which you'd pass `chip8 -r game`.

**TODO** Maybe enumerate the games folder, and let the user specify it?  Or provide a hard path?  And/or hardcode the gameset into the binary??

## Acknowledgements

These are the reference resources I needed:

* This [awesome blog post](http://www.multigesture.net/articles/how-to-write-an-emulator-chip-8-interpreter/) by [Laurence Muller](http://www.multigesture.net/about/).
* The [Chip8](https://en.wikipedia.org/wiki/CHIP-8) Wikipedia article.
* The [Binary-coded decimal](https://en.wikipedia.org/wiki/Binary-coded_decimal) Wikipedia article.