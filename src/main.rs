use macroquad::prelude::*;

// Virtual resolution (matches original Phaser config)
const VIRTUAL_W: f32 = 320.0;
const VIRTUAL_H: f32 = 180.0;

// Tilemap
const TILE_SIZE: f32 = 24.0;
const MAP_W: usize = 2000;
const MAP_H: usize = 8;

// Character spritesheet: 448x392, 56x56 frames → 8 columns, 7 rows
const CHAR_FRAME_W: f32 = 56.0;
const CHAR_FRAME_H: f32 = 56.0;
const CHAR_COLS: u32 = 8;

// Tileset: 504x360, 24x24 tiles → 21 columns
const TILESET_COLS: u16 = 21;

// Tile indices (from original)
const TL: u16 = 0;
const TC: u16 = 1;
const TC2: u16 = 2;
const TR: u16 = 3;
const ML: u16 = 21;
const MC: u16 = 22;
const MR: u16 = 24;

// Physics
const GRAVITY: f32 = 800.0;
const SPEED: f32 = 100.0;
const JUMP_VEL: f32 = -280.0;

// Camera Y is fixed (original: player.y - height/2 - 24 = 100 - 90 - 24 = -14)
const CAM_Y: f32 = -14.0;

// World offset for center section decorations (1000 * 24)
const DECO_OFFSET: f32 = 24000.0;

// ── Assets ──────────────────────────────────────────────────────────────────

struct Assets {
    bg1: Texture2D,
    bg2: Texture2D,
    bg3: Texture2D,
    tileset: Texture2D,
    char_sheet: Texture2D,
    deco_lamp: Texture2D,
    deco_fence1: Texture2D,
    deco_sign: Texture2D,
    deco_grass1: Texture2D,
    deco_grass2: Texture2D,
    deco_grass3: Texture2D,
    font: Font,
}

async fn load_tex(path: &str) -> Texture2D {
    let tex = load_texture(path)
        .await
        .unwrap_or_else(|e| panic!("Failed to load {path}: {e}"));
    tex.set_filter(FilterMode::Nearest);
    tex
}

impl Assets {
    async fn load() -> Self {
        let b = "assets/oak_woods";
        let font = load_ttf_font("assets/PressStart2P-Regular.ttf")
            .await
            .expect("Failed to load font");
        Assets {
            bg1: load_tex(&format!("{b}/background/background_layer_1.png")).await,
            bg2: load_tex(&format!("{b}/background/background_layer_2.png")).await,
            bg3: load_tex(&format!("{b}/background/background_layer_3.png")).await,
            tileset: load_tex(&format!("{b}/oak_woods_tileset.png")).await,
            char_sheet: load_tex(&format!("{b}/character/char_blue.png")).await,
            deco_lamp: load_tex(&format!("{b}/decorations/lamp.png")).await,
            deco_fence1: load_tex(&format!("{b}/decorations/fence_1.png")).await,
            deco_sign: load_tex(&format!("{b}/decorations/sign.png")).await,
            deco_grass1: load_tex(&format!("{b}/decorations/grass_1.png")).await,
            deco_grass2: load_tex(&format!("{b}/decorations/grass_2.png")).await,
            deco_grass3: load_tex(&format!("{b}/decorations/grass_3.png")).await,
            font,
        }
    }
}

// ── Tilemap ─────────────────────────────────────────────────────────────────

struct Tilemap {
    tiles: Vec<Vec<Option<u16>>>,
    width: usize,
    height: usize,
}

impl Tilemap {
    fn new() -> Self {
        let mut tiles = vec![vec![None; MAP_W]; MAP_H];

        // Fill ground: row 6 with TC/TC2, row 7 with MC
        for x in 0..MAP_W {
            tiles[6][x] = Some(if x % 2 == 0 { TC } else { TC2 });
            tiles[7][x] = Some(MC);
        }

        // Step section
        let sl: usize = 1008;
        let sr: usize = 1013;

        // Right edge of left ground before step
        tiles[6][sl - 1] = Some(TR);
        tiles[7][sl - 1] = Some(MR);

        // Step body (rows 6-7)
        tiles[6][sl] = Some(ML);
        tiles[7][sl] = Some(ML);
        for x in (sl + 1)..sr {
            tiles[6][x] = Some(MC);
            tiles[7][x] = Some(MC);
        }
        tiles[6][sr] = Some(MR);
        tiles[7][sr] = Some(MR);

        // Left edge of right ground after step
        tiles[6][sr + 1] = Some(TL);
        tiles[7][sr + 1] = Some(ML);

        // Upper platform surface (row 5)
        tiles[5][sl] = Some(TL);
        for x in (sl + 1)..sr {
            tiles[5][x] = Some(if x % 2 == 0 { TC } else { TC2 });
        }
        tiles[5][sr] = Some(TR);

        Tilemap {
            tiles,
            width: MAP_W,
            height: MAP_H,
        }
    }

