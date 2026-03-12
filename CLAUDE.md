# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

2D pixel-art platformer built with Rust + macroquad, compiled to WebAssembly. Uses the [Oak Woods](https://brullov.itch.io/oak-woods) asset pack.

## Commands

- `cargo build` — Build native debug binary
- `cargo run` — Run natively (needs `ln -sf public/assets assets` symlink first)
- `./build.sh` — Build WASM release + package into `dist/`
- `cd dist && python3 -m http.server 8080` — Serve WASM build locally

## Architecture

- **`src/main.rs`** — Complete game in a single file: asset loading, tilemap, animation system, physics, scene management, rendering
- **`web/index.html`** — WASM entry point, loads macroquad JS bundle + .wasm
- **`build.sh`** — Builds WASM target, downloads macroquad JS bundle, copies assets into `dist/`
- **`public/assets/oak_woods/`** — Static asset pack (backgrounds, tileset, character spritesheet, decorations)

## Key Technical Details

- Virtual resolution 320×180 rendered to a `RenderTarget`, then scaled with `FIT` + center to fill the window/canvas
- Tilemap is procedurally generated at init (2000×8 tiles, 24×24px each)
- Tileset tile indices: TL=0, TC=1, TC2=2, TR=3, ML=21, MC=22, MR=24 (tileset is 21 columns wide)
- Character spritesheet: 56×56 frames, 8 columns; animations: idle(0-5), attack(8-13), run(16-23), jump(24-31), fall(32-39)
- Parallax backgrounds drawn in screen space with rate offsets (0, 0.3, 0.6)
- Camera follows player horizontally only (Y fixed at -14); lerp 0.1
- Custom AABB tile collision (no physics engine): separate X then Y resolution
- Scene state machine: Title → Menu → Game (with Inventory/Dialog overlays)
- WASM build target: `wasm32-unknown-unknown`, served with macroquad's `mq_js_bundle.js`

## Legacy Files

The original TypeScript/Phaser 3 source files remain in `src/` (`.ts` files) and can be removed. The Rust build ignores them.
