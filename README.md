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

## Developing

After cloning the repository, ensure the dependencies are installed:

```sh
devenv shell # or `direnv allow`, only if you are using devenv
pnpm wasm # bootstraps the backend
pnpm install # installs JavaScript dependencies
```

Now you can run `pnpm dev` which opens a live preview of the site which updates as you make changes.

To run the back end go inside the `tetris-ai` subdirectory and run `cargo run`.

### Updating translations

Edit the JSON files in `src/lib/translations/[lang].json`.

### Updating levels

Edit the configurations in `src/lib/levels.ts`.

### Adding a new feature

1. Create a function that implements it insie `tetris-ai/src/feature.rs`
2. Add it to the `FEATURE_LOOKUP` array at the top of the file
3. Now the feature should be available for use. Optionally you can add the translations to the JSON translation files under `feature.[name]` where the `name` corresponds to the string you defined in the lookup table.

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
If it is different on your web host make sure to change it in `svelte.config.ts`. See also [the instructions from SvelteKit](https://svelte.dev/docs/kit/single-page-apps#Usage).

An automatically build version is available from [GitHub Releases](https://github.com/rijkvp/tetris-ai/releases/tag/latest), however note that this version has the `/tetris-ai` path built into it and only works if you deploy to a domain like `example.com/tetris-ai`.

## License

This project is licensed under the [MIT License](/LICENSE).

The Rust code was originally based on a [Python implementation](https://github.com/clsibert/Tetris-AI). This repository still includes a simplified copy of the Python code used for testing. All credit goes to the original authors.