    fn is_solid(&self, x: i32, y: i32) -> bool {
        if x < 0 || y < 0 || x >= self.width as i32 || y >= self.height as i32 {
            return false;
        }
        self.tiles[y as usize][x as usize].is_some()
    }

    fn get(&self, x: usize, y: usize) -> Option<u16> {
        if x >= self.width || y >= self.height {
            return None;
        }
        self.tiles[y][x]
    }
}

// ── Animation ───────────────────────────────────────────────────────────────

#[derive(Clone, Copy, PartialEq)]
enum Anim {
    Idle,
    Run,
    Jump,
    Fall,
    Attack,
}

struct AnimDef {
    start: u32,
    end: u32,
    fps: f32,
    looping: bool,
}

fn anim_def(a: Anim) -> AnimDef {
    match a {
        Anim::Idle => AnimDef { start: 0, end: 5, fps: 8.0, looping: true },
        Anim::Run => AnimDef { start: 16, end: 23, fps: 10.0, looping: true },
        Anim::Jump => AnimDef { start: 24, end: 31, fps: 10.0, looping: false },
        Anim::Fall => AnimDef { start: 32, end: 39, fps: 10.0, looping: true },
        Anim::Attack => AnimDef { start: 8, end: 13, fps: 12.0, looping: false },
    }
}

struct AnimPlayer {
    current: Anim,
    frame: u32,
    timer: f32,
    finished: bool,
}

impl AnimPlayer {
    fn new() -> Self {
        let def = anim_def(Anim::Idle);
        AnimPlayer {
            current: Anim::Idle,
            frame: def.start,
            timer: 0.0,
            finished: false,
        }
    }

    fn play(&mut self, anim: Anim) {
        if self.current == anim {
            return;
        }
        self.current = anim;
        let def = anim_def(anim);
        self.frame = def.start;
        self.timer = 0.0;
        self.finished = false;
    }

    fn update(&mut self, dt: f32) {
        if self.finished {
            return;
        }
        let def = anim_def(self.current);
        self.timer += dt;
        let frame_dur = 1.0 / def.fps;
        if self.timer >= frame_dur {
            self.timer -= frame_dur;
            if self.frame < def.end {
                self.frame += 1;
            } else if def.looping {
                self.frame = def.start;
            } else {
                self.finished = true;
            }
        }
    }

    fn current_frame(&self) -> u32 {
        self.frame
    }
}

// ── Player ──────────────────────────────────────────────────────────────────

struct Player {
    x: f32,
    y: f32,
    vx: f32,
    vy: f32,
    flip_x: bool,
    on_ground: bool,
    is_attacking: bool,
    anim: AnimPlayer,
}

impl Player {
    fn new(x: f32, y: f32) -> Self {
        Player {
            x,
            y,
            vx: 0.0,
            vy: 0.0,
            flip_x: false,
            on_ground: false,
            is_attacking: false,
            anim: AnimPlayer::new(),
        }
    }

    /// Collision box: 16x28, offset from sprite center by (-8, -4) to (+8, +24)
    /// Matches original: body size (16,28), offset (20,24) from 56x56 sprite top-left
    fn body_left(&self) -> f32 { self.x - 8.0 }
    fn body_top(&self) -> f32 { self.y - 4.0 }
    fn body_right(&self) -> f32 { self.x + 8.0 }
    fn body_bottom(&self) -> f32 { self.y + 24.0 }
}

// ── Physics ─────────────────────────────────────────────────────────────────

