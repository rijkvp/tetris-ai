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
The available training criteria are: `score`,`levels` and `tetrisses`.

## Install

### Using devenv

A [devenv](https://devenv.sh/) is provided which creates a development environment with all the dependencies. 

### Manually

For developing the backend:

- Rust 1.68 or higher

For developing front end:

- `wasm32-unknown-unknown` target for Rust
- Node.js with the `pnpm` package manager

For running the tests:

- Python 3.12 with the `numpy` package

## Development

### Adding/changing translations

TODO

### Adding a new feature

TODO

## Deployment

First install the dependencies as described in the Install section.
Clone the repository and run the following commands:

```sh
pnpm wasm 
pnpm install --frozen-lockfile
export BASE_PATH=/tetris-ai # set to the path where the site will be deployed e.g. example.com/tetris-ai
pnpm build
```

The files will be build to the `build` directory which you can upload to your web host.

The fallback page is set to `404.html`, this works well with GitHub pages.
If this is different on your web host make sure to change this in `svelte.config.ts`. See also [the instructions from SvelteKit](https://svelte.dev/docs/kit/single-page-apps#Usage).


