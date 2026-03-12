# Oak Woods

A 2D pixel-art platformer built with Rust + [macroquad](https://macroquad.rs/), compiled to WebAssembly.

Just a demo of what can be done with vibe coding. Does not actually go anywhere or do anything.

[Play](https://orangetide.github.io/vibe-oak-wood-game/)

## Prerequisites

- [Rust](https://rustup.rs/) toolchain
- `wasm32-unknown-unknown` target (`rustup target add wasm32-unknown-unknown`)

## Build & Run (WASM)

```bash
./build.sh
cd dist && python3 -m http.server 8080
```

Open `http://localhost:8080` in your browser.

## Build & Run (Native)

```bash
ln -sf public/assets assets
cargo run
```

## Controls

| Key | Action |
|-----|--------|
| Left / Right arrows | Run |
| Space / Up arrow | Jump |
| Z | Attack |
| I | Inventory (does nothing) |
| T | Test dialog |

## Assets

Uses the [Oak Woods](https://brullov.itch.io/oak-woods) asset pack by brullov.

## Development

See [my notes](vibe-20260311.md) for how I got it here.