fn resolve_x(player: &mut Player, tilemap: &Tilemap) {
    let top_tile = (player.body_top() / TILE_SIZE) as i32;
    let bottom_tile = ((player.body_bottom() - 0.01) / TILE_SIZE) as i32;

    if player.vx > 0.0 {
        let right_tile = (player.body_right() / TILE_SIZE) as i32;
        for ty in top_tile..=bottom_tile {
            if tilemap.is_solid(right_tile, ty) {
                player.x = right_tile as f32 * TILE_SIZE - 8.01;
                player.vx = 0.0;
                return;
            }
        }
    } else if player.vx < 0.0 {
        let left_tile = ((player.body_left() - 0.01) / TILE_SIZE) as i32;
        for ty in top_tile..=bottom_tile {
            if tilemap.is_solid(left_tile, ty) {
                player.x = (left_tile + 1) as f32 * TILE_SIZE + 8.01;
                player.vx = 0.0;
                return;
            }
        }
    }
}

fn resolve_y(player: &mut Player, tilemap: &Tilemap) {
    player.on_ground = false;

    let left_tile = (player.body_left() / TILE_SIZE) as i32;
    let right_tile = ((player.body_right() - 0.01) / TILE_SIZE) as i32;

    if player.vy >= 0.0 {
        let bottom_tile = (player.body_bottom() / TILE_SIZE) as i32;
        for tx in left_tile..=right_tile {
            if tilemap.is_solid(tx, bottom_tile) {
                let ground = bottom_tile as f32 * TILE_SIZE;
                player.y = ground - 24.0;
                player.vy = 0.0;
                player.on_ground = true;
                return;
            }
        }
    } else {
        let top_tile = ((player.body_top() - 0.01) / TILE_SIZE) as i32;
        for tx in left_tile..=right_tile {
            if tilemap.is_solid(tx, top_tile) {
                let ceiling = (top_tile + 1) as f32 * TILE_SIZE;
                player.y = ceiling + 4.01;
                player.vy = 0.0;
                return;
            }
        }
    }
}

fn update_player(player: &mut Player, tilemap: &Tilemap, dt: f32) {
    // Input → velocity
    if player.is_attacking {
        player.vx = 0.0;
    } else if is_key_down(KeyCode::Left) {
        player.vx = -SPEED;
        player.flip_x = true;
    } else if is_key_down(KeyCode::Right) {
        player.vx = SPEED;
        player.flip_x = false;
    } else {
        player.vx = 0.0;
    }

    if (is_key_down(KeyCode::Space) || is_key_down(KeyCode::Up)) && player.on_ground {
        player.vy = JUMP_VEL;
    }

    if is_key_pressed(KeyCode::Z) && player.on_ground && !player.is_attacking {
        player.is_attacking = true;
        player.anim.play(Anim::Attack);
    }

    // Physics
    player.vy += GRAVITY * dt;
    player.x += player.vx * dt;
    resolve_x(player, tilemap);
    player.y += player.vy * dt;
    resolve_y(player, tilemap);

    // Clamp to map bounds
    player.x = player.x.clamp(8.0, MAP_W as f32 * TILE_SIZE - 8.0);

    // Animation state (skip if attacking)
    if !player.is_attacking {
        if !player.on_ground {
            if player.vy < 0.0 {
                player.anim.play(Anim::Jump);
            } else {
                player.anim.play(Anim::Fall);
            }
        } else if player.vx != 0.0 {
            player.anim.play(Anim::Run);
        } else {
            player.anim.play(Anim::Idle);
        }
    }

    player.anim.update(dt);

    // Attack animation complete
    if player.is_attacking && player.anim.finished {
        player.is_attacking = false;
    }
}

// ── Scene / Overlay ─────────────────────────────────────────────────────────

enum Overlay {
    Inventory,
    Dialog { lines: Vec<String>, index: usize },
    Pause { selected: usize },
}

enum Scene {
    Title { blink_timer: f32 },
    Menu { selected: usize },
    Game {
        player: Player,
        cam_x: f32,
        overlay: Option<Overlay>,
    },
}

// ── Drawing helpers ─────────────────────────────────────────────────────────

fn draw_centered_text(text: &str, x: f32, y: f32, world_size: f32, font: &Font, color: Color) {
    let (fs, scale, aspect) = camera_font_scale(world_size);
    let total_scale = scale * aspect;
    let m = measure_text(text, Some(font), fs, total_scale);
    draw_text_ex(
        text,
        x - m.width / 2.0,
        y - m.height / 2.0 + m.offset_y,
        TextParams {
            font: Some(font),
            font_size: fs,
            font_scale: total_scale,
            color,
            ..Default::default()
        },
    );
}

