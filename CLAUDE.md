# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

2D pixel-art platformer built with Phaser 3, TypeScript, and Vite. Uses the [Oak Woods](https://brullov.itch.io/oak-woods) asset pack.

## Commands

- `npm run dev` — Start Vite dev server (default: http://localhost:5173)
- `npm run build` — Type-check with tsc then bundle with Vite
- No test framework is configured yet

## Architecture

- **`src/main.ts`** — Phaser game config (320×180 canvas, pixel art, Arcade physics, gravity 800)
- **`src/scenes/GameScene.ts`** — Single scene containing all game logic: asset loading, parallax backgrounds, procedural tilemap ground, character animations, input handling, and camera follow
- **`public/assets/oak_woods/`** — Static asset pack with `assets.json` manifest describing all images, spritesheets, and animations
- **`index.html`** — Entry point, loads `src/main.ts` as ES module

## Key Technical Details

- Canvas is 320×180 native resolution with `Phaser.Scale.FIT` + `CENTER_BOTH` to fill browser
- Tilemap is procedurally generated via `this.make.tilemap()` + `createBlankLayer()` (no Tiled JSON)
- Tileset tile indices: TL=0, TC=1, TC2=2, TR=3, ML=21, MC=22, MR=24
- Ground is 2000 tiles wide (~48000px) for effectively infinite scrolling
- Parallax driven by `tilePositionX = camera.scrollX * rate` in update() (rates: 0, 0.3, 0.6)
- Camera follows player horizontally only (Y lerp = 0 to prevent bounce on jump)
- Character spritesheet: 56×56 frames; animations: idle(0-5), attack(8-13), run(16-23), jump(24-31), fall(32-39), death(40-52)

## Vite Config Note

`server.watch.usePolling` is enabled in `vite.config.ts` to work around inotify watcher limits on this machine.
