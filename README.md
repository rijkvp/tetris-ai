# Tetris AI

This is an educational 'game' to teach people about machine learning in the context of a Tetris AI.

## Usage

The Rust back end is both available as a CLI and as WASM library.

First, run `cargo build --release` to compile the code.
The executable will then be available at `./target/release/tetris-ai`.

The binary can be run like:
```sh
tetris-ai run|train [preset|criterion]
```

The available weight presets are: `score` and `levels`.
The available training criteria are: `score`,`levels` and`tetrisses`.


## Development

### Dependencies

A [devenv](https://devenv.sh/) is provided which creates a environment with all the dependencies. Otherwise, install the following:

For developing the backend:

- Rust 1.68 or higher

For developing front end:

- `wasm32-unknown-unknown` target for Rust
- Node.js with the `pnpm` package manager

For running the tests:

- Python 3.12 with the `numpy` package

### Adding/changing translations

TODO

### Adding a new feature

TODO

## Deployment

TODO