fn draw_retro_text(text: &str, x: f32, y: f32, world_size: f32, font: &Font, color: Color) {
    let (fs, scale, aspect) = camera_font_scale(world_size);
    draw_text_ex(
        text,
        x,
        y,
        TextParams {
            font: Some(font),
            font_size: fs,
            font_scale: scale * aspect,
            color,
            ..Default::default()
        },
    );
}

/// Word-wrap text to fit within max_width world units, then draw each line.
fn draw_retro_text_wrapped(
    text: &str, x: f32, y: f32, world_size: f32, max_width: f32,
    line_spacing: f32, font: &Font, color: Color,
) {
    let (fs, scale, aspect) = camera_font_scale(world_size);
    let total_scale = scale * aspect;
    let params = TextParams {
        font: Some(font),
        font_size: fs,
        font_scale: total_scale,
        color,
        ..Default::default()
    };

    let mut lines: Vec<String> = Vec::new();
    let mut current_line = String::new();

    for word in text.split_whitespace() {
        let candidate = if current_line.is_empty() {
            word.to_string()
        } else {
            format!("{current_line} {word}")
        };
        let m = measure_text(&candidate, Some(font), fs, total_scale);
        if m.width > max_width && !current_line.is_empty() {
            lines.push(current_line);
            current_line = word.to_string();
        } else {
            current_line = candidate;
        }
    }
    if !current_line.is_empty() {
        lines.push(current_line);
    }

    for (i, line) in lines.iter().enumerate() {
        draw_text_ex(line, x, y + i as f32 * line_spacing, params.clone());
    }
}

fn draw_parallax(tex: &Texture2D, cam_x: f32, rate: f32) {
    let tw = tex.width();
    let offset = ((cam_x * rate) % tw + tw) % tw;
    draw_texture(tex, -offset, 0.0, WHITE);
    draw_texture(tex, tw - offset, 0.0, WHITE);
}

fn draw_tile(tileset: &Texture2D, idx: u16, wx: f32, wy: f32) {
    let col = (idx % TILESET_COLS) as f32;
    let row = (idx / TILESET_COLS) as f32;
    draw_texture_ex(
        tileset,
        wx,
        wy,
        WHITE,
        DrawTextureParams {
            source: Some(Rect::new(
                col * TILE_SIZE,
                row * TILE_SIZE,
                TILE_SIZE,
                TILE_SIZE,
            )),
            ..Default::default()
        },
    );
}

fn draw_char_frame(tex: &Texture2D, x: f32, y: f32, frame: u32, flip_x: bool) {
    let col = (frame % CHAR_COLS) as f32;
    let row = (frame / CHAR_COLS) as f32;
    draw_texture_ex(
        tex,
        x - CHAR_FRAME_W / 2.0,
        y - CHAR_FRAME_H / 2.0,
        WHITE,
        DrawTextureParams {
            source: Some(Rect::new(
                col * CHAR_FRAME_W,
                row * CHAR_FRAME_H,
                CHAR_FRAME_W,
                CHAR_FRAME_H,
            )),
            flip_x,
            ..Default::default()
        },
    );
}

/// Draw decoration with bottom-center origin (matches Phaser origin 0.5, 1)
fn draw_deco(tex: &Texture2D, x: f32, y: f32) {
    draw_texture(tex, x - tex.width() / 2.0, y - tex.height(), WHITE);
}

fn draw_tilemap_visible(tilemap: &Tilemap, tileset: &Texture2D, cam_x: f32) {
    let start_col = ((cam_x / TILE_SIZE) as i32).max(0) as usize;
    let end_col =
        (((cam_x + VIRTUAL_W) / TILE_SIZE) as i32 + 1).min(tilemap.width as i32) as usize;
    let start_row = ((CAM_Y / TILE_SIZE) as i32).max(0) as usize;
    let end_row =
        (((CAM_Y + VIRTUAL_H) / TILE_SIZE) as i32 + 1).min(tilemap.height as i32) as usize;

    for y in start_row..end_row {
        for x in start_col..end_col {
            if let Some(idx) = tilemap.get(x, y) {
                draw_tile(tileset, idx, x as f32 * TILE_SIZE, y as f32 * TILE_SIZE);
            }
        }
    }
}

fn draw_decorations(assets: &Assets) {
    let left_ground = 6.0 * TILE_SIZE; // y=144
    let right_ground = 5.0 * TILE_SIZE; // y=120

    draw_deco(&assets.deco_lamp, DECO_OFFSET + 20.0, left_ground);
    draw_deco(&assets.deco_fence1, DECO_OFFSET + 90.0, left_ground);
    draw_deco(&assets.deco_grass1, DECO_OFFSET + 50.0, left_ground);
    draw_deco(&assets.deco_grass2, DECO_OFFSET + 140.0, left_ground);
    draw_deco(&assets.deco_sign, DECO_OFFSET + 270.0, right_ground);
    draw_deco(&assets.deco_grass3, DECO_OFFSET + 250.0, right_ground);
}

fn draw_inventory_overlay(font: &Font) {
    draw_rectangle(20.0, 20.0, 280.0, 140.0, Color::new(0.0, 0.0, 0.0, 0.8));
    draw_rectangle_lines(20.0, 20.0, 280.0, 140.0, 1.0, WHITE);

    draw_centered_text("Inventory", 160.0, 34.0, 12.0, font, WHITE);

    for row in 0..2 {
        for col in 0..4 {
            let x = 64.0 + col as f32 * 40.0;
            let y = 52.0 + row as f32 * 40.0;
            draw_rectangle(x, y, 32.0, 32.0, Color::new(0.2, 0.2, 0.2, 1.0));
            draw_rectangle_lines(x, y, 32.0, 32.0, 1.0, Color::new(0.4, 0.4, 0.4, 1.0));
        }
    }

    draw_centered_text(
        "Press I to close",
        160.0,
        150.0,
        6.0,
        font,
        Color::new(0.53, 0.53, 0.53, 1.0),
    );
}

fn draw_dialog_overlay(lines: &[String], index: usize, time: f32, font: &Font) {
    let bx = 10.0;
    let by = 133.0;
    let bw = 300.0;
    let bh = 44.0;

    draw_rectangle(bx, by, bw, bh, Color::new(0.0, 0.0, 0.0, 0.85));
    draw_rectangle_lines(bx, by, bw, bh, 1.0, WHITE);

    if index < lines.len() {
        draw_retro_text_wrapped(
            &lines[index], 18.0, 140.0, 6.0, bw - 16.0, 10.0, font, WHITE,
        );
    }

    // Blinking advance prompt
    let alpha = if (time * 2.0 * std::f32::consts::PI / 1.0).sin() > 0.0 {
        1.0
    } else {
        0.3
    };
    draw_retro_text(
        ">",
        298.0,
        172.0,
        8.0,
        font,
        Color::new(0.8, 0.8, 0.8, alpha),
    );
}

fn draw_pause_overlay(selected: usize, font: &Font) {
    let items = ["Save", "Help", "Quit to Main"];
    draw_rectangle(60.0, 40.0, 200.0, 100.0, Color::new(0.0, 0.0, 0.0, 0.85));
    draw_rectangle_lines(60.0, 40.0, 200.0, 100.0, 1.0, WHITE);

    draw_centered_text("Paused", 160.0, 54.0, 10.0, font, WHITE);

    for (i, label) in items.iter().enumerate() {
        let color = if i == selected { WHITE } else { Color::new(0.67, 0.67, 0.67, 1.0) };
        let text = if i == selected {
            format!("> {label}")
        } else {
            label.to_string()
        };
        draw_centered_text(&text, 160.0, 76.0 + i as f32 * 16.0, 7.0, font, color);
    }
}

// ── Main ────────────────────────────────────────────────────────────────────

fn window_conf() -> Conf {
    Conf {
        window_title: "Oak Woods".to_string(),
        window_width: 960,   // 16:9 matching 320:180
        window_height: 540,
        window_resizable: true,
        high_dpi: false,
        ..Default::default()
    }
}

/// Camera showing a fixed 320×180 world rect (for backgrounds, UI, overlays).
fn fixed_cam() -> Camera2D {
    Camera2D {
        zoom: vec2(2.0 / VIRTUAL_W, 2.0 / VIRTUAL_H),
        target: vec2(VIRTUAL_W / 2.0, VIRTUAL_H / 2.0),
        ..Default::default()
    }
}

/// Camera showing a scrolling 320×180 world rect at (cam_x, CAM_Y).
fn world_cam(cam_x: f32) -> Camera2D {
    Camera2D {
        zoom: vec2(2.0 / VIRTUAL_W, 2.0 / VIRTUAL_H),
        target: vec2(cam_x + VIRTUAL_W / 2.0, CAM_Y + VIRTUAL_H / 2.0),
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let assets = Assets::load().await;
    let tilemap = Tilemap::new();

    let intro_lines = vec![
        "You awaken in the Oak Woods, a forgotten forest.".into(),
        "The trees whisper of ancient ruins nearby.".into(),
        "Arrow keys to move, Space to jump, Z to attack.".into(),
        "Press I to open your inventory.".into(),
        "Now go, brave knight. The woods await.".into(),
    ];

    let spawn_x = DECO_OFFSET + 80.0;
    let mut scene = Scene::Title {
        blink_timer: 0.0,
    };
    let mut time: f32 = 0.0;

    loop {
        let dt = get_frame_time().min(0.05);
        time += dt;

        // Clear entire screen (letterbox bars stay black)
        set_default_camera();
        clear_background(BLACK);

        let mut next_scene: Option<Scene> = None;

        match &mut scene {
            // ── Title Scene ─────────────────────────────────────────────
            Scene::Title { blink_timer } => {
                *blink_timer += dt;

                set_camera(&fixed_cam());

                // Backgrounds
                draw_texture(&assets.bg1, 0.0, 0.0, WHITE);
                draw_texture(&assets.bg2, 0.0, 0.0, WHITE);
                draw_texture(&assets.bg3, 0.0, 0.0, WHITE);

                // Title
                draw_centered_text("Oak Woods", 160.0, 60.0, 16.0, &assets.font, WHITE);

                // Blinking prompt
                let alpha = if (*blink_timer * 1.25).sin() > 0.0 {
                    1.0
                } else {
                    0.0
                };
                let c = Color::new(0.8, 0.8, 0.8, alpha);
                draw_centered_text("Press any key", 160.0, 120.0, 8.0, &assets.font, c);

                if get_last_key_pressed().is_some() {
                    next_scene = Some(Scene::Menu { selected: 0 });
                }
            }

            // ── Menu Scene ──────────────────────────────────────────────
            Scene::Menu { selected } => {
                set_camera(&fixed_cam());

                draw_texture(&assets.bg1, 0.0, 0.0, WHITE);
                draw_texture(&assets.bg2, 0.0, 0.0, WHITE);
                draw_texture(&assets.bg3, 0.0, 0.0, WHITE);

                draw_centered_text("Oak Woods", 160.0, 40.0, 16.0, &assets.font, WHITE);

                let mut items: Vec<&str> = vec!["New Game", "Continue", "Options"];
                #[cfg(not(target_arch = "wasm32"))]
                items.push("Quit Game");

                for (i, label) in items.iter().enumerate() {
                    let color = if i == *selected {
                        WHITE
                    } else {
                        Color::new(0.67, 0.67, 0.67, 1.0)
                    };
                    let text = if i == *selected {
                        format!("> {label}")
                    } else {
                        label.to_string()
                    };
                    draw_centered_text(&text, 160.0, 80.0 + i as f32 * 18.0, 8.0, &assets.font, color);
                }

                if is_key_pressed(KeyCode::Up) {
                    *selected = (*selected + items.len() - 1) % items.len();
                }
                if is_key_pressed(KeyCode::Down) {
                    *selected = (*selected + 1) % items.len();
                }
                if is_key_pressed(KeyCode::Enter) || is_key_pressed(KeyCode::Space) {
                    match *selected {
                        0 | 1 => {
                            next_scene = Some(Scene::Game {
                                player: Player::new(spawn_x, 100.0),
                                cam_x: spawn_x - VIRTUAL_W / 2.0,
                                overlay: Some(Overlay::Dialog {
                                    lines: intro_lines.clone(),
                                    index: 0,
                                }),
                            });
                        }
                        3 => {
                            #[cfg(not(target_arch = "wasm32"))]
                            std::process::exit(0);
                        }
                        _ => {} // Options: no-op
                    }
                }
            }

            // ── Game Scene ──────────────────────────────────────────────
            Scene::Game {
                player,
                cam_x,
                overlay,
            } => {
                let paused = overlay.is_some();

                // Handle overlay input — compute next state without borrowing overlay
                let mut overlay_action: Option<Option<Overlay>> = None;
                if let Some(ov) = overlay.as_mut() {
                    match ov {
                        Overlay::Inventory => {
                            if is_key_pressed(KeyCode::I) || is_key_pressed(KeyCode::Escape) {
                                overlay_action = Some(None);
                            }
                        }
                        Overlay::Dialog { lines, index } => {
                            if is_key_pressed(KeyCode::Space) || is_key_pressed(KeyCode::Enter) {
                                *index += 1;
                                if *index >= lines.len() {
                                    overlay_action = Some(None);
                                }
                            }
                            if is_key_pressed(KeyCode::Escape) {
                                overlay_action = Some(None);
                            }
                        }
                        Overlay::Pause { selected } => {
                            if is_key_pressed(KeyCode::Up) {
                                *selected = (*selected + 2) % 3;
                            }
                            if is_key_pressed(KeyCode::Down) {
                                *selected = (*selected + 1) % 3;
                            }
                            if is_key_pressed(KeyCode::Escape) {
                                overlay_action = Some(None);
                            }
                            if is_key_pressed(KeyCode::Enter) || is_key_pressed(KeyCode::Space) {
                                match *selected {
                                    0 => { /* Save: no-op for now */ }
                                    1 => {
                                        overlay_action = Some(Some(Overlay::Dialog {
                                            lines: vec![
                                                "Arrow keys to move, Space to jump, Z to attack.".into(),
                                                "Press I for inventory, T for dialog, Esc to pause.".into(),
                                            ],
                                            index: 0,
                                        }));
                                    }
                                    2 => {
                                        next_scene = Some(Scene::Menu { selected: 0 });
                                        overlay_action = Some(None);
                                    }
                                    _ => {}
                                }
                            }
                        }
                    }
                }
                if let Some(new_overlay) = overlay_action {
                    *overlay = new_overlay;
                }

                // Update game if not paused
                if !paused {
                    update_player(player, &tilemap, dt);

                    // Camera follow (horizontal only, lerp 0.1)
                    let target_x = player.x - VIRTUAL_W / 2.0;
                    *cam_x += (target_x - *cam_x) * 0.1;
                    *cam_x = cam_x.clamp(0.0, MAP_W as f32 * TILE_SIZE - VIRTUAL_W);

                    // Pause menu
                    if is_key_pressed(KeyCode::Escape) {
                        *overlay = Some(Overlay::Pause { selected: 0 });
                    }

                    // Open inventory
                    if is_key_pressed(KeyCode::I) {
                        *overlay = Some(Overlay::Inventory);
                    }

                    // Test dialog (T key)
                    if is_key_pressed(KeyCode::T) {
                        *overlay = Some(Overlay::Dialog {
                            lines: vec![
                                "Welcome to Oak Woods, traveler.".into(),
                                "Beware the creatures in the shadows...".into(),
                                "Good luck on your journey!".into(),
                            ],
                            index: 0,
                        });
                    }
                }

                // ── Draw backgrounds (fixed camera) ─────────────────────
                set_camera(&fixed_cam());

                draw_parallax(&assets.bg1, *cam_x, 0.0);
                draw_parallax(&assets.bg2, *cam_x, 0.3);
                draw_parallax(&assets.bg3, *cam_x, 0.6);

                // ── Draw world (scrolling camera) ───────────────────────
                set_camera(&world_cam(*cam_x));

                draw_tilemap_visible(&tilemap, &assets.tileset, *cam_x);
                draw_decorations(&assets);

                // Player
                draw_char_frame(
                    &assets.char_sheet,
                    player.x,
                    player.y,
                    player.anim.current_frame(),
                    player.flip_x,
                );

                // ── Draw overlays (fixed camera) ────────────────────────
                if let Some(ov) = overlay {
                    set_camera(&fixed_cam());
                    match ov {
                        Overlay::Inventory => draw_inventory_overlay(&assets.font),
                        Overlay::Dialog { lines, index } => {
                            draw_dialog_overlay(lines, *index, time, &assets.font);
                        }
                        Overlay::Pause { selected } => {
                            draw_pause_overlay(*selected, &assets.font);
                        }
                    }
                }
            }
        }

        // Apply scene transition
        if let Some(s) = next_scene {
            scene = s;
        }

        next_frame().await;
    }
}
