#![allow(dead_code)]
use macroquad::prelude::*;
use crate::types::{Astra, AstraProjectileType, CutsceneType, PlayerStats};

pub const SCREEN_W: f32 = 640.0;
pub const SCREEN_H: f32 = 360.0;

// Arena bounds for 16:9 widescreen (640x360)
pub const ARENA_MIN_X: f32 = 44.0;
pub const ARENA_MAX_X: f32 = 596.0;
pub const ARENA_MIN_Y: f32 = 38.0;
pub const ARENA_MAX_Y: f32 = 324.0;

// ============================================================================
// AUTHENTIC 16-BIT VEDIC RPG PALETTE (FROM RAMAYANA YATRA)
// ============================================================================

// Prince Rama (Shyamavarna - Divine Blue)
pub const PAL_RAMA_0: Color = Color::new(0.54, 0.71, 0.86, 1.0); // #8ab4dc highlight
pub const PAL_RAMA_1: Color = Color::new(0.37, 0.53, 0.75, 1.0); // #5e86c0 midtone
pub const PAL_RAMA_2: Color = Color::new(0.24, 0.37, 0.61, 1.0); // #3e5e9c shadow
pub const PAL_RAMA_3: Color = Color::new(0.16, 0.24, 0.42, 1.0); // #283c6c contour

// Gold & Celestial Ornaments
pub const PAL_GOLD_0: Color = Color::new(1.00, 0.96, 0.69, 1.0); // #fff6b0 highlight
pub const PAL_GOLD_1: Color = Color::new(1.00, 0.85, 0.38, 1.0); // #ffd860 gold
pub const PAL_GOLD_2: Color = Color::new(0.94, 0.66, 0.19, 1.0); // #f0a830 deep gold
pub const PAL_GOLD_3: Color = Color::new(0.72, 0.45, 0.13, 1.0); // #b87420 dark contour

// Pitambara / Turmeric Dhoti (Sacred Yellow)
pub const PAL_TURMERIC_0: Color = Color::new(1.00, 0.94, 0.54, 1.0); // #fff08a
pub const PAL_TURMERIC_1: Color = Color::new(0.97, 0.82, 0.25, 1.0); // #f8d040
pub const PAL_TURMERIC_2: Color = Color::new(0.88, 0.64, 0.16, 1.0); // #e0a428
pub const PAL_TURMERIC_3: Color = Color::new(0.66, 0.45, 0.11, 1.0); // #a8741c

// Tataka (Rakshasa Purple)
pub const PAL_PURPLE_0: Color = Color::new(0.75, 0.55, 0.82, 1.0); // #c08cd0 highlight
pub const PAL_PURPLE_1: Color = Color::new(0.56, 0.38, 0.67, 1.0); // #9060ac midtone
pub const PAL_PURPLE_2: Color = Color::new(0.41, 0.24, 0.53, 1.0); // #683e88 body
pub const PAL_PURPLE_3: Color = Color::new(0.27, 0.14, 0.38, 1.0); // #442462 shadow
pub const PAL_PURPLE_4: Color = Color::new(0.16, 0.08, 0.25, 1.0); // #2a1440 contour

// Subahu (Rakshasa Red / Iron armor)
pub const PAL_RED_0: Color = Color::new(0.91, 0.55, 0.45, 1.0); // #e88c74
pub const PAL_RED_1: Color = Color::new(0.75, 0.36, 0.31, 1.0); // #c05c50
pub const PAL_RED_2: Color = Color::new(0.55, 0.23, 0.24, 1.0); // #8c3a3c
pub const PAL_RED_3: Color = Color::new(0.36, 0.14, 0.19, 1.0); // #5c2430

// Maricha (Rakshasa Blue / Illusionist Indigo)
pub const PAL_BLUE_0: Color = Color::new(0.55, 0.61, 0.80, 1.0); // #8c9ccc
pub const PAL_BLUE_1: Color = Color::new(0.37, 0.42, 0.66, 1.0); // #5e6ca8
pub const PAL_BLUE_2: Color = Color::new(0.25, 0.29, 0.50, 1.0); // #404a80
pub const PAL_BLUE_3: Color = Color::new(0.16, 0.19, 0.35, 1.0); // #2a3058

// Sandstone, Mud & Ancient Vedic Ruins (Malada & Karusha)
pub const PAL_SAND_0: Color = Color::new(0.99, 0.88, 0.75, 1.0); // #fce0c0
pub const PAL_SAND_1: Color = Color::new(0.94, 0.74, 0.55, 1.0); // #f0bc8c
pub const PAL_SAND_2: Color = Color::new(0.85, 0.58, 0.42, 1.0); // #d8946a
pub const PAL_SAND_3: Color = Color::new(0.69, 0.41, 0.30, 1.0); // #b0684c
pub const PAL_SAND_4: Color = Color::new(0.48, 0.27, 0.20, 1.0); // #7a4434

// Sacred Agni Fire (6-stop divine gradient)
pub const PAL_FIRE_0: Color = Color::new(1.00, 0.98, 0.88, 1.0); // #fffbe0 white core
pub const PAL_FIRE_1: Color = Color::new(1.00, 0.94, 0.50, 1.0); // #fff080 radiant yellow
pub const PAL_FIRE_2: Color = Color::new(1.00, 0.75, 0.19, 1.0); // #ffc030 gold
pub const PAL_FIRE_3: Color = Color::new(1.00, 0.48, 0.13, 1.0); // #ff7a20 flame orange
pub const PAL_FIRE_4: Color = Color::new(0.85, 0.25, 0.11, 1.0); // #d8401c crimson
pub const PAL_FIRE_5: Color = Color::new(0.55, 0.13, 0.09, 1.0); // #8c2018 deep ember

// Saffron Valkala (Sage Robes & Dhvaja Flags)
pub const PAL_SAFFRON_0: Color = Color::new(1.00, 0.82, 0.48, 1.0); // #ffd27a
pub const PAL_SAFFRON_1: Color = Color::new(1.00, 0.66, 0.24, 1.0); // #ffa83c
pub const PAL_SAFFRON_2: Color = Color::new(0.91, 0.48, 0.14, 1.0); // #e87a24
pub const PAL_SAFFRON_3: Color = Color::new(0.69, 0.32, 0.11, 1.0); // #b0521c

// Verdant Foliage & Vedic Earth
pub const PAL_GRASS_0: Color = Color::new(0.71, 0.88, 0.44, 1.0); // #b4e070 sunlit blade
pub const PAL_GRASS_1: Color = Color::new(0.53, 0.78, 0.33, 1.0); // #86c654 meadow
pub const PAL_GRASS_2: Color = Color::new(0.37, 0.65, 0.27, 1.0); // #5ea544 forest moss
pub const PAL_GRASS_3: Color = Color::new(0.24, 0.49, 0.23, 1.0); // #3d7d3a shade
pub const PAL_GRASS_4: Color = Color::new(0.16, 0.35, 0.19, 1.0); // #285a30 deep undergrowth

// Legacy Aliases
pub const COLOR_RAMA_SKIN: Color = PAL_RAMA_1;
pub const COLOR_RAMA_DHOTI: Color = PAL_TURMERIC_1;
pub const COLOR_RAMA_GOLD: Color = PAL_GOLD_1;
pub const COLOR_TATAKA_SKIN: Color = PAL_PURPLE_2;
pub const COLOR_TATAKA_EYES: Color = Color::new(1.0, 0.25, 0.25, 1.0);

// Helper for crisp pixel text rendering with drop shadow
pub fn draw_pixel_text(font: &Font, text: &str, x: f32, y: f32, size: u16, color: Color) {
    let shadow_color = Color::new(0.0, 0.0, 0.0, color.a * 0.95);
    draw_text_ex(
        text,
        x + 1.0,
        y + 1.0,
        TextParams {
            font: Some(font),
            font_size: size,
            font_scale: 1.0,
            color: shadow_color,
            ..Default::default()
        },
    );
    draw_text_ex(
        text,
        x,
        y,
        TextParams {
            font: Some(font),
            font_size: size,
            font_scale: 1.0,
            color,
            ..Default::default()
        },
    );
}

/// Robust pixel text word-wrapping to guarantee text never overflows bounding boxes.
pub fn wrap_pixel_text(font: &Font, text: &str, font_size: u16, max_width: f32) -> Vec<String> {
    let mut lines = Vec::new();
    for paragraph in text.split('\n') {
        let words: Vec<&str> = paragraph.split_whitespace().collect();
        let mut current_line = String::new();

        for word in words {
            let test_line = if current_line.is_empty() {
                word.to_string()
            } else {
                format!("{} {}", current_line, word)
            };
            let dims = measure_text(&test_line, Some(font), font_size, 1.0);
            let approx_w = test_line.len() as f32 * (font_size as f32);
            let w = if dims.width > 0.0 { dims.width } else { approx_w };

            if w > max_width && !current_line.is_empty() {
                lines.push(current_line);
                current_line = word.to_string();
            } else {
                current_line = test_line;
            }
        }
        if !current_line.is_empty() {
            lines.push(current_line);
        }
    }
    if lines.is_empty() && !text.is_empty() {
        lines.push(text.to_string());
    }
    lines
}

pub struct Particle {
    pub pos: Vec2,
    pub vel: Vec2,
    pub color: Color,
    pub size: f32,
    pub lifetime: f32,
    pub max_lifetime: f32,
}

impl Particle {
    pub fn update(&mut self, dt: f32) {
        self.pos += self.vel * dt;
        self.lifetime -= dt;
    }

    pub fn is_alive(&self) -> bool {
        self.lifetime > 0.0
    }

    pub fn draw(&self) {
        let alpha = (self.lifetime / self.max_lifetime).clamp(0.0, 1.0);
        let mut col = self.color;
        col.a *= alpha;
        draw_rectangle(
            self.pos.x - self.size * 0.5,
            self.pos.y - self.size * 0.5,
            self.size,
            self.size,
            col,
        );
    }
}

pub struct FloatingText {
    pub pos: Vec2,
    pub text: String,
    pub color: Color,
    pub lifetime: f32,
    pub max_lifetime: f32,
    pub font_size: u16,
}

impl FloatingText {
    pub fn update(&mut self, dt: f32) {
        self.pos.y -= 22.0 * dt;
        self.lifetime -= dt;
    }

    pub fn is_alive(&self) -> bool {
        self.lifetime > 0.0
    }

    pub fn draw(&self, font: &Font) {
        let alpha = (self.lifetime / self.max_lifetime).clamp(0.0, 1.0);
        let mut col = self.color;
        col.a *= alpha;
        draw_pixel_text(font, &self.text, self.pos.x, self.pos.y, self.font_size, col);
    }
}

// ============================================================================
// BATTLEFIELD RENDERING: TATAKAVANA (THE CURSED VEDIC FOREST)
// ============================================================================
pub fn draw_battlefield(time: f32, sunset_timer: f32) {
    let dusk_factor = (1.0 - (sunset_timer / 90.0)).clamp(0.0, 1.0);

    // 1. Base Layer: Earth, Loam & Grass Grid (32x32 tiles, 20 cols x 12 rows)
    let tile_size = 32.0;
    let cols = (SCREEN_W / tile_size) as usize; // 20
    let rows = (SCREEN_H / tile_size) as usize; // 11.25 -> 12

    for y in 0..rows {
        for x in 0..cols {
            let px = x as f32 * tile_size;
            let py = y as f32 * tile_size;

            // Rich natural earth variation with dirt and withered foliage
            let hash = (x * 17 + y * 37) % 11;
            let base_col = match hash {
                0 | 1 => Color::new(0.22, 0.29, 0.14, 1.0), // Deep forest moss
                2 | 3 => Color::new(0.25, 0.32, 0.16, 1.0), // Lush woodland patch
                4 | 5 => Color::new(0.28, 0.26, 0.15, 1.0), // Dry loamy earth
                6 | 7 => Color::new(0.24, 0.22, 0.13, 1.0), // Hardpacked soil
                8 => Color::new(0.20, 0.19, 0.12, 1.0),     // Dark withered soil
                _ => Color::new(0.23, 0.30, 0.15, 1.0),     // Standard clearing grass
            };

            // Interpolate towards dusk twilight
            let tinted = Color::new(
                base_col.r + dusk_factor * 0.10,
                base_col.g - dusk_factor * 0.08,
                base_col.b - dusk_factor * 0.04,
                1.0,
            );
            draw_rectangle(px, py, tile_size, tile_size, tinted);

            // Subtle soil texture specks
            if hash % 3 == 0 {
                draw_rectangle(px + 6.0, py + 14.0, 2.0, 2.0, Color::new(0.18, 0.16, 0.10, 0.6));
                draw_rectangle(px + 18.0, py + 8.0, 2.0, 2.0, Color::new(0.30, 0.28, 0.18, 0.5));
            }
        }
    }

    // 2. Winding Dirt & Stone Pilgrim Path (traversing east-west across the hermitage)
    draw_earthen_path();

    // 3. Ancient Sandstone Flagstones & Vedic Pavers
    draw_stone_pavers();

    // 4. Scorched Earth & Demon Ash Patches (Tataka's sorcery)
    draw_scorched_earth_patches(time);

    // 5. Wildflowers, Forest Herbs & Autumn Fallen Leaves
    draw_wilderness_flora(time);

    // 6. Battlefield Relics: Overgrown Ruins of Ancient Malada & Karusha
    // Collapsed Sandstone Gateway of Malada in the upper clearing
    draw_ruined_gateway(110.0, 85.0);

    // Ruined Overgrown Plinth of Karusha on the rocky upper-right clearing
    draw_ruined_plinth(510.0, 90.0);

    // Ancient Ruined Stone Pillars with Devanagari carvings
    draw_stone_pillar(190.0, 255.0);
    draw_stone_pillar(450.0, 255.0);

    // Tataka's Devastation: Bleached Skeletons, Ribcage & Demon Claw Scratches
    draw_skeletal_remains(320.0, 115.0);
    draw_claw_scratches(360.0, 220.0);

    // Decaying Banyan Tree Stumps with Fungi & Tangled Roots
    draw_banyan_stump(75.0, 270.0);
    draw_banyan_stump(535.0, 270.0);

    // Craggy Granite Boulders
    draw_granite_boulder_cluster(240.0, 80.0);
    draw_granite_boulder_cluster(380.0, 280.0);

    // 7. Thick Forest Border Canopies (Top, Bottom, Left, Right)
    draw_forest_borders(cols, rows, tile_size, time);

    // 8. Low-lying Atmospheric Twilight Mist & Drifting Embers
    draw_twilight_mist(time, dusk_factor);

    // 9. Ambient Sunset / Frenzy Overlay
    if sunset_timer <= 0.0 {
        let pulse = (time * 4.0).sin() * 0.05 + 0.16;
        draw_rectangle(0.0, 0.0, SCREEN_W, SCREEN_H, Color::new(0.65, 0.04, 0.08, pulse));
    } else if sunset_timer < 25.0 {
        let t = (25.0 - sunset_timer) / 25.0;
        draw_rectangle(0.0, 0.0, SCREEN_W, SCREEN_H, Color::new(0.55, 0.12, 0.0, t * 0.14));
    } else {
        draw_rectangle(0.0, 0.0, SCREEN_W, SCREEN_H, Color::new(0.40, 0.20, 0.0, dusk_factor * 0.10));
    }
}

// Winding dirt path connecting across the hermitage
fn draw_earthen_path() {
    let path_color = Color::new(0.36, 0.30, 0.18, 0.85);
    let path_edge = Color::new(0.30, 0.24, 0.14, 0.7);

    // Path segments
    let path_points = [
        vec2(44.0, 195.0),
        vec2(130.0, 185.0),
        vec2(220.0, 175.0),
        vec2(320.0, 180.0),
        vec2(420.0, 165.0),
        vec2(510.0, 175.0),
        vec2(596.0, 190.0),
    ];

    for i in 0..(path_points.len() - 1) {
        let p1 = path_points[i];
        let p2 = path_points[i + 1];
        draw_line(p1.x, p1.y, p2.x, p2.y, 22.0, path_edge);
        draw_line(p1.x, p1.y, p2.x, p2.y, 18.0, path_color);

        // Path pebbles
        draw_circle(p1.x + 10.0, p1.y - 4.0, 2.0, Color::new(0.45, 0.42, 0.35, 0.7));
        draw_circle(p1.x + 24.0, p1.y + 3.0, 1.5, Color::new(0.42, 0.38, 0.30, 0.7));
    }
}

// Ancient mossy stone slabs embedded in the earth
fn draw_stone_pavers() {
    let paver_col = Color::new(0.46, 0.48, 0.44, 0.85);
    let crack_col = Color::new(0.28, 0.29, 0.26, 0.9);
    let moss_col = Color::new(0.22, 0.34, 0.16, 0.8);

    let pavers = [
        vec2(150.0, 120.0),
        vec2(168.0, 122.0),
        vec2(154.0, 138.0),
        vec2(172.0, 140.0),
        vec2(470.0, 130.0),
        vec2(488.0, 132.0),
        vec2(290.0, 220.0),
        vec2(310.0, 224.0),
        vec2(330.0, 218.0),
    ];

    for p in pavers {
        draw_rectangle(p.x, p.y, 16.0, 12.0, paver_col);
        draw_rectangle_lines(p.x, p.y, 16.0, 12.0, 1.0, crack_col);
        draw_line(p.x + 4.0, p.y + 2.0, p.x + 10.0, p.y + 9.0, 1.0, crack_col);
        draw_circle(p.x + 2.0, p.y + 10.0, 2.0, moss_col);
    }
}

// Scorched earth patches where demonic hellfire struck
fn draw_scorched_earth_patches(time: f32) {
    let ash_col = Color::new(0.12, 0.10, 0.10, 0.75);
    let ember_glow = (time * 5.0).sin() * 0.2 + 0.5;
    let ember_col = Color::new(0.85, 0.25, 0.05, ember_glow * 0.7);

    // Burn patch 1
    draw_ellipse(280.0, 145.0, 24.0, 10.0, 0.0, ash_col);
    draw_circle(282.0, 144.0, 2.0, ember_col);
    draw_circle(274.0, 146.0, 1.5, ember_col);

    // Burn patch 2
    draw_ellipse(420.0, 215.0, 28.0, 12.0, 0.0, ash_col);
    draw_circle(418.0, 214.0, 2.0, ember_col);
    draw_circle(426.0, 217.0, 1.5, ember_col);
}

// Forest flora: Ashoka blossoms, marigolds, reeds, fallen autumn leaves
fn draw_wilderness_flora(time: f32) {
    // Red Ashoka & Marigold flower clusters
    let flowers = [
        (vec2(180.0, 95.0), Color::new(0.95, 0.25, 0.25, 1.0)),
        (vec2(184.0, 98.0), Color::new(0.95, 0.75, 0.15, 1.0)),
        (vec2(210.0, 220.0), Color::new(0.90, 0.30, 0.50, 1.0)),
        (vec2(390.0, 110.0), Color::new(0.95, 0.75, 0.15, 1.0)),
        (vec2(440.0, 185.0), Color::new(0.95, 0.25, 0.25, 1.0)),
        (vec2(480.0, 235.0), Color::new(0.95, 0.75, 0.15, 1.0)),
        (vec2(130.0, 240.0), Color::new(0.90, 0.30, 0.50, 1.0)),
    ];

    for (pos, col) in flowers {
        draw_circle(pos.x, pos.y, 2.5, col);
        draw_circle(pos.x, pos.y, 1.0, WHITE);
    }

    // Tall thorny reeds / elephant grass
    let reed_col = Color::new(0.18, 0.28, 0.10, 0.9);
    let sway = (time * 3.0).sin() * 1.5;
    let reeds = [
        vec2(90.0, 150.0),
        vec2(250.0, 230.0),
        vec2(360.0, 90.0),
        vec2(520.0, 220.0),
    ];
    for pos in reeds {
        draw_line(pos.x, pos.y, pos.x + sway - 2.0, pos.y - 12.0, 2.0, reed_col);
        draw_line(pos.x + 3.0, pos.y, pos.x + sway + 2.0, pos.y - 14.0, 1.5, reed_col);
        draw_line(pos.x - 3.0, pos.y, pos.x + sway - 5.0, pos.y - 10.0, 1.5, reed_col);
    }

    // Fallen autumn leaves scattered on the ground
    let leaves = [
        (vec2(160.0, 170.0), Color::new(0.85, 0.45, 0.15, 0.75)),
        (vec2(230.0, 130.0), Color::new(0.75, 0.25, 0.12, 0.75)),
        (vec2(340.0, 200.0), Color::new(0.85, 0.65, 0.15, 0.75)),
        (vec2(400.0, 160.0), Color::new(0.70, 0.20, 0.10, 0.75)),
        (vec2(470.0, 205.0), Color::new(0.85, 0.45, 0.15, 0.75)),
    ];
    for (pos, col) in leaves {
        draw_rectangle(pos.x, pos.y, 3.0, 2.0, col);
    }
}

// Bleached skeletal ribcage and skull of Tataka's demonic prey
fn draw_skeletal_remains(x: f32, y: f32) {
    let bone_col = Color::new(0.88, 0.86, 0.78, 0.9);
    let shadow = Color::new(0.0, 0.0, 0.0, 0.35);

    // Drop shadow
    draw_ellipse(x, y + 4.0, 14.0, 5.0, 0.0, shadow);

    // Spine
    draw_line(x - 10.0, y, x + 8.0, y, 2.0, bone_col);

    // Rib bones
    for i in -2..=2 {
        let rx = x + i as f32 * 4.0;
        draw_line(rx, y - 5.0, rx, y + 4.0, 1.5, bone_col);
    }

    // Skull
    draw_circle(x - 12.0, y - 1.0, 4.0, bone_col);
    draw_circle(x - 13.0, y - 2.0, 1.2, Color::new(0.2, 0.15, 0.12, 1.0)); // eye socket
}

// Claw marks gouged into the earth
fn draw_claw_scratches(x: f32, y: f32) {
    let scratch_col = Color::new(0.12, 0.08, 0.06, 0.7);
    draw_line(x - 8.0, y - 6.0, x + 4.0, y + 4.0, 1.5, scratch_col);
    draw_line(x - 5.0, y - 7.0, x + 7.0, y + 3.0, 1.5, scratch_col);
    draw_line(x - 2.0, y - 8.0, x + 10.0, y + 2.0, 1.5, scratch_col);
}

// Gnarled old banyan tree stump with fungi
fn draw_banyan_stump(x: f32, y: f32) {
    let bark_col = Color::new(0.28, 0.18, 0.11, 1.0);
    let top_col = Color::new(0.38, 0.28, 0.18, 1.0);
    let fungi_col = Color::new(0.85, 0.75, 0.45, 0.9);

    // Shadow
    draw_ellipse(x, y + 8.0, 14.0, 5.0, 0.0, Color::new(0.0, 0.0, 0.0, 0.4));

    // Stump body & roots
    draw_rectangle(x - 9.0, y - 10.0, 18.0, 16.0, bark_col);
    draw_line(x - 9.0, y + 4.0, x - 15.0, y + 8.0, 3.0, bark_col); // root left
    draw_line(x + 9.0, y + 4.0, x + 15.0, y + 8.0, 3.0, bark_col); // root right

    // Stump cut top
    draw_ellipse(x, y - 10.0, 9.0, 4.0, 0.0, top_col);
    draw_circle_lines(x, y - 10.0, 4.0, 1.0, bark_col); // growth ring

    // Bracket fungi
    draw_ellipse(x + 9.0, y - 2.0, 4.0, 2.0, 0.0, fungi_col);
    draw_ellipse(x - 9.0, y + 2.0, 3.5, 1.8, 0.0, fungi_col);
}

// Collapsed Ancient Sandstone Gateway of Malada
fn draw_ruined_gateway(x: f32, y: f32) {
    let stone_dark = Color::new(0.38, 0.35, 0.32, 1.0);
    let stone_mid = Color::new(0.48, 0.45, 0.40, 1.0);
    let stone_light = Color::new(0.60, 0.57, 0.50, 1.0);
    let moss_col = Color::new(0.25, 0.35, 0.18, 0.85);

    // Drop shadows
    draw_ellipse(x - 14.0, y + 8.0, 12.0, 5.0, 0.0, Color::new(0.0, 0.0, 0.0, 0.4));
    draw_ellipse(x + 14.0, y + 8.0, 10.0, 4.5, 0.0, Color::new(0.0, 0.0, 0.0, 0.4));
    draw_ellipse(x, y + 4.0, 18.0, 4.0, 0.0, Color::new(0.0, 0.0, 0.0, 0.3));

    // Left gate pillar (standing, weathered)
    draw_rectangle(x - 18.0, y - 22.0, 10.0, 28.0, stone_mid);
    draw_rectangle(x - 20.0, y - 24.0, 14.0, 4.0, stone_light); // capital
    draw_rectangle(x - 20.0, y + 4.0, 14.0, 4.0, stone_dark);  // base
    draw_line(x - 15.0, y - 18.0, x - 13.0, y - 6.0, 1.2, stone_dark); // stone crack

    // Right gate pillar (partially collapsed stump)
    draw_rectangle(x + 10.0, y - 8.0, 10.0, 14.0, stone_mid);
    draw_rectangle(x + 8.0, y + 4.0, 14.0, 4.0, stone_dark); // base
    draw_triangle(vec2(x + 10.0, y - 8.0), vec2(x + 20.0, y - 8.0), vec2(x + 14.0, y - 14.0), stone_light); // jagged broken top

    // Fallen shattered lintel stone resting on the ground
    draw_line(x - 10.0, y + 2.0, x + 8.0, y + 6.0, 5.0, stone_dark);
    draw_line(x - 9.0, y + 1.0, x + 7.0, y + 5.0, 3.5, stone_mid);
    draw_line(x - 2.0, y, x + 3.0, y + 8.0, 1.2, Color::new(0.2, 0.18, 0.15, 1.0)); // break fracture

    // Creepers and moss climbing the ruins
    draw_circle(x - 18.0, y - 12.0, 2.5, moss_col);
    draw_circle(x - 17.0, y - 4.0, 3.0, moss_col);
    draw_circle(x + 14.0, y - 2.0, 2.2, moss_col);
    draw_circle(x, y + 6.0, 2.5, moss_col);
}

// Granite Boulder Clusters
fn draw_granite_boulder_cluster(x: f32, y: f32) {
    let main_col = Color::new(0.40, 0.42, 0.44, 1.0);
    let lit_col = Color::new(0.52, 0.55, 0.58, 1.0);
    let moss_col = Color::new(0.25, 0.35, 0.18, 0.7);

    // Drop shadow
    draw_ellipse(x, y + 8.0, 18.0, 6.0, 0.0, Color::new(0.0, 0.0, 0.0, 0.4));

    // Large boulder
    draw_circle(x, y, 12.0, main_col);
    draw_circle(x - 3.0, y - 3.0, 9.0, lit_col);
    draw_circle(x + 4.0, y + 3.0, 5.0, moss_col);

    // Small adjacent rock
    draw_circle(x + 12.0, y + 3.0, 6.0, main_col);
    draw_circle(x + 10.0, y + 1.0, 4.0, lit_col);
}

// Overgrown Ruined Plinth of Karusha (Devastated Pavilion Foundation)
fn draw_ruined_plinth(x: f32, y: f32) {
    let stone_base = Color::new(0.42, 0.40, 0.36, 1.0);
    let stone_step = Color::new(0.52, 0.50, 0.44, 1.0);
    let stone_top = Color::new(0.62, 0.60, 0.54, 1.0);
    let weed_col = Color::new(0.26, 0.36, 0.18, 0.9);

    // Shadow
    draw_ellipse(x, y + 12.0, 26.0, 8.0, 0.0, Color::new(0.0, 0.0, 0.0, 0.38));

    // Tier 1 (Broad ancient stone base)
    draw_rectangle(x - 22.0, y, 44.0, 10.0, stone_base);
    draw_rectangle_lines(x - 22.0, y, 44.0, 10.0, 1.0, Color::new(0.28, 0.26, 0.22, 1.0));

    // Tier 2 (Step)
    draw_rectangle(x - 16.0, y - 6.0, 32.0, 8.0, stone_step);

    // Tier 3 (Platform top with cracked flagstones)
    draw_rectangle(x - 10.0, y - 11.0, 20.0, 6.0, stone_top);
    draw_line(x - 4.0, y - 11.0, x + 2.0, y - 5.0, 1.2, Color::new(0.25, 0.23, 0.20, 1.0));

    // Toppled pillar segment lying on the plinth
    draw_rectangle(x + 8.0, y - 8.0, 12.0, 5.0, stone_step);
    draw_circle(x + 18.0, y - 5.5, 2.5, stone_base);

    // Weeds and wild briars growing through the ancient cracks
    draw_line(x - 12.0, y - 6.0, x - 14.0, y - 14.0, 1.5, weed_col);
    draw_line(x - 10.0, y - 6.0, x - 8.0, y - 12.0, 1.2, weed_col);
    draw_circle(x + 3.0, y - 1.0, 2.5, weed_col);
}

// Ancient Ruined Stone Pillars with Devanagari Carvings
fn draw_stone_pillar(x: f32, y: f32) {
    draw_ellipse(x, y + 12.0, 12.0, 5.0, 0.0, Color::new(0.0, 0.0, 0.0, 0.35));
    draw_rectangle(x - 10.0, y + 6.0, 20.0, 7.0, Color::new(0.42, 0.44, 0.45, 1.0));
    draw_rectangle(x - 8.0, y - 14.0, 16.0, 20.0, Color::new(0.55, 0.58, 0.60, 1.0));

    // Devanagari vertical glyph groove
    draw_line(x - 3.0, y - 10.0, x - 3.0, y + 2.0, 1.5, Color::new(0.35, 0.38, 0.40, 1.0));
    draw_line(x + 2.0, y - 6.0, x + 2.0, y + 3.0, 1.5, Color::new(0.35, 0.38, 0.40, 1.0));
    draw_rectangle(x - 12.0, y - 19.0, 24.0, 5.0, Color::new(0.65, 0.68, 0.70, 1.0));
}

// Thick Forest Border Trees
fn draw_forest_borders(cols: usize, rows: usize, tile_size: f32, time: f32) {
    let tree_dark = Color::new(0.08, 0.14, 0.07, 1.0);
    let tree_mid = Color::new(0.14, 0.25, 0.11, 1.0);
    let tree_light = Color::new(0.22, 0.38, 0.16, 1.0);
    let trunk_col = Color::new(0.28, 0.18, 0.11, 1.0);

    // Top border
    for x in 0..cols {
        let px = x as f32 * tile_size;
        let sway = ((x * 13) as f32 + time * 0.5).sin() * 1.5;
        draw_rectangle(px - 4.0, 4.0, 8.0, 18.0, trunk_col);
        draw_circle(px + sway, 6.0, 18.0, tree_dark);
        draw_circle(px + sway - 3.0, 4.0, 14.0, tree_mid);
        draw_circle(px + sway + 2.0, 2.0, 10.0, tree_light);
    }
    // Bottom border
    for x in 0..cols {
        let px = x as f32 * tile_size;
        let sway = ((x * 23) as f32 + time * 0.5).sin() * 1.5;
        draw_rectangle(px - 4.0, SCREEN_H - 18.0, 8.0, 18.0, trunk_col);
        draw_circle(px + sway, SCREEN_H - 8.0, 18.0, tree_dark);
        draw_circle(px + sway - 3.0, SCREEN_H - 10.0, 14.0, tree_mid);
        draw_circle(px + sway + 2.0, SCREEN_H - 12.0, 10.0, tree_light);
    }
    // Left & Right borders
    for y in 1..(rows - 1) {
        let py = y as f32 * tile_size;
        // Left
        let sway_l = ((y * 19) as f32 + time * 0.5).sin() * 1.5;
        draw_rectangle(6.0, py - 4.0, 14.0, 8.0, trunk_col);
        draw_circle(14.0 + sway_l, py, 16.0, tree_dark);
        draw_circle(12.0 + sway_l, py - 2.0, 12.0, tree_mid);

        // Right
        let sway_r = ((y * 29) as f32 + time * 0.5).sin() * 1.5;
        draw_rectangle(SCREEN_W - 20.0, py - 4.0, 14.0, 8.0, trunk_col);
        draw_circle(SCREEN_W - 14.0 + sway_r, py, 16.0, tree_dark);
        draw_circle(SCREEN_W - 12.0 + sway_r, py - 2.0, 12.0, tree_mid);
    }
}

// Low-lying atmospheric twilight mist & drifting embers
fn draw_twilight_mist(time: f32, dusk_factor: f32) {
    let mist_alpha = 0.08 + dusk_factor * 0.08;
    let mist_col = Color::new(0.55, 0.40, 0.65, mist_alpha);

    // Drifting horizontal mist bands
    let m1_x = (time * 18.0) % (SCREEN_W + 160.0) - 80.0;
    let m2_x = (time * 12.0 + 320.0) % (SCREEN_W + 160.0) - 80.0;

    draw_ellipse(m1_x, 170.0, 90.0, 14.0, 0.0, mist_col);
    draw_ellipse(m2_x, 240.0, 110.0, 16.0, 0.0, mist_col);

    // Drifting twilight fireflies / sparks
    for i in 0..8 {
        let fx = ((i as f32 * 83.0 + time * 14.0) % (SCREEN_W - 80.0)) + 40.0;
        let fy = 90.0 + ((i as f32 * 53.0 + time * 8.0).sin() * 60.0 + 60.0);
        let glow = (time * 8.0 + i as f32).sin() * 0.3 + 0.6;
        let f_col = if i % 2 == 0 {
            Color::new(0.3, 0.9, 1.0, glow * 0.8) // cyan firefly
        } else {
            Color::new(1.0, 0.8, 0.2, glow * 0.8) // golden ember
        };
        draw_circle(fx, fy, 1.5, f_col);
    }
}

// ============================================================================
// 16-BIT RETRO SPRITES: PRINCE RAMA (SHYAMAVARNA)
// ============================================================================
pub fn draw_rama(pos: Vec2, aim_dir: Vec2, invuln_timer: f32, bow_cooldown: f32, time: f32) {
    if invuln_timer > 0.0 && ((invuln_timer * 18.0) as i32 % 2 == 0) {
        return; // Flash effect during damage invulnerability
    }

    let x = pos.x;
    let y = pos.y;

    // 1. Ground Drop Shadow (16-bit semi-translucent ellipse)
    draw_ellipse(x, y + 14.0, 13.0, 5.0, 0.0, Color::new(0.04, 0.08, 0.06, 0.45));

    // 2. Leather Paduka Sandals & Anklets
    draw_rectangle(x - 7.0, y + 9.0, 5.0, 5.0, PAL_SAND_4);
    draw_rectangle(x + 2.0, y + 9.0, 5.0, 5.0, PAL_SAND_4);
    draw_rectangle(x - 7.0, y + 8.0, 5.0, 2.0, PAL_GOLD_1); // Golden ankle bands (nupura)
    draw_rectangle(x + 2.0, y + 8.0, 5.0, 2.0, PAL_GOLD_1);

    // 3. Pitambara Dhoti (4-tone turmeric yellow with authentic Vedic pleats)
    // Dark base/underdrape
    draw_rectangle(x - 8.0, y + 1.0, 16.0, 9.0, PAL_TURMERIC_3);
    // Midtone cloth body
    draw_rectangle(x - 7.0, y + 2.0, 14.0, 7.0, PAL_TURMERIC_2);
    // Highlight pleats (Kachha folds)
    draw_rectangle(x - 5.0, y + 2.0, 3.0, 6.0, PAL_TURMERIC_1);
    draw_rectangle(x + 2.0, y + 2.0, 3.0, 6.0, PAL_TURMERIC_0);
    // Vertical pleat separation lines
    draw_line(x - 2.0, y + 2.0, x - 2.0, y + 9.0, 1.0, PAL_TURMERIC_3);
    draw_line(x + 2.0, y + 2.0, x + 2.0, y + 9.0, 1.0, PAL_TURMERIC_3);
    // Crimson Waist Sash (Uttariya sash waistband)
    draw_rectangle(x - 8.0, y, 16.0, 3.0, PAL_RED_2);
    draw_rectangle(x - 8.0, y + 1.0, 16.0, 1.0, PAL_RED_1);
    // Fluttering sash side ends
    let sash_sway = (time * 4.0).sin() * 2.0;
    draw_triangle(
        vec2(x - 8.0, y + 1.0),
        vec2(x - 12.0 + sash_sway, y + 7.0),
        vec2(x - 6.0, y + 4.0),
        PAL_RED_2,
    );

    // 4. Shyamavarna Torso (Divine lotus-blue 4-tone musculature)
    // Dark outline / shadow base
    draw_rectangle(x - 7.0, y - 10.0, 14.0, 11.0, PAL_RAMA_3);
    // Midtone divine blue
    draw_rectangle(x - 6.0, y - 9.0, 12.0, 9.0, PAL_RAMA_1);
    // Sunlit chest highlight
    draw_rectangle(x - 4.0, y - 8.0, 4.0, 4.0, PAL_RAMA_0);
    draw_rectangle(x + 1.0, y - 8.0, 4.0, 4.0, PAL_RAMA_0);

    // Golden Armbands (Keyura) & Wrist Bracelets (Kankana)
    draw_rectangle(x - 8.0, y - 6.0, 2.0, 3.0, PAL_GOLD_1);
    draw_rectangle(x + 6.0, y - 6.0, 2.0, 3.0, PAL_GOLD_1);

    // Sacred Triple Cord (Yajnopavita) crossing diagonally from left shoulder
    draw_line(x - 5.0, y - 10.0, x + 6.0, y + 1.0, 1.5, Color::new(0.98, 0.98, 0.98, 0.95));
    draw_circle(x + 5.0, y, 1.5, PAL_GOLD_0); // Golden Yajnopavita knot

    // Golden Pearl Choker / Haras
    draw_circle(x, y - 9.0, 2.5, PAL_GOLD_1);
    draw_circle(x, y - 9.0, 1.2, PAL_GOLD_0);

    // 5. Quiver slung over back (Sandstone leather & celestial fletching)
    let qx = x - 9.0;
    let qy = y - 10.0;
    draw_line(qx - 2.0, qy - 6.0, qx + 3.0, qy + 8.0, 4.0, PAL_SAND_4);
    draw_line(qx - 1.0, qy - 5.0, qx + 2.0, qy + 7.0, 2.0, PAL_GOLD_2);
    // Arrow fletching feathers (white & red goose feathers)
    draw_line(qx - 4.0, qy - 7.0, qx - 7.0, qy - 13.0, 1.5, WHITE);
    draw_line(qx - 1.0, qy - 7.0, qx - 3.0, qy - 14.0, 1.5, PAL_RED_1);
    draw_line(qx + 2.0, qy - 7.0, qx + 1.0, qy - 13.0, 1.5, WHITE);

    // 6. Divine Visage (Head, ascetic Jata topknot, Mukuta crown & Tilaka)
    // Head base (Shyamavarna)
    draw_circle(x, y - 15.0, 6.5, PAL_RAMA_3);
    draw_circle(x, y - 15.0, 5.5, PAL_RAMA_1);
    draw_circle(x + 1.5, y - 16.0, 3.0, PAL_RAMA_0); // Face highlight

    // Ascetic Jata (Dark flowing hair locks tied in ascetic knot)
    draw_circle(x - 4.0, y - 18.0, 3.5, Color::new(0.08, 0.08, 0.12, 1.0));
    draw_circle(x + 4.0, y - 18.0, 3.5, Color::new(0.08, 0.08, 0.12, 1.0));
    draw_circle(x, y - 21.0, 4.0, Color::new(0.08, 0.08, 0.12, 1.0));
    draw_circle(x, y - 22.0, 1.5, PAL_SAND_3); // Sacred Rudraksha bead in topknot

    // Golden Celestial Mukuta Crown
    draw_triangle(
        vec2(x - 5.5, y - 19.0),
        vec2(x + 5.5, y - 19.0),
        vec2(x, y - 28.0),
        PAL_GOLD_2,
    );
    draw_triangle(
        vec2(x - 3.5, y - 19.0),
        vec2(x + 3.5, y - 19.0),
        vec2(x, y - 27.0),
        PAL_GOLD_0,
    );
    // Ruby jewel in center of Mukuta
    draw_circle(x, y - 23.0, 1.8, PAL_RED_1);
    draw_circle(x, y - 23.0, 0.8, PAL_RED_0);

    // Kundala (Golden earrings)
    draw_circle(x - 5.5, y - 14.0, 1.5, PAL_GOLD_1);
    draw_circle(x + 5.5, y - 14.0, 1.5, PAL_GOLD_1);

    // Sandalwood & Kumkum Tilaka on forehead
    draw_line(x, y - 18.0, x, y - 14.0, 1.4, PAL_RED_1);
    draw_circle(x, y - 15.0, 1.0, PAL_GOLD_0);

    // Expressive Eyes facing aim direction
    let eye_dx = (aim_dir.x * 2.0).clamp(-1.8, 1.8);
    let eye_dy = (aim_dir.y * 1.5).clamp(-1.2, 1.2);
    draw_circle(x - 2.0 + eye_dx * 0.4, y - 15.0 + eye_dy * 0.4, 1.2, WHITE);
    draw_circle(x + 2.0 + eye_dx * 0.4, y - 15.0 + eye_dy * 0.4, 1.2, WHITE);
    draw_circle(x - 2.0 + eye_dx * 0.6, y - 15.0 + eye_dy * 0.6, 0.7, Color::new(0.05, 0.05, 0.08, 1.0));
    draw_circle(x + 2.0 + eye_dx * 0.6, y - 15.0 + eye_dy * 0.6, 0.7, Color::new(0.05, 0.05, 0.08, 1.0));

    // 7. Celestial Kodanda Bow (Animated draw tension & arrow fletching)
    let bow_center = pos + vec2(aim_dir.x * 10.0, aim_dir.y * 10.0 - 2.0);
    let bow_len = 16.0;
    let perp = vec2(-aim_dir.y, aim_dir.x);
    let bow_top = bow_center + perp * bow_len;
    let bow_bot = bow_center - perp * bow_len;
    let bow_grip = bow_center + aim_dir * 4.0;

    // Recurve tips
    let tip_top = bow_top + aim_dir * 2.0;
    let tip_bot = bow_bot + aim_dir * 2.0;

    // Bow limbs in 3-shade polished gold
    draw_line(tip_bot.x, tip_bot.y, bow_grip.x, bow_grip.y, 3.2, PAL_GOLD_3);
    draw_line(tip_bot.x, tip_bot.y, bow_grip.x, bow_grip.y, 2.0, PAL_GOLD_1);
    draw_line(bow_grip.x, bow_grip.y, tip_top.x, tip_top.y, 3.2, PAL_GOLD_3);
    draw_line(bow_grip.x, bow_grip.y, tip_top.x, tip_top.y, 2.0, PAL_GOLD_1);
    draw_circle(bow_grip.x, bow_grip.y, 2.5, PAL_GOLD_0); // Bow handle grip jewel

    // Bowstring pulled dynamically on reload / fire
    let string_pull = if bow_cooldown > 0.0 { -aim_dir * 5.0 } else { Vec2::ZERO };
    let nock_point = bow_center + string_pull;
    draw_line(tip_bot.x, tip_bot.y, nock_point.x, nock_point.y, 1.2, Color::new(0.95, 0.95, 1.0, 0.9));
    draw_line(tip_top.x, tip_top.y, nock_point.x, nock_point.y, 1.2, Color::new(0.95, 0.95, 1.0, 0.9));

    // Notched Arrow ready to fire
    if bow_cooldown <= 0.0 {
        let arrow_tip = nock_point + aim_dir * 18.0;
        // Shaft in gold
        draw_line(nock_point.x, nock_point.y, arrow_tip.x, arrow_tip.y, 2.0, PAL_GOLD_1);
        // Razor-sharp arrowhead (diamond shape)
        draw_circle(arrow_tip.x, arrow_tip.y, 2.2, PAL_GOLD_0);
        draw_circle(arrow_tip.x + aim_dir.x * 2.0, arrow_tip.y + aim_dir.y * 2.0, 1.0, WHITE);
        // Feather fletching at nock
        let fletch_p1 = nock_point + perp * 2.5;
        let fletch_p2 = nock_point - perp * 2.5;
        draw_line(nock_point.x, nock_point.y, fletch_p1.x, fletch_p1.y, 1.5, WHITE);
        draw_line(nock_point.x, nock_point.y, fletch_p2.x, fletch_p2.y, 1.5, WHITE);
    }
}

// ============================================================================
// 16-BIT RETRO SPRITES: TATAKA (YAKSHINI OF THE CURSED FOREST)
// ============================================================================
pub fn draw_tataka(
    pos: Vec2,
    is_visible: bool,
    shimmer_timer: f32,
    hit_flash: f32,
    is_frenzy: bool,
    time: f32,
) {
    let x = pos.x;
    let y = pos.y;

    if !is_visible {
        if shimmer_timer > 0.0 {
            let alpha = (shimmer_timer / 0.35).clamp(0.0, 1.0) * 0.85;
            let aura_col = if is_frenzy {
                Color::new(0.95, 0.15, 0.20, alpha)
            } else {
                Color::new(0.65, 0.25, 0.75, alpha)
            };
            draw_circle(x, y - 6.0, 26.0, aura_col);
            draw_circle(x, y - 24.0, 16.0, aura_col);
            // Glinting demonic eyes in the darkness
            draw_circle(x - 6.0, y - 24.0, 3.5, Color::new(1.0, 0.15, 0.15, alpha));
            draw_circle(x + 6.0, y - 24.0, 3.5, Color::new(1.0, 0.15, 0.15, alpha));
        }
        return;
    }

    // Ground Shadow
    draw_ellipse(x, y + 18.0, 22.0, 7.5, 0.0, Color::new(0.0, 0.0, 0.0, 0.55));

    // Swirling Sorcery Miasma Aura
    let pulse = (time * 5.0).sin() * 4.0;
    let aura_col = if is_frenzy {
        Color::new(0.85, 0.10, 0.15, 0.32)
    } else {
        Color::new(0.45, 0.15, 0.55, 0.25)
    };
    draw_circle(x, y - 8.0, 34.0 + pulse, aura_col);

    // Miasma Orbiting Ember Specks
    for i in 0..4 {
        let a = time * 3.0 + i as f32 * 1.57;
        let ox = x + a.cos() * 28.0;
        let oy = y - 8.0 + a.sin() * 18.0;
        let ember_c = if is_frenzy { PAL_RED_0 } else { PAL_PURPLE_0 };
        draw_circle(ox, oy, 2.0, ember_c);
    }

    let col_body = if hit_flash > 0.0 {
        WHITE
    } else if is_frenzy {
        Color::new(0.42, 0.10, 0.16, 1.0)
    } else {
        PAL_PURPLE_2
    };

    let col_shadow = if hit_flash > 0.0 {
        Color::new(0.9, 0.9, 0.9, 1.0)
    } else {
        PAL_PURPLE_4
    };

    let col_highlight = if hit_flash > 0.0 {
        WHITE
    } else {
        PAL_PURPLE_0
    };

    // 1. Demonic Talons & Digging Feet
    draw_rectangle(x - 14.0, y + 8.0, 9.0, 9.0, col_shadow);
    draw_rectangle(x + 5.0, y + 8.0, 9.0, 9.0, col_shadow);
    draw_triangle(
        vec2(x - 14.0, y + 17.0),
        vec2(x - 5.0, y + 17.0),
        vec2(x - 10.0, y + 22.0),
        Color::new(0.1, 0.1, 0.12, 1.0),
    );
    draw_triangle(
        vec2(x + 5.0, y + 17.0),
        vec2(x + 14.0, y + 17.0),
        vec2(x + 10.0, y + 22.0),
        Color::new(0.1, 0.1, 0.12, 1.0),
    );

    // 2. Billowing Tattered Demon Mantle (Ragged dark skirt)
    draw_rectangle(x - 18.0, y - 4.0, 36.0, 16.0, PAL_PURPLE_4);
    let skirt_flutter = (time * 6.0).sin() * 3.0;
    for i in 0..5 {
        let sx = x - 16.0 + i as f32 * 7.5;
        draw_triangle(
            vec2(sx, y + 12.0),
            vec2(sx + 7.0, y + 12.0),
            vec2(sx + 3.5 + skirt_flutter, y + 17.0),
            PAL_PURPLE_3,
        );
    }

    // 3. Bulky Demonic Torso (Contoured Rakshasi musculature)
    draw_rectangle(x - 16.0, y - 16.0, 32.0, 22.0, col_shadow);
    draw_rectangle(x - 14.0, y - 15.0, 28.0, 19.0, col_body);
    draw_rectangle(x - 10.0, y - 13.0, 20.0, 7.0, col_highlight);

    // Bone Mala Garland (Bleached skull & bone necklace)
    for i in -3..=3 {
        let bx = x + i as f32 * 5.0;
        let by = y - 11.0 + (i as f32).abs() * 2.2;
        draw_circle(bx, by, 2.6, Color::new(0.92, 0.90, 0.82, 1.0));
        draw_circle(bx, by, 1.2, Color::new(0.2, 0.18, 0.15, 1.0));
    }

    // 4. Heavy Clawed Arms & Obsidian Gauntlets
    let arm_swing = (time * 8.0).sin() * 5.0;
    // Left arm
    draw_rectangle(x - 24.0, y - 13.0 + arm_swing, 9.0, 20.0, col_shadow);
    draw_rectangle(x - 23.0, y - 12.0 + arm_swing, 7.0, 17.0, col_body);
    draw_triangle(
        vec2(x - 24.0, y + 7.0 + arm_swing),
        vec2(x - 15.0, y + 7.0 + arm_swing),
        vec2(x - 22.0, y + 16.0 + arm_swing),
        Color::new(0.12, 0.12, 0.15, 1.0),
    );
    // Right arm
    draw_rectangle(x + 15.0, y - 13.0 - arm_swing, 9.0, 20.0, col_shadow);
    draw_rectangle(x + 16.0, y - 12.0 - arm_swing, 7.0, 17.0, col_body);
    draw_triangle(
        vec2(x + 15.0, y + 7.0 - arm_swing),
        vec2(x + 24.0, y + 7.0 - arm_swing),
        vec2(x + 22.0, y + 16.0 - arm_swing),
        Color::new(0.12, 0.12, 0.15, 1.0),
    );

    // 5. Demonic Visage (Head, wild windswept hair locks, horns, fangs & glowing eyes)
    // Head silhouette
    draw_circle(x, y - 24.0, 14.5, col_shadow);
    draw_circle(x, y - 24.0, 13.0, col_body);
    draw_circle(x - 3.0, y - 25.0, 8.0, col_highlight);

    // Curved Demonic Obsidian Horns
    draw_line(x - 7.0, y - 30.0, x - 17.0, y - 40.0, 3.5, Color::new(0.15, 0.10, 0.18, 1.0));
    draw_line(x - 17.0, y - 40.0, x - 13.0, y - 44.0, 2.5, PAL_PURPLE_0);
    draw_line(x + 7.0, y - 30.0, x + 17.0, y - 40.0, 3.5, Color::new(0.15, 0.10, 0.18, 1.0));
    draw_line(x + 17.0, y - 40.0, x + 13.0, y - 44.0, 2.5, PAL_PURPLE_0);

    // Wild Demonic Mane (Billowing hair strands swayed by dark sorcery)
    for i in -4..=4 {
        let hx = x + i as f32 * 4.8;
        let hy = y - 32.0 - (time * 7.0 + i as f32 * 1.2).sin() * 3.5;
        draw_circle(hx, hy, 5.0, Color::new(0.08, 0.05, 0.10, 1.0));
    }

    // Piercing Crimson Blazing Eyes
    let eye_glow = (time * 10.0).sin() * 0.8;
    draw_circle(x - 5.5, y - 24.0, 3.0 + eye_glow, PAL_RED_0);
    draw_circle(x + 5.5, y - 24.0, 3.0 + eye_glow, PAL_RED_0);
    draw_circle(x - 5.5, y - 24.0, 1.5, WHITE);
    draw_circle(x + 5.5, y - 24.0, 1.5, WHITE);

    // Bared Sharp Ivory Fangs & Gaping Maw
    draw_rectangle(x - 6.0, y - 18.0, 12.0, 4.0, Color::new(0.1, 0.05, 0.05, 1.0));
    draw_triangle(vec2(x - 5.0, y - 18.0), vec2(x - 2.0, y - 18.0), vec2(x - 3.5, y - 14.0), WHITE);
    draw_triangle(vec2(x + 2.0, y - 18.0), vec2(x + 5.0, y - 18.0), vec2(x + 3.5, y - 14.0), WHITE);
}

// Draw Rama's golden arrow
pub fn draw_arrow(pos: Vec2, vel: Vec2) {
    let dir = vel.normalize_or_zero();
    let tip = pos + dir * 10.0;
    let tail = pos - dir * 10.0;

    draw_line(tail.x, tail.y, tip.x, tip.y, 2.2, COLOR_RAMA_GOLD);

    let perp = vec2(-dir.y, dir.x);
    let head_left = tip - dir * 4.5 + perp * 3.0;
    let head_right = tip - dir * 4.5 - perp * 3.0;
    draw_triangle(tip, head_left, head_right, WHITE);

    let fletch_l = tail + dir * 2.5 + perp * 2.5;
    let fletch_r = tail + dir * 2.5 - perp * 2.5;
    draw_line(tail.x, tail.y, fletch_l.x, fletch_l.y, 1.2, WHITE);
    draw_line(tail.x, tail.y, fletch_r.x, fletch_r.y, 1.2, WHITE);
}

// Draw Boulder thrown by Tataka
pub fn draw_boulder(pos: Vec2, angle: f32) {
    let r = 11.0;
    draw_circle(pos.x, pos.y, r, Color::new(0.38, 0.34, 0.32, 1.0));
    draw_circle(pos.x + 2.0, pos.y - 2.0, r * 0.75, Color::new(0.50, 0.46, 0.42, 1.0));

    let rot = vec2(angle.cos(), angle.sin());
    let crack1 = pos + rot * 5.0;
    let crack2 = pos - rot * 5.0;
    draw_line(crack1.x, crack1.y, crack2.x, crack2.y, 1.8, Color::new(0.20, 0.18, 0.16, 1.0));
}

// Draw Shabdavedhi Sound Ripple
pub fn draw_sound_ripple(pos: Vec2, current_radius: f32, _max_radius: f32, alpha: f32) {
    let fade = alpha.clamp(0.0, 1.0);
    if fade <= 0.01 {
        return;
    }

    let col_outer = Color::new(0.3, 0.85, 1.0, fade * 0.85);
    draw_circle_lines(pos.x, pos.y, current_radius, 2.2, col_outer);

    let inner_r = current_radius * 0.65;
    if inner_r > 3.0 {
        let col_inner = Color::new(1.0, 0.9, 0.4, fade * 0.6);
        draw_circle_lines(pos.x, pos.y, inner_r, 1.2, col_inner);
    }

    if current_radius < 16.0 {
        draw_circle(pos.x, pos.y, 2.5, Color::new(1.0, 1.0, 1.0, fade * 0.9));
    }
}

// Draw Ground Stomp Shockwave
pub fn draw_shockwave(pos: Vec2, radius: f32, alpha: f32) {
    let col = Color::new(0.9, 0.2, 0.15, alpha * 0.7);
    draw_circle_lines(pos.x, pos.y, radius, 2.5, col);
    draw_circle_lines(pos.x, pos.y, radius * 0.85, 1.2, Color::new(1.0, 0.6, 0.1, alpha * 0.5));
}

// Draw Mouse Crosshair / Reticle
pub fn draw_reticle(pos: Vec2, is_firing: bool) {
    let r = if is_firing { 6.0 } else { 8.0 };
    let col = if is_firing {
        Color::new(1.0, 0.3, 0.2, 0.95)
    } else {
        COLOR_RAMA_GOLD
    };

    draw_circle(pos.x, pos.y, 1.5, col);
    draw_line(pos.x - r - 3.0, pos.y, pos.x - 2.5, pos.y, 1.2, col);
    draw_line(pos.x + 2.5, pos.y, pos.x + r + 3.0, pos.y, 1.2, col);
    draw_line(pos.x, pos.y - r - 3.0, pos.x, pos.y - 2.5, 1.2, col);
    draw_line(pos.x, pos.y + 2.5, pos.x, pos.y + r + 3.0, 1.2, col);
    draw_circle_lines(pos.x, pos.y, r, 1.0, Color::new(col.r, col.g, col.b, 0.5));
}

// Top HUD for 16:9 widescreen
pub fn draw_hud(
    font: &Font,
    stats: &PlayerStats,
    sunset_timer: f32,
    is_frenzy: bool,
    is_phase2: bool,
    pin_bar: f32,
    boss_hp: f32,
    boss_max_hp: f32,
) {
    // Top HUD bar
    draw_rectangle(0.0, 0.0, SCREEN_W, 32.0, Color::new(0.06, 0.08, 0.10, 0.92));
    draw_line(0.0, 32.0, SCREEN_W, 32.0, 2.0, Color::new(0.35, 0.30, 0.20, 1.0));

    // Prana
    let hp_ratio = (stats.prana as f32 / stats.max_prana as f32).clamp(0.0, 1.0);
    draw_pixel_text(font, "PRANA", 10.0, 12.0, 8, Color::new(0.9, 0.8, 0.4, 1.0));
    draw_rectangle(52.0, 6.0, 80.0, 8.0, Color::new(0.18, 0.06, 0.06, 1.0));
    let hp_color = if hp_ratio > 0.4 {
        Color::new(0.2, 0.85, 0.3, 1.0)
    } else {
        Color::new(0.9, 0.2, 0.2, 1.0)
    };
    draw_rectangle(52.0, 6.0, 80.0 * hp_ratio, 8.0, hp_color);
    draw_rectangle_lines(52.0, 6.0, 80.0, 8.0, 1.0, COLOR_RAMA_GOLD);
    draw_pixel_text(font, &format!("{}/{}", stats.prana, stats.max_prana), 58.0, 24.0, 8, WHITE);

    // Tejas
    let mp_ratio = (stats.tejas as f32 / stats.max_tejas as f32).clamp(0.0, 1.0);
    draw_pixel_text(font, "TEJAS", 145.0, 12.0, 8, Color::new(0.4, 0.8, 1.0, 1.0));
    draw_rectangle(188.0, 6.0, 70.0, 8.0, Color::new(0.06, 0.12, 0.20, 1.0));
    draw_rectangle(188.0, 6.0, 70.0 * mp_ratio, 8.0, Color::new(0.2, 0.6, 1.0, 1.0));
    draw_rectangle_lines(188.0, 6.0, 70.0, 8.0, 1.0, Color::new(0.5, 0.8, 1.0, 1.0));
    draw_pixel_text(font, &format!("{}/{}", stats.tejas, stats.max_tejas), 194.0, 24.0, 8, WHITE);

    // Sunset Countdown (Center)
    let time_left = sunset_timer.max(0.0);
    let mins = (time_left / 60.0) as i32;
    let secs = (time_left % 60.0) as i32;
    let timer_str = format!("{:02}:{:02}", mins, secs);

    let (time_col, status_label) = if is_frenzy {
        (Color::new(1.0, 0.15, 0.15, 1.0), "DUSK FRENZY!")
    } else if time_left < 20.0 {
        (Color::new(1.0, 0.4, 0.1, 1.0), "DUSK NEARS!")
    } else {
        (COLOR_RAMA_GOLD, "SUNSET")
    };

    draw_pixel_text(font, status_label, 280.0, 12.0, 8, time_col);
    draw_pixel_text(font, &timer_str, 285.0, 26.0, 12, time_col);

    // Boss Bar (Right)
    if !is_phase2 {
        draw_pixel_text(font, "SHABDAVEDHI PIN", 430.0, 12.0, 8, Color::new(0.4, 0.85, 1.0, 1.0));
        draw_rectangle(430.0, 16.0, 150.0, 10.0, Color::new(0.12, 0.14, 0.18, 1.0));
        let pin_ratio = (pin_bar / 100.0).clamp(0.0, 1.0);
        draw_rectangle(430.0, 16.0, 150.0 * pin_ratio, 10.0, Color::new(0.3, 0.75, 1.0, 1.0));
        draw_rectangle_lines(430.0, 16.0, 150.0, 10.0, 1.0, Color::new(0.6, 0.9, 1.0, 1.0));
        for i in 1..5 {
            let sx = 430.0 + i as f32 * (150.0 / 5.0);
            draw_line(sx, 16.0, sx, 26.0, 1.0, Color::new(0.1, 0.2, 0.3, 0.8));
        }
        let pin_pct = (pin_bar as i32).min(100);
        draw_pixel_text(font, &format!("{}%", pin_pct), 585.0, 24.0, 8, WHITE);
    } else {
        let label = if is_frenzy { "TATAKA (FRENZY)" } else { "TATAKA (RUSH)" };
        draw_pixel_text(font, label, 430.0, 12.0, 8, Color::new(1.0, 0.25, 0.25, 1.0));
        draw_rectangle(430.0, 16.0, 150.0, 10.0, Color::new(0.2, 0.06, 0.08, 1.0));
        let boss_ratio = (boss_hp / boss_max_hp).clamp(0.0, 1.0);
        draw_rectangle(430.0, 16.0, 150.0 * boss_ratio, 10.0, Color::new(0.85, 0.15, 0.2, 1.0));
        draw_rectangle_lines(430.0, 16.0, 150.0, 10.0, 1.0, Color::new(1.0, 0.35, 0.4, 1.0));
        draw_pixel_text(font, &format!("{:.0}", boss_hp.max(0.0)), 585.0, 24.0, 8, WHITE);
    }

    // Bottom Toolbar for 16:9
    draw_rectangle(0.0, SCREEN_H - 18.0, SCREEN_W, 18.0, Color::new(0.05, 0.06, 0.08, 0.95));
    draw_line(0.0, SCREEN_H - 18.0, SCREEN_W, SCREEN_H - 18.0, 1.0, Color::new(0.25, 0.22, 0.16, 1.0));

    draw_pixel_text(font, "[1] Kodanda", 12.0, SCREEN_H - 6.0, 8, COLOR_RAMA_GOLD);
    draw_pixel_text(font, "[2] Vayavya", 100.0, SCREEN_H - 6.0, 8, Color::new(0.5, 0.5, 0.5, 1.0));
    draw_pixel_text(font, "[3] Manava", 185.0, SCREEN_H - 6.0, 8, Color::new(0.5, 0.5, 0.5, 1.0));
    draw_pixel_text(font, "[4] Agneya", 265.0, SCREEN_H - 6.0, 8, Color::new(0.5, 0.5, 0.5, 1.0));

    draw_pixel_text(font, "WASD:Move | MOUSE:Aim/Shoot", 380.0, SCREEN_H - 6.0, 8, Color::new(0.85, 0.85, 0.9, 1.0));
}

// Retro RPG Dialogue Box for 16:9 with Re-Read Support
pub fn is_dialogue_prev_button_hovered(mouse: Vec2) -> bool {
    let box_x = 30.0;
    let box_y = SCREEN_H - 108.0;
    let box_h = 96.0;
    let btn_x = box_x + 16.0;
    let btn_y = box_y + box_h - 22.0;
    let btn_w = 100.0;
    let btn_h = 16.0;
    mouse.x >= btn_x && mouse.x <= btn_x + btn_w && mouse.y >= btn_y && mouse.y <= btn_y + btn_h
}

pub fn is_cutscene_prev_button_hovered(mouse: Vec2) -> bool {
    let nar_x = 30.0;
    let nar_y = 236.0;
    let nar_h = 100.0;
    let btn_x = nar_x + 16.0;
    let btn_y = nar_y + nar_h - 22.0;
    let btn_w = 95.0;
    let btn_h = 16.0;
    mouse.x >= btn_x && mouse.x <= btn_x + btn_w && mouse.y >= btn_y && mouse.y <= btn_y + btn_h
}

pub fn draw_dialogue_box(
    font: &Font,
    speaker: &str,
    line: &str,
    time: f32,
    idx: usize,
    total: usize,
    virtual_mouse: Vec2,
) {
    let box_x = 30.0;
    let box_y = SCREEN_H - 108.0;
    let box_w = SCREEN_W - 60.0;
    let box_h = 96.0;

    draw_rectangle(box_x, box_y, box_w, box_h, Color::new(0.08, 0.09, 0.12, 0.96));
    draw_rectangle_lines(box_x, box_y, box_w, box_h, 2.5, COLOR_RAMA_GOLD);
    draw_rectangle_lines(box_x + 2.0, box_y + 2.0, box_w - 4.0, box_h - 4.0, 1.0, Color::new(0.6, 0.5, 0.2, 0.8));

    // Speaker Nameplate
    let plate_w = 185.0;
    let plate_h = 20.0;
    draw_rectangle(box_x + 12.0, box_y - 10.0, plate_w, plate_h, Color::new(0.18, 0.12, 0.08, 1.0));
    draw_rectangle_lines(box_x + 12.0, box_y - 10.0, plate_w, plate_h, 1.5, COLOR_RAMA_GOLD);
    draw_pixel_text(font, speaker, box_x + 18.0, box_y + 4.0, 9, COLOR_RAMA_GOLD);

    // Word-wrapped dialogue text guaranteed to stay inside bounds
    let max_text_w = box_w - 36.0;
    let wrapped_8 = wrap_pixel_text(font, line, 8, max_text_w);
    let (font_sz, line_step, final_lines) = if wrapped_8.len() <= 4 {
        (8, 13.5, wrapped_8)
    } else {
        let wrapped_7 = wrap_pixel_text(font, line, 7, max_text_w);
        (7, 11.5, wrapped_7)
    };

    let mut line_y = box_y + 22.0;
    for (i, wline) in final_lines.iter().enumerate().take(5) {
        let col = if i == final_lines.len() - 1 && i > 1 {
            Color::new(0.95, 0.95, 0.80, 1.0)
        } else {
            WHITE
        };
        draw_pixel_text(font, wline, box_x + 18.0, line_y, font_sz, col);
        line_y += line_step;
    }

    // Re-Read / Previous Button if idx > 0
    if idx > 0 {
        let hovered = is_dialogue_prev_button_hovered(virtual_mouse);
        let btn_x = box_x + 16.0;
        let btn_y = box_y + box_h - 22.0;
        let btn_w = 100.0;
        let btn_h = 16.0;
        let bg = if hovered {
            Color::new(0.28, 0.20, 0.12, 1.0)
        } else {
            Color::new(0.15, 0.12, 0.18, 0.9)
        };
        let border = if hovered { PAL_GOLD_0 } else { PAL_GOLD_2 };
        draw_rectangle(btn_x, btn_y, btn_w, btn_h, bg);
        draw_rectangle_lines(btn_x, btn_y, btn_w, btn_h, 1.0, border);
        draw_pixel_text(font, "◀ PREV [LEFT]", btn_x + 8.0, btn_y + 11.0, 7, border);
    }

    // Page indicator in center
    let page_str = format!("{}/{}", idx + 1, total);
    draw_pixel_text(font, &page_str, box_x + box_w * 0.5 - 14.0, box_y + box_h - 10.0, 7, PAL_TURMERIC_1);

    // Next / Continue on Right
    let blink = (time * 5.0).sin() > 0.0;
    let next_text = if idx + 1 < total { "[CLICK / SPACE ▶]" } else { "[PROCEED ▶]" };
    let col = if blink { COLOR_RAMA_GOLD } else { PAL_GOLD_2 };
    draw_pixel_text(font, next_text, box_x + box_w - 145.0, box_y + box_h - 10.0, 8, col);
}

// Retro Title Screen for 16:9
// ============================================================================
// 16-BIT RETRO TITLE SCREEN
// ============================================================================
pub fn draw_title_screen(font: &Font, time: f32, virtual_mouse: Vec2) {
    clear_background(Color::new(0.06, 0.05, 0.09, 1.0));

    // Dynamic forest tree silhouettes swaying
    let sway = (time * 0.8).sin() * 4.0;
    for i in 0..11 {
        let x = i as f32 * 65.0 + sway;
        draw_circle(x, 190.0, 44.0, Color::new(0.10, 0.14, 0.10, 1.0));
        draw_circle(x - 6.0, 185.0, 32.0, Color::new(0.13, 0.18, 0.12, 1.0));
    }

    // Epic Title in Press Start 2P font
    let title = "RAMAYANA: THE RETRO EPIC";
    draw_pixel_text(font, title, 70.0, 38.0, 20, PAL_GOLD_1);

    let sub = "BALA & AYODHYA KANDAS — THE RETRO ADVENTURES OF RAMA";
    draw_pixel_text(font, sub, 95.0, 56.0, 9, PAL_TURMERIC_1);

    // Decorative Bow & Arrow Divider
    draw_line(150.0, 66.0, 490.0, 66.0, 2.0, PAL_GOLD_2);
    draw_circle(320.0, 66.0, 5.0, PAL_RED_1);
    draw_circle(320.0, 66.0, 2.5, PAL_GOLD_0);

    // Chapter Select / Preview Box
    let box_x = 35.0;
    let box_y = 74.0;
    let box_w = 570.0;
    let box_h = 248.0;
    draw_rectangle(box_x, box_y, box_w, box_h, Color::new(0.10, 0.09, 0.15, 0.95));
    draw_rectangle_lines(box_x, box_y, box_w, box_h, 1.5, PAL_GOLD_3);

    let hovered_action = get_title_screen_action_at(virtual_mouse);

    // Level 1 Row
    let r0_bg = if hovered_action == Some(0) {
        Color::new(0.18, 0.16, 0.26, 0.95)
    } else {
        Color::new(0.12, 0.11, 0.18, 0.5)
    };
    draw_rectangle(box_x + 8.0, box_y + 6.0, box_w - 16.0, 24.0, r0_bg);
    if hovered_action == Some(0) {
        draw_rectangle_lines(box_x + 8.0, box_y + 6.0, box_w - 16.0, 24.0, 1.0, PAL_GOLD_1);
        draw_pixel_text(font, "▶ PLAY [1]", box_x + box_w - 100.0, box_y + 19.0, 8, PAL_GOLD_0);
    }
    draw_pixel_text(font, "[1] LEVEL 1: TATAKA'S FOREST (SHABDAVEDHI)", box_x + 16.0, box_y + 18.0, 8, PAL_GOLD_1);
    draw_pixel_text(font, "    Track sound ripples & strike unseen Yakshini before dusk (90s)", box_x + 16.0, box_y + 27.0, 6, Color::new(0.4, 0.85, 1.0, 1.0));

    // Level 2 Row
    let r1_bg = if hovered_action == Some(1) {
        Color::new(0.22, 0.16, 0.18, 0.95)
    } else {
        Color::new(0.14, 0.11, 0.15, 0.5)
    };
    draw_rectangle(box_x + 8.0, box_y + 33.0, box_w - 16.0, 24.0, r1_bg);
    if hovered_action == Some(1) {
        draw_rectangle_lines(box_x + 8.0, box_y + 33.0, box_w - 16.0, 24.0, 1.0, PAL_GOLD_1);
        draw_pixel_text(font, "▶ PLAY [2]", box_x + box_w - 100.0, box_y + 46.0, 8, PAL_GOLD_0);
    }
    draw_pixel_text(font, "[2] LEVEL 2: SIDDHASHRAMA (SACRED YAJNA DEFENSE)", box_x + 16.0, box_y + 45.0, 8, Color::new(1.0, 0.65, 0.25, 1.0));
    draw_pixel_text(font, "    Defend holy Altar with Astras: Kodanda, Vayavya, Manava & Agneya", box_x + 16.0, box_y + 54.0, 6, Color::new(1.0, 0.85, 0.40, 1.0));

    // Level 3 Row
    let r2_bg = if hovered_action == Some(2) {
        Color::new(0.20, 0.14, 0.26, 0.95)
    } else {
        Color::new(0.13, 0.10, 0.18, 0.5)
    };
    draw_rectangle(box_x + 8.0, box_y + 60.0, box_w - 16.0, 24.0, r2_bg);
    if hovered_action == Some(2) {
        draw_rectangle_lines(box_x + 8.0, box_y + 60.0, box_w - 16.0, 24.0, 1.0, PAL_GOLD_1);
        draw_pixel_text(font, "▶ PLAY [3]", box_x + box_w - 100.0, box_y + 73.0, 8, PAL_GOLD_0);
    }
    draw_pixel_text(font, "[3] LEVEL 3: MITHILA (THE TRIAL OF SHIVA DHANUSH)", box_x + 16.0, box_y + 72.0, 8, Color::new(0.85, 0.65, 1.0, 1.0));
    draw_pixel_text(font, "    Lift Pinaka bow, Dhanurbhanga, lineage recitals & Parashurama", box_x + 16.0, box_y + 81.0, 6, Color::new(0.75, 0.85, 1.0, 1.0));

    // Level 4 Row (Ayodhya Coronation Eve)
    let r3_bg = if hovered_action == Some(3) {
        Color::new(0.26, 0.20, 0.14, 0.95)
    } else {
        Color::new(0.15, 0.13, 0.12, 0.5)
    };
    draw_rectangle(box_x + 8.0, box_y + 87.0, box_w - 16.0, 24.0, r3_bg);
    if hovered_action == Some(3) {
        draw_rectangle_lines(box_x + 8.0, box_y + 87.0, box_w - 16.0, 24.0, 1.0, PAL_GOLD_1);
        draw_pixel_text(font, "▶ PLAY [4]", box_x + box_w - 100.0, box_y + 100.0, 8, PAL_GOLD_0);
    }
    draw_pixel_text(font, "[4] LEVEL 4: AYODHYA (EVE OF THE CORONATION)", box_x + 16.0, box_y + 99.0, 8, Color::new(1.0, 0.85, 0.35, 1.0));
    draw_pixel_text(font, "    Festive Raja Marga, Royal Sabha, Upavasa vow & Manthara's plot", box_x + 16.0, box_y + 108.0, 6, Color::new(0.9, 0.85, 0.6, 1.0));

    // Level 5 Row (Chitrakuta - Raft, Hermitage & Padukas)
    let r4_bg = if hovered_action == Some(4) {
        Color::new(0.16, 0.26, 0.18, 0.95)
    } else {
        Color::new(0.11, 0.16, 0.13, 0.5)
    };
    draw_rectangle(box_x + 8.0, box_y + 114.0, box_w - 16.0, 24.0, r4_bg);
    if hovered_action == Some(4) {
        draw_rectangle_lines(box_x + 8.0, box_y + 114.0, box_w - 16.0, 24.0, 1.0, PAL_GOLD_1);
        draw_pixel_text(font, "▶ PLAY [5]", box_x + box_w - 100.0, box_y + 127.0, 8, PAL_GOLD_0);
    }
    draw_pixel_text(font, "[5] LEVEL 5: CHITRAKUTA (RAFT, HERMITAGE & PADUKAS)", box_x + 16.0, box_y + 126.0, 8, Color::new(0.4, 0.9, 0.5, 1.0));
    draw_pixel_text(font, "    Lakshmana crafts river raft & Parnashala; Bharata's Padukas", box_x + 16.0, box_y + 135.0, 6, Color::new(0.7, 0.95, 0.75, 1.0));

    // Level Select Button Highlight
    let r5_bg = if hovered_action == Some(5) {
        Color::new(0.24, 0.18, 0.34, 1.0)
    } else {
        Color::new(0.16, 0.13, 0.24, 1.0)
    };
    draw_rectangle(box_x + 8.0, box_y + 144.0, box_w - 16.0, 22.0, r5_bg);
    let border_btn = if hovered_action == Some(5) { PAL_GOLD_0 } else { PAL_GOLD_2 };
    draw_rectangle_lines(box_x + 8.0, box_y + 144.0, box_w - 16.0, 22.0, 1.0, border_btn);
    draw_pixel_text(font, "▶ PRESS [L], [TAB] OR CLICK FOR FULL CHAPTER SELECT SCREEN ◀", box_x + 32.0, box_y + 158.0, 8, PAL_GOLD_0);

    // Controls & Instructions line
    draw_pixel_text(font, "Controls: WASD Move | Mouse Aim & Tension | 1-5 Chapters | ESC Pause", box_x + 16.0, box_y + 182.0, 7, WHITE);
    draw_pixel_text(font, "Click any chapter row above or press [1]-[5] to launch immediately!", box_x + 16.0, box_y + 196.0, 7, Color::new(0.8, 0.9, 0.7, 1.0));
    draw_pixel_text(font, "Next Rakshasa Boss: Viradha (invulnerable man-eating giant of Dandaka)", box_x + 16.0, box_y + 210.0, 6, PAL_FIRE_1);

    let blink = (time * 4.0).sin() > 0.0;
    if blink {
        let prompt = "▶ PRESS [1]-[5] TO PLAY, [L] FOR LEVEL SELECT, OR [SPACE] ◀";
        draw_pixel_text(font, prompt, 38.0, 334.0, 9, PAL_GOLD_1);
    }

    draw_pixel_text(font, "16:9 Widescreen | 16-Bit Vedic Retro RPG | Rust + WASM", 100.0, 350.0, 7, Color::new(0.65, 0.65, 0.65, 1.0));
}

// ============================================================================
// 16-BIT LEVEL SELECT SCREEN (INTERACTIVE TEST HARNESS)
// ============================================================================
pub fn draw_level_select(font: &Font, selected: usize, time: f32, virtual_mouse: Vec2) {
    clear_background(Color::new(0.06, 0.05, 0.09, 1.0));

    // Decorative background Vedic pattern
    for i in 0..16 {
        let x = i as f32 * 42.0;
        draw_line(x, 0.0, x, SCREEN_H, 1.0, Color::new(0.12, 0.10, 0.18, 0.4));
    }

    // Top Header Banner
    let title = "RAMAYANA — SELECT BATTLE CHAPTER";
    draw_pixel_text(font, title, 140.0, 30.0, 13, PAL_GOLD_1);

    // Decorative bow divider line
    draw_line(80.0, 42.0, 560.0, 42.0, 2.0, PAL_GOLD_2);
    draw_circle(320.0, 42.0, 4.0, PAL_RED_1);
    draw_circle(320.0, 42.0, 2.0, PAL_GOLD_0);

    // 5 Level Cards across 640px widescreen
    let card_w = 114.0;
    let card_h = 244.0;
    let card_y = 54.0;

    let cards = [
        (
            0,
            10.0,
            "LEVEL 1",
            "TATAKA",
            "Shabdavedhi",
            PAL_GOLD_1,
            Color::new(0.40, 0.85, 1.0, 1.0),
            vec![
                "• Dark cursed forest",
                "• Sound ripple track",
                "• Strike unseen demon",
                "• 90s Dusk Countdown",
            ],
            "Kodanda Bow",
            "▶ READY ◀",
            Color::new(0.35, 0.85, 0.45, 1.0),
        ),
        (
            1,
            136.0,
            "LEVEL 2",
            "SIDDHASHRAMA",
            "Yajna Defense",
            Color::new(1.0, 0.65, 0.25, 1.0),
            Color::new(1.0, 0.85, 0.40, 1.0),
            vec![
                "• Defend holy Altar",
                "• Maricha & Subahu",
                "• Asura blood swarms",
                "• 4 Divine Astras",
            ],
            "All 4 Astras",
            "▶ READY ◀",
            Color::new(0.35, 0.85, 0.45, 1.0),
        ),
        (
            2,
            262.0,
            "LEVEL 3",
            "MITHILA",
            "Pinaka Bow",
            Color::new(0.85, 0.65, 1.0, 1.0),
            Color::new(0.75, 0.55, 0.95, 1.0),
            vec![
                "• King Janaka Court",
                "• 8-Wheel Bow Cart",
                "• Snap Pinaka bow",
                "• Lineage Recital",
            ],
            "Pinaka Bow",
            "▶ READY ◀",
            Color::new(0.35, 0.85, 0.45, 1.0),
        ),
        (
            3,
            388.0,
            "LEVEL 4",
            "AYODHYA",
            "Coronation Eve",
            Color::new(1.0, 0.85, 0.35, 1.0),
            Color::new(0.9, 0.65, 0.25, 1.0),
            vec![
                "• Festive Raja Marga",
                "• Royal Sabha Decree",
                "• Queen Shrine Vows",
                "• Manthara's Plot",
            ],
            "Yuvaraja Vow",
            "▶ READY ◀",
            Color::new(0.35, 0.85, 0.45, 1.0),
        ),
        (
            4,
            514.0,
            "LEVEL 5",
            "CHITRAKUTA",
            "Forest Sanctuary",
            Color::new(0.4, 0.9, 0.5, 1.0),
            Color::new(0.7, 0.95, 0.75, 1.0),
            vec![
                "• Craft River Raft",
                "• Build Parnashala",
                "• Bharata's Padukas",
                "• Next: Viradha Boss",
            ],
            "Lakshmana Craft",
            "▶ READY ◀",
            Color::new(0.35, 0.85, 0.45, 1.0),
        ),
    ];

    let hovered_idx = get_level_select_card_at(virtual_mouse);

    for (idx, cx, badge, ch_name, sub, title_col, sub_col, bullets, astras, status, status_col) in cards {
        let is_sel = selected == idx;
        let is_hov = hovered_idx == Some(idx);
        let active = is_sel || is_hov;

        // Card Base
        let bg_col = if active {
            Color::new(0.14, 0.12, 0.20, 0.98)
        } else {
            Color::new(0.09, 0.08, 0.13, 0.92)
        };
        draw_rectangle(cx, card_y, card_w, card_h, bg_col);

        // Border
        let border_col = if active {
            let glow = ((time * 6.0).sin() * 0.2 + 0.8).clamp(0.0, 1.0);
            Color::new(PAL_GOLD_1.r * glow, PAL_GOLD_1.g * glow, PAL_GOLD_1.b * glow, 1.0)
        } else {
            Color::new(0.35, 0.30, 0.42, 0.8)
        };
        draw_rectangle_lines(cx, card_y, card_w, card_h, if active { 2.0 } else { 1.2 }, border_col);

        // Ornamental Corners
        let c_sz = 6.0;
        let corner_col = if active { PAL_GOLD_0 } else { PAL_SAND_3 };
        draw_rectangle(cx, card_y, c_sz, 2.0, corner_col);
        draw_rectangle(cx, card_y, 2.0, c_sz, corner_col);
        draw_rectangle(cx + card_w - c_sz, card_y, c_sz, 2.0, corner_col);
        draw_rectangle(cx + card_w - 2.0, card_y, 2.0, c_sz, corner_col);
        draw_rectangle(cx, card_y + card_h - 2.0, c_sz, 2.0, corner_col);
        draw_rectangle(cx, card_y + card_h - c_sz, 2.0, c_sz, corner_col);
        draw_rectangle(cx + card_w - c_sz, card_y + card_h - 2.0, c_sz, 2.0, corner_col);
        draw_rectangle(cx + card_w - 2.0, card_y + card_h - c_sz, 2.0, c_sz, corner_col);

        // Card Header Badge
        draw_rectangle(cx + 6.0, card_y + 8.0, 58.0, 15.0, Color::new(0.18, 0.15, 0.25, 1.0));
        draw_rectangle_lines(cx + 6.0, card_y + 8.0, 58.0, 15.0, 1.0, border_col);
        draw_pixel_text(font, badge, cx + 9.0, card_y + 19.0, 7, PAL_GOLD_1);

        // Chapter Name & Subtitle
        draw_pixel_text(font, ch_name, cx + 6.0, card_y + 38.0, 7, title_col);
        draw_pixel_text(font, sub, cx + 6.0, card_y + 50.0, 6, sub_col);

        // Mini Diorama / Visual Box
        let box_y = card_y + 58.0;
        let box_h = 58.0;
        draw_rectangle(cx + 6.0, box_y, card_w - 12.0, box_h, Color::new(0.06, 0.05, 0.09, 1.0));
        draw_rectangle_lines(cx + 6.0, box_y, card_w - 12.0, box_h, 1.0, Color::new(0.25, 0.22, 0.30, 1.0));

        // Draw thematic mini diorama inside
        match idx {
            0 => {
                // Tataka forest mini-silhouette
                draw_rectangle(cx + 6.0, box_y + box_h - 16.0, card_w - 12.0, 16.0, PAL_GRASS_4);
                draw_circle(cx + 25.0, box_y + 26.0, 11.0, PAL_GRASS_3);
                draw_circle(cx + 90.0, box_y + 28.0, 12.0, PAL_GRASS_3);
                let rip_r = (time * 18.0) % 20.0;
                draw_circle_lines(cx + 56.0, box_y + 28.0, rip_r, 1.0, Color::new(0.4, 0.9, 1.0, 0.7));
                draw_circle(cx + 56.0, box_y + 28.0, 3.0, PAL_RED_0);
            }
            1 => {
                // Siddhashrama Yajna altar mini
                draw_rectangle(cx + 6.0, box_y + box_h - 16.0, card_w - 12.0, 16.0, Color::new(0.25, 0.40, 0.20, 1.0));
                draw_rectangle(cx + 36.0, box_y + 30.0, 42.0, 18.0, PAL_SAND_4);
                draw_rectangle(cx + 44.0, box_y + 22.0, 26.0, 10.0, PAL_SAND_2);
                let f_h = 10.0 + (time * 8.0).sin() * 3.0;
                draw_triangle(
                    vec2(cx + 49.0, box_y + 22.0),
                    vec2(cx + 65.0, box_y + 22.0),
                    vec2(cx + 57.0, box_y + 22.0 - f_h),
                    PAL_FIRE_2,
                );
                draw_circle(cx + 57.0, box_y + 20.0, 2.5, PAL_FIRE_0);
            }
            2 => {
                // Mithila Pinaka Bow mini
                draw_rectangle(cx + 6.0, box_y + box_h - 14.0, card_w - 12.0, 14.0, PAL_SAND_3);
                draw_rectangle(cx + 28.0, box_y + 36.0, 58.0, 8.0, Color::new(0.3, 0.28, 0.32, 1.0));
                draw_circle(cx + 38.0, box_y + 44.0, 4.0, PAL_GOLD_2);
                draw_circle(cx + 76.0, box_y + 44.0, 4.0, PAL_GOLD_2);
                draw_line(cx + 30.0, box_y + 34.0, cx + 57.0, box_y + 16.0, 2.5, PAL_GOLD_1);
                draw_line(cx + 57.0, box_y + 16.0, cx + 84.0, box_y + 34.0, 2.5, PAL_GOLD_1);
                draw_circle(cx + 57.0, box_y + 16.0, 3.0, PAL_GOLD_0);
            }
            3 => {
                // Ayodhya Palace & Saffron Flag mini
                draw_rectangle(cx + 6.0, box_y + box_h - 14.0, card_w - 12.0, 14.0, Color::new(0.88, 0.82, 0.72, 1.0));
                draw_rectangle(cx + 36.0, box_y + 24.0, 42.0, 20.0, Color::new(0.95, 0.94, 0.98, 1.0));
                draw_triangle(
                    vec2(cx + 36.0, box_y + 24.0),
                    vec2(cx + 78.0, box_y + 24.0),
                    vec2(cx + 57.0, box_y + 10.0),
                    PAL_GOLD_1,
                );
                draw_circle(cx + 57.0, box_y + 9.0, 2.5, PAL_GOLD_0);
                let fw = (time * 6.0).sin() * 3.0;
                draw_line(cx + 57.0, box_y + 10.0, cx + 57.0, box_y + 2.0, 1.5, PAL_SAND_2);
                draw_triangle(
                    vec2(cx + 57.0, box_y + 2.0),
                    vec2(cx + 57.0, box_y + 8.0),
                    vec2(cx + 68.0 + fw, box_y + 5.0),
                    PAL_TURMERIC_1,
                );
            }
            _ => {
                // Chitrakuta Parnashala & Padukas mini
                draw_rectangle(cx + 6.0, box_y + box_h - 14.0, card_w - 12.0, 14.0, PAL_GRASS_2);
                // Mountain slope
                draw_triangle(
                    vec2(cx + 10.0, box_y + box_h - 14.0),
                    vec2(cx + 50.0, box_y + box_h - 14.0),
                    vec2(cx + 30.0, box_y + 16.0),
                    Color::new(0.35, 0.45, 0.55, 1.0),
                );
                // Thatched cottage
                draw_rectangle(cx + 46.0, box_y + 26.0, 36.0, 18.0, Color::new(0.72, 0.58, 0.32, 1.0));
                draw_triangle(
                    vec2(cx + 42.0, box_y + 26.0),
                    vec2(cx + 86.0, box_y + 26.0),
                    vec2(cx + 64.0, box_y + 12.0),
                    PAL_GOLD_1,
                );
                // Golden Padukas
                draw_rectangle(cx + 56.0, box_y + 36.0, 16.0, 4.0, PAL_GOLD_0);
                draw_circle(cx + 60.0, box_y + 34.0, 1.5, PAL_RED_1);
                draw_circle(cx + 68.0, box_y + 34.0, 1.5, PAL_RED_1);
            }
        }

        // Bullets
        let mut by = box_y + box_h + 10.0;
        for line in &bullets {
            draw_pixel_text(font, line, cx + 5.0, by, 6, Color::new(0.85, 0.85, 0.88, 1.0));
            by += 10.5;
        }

        // Astras line
        draw_pixel_text(font, "KEY:", cx + 5.0, card_y + 196.0, 6, PAL_GOLD_2);
        draw_pixel_text(font, astras, cx + 32.0, card_y + 196.0, 6, PAL_GOLD_0);

        // Status Launch Button
        let btn_y = card_y + 212.0;
        let btn_h = 24.0;
        let btn_bg = if active {
            Color::new(0.22, 0.18, 0.32, 1.0)
        } else {
            Color::new(0.12, 0.10, 0.16, 1.0)
        };
        draw_rectangle(cx + 6.0, btn_y, card_w - 12.0, btn_h, btn_bg);
        draw_rectangle_lines(cx + 6.0, btn_y, card_w - 12.0, btn_h, 1.0, border_col);

        let pulse_text = if active {
            let blink = (time * 5.0).sin() > 0.0;
            if blink { status_col } else { WHITE }
        } else {
            status_col
        };
        draw_pixel_text(font, status, cx + 18.0, btn_y + 16.0, 7, pulse_text);
    }

    // Bottom Navigation Bar
    let nav_y = 318.0;
    draw_line(40.0, nav_y, 600.0, nav_y, 1.0, Color::new(0.35, 0.30, 0.42, 0.7));
    draw_pixel_text(
        font,
        "[A/D / Arrows / Mouse] Select Card   |   [ENTER / CLICK] Launch",
        62.0,
        333.0,
        8,
        PAL_GOLD_1,
    );
    draw_pixel_text(
        font,
        "[1] Tataka  [2] Siddhashrama  [3] Mithila  [4] Ayodhya  [5] Chitrakuta  [ESC] Title",
        30.0,
        348.0,
        7,
        Color::new(0.75, 0.75, 0.75, 1.0),
    );
}

pub fn get_title_screen_action_at(mouse: Vec2) -> Option<usize> {
    let box_x = 35.0;
    let box_y = 74.0;
    let box_w = 570.0;

    if mouse.x >= box_x + 8.0 && mouse.x <= box_x + box_w - 8.0 {
        if mouse.y >= box_y + 6.0 && mouse.y < box_y + 30.0 {
            return Some(0); // Level 1
        }
        if mouse.y >= box_y + 33.0 && mouse.y < box_y + 57.0 {
            return Some(1); // Level 2
        }
        if mouse.y >= box_y + 60.0 && mouse.y < box_y + 84.0 {
            return Some(2); // Level 3
        }
        if mouse.y >= box_y + 87.0 && mouse.y < box_y + 111.0 {
            return Some(3); // Level 4
        }
        if mouse.y >= box_y + 114.0 && mouse.y < box_y + 138.0 {
            return Some(4); // Level 5
        }
        if mouse.y >= box_y + 144.0 && mouse.y < box_y + 168.0 {
            return Some(5); // Full Chapter Select
        }
    }
    None
}

pub fn get_level_select_card_at(mouse: Vec2) -> Option<usize> {
    let card_y = 54.0;
    let card_h = 244.0;
    let card_w = 114.0;

    if mouse.y >= card_y && mouse.y <= card_y + card_h {
        if mouse.x >= 10.0 && mouse.x <= 10.0 + card_w {
            return Some(0);
        }
        if mouse.x >= 136.0 && mouse.x <= 136.0 + card_w {
            return Some(1);
        }
        if mouse.x >= 262.0 && mouse.x <= 262.0 + card_w {
            return Some(2);
        }
        if mouse.x >= 388.0 && mouse.x <= 388.0 + card_w {
            return Some(3);
        }
        if mouse.x >= 514.0 && mouse.x <= 514.0 + card_w {
            return Some(4);
        }
    }
    None
}

// ============================================================================
// IN-GAME PAUSE MENU
// ============================================================================
pub fn draw_pause_menu(font: &Font, selected: usize, _time: f32, virtual_mouse: Vec2) {
    // Semi-translucent dark backdrop
    draw_rectangle(0.0, 0.0, SCREEN_W, SCREEN_H, Color::new(0.04, 0.03, 0.06, 0.85));

    // Centered modal box (290 x 205)
    let box_x = 175.0;
    let box_y = 75.0;
    let box_w = 290.0;
    let box_h = 205.0;

    draw_rectangle(box_x, box_y, box_w, box_h, Color::new(0.10, 0.09, 0.14, 0.98));
    draw_rectangle_lines(box_x, box_y, box_w, box_h, 2.0, PAL_GOLD_1);

    // Decorative corner accents
    let c = 8.0;
    draw_rectangle(box_x, box_y, c, 2.0, PAL_GOLD_0);
    draw_rectangle(box_x, box_y, 2.0, c, PAL_GOLD_0);
    draw_rectangle(box_x + box_w - c, box_y, c, 2.0, PAL_GOLD_0);
    draw_rectangle(box_x + box_w - 2.0, box_y, 2.0, c, PAL_GOLD_0);
    draw_rectangle(box_x, box_y + box_h - 2.0, c, 2.0, PAL_GOLD_0);
    draw_rectangle(box_x, box_y + box_h - c, 2.0, c, PAL_GOLD_0);
    draw_rectangle(box_x + box_w - c, box_y + box_h - 2.0, c, 2.0, PAL_GOLD_0);
    draw_rectangle(box_x + box_w - 2.0, box_y + box_h - c, 2.0, c, PAL_GOLD_0);

    // Header
    draw_pixel_text(font, "— GAME PAUSED —", box_x + 58.0, box_y + 26.0, 11, PAL_GOLD_1);
    draw_line(box_x + 30.0, box_y + 36.0, box_x + box_w - 30.0, box_y + 36.0, 1.0, PAL_GOLD_3);

    let options = [
        ("RESUME GAME", "[ESC / CLICK]"),
        ("RESTART LEVEL", "[R]"),
        ("SELECT LEVEL", "[L]"),
        ("QUIT TO TITLE", "[T]"),
    ];

    let hov_action = get_pause_menu_action_at(virtual_mouse);

    let mut item_y = box_y + 50.0;
    for (idx, (label, shortcut)) in options.iter().enumerate() {
        let is_sel = selected == idx;
        let is_hov = hov_action == Some(idx);
        let active = is_sel || is_hov;

        let btn_x = box_x + 20.0;
        let btn_w = box_w - 40.0;
        let btn_h = 28.0;

        let btn_bg = if active {
            Color::new(0.24, 0.18, 0.32, 1.0)
        } else {
            Color::new(0.14, 0.12, 0.18, 0.9)
        };
        draw_rectangle(btn_x, item_y, btn_w, btn_h, btn_bg);

        let border_c = if active {
            PAL_GOLD_1
        } else {
            Color::new(0.35, 0.30, 0.40, 0.8)
        };
        draw_rectangle_lines(btn_x, item_y, btn_w, btn_h, if active { 1.8 } else { 1.0 }, border_c);

        if active {
            draw_pixel_text(font, "▶", btn_x + 8.0, item_y + 18.0, 9, PAL_GOLD_0);
        }

        let text_c = if active { PAL_GOLD_0 } else { WHITE };
        draw_pixel_text(font, label, btn_x + 24.0, item_y + 18.0, 9, text_c);
        draw_pixel_text(font, shortcut, btn_x + btn_w - 85.0, item_y + 18.0, 7, Color::new(0.7, 0.7, 0.7, 1.0));

        item_y += 36.0;
    }
}

pub fn get_pause_menu_action_at(mouse: Vec2) -> Option<usize> {
    let box_x = 175.0;
    let box_y = 75.0;
    let box_w = 290.0;
    let btn_x = box_x + 20.0;
    let btn_w = box_w - 40.0;
    let btn_h = 28.0;

    let mut item_y = box_y + 50.0;
    for idx in 0..4 {
        if mouse.x >= btn_x && mouse.x <= btn_x + btn_w && mouse.y >= item_y && mouse.y <= item_y + btn_h {
            return Some(idx);
        }
        item_y += 36.0;
    }
    None
}

// Victory Screen for 16:9
pub fn draw_victory_screen(font: &Font, time: f32) {
    draw_rectangle(0.0, 0.0, SCREEN_W, SCREEN_H, Color::new(0.04, 0.08, 0.05, 0.94));

    let sway = (time * 5.0).sin() * 3.0;
    draw_pixel_text(font, "VICTORY!", 220.0, 65.0 + sway, 26, COLOR_RAMA_GOLD);

    let sub = "TATAKA HAS BEEN VANQUISHED";
    draw_pixel_text(font, sub, 175.0, 95.0, 10, Color::new(0.4, 0.9, 0.5, 1.0));

    let rw_x = 70.0;
    let rw_y = 120.0;
    let rw_w = 500.0;
    let rw_h = 150.0;
    draw_rectangle(rw_x, rw_y, rw_w, rw_h, Color::new(0.10, 0.14, 0.11, 0.95));
    draw_rectangle_lines(rw_x, rw_y, rw_w, rw_h, 1.5, COLOR_RAMA_GOLD);

    draw_pixel_text(font, "DIVINE ASTRAS UNLOCKED FOR LEVEL 2:", rw_x + 16.0, rw_y + 24.0, 9, COLOR_RAMA_GOLD);
    draw_pixel_text(font, "✓ Vayavyastra  : Gale force wind to disperse hordes", rw_x + 16.0, rw_y + 50.0, 8, WHITE);
    draw_pixel_text(font, "✓ Manavastra   : Seeker arrow against demon Maricha", rw_x + 16.0, rw_y + 74.0, 8, WHITE);
    draw_pixel_text(font, "✓ Agneyastra   : Sacred fire to incinerate Subahu", rw_x + 16.0, rw_y + 98.0, 8, WHITE);
    draw_pixel_text(font, "✓ Next Chapter : Level 2 - Siddhashrama Yajna Guard", rw_x + 16.0, rw_y + 124.0, 9, Color::new(0.4, 0.85, 1.0, 1.0));

    let blink = (time * 4.0).sin() > 0.0;
    if blink {
        draw_pixel_text(font, "PRESS SPACE OR CLICK TO PROCEED", 145.0, 305.0, 11, COLOR_RAMA_GOLD);
    }
    draw_pixel_text(font, "Press [R] to Replay Level 1", 205.0, 332.0, 8, Color::new(0.7, 0.7, 0.7, 1.0));
}

// Game Over Screen for 16:9
pub fn draw_game_over(font: &Font, time: f32) {
    draw_rectangle(0.0, 0.0, SCREEN_W, SCREEN_H, Color::new(0.12, 0.02, 0.03, 0.95));

    let sway = (time * 2.0).sin() * 2.5;
    draw_pixel_text(font, "DEFEAT", 235.0, 75.0 + sway, 26, Color::new(0.95, 0.15, 0.15, 1.0));

    let quote = "\"Darkness claims the forest... Tataka reigns.\"";
    draw_pixel_text(font, quote, 110.0, 110.0, 9, Color::new(0.8, 0.6, 0.6, 1.0));

    let tip_x = 80.0;
    let tip_y = 135.0;
    let tip_w = 480.0;
    let tip_h = 115.0;
    draw_rectangle(tip_x, tip_y, tip_w, tip_h, Color::new(0.18, 0.05, 0.07, 0.92));
    draw_rectangle_lines(tip_x, tip_y, tip_w, tip_h, 1.5, Color::new(0.6, 0.2, 0.2, 0.85));

    draw_pixel_text(font, "TACTICAL HINTS (SHABDAVEDHI):", tip_x + 16.0, tip_y + 24.0, 9, COLOR_RAMA_GOLD);
    draw_pixel_text(font, "- Aim directly for sound ripple origins to pin her", tip_x + 16.0, tip_y + 48.0, 8, WHITE);
    draw_pixel_text(font, "- Sidestep thrown boulders with WASD while aiming", tip_x + 16.0, tip_y + 70.0, 8, WHITE);
    draw_pixel_text(font, "- Defeat her before sunset to avoid the 2x Frenzy!", tip_x + 16.0, tip_y + 92.0, 8, Color::new(1.0, 0.8, 0.3, 1.0));

    let blink = (time * 4.0).sin() > 0.0;
    if blink {
        draw_pixel_text(font, "PRESS [R] OR CLICK TO RETRY LEVEL 1", 125.0, 285.0, 11, COLOR_RAMA_GOLD);
    }
    draw_pixel_text(font, "Press [ESC] to Return to Title", 195.0, 315.0, 8, Color::new(0.7, 0.7, 0.7, 1.0));
}

// ============================================================================
// LEVEL 2: SIDDHASHRAMA (THE HERMITAGE OF ACCOMPLISHMENT) RENDERING
// ============================================================================

pub fn draw_siddhashrama_arena(time: f32, yajna_purity: f32) {
    let tile_size = 32.0;
    let cols = (SCREEN_W / tile_size) as usize; // 20
    let rows = (SCREEN_H / tile_size) as usize; // 12

    // 1. Lush Green Hermitage Lawn & Sacred Sandstone Flagstone Crossways
    for y in 0..rows {
        for x in 0..cols {
            let px = x as f32 * tile_size;
            let py = y as f32 * tile_size;

            let is_road_x = py >= 160.0 && py < 192.0;
            let is_road_y = px >= 304.0 && px < 336.0;

            if is_road_x || is_road_y {
                // Sacred terracotta sandstone walkway leading to the Yajna Altar
                let road_hash = (x * 19 + y * 41) % 4;
                let road_col = match road_hash {
                    0 => Color::new(0.44, 0.28, 0.18, 1.0),
                    1 => Color::new(0.48, 0.31, 0.20, 1.0),
                    2 => Color::new(0.42, 0.26, 0.16, 1.0),
                    _ => Color::new(0.46, 0.30, 0.19, 1.0),
                };
                draw_rectangle(px, py, tile_size, tile_size, road_col);
                draw_rectangle_lines(px, py, tile_size, tile_size, 1.0, Color::new(0.35, 0.22, 0.14, 0.7));
            } else {
                // Vibrant hermitage grass with flowers and clover
                let grass_hash = (x * 23 + y * 31) % 8;
                let grass_col = match grass_hash {
                    0 | 1 => Color::new(0.18, 0.38, 0.15, 1.0), // Emerald lawn
                    2 | 3 => Color::new(0.20, 0.42, 0.17, 1.0), // Fresh meadow
                    4 | 5 => Color::new(0.16, 0.35, 0.14, 1.0), // Deep shade grass
                    6 => Color::new(0.22, 0.45, 0.18, 1.0),     // Sunlit patch
                    _ => Color::new(0.19, 0.40, 0.16, 1.0),     // Standard clearing
                };
                draw_rectangle(px, py, tile_size, tile_size, grass_col);

                // Sacred white and yellow hermit grove flowers
                if grass_hash == 2 {
                    draw_circle(px + 10.0, py + 12.0, 1.8, Color::new(1.0, 0.95, 0.6, 0.9));
                    draw_circle(px + 10.0, py + 12.0, 0.8, Color::new(1.0, 0.6, 0.1, 1.0));
                } else if grass_hash == 6 {
                    draw_circle(px + 22.0, py + 20.0, 1.5, Color::new(1.0, 0.7, 0.85, 0.85));
                }
            }
        }
    }

    // 2. Perimeter Sacred Kadamba & Ashoka Grove Trees
    let tree_count = 14;
    for i in 0..tree_count {
        let tx = 40.0 + i as f32 * 43.0;
        // Top boundary foliage
        let sway = (time * 1.5 + i as f32).sin() * 2.0;
        draw_circle(tx + sway, 16.0, 22.0, Color::new(0.12, 0.28, 0.10, 0.95));
        draw_circle(tx + sway - 5.0, 14.0, 16.0, Color::new(0.16, 0.34, 0.12, 0.9));
        // Blossom clusters
        if i % 2 == 0 {
            draw_circle(tx + sway + 4.0, 18.0, 2.5, Color::new(1.0, 0.6, 0.3, 0.9));
        }

        // Bottom boundary foliage
        draw_circle(tx - sway, SCREEN_H - 10.0, 20.0, Color::new(0.12, 0.28, 0.10, 0.95));
        draw_circle(tx - sway + 4.0, SCREEN_H - 14.0, 15.0, Color::new(0.15, 0.33, 0.12, 0.9));
    }

    // 3. Thatched Vedic Hermitages (Parnakuti) in corners
    // Left Ashram Kutir
    draw_rectangle(44.0, 42.0, 56.0, 38.0, Color::new(0.45, 0.32, 0.20, 1.0));
    draw_triangle(
        vec2(38.0, 44.0),
        vec2(72.0, 20.0),
        vec2(106.0, 44.0),
        Color::new(0.72, 0.58, 0.26, 1.0),
    );
    draw_rectangle(64.0, 56.0, 16.0, 24.0, Color::new(0.20, 0.12, 0.08, 1.0)); // Door
    draw_circle(52.0, 72.0, 3.5, Color::new(0.7, 0.35, 0.15, 1.0));            // Kamandalu water pot

    // Right Ashram Kutir
    draw_rectangle(540.0, 42.0, 56.0, 38.0, Color::new(0.45, 0.32, 0.20, 1.0));
    draw_triangle(
        vec2(534.0, 44.0),
        vec2(568.0, 20.0),
        vec2(602.0, 44.0),
        Color::new(0.72, 0.58, 0.26, 1.0),
    );
    draw_rectangle(560.0, 56.0, 16.0, 24.0, Color::new(0.20, 0.12, 0.08, 1.0)); // Door
    draw_circle(590.0, 72.0, 3.5, Color::new(0.7, 0.35, 0.15, 1.0));             // Kamandalu

    // Saffron Pennants (Dhvaja) fluttering
    let flag_wave = (time * 6.0).sin() * 5.0;
    // Flag pole left
    draw_line(150.0, 30.0, 150.0, 68.0, 2.5, Color::new(0.35, 0.25, 0.15, 1.0));
    draw_triangle(
        vec2(150.0, 32.0),
        vec2(172.0 + flag_wave, 38.0),
        vec2(150.0, 46.0),
        Color::new(1.0, 0.55, 0.05, 1.0),
    );
    // Flag pole right
    draw_line(490.0, 30.0, 490.0, 68.0, 2.5, Color::new(0.35, 0.25, 0.15, 1.0));
    draw_triangle(
        vec2(490.0, 32.0),
        vec2(512.0 + flag_wave, 38.0),
        vec2(490.0, 46.0),
        Color::new(1.0, 0.55, 0.05, 1.0),
    );

    // 4. Central Sacred Yajna Altar (320.0, 180.0)
    let altar_c = vec2(320.0, 180.0);

    // Outer Altar Tier (Stepped Brick Platform)
    let outer_sz = 84.0;
    draw_rectangle(
        altar_c.x - outer_sz * 0.5,
        altar_c.y - outer_sz * 0.5,
        outer_sz,
        outer_sz,
        Color::new(0.55, 0.22, 0.15, 1.0),
    );
    draw_rectangle_lines(
        altar_c.x - outer_sz * 0.5,
        altar_c.y - outer_sz * 0.5,
        outer_sz,
        outer_sz,
        2.5,
        Color::new(0.85, 0.75, 0.50, 1.0),
    );

    // Sacred White Lime Rangoli Geometrical Markings on Altar Edges
    let corner_ofs = outer_sz * 0.5 - 6.0;
    for &dx in &[-corner_ofs, corner_ofs] {
        for &dy in &[-corner_ofs, corner_ofs] {
            draw_circle_lines(altar_c.x + dx, altar_c.y + dy, 4.0, 1.2, Color::new(1.0, 1.0, 1.0, 0.8));
        }
    }

    // Middle Altar Tier
    let mid_sz = 58.0;
    draw_rectangle(
        altar_c.x - mid_sz * 0.5,
        altar_c.y - mid_sz * 0.5,
        mid_sz,
        mid_sz,
        Color::new(0.40, 0.15, 0.10, 1.0),
    );
    draw_rectangle_lines(
        altar_c.x - mid_sz * 0.5,
        altar_c.y - mid_sz * 0.5,
        mid_sz,
        mid_sz,
        1.5,
        COLOR_RAMA_GOLD,
    );

    // Inner Fire Kunda Pit
    let pit_sz = 36.0;
    draw_rectangle(
        altar_c.x - pit_sz * 0.5,
        altar_c.y - pit_sz * 0.5,
        pit_sz,
        pit_sz,
        Color::new(0.18, 0.08, 0.06, 1.0),
    );

    // Divine Fire of Agni
    let purity_factor = (yajna_purity / 100.0).clamp(0.05, 1.0);
    let flame_h = 18.0 * purity_factor + ((time * 15.0).sin() * 3.0);
    let flame_w = 14.0 * purity_factor + ((time * 20.0).cos() * 2.0);

    // Sacred Fire Glow Halo
    let glow_alpha = (0.2 + (time * 5.0).sin() * 0.08) * purity_factor;
    draw_circle(altar_c.x, altar_c.y - 4.0, 30.0 * purity_factor, Color::new(1.0, 0.6, 0.1, glow_alpha));

    // Outer Red/Orange Flame
    draw_triangle(
        vec2(altar_c.x - flame_w, altar_c.y + 4.0),
        vec2(altar_c.x + flame_w, altar_c.y + 4.0),
        vec2(altar_c.x, altar_c.y - flame_h - 4.0),
        Color::new(0.95, 0.25, 0.05, 0.95),
    );

    // Middle Golden Flame
    let mid_flame_h = flame_h * 0.75 + ((time * 22.0).sin() * 2.5);
    draw_triangle(
        vec2(altar_c.x - flame_w * 0.65, altar_c.y + 4.0),
        vec2(altar_c.x + flame_w * 0.65, altar_c.y + 4.0),
        vec2(altar_c.x, altar_c.y - mid_flame_h - 2.0),
        Color::new(1.0, 0.75, 0.15, 0.98),
    );

    // Inner White-Hot Flame Core
    let core_h = flame_h * 0.45;
    draw_triangle(
        vec2(altar_c.x - flame_w * 0.35, altar_c.y + 3.0),
        vec2(altar_c.x + flame_w * 0.35, altar_c.y + 3.0),
        vec2(altar_c.x, altar_c.y - core_h),
        Color::new(1.0, 1.0, 0.9, 1.0),
    );

    // Rising Holy Sparks / Ash
    for i in 0..5 {
        let spark_t = (time * 3.0 + i as f32 * 0.6) % 1.0;
        let sx = altar_c.x + ((i as f32 * 2.3 + time * 4.0).sin() * 12.0);
        let sy = altar_c.y - (spark_t * 35.0);
        let s_alpha = (1.0 - spark_t) * purity_factor;
        draw_circle(sx, sy, 1.2, Color::new(1.0, 0.85, 0.3, s_alpha));
    }

    // Impurity smoke if purity is compromised
    if yajna_purity < 50.0 {
        let smoke_intensity = (1.0 - (yajna_purity / 50.0)).clamp(0.0, 1.0);
        for i in 0..4 {
            let smk_t = (time * 2.5 + i as f32 * 0.8) % 1.0;
            let smk_x = altar_c.x + ((time * 3.0 + i as f32).cos() * 14.0);
            let smk_y = altar_c.y - 10.0 - (smk_t * 40.0);
            draw_circle(smk_x, smk_y, 4.0 + smk_t * 6.0, Color::new(0.15, 0.08, 0.10, (1.0 - smk_t) * 0.7 * smoke_intensity));
        }
    }

    // 5. Four Meditating Vedic Rishis Chanting at the Altar
    let rishi_positions = [
        vec2(altar_c.x - 56.0, altar_c.y - 56.0), // NW
        vec2(altar_c.x + 56.0, altar_c.y - 56.0), // NE
        vec2(altar_c.x - 56.0, altar_c.y + 56.0), // SW
        vec2(altar_c.x + 56.0, altar_c.y + 56.0), // SE
    ];

    for (idx, &rpos) in rishi_positions.iter().enumerate() {
        let breath = (time * 2.5 + idx as f32 * 1.5).sin() * 1.0;

        // Chanting holy aura
        let aura_r = 13.0 + breath * 1.5;
        draw_circle(rpos.x, rpos.y, aura_r, Color::new(1.0, 0.85, 0.4, 0.18));
        draw_circle_lines(rpos.x, rpos.y, aura_r, 1.0, Color::new(1.0, 0.75, 0.3, 0.35));

        // Seated Padmasana Robes (Saffron/Ochre Valkala)
        draw_ellipse(rpos.x, rpos.y + 5.0, 8.5, 4.5, 0.0, Color::new(0.92, 0.52, 0.12, 1.0));
        draw_rectangle(rpos.x - 4.5, rpos.y - 2.0, 9.0, 8.0, Color::new(0.88, 0.48, 0.10, 1.0));

        // Rishi Head
        draw_circle(rpos.x, rpos.y - 4.0, 4.5, Color::new(0.88, 0.72, 0.56, 1.0));

        // Topknot (Jata) with rudraksha
        draw_circle(rpos.x, rpos.y - 8.5, 2.5, Color::new(0.35, 0.22, 0.12, 1.0));
        draw_circle(rpos.x, rpos.y - 8.5, 1.0, Color::new(0.65, 0.25, 0.10, 1.0));

        // Flowing White Beard
        draw_triangle(
            vec2(rpos.x - 3.5, rpos.y - 2.0),
            vec2(rpos.x + 3.5, rpos.y - 2.0),
            vec2(rpos.x, rpos.y + 4.0),
            Color::new(0.95, 0.95, 0.95, 0.95),
        );
    }
}

// Draw Rakshasa Minion Swarmer
pub fn draw_rakshasa_minion(pos: Vec2, hit_flash: f32, time: f32) {
    if hit_flash > 0.0 {
        draw_rectangle(pos.x - 8.0, pos.y - 12.0, 16.0, 24.0, WHITE);
        return;
    }

    let leg_bob = (time * 14.0).sin() * 2.5;

    // Body shadow
    draw_ellipse(pos.x, pos.y + 9.0, 7.0, 3.5, 0.0, Color::new(0.0, 0.0, 0.0, 0.4));

    // Demon Torso (Dark charcoal-violet)
    let body_col = Color::new(0.24, 0.18, 0.28, 1.0);
    draw_rectangle(pos.x - 5.0, pos.y - 6.0 + leg_bob, 10.0, 12.0, body_col);

    // Running legs
    draw_line(pos.x - 3.0, pos.y + 6.0 + leg_bob, pos.x - 4.0, pos.y + 11.0, 2.0, body_col);
    draw_line(pos.x + 3.0, pos.y + 6.0 + leg_bob, pos.x + 4.0, pos.y + 11.0, 2.0, body_col);

    // Demon Head
    draw_circle(pos.x, pos.y - 9.0 + leg_bob, 5.0, body_col);

    // Glowing Menacing Eyes
    draw_circle(pos.x - 2.0, pos.y - 10.0 + leg_bob, 1.2, Color::new(1.0, 0.8, 0.1, 1.0));
    draw_circle(pos.x + 2.0, pos.y - 10.0 + leg_bob, 1.2, Color::new(1.0, 0.8, 0.1, 1.0));

    // Jagged Horns
    draw_line(pos.x - 3.0, pos.y - 12.0 + leg_bob, pos.x - 6.0, pos.y - 17.0 + leg_bob, 1.8, Color::new(0.12, 0.08, 0.14, 1.0));
    draw_line(pos.x + 3.0, pos.y - 12.0 + leg_bob, pos.x + 6.0, pos.y - 17.0 + leg_bob, 1.8, Color::new(0.12, 0.08, 0.14, 1.0));

    // Vessel of Defiling Blood in hands
    draw_circle(pos.x, pos.y + leg_bob, 3.5, Color::new(0.55, 0.10, 0.12, 1.0));
    draw_circle(pos.x, pos.y - 1.0 + leg_bob, 2.2, Color::new(0.75, 0.05, 0.10, 1.0));
}

// Draw Winged Asura (Airborne Blood Bomber)
pub fn draw_winged_asura(pos: Vec2, hit_flash: f32, time: f32) {
    // Cast shadow on the ground
    draw_ellipse(pos.x, pos.y + 40.0, 12.0, 5.0, 0.0, Color::new(0.0, 0.0, 0.0, 0.35));

    if hit_flash > 0.0 {
        draw_rectangle(pos.x - 12.0, pos.y - 12.0, 24.0, 24.0, WHITE);
        return;
    }

    let flap = (time * 12.0).sin() * 8.0;

    // Bat-like Leathery Wings
    let wing_col = Color::new(0.35, 0.08, 0.15, 0.95);
    // Left Wing
    draw_triangle(
        vec2(pos.x - 4.0, pos.y - 2.0),
        vec2(pos.x - 22.0, pos.y - 12.0 + flap),
        vec2(pos.x - 14.0, pos.y + 4.0 + flap * 0.5),
        wing_col,
    );
    // Right Wing
    draw_triangle(
        vec2(pos.x + 4.0, pos.y - 2.0),
        vec2(pos.x + 22.0, pos.y - 12.0 + flap),
        vec2(pos.x + 14.0, pos.y + 4.0 + flap * 0.5),
        wing_col,
    );

    // Torso (Dark Crimson)
    let body_col = Color::new(0.45, 0.10, 0.18, 1.0);
    draw_ellipse(pos.x, pos.y, 6.0, 9.0, 0.0, body_col);

    // Head with glowing green eyes
    draw_circle(pos.x, pos.y - 8.0, 4.5, body_col);
    draw_circle(pos.x - 1.8, pos.y - 9.0, 1.2, Color::new(0.2, 1.0, 0.4, 1.0));
    draw_circle(pos.x + 1.8, pos.y - 9.0, 1.2, Color::new(0.2, 1.0, 0.4, 1.0));

    // Blood bomb in talons
    draw_circle(pos.x, pos.y + 8.0, 4.0, Color::new(0.70, 0.05, 0.10, 1.0));
}

// ============================================================================
// 16-BIT RETRO SPRITES: MARICHA (THE ILLUSIONIST SORCERER)
// ============================================================================
pub fn draw_maricha(pos: Vec2, hit_flash: f32, time: f32) {
    if hit_flash > 0.0 {
        draw_rectangle(pos.x - 14.0, pos.y - 20.0, 28.0, 40.0, WHITE);
        return;
    }

    // Sorcerous Cloud Base (Swirling Indigo Fog)
    let cloud_w = 22.0 + ((time * 8.0).sin() * 3.5);
    draw_ellipse(pos.x, pos.y + 14.0, cloud_w, 6.5, 0.0, Color::new(0.16, 0.19, 0.35, 0.65));
    draw_ellipse(pos.x, pos.y + 12.0, cloud_w * 0.7, 4.5, 0.0, Color::new(0.25, 0.29, 0.50, 0.55));

    // Maya Illusion Afterimages (Telegraphing Deceptive Clones)
    let mirage_ofs = (time * 5.0).sin() * 14.0;
    for &dir in &[-1.0f32, 1.0f32] {
        let mirage_x = pos.x + dir * mirage_ofs;
        draw_rectangle(
            mirage_x - 7.0,
            pos.y - 12.0,
            14.0,
            24.0,
            Color::new(0.37, 0.42, 0.66, 0.25),
        );
        draw_circle(mirage_x, pos.y - 16.0, 6.0, Color::new(0.55, 0.61, 0.80, 0.25));
    }

    // Regal Indigo Robes (4-tone Rakshasa Blue)
    draw_rectangle(pos.x - 8.0, pos.y - 8.0, 16.0, 22.0, PAL_BLUE_3);
    draw_rectangle(pos.x - 7.0, pos.y - 7.0, 14.0, 20.0, PAL_BLUE_2);
    draw_rectangle(pos.x - 4.0, pos.y - 6.0, 8.0, 18.0, PAL_BLUE_1);
    draw_rectangle_lines(pos.x - 7.0, pos.y - 7.0, 14.0, 20.0, 1.2, PAL_GOLD_1);

    // Demon Head & Visage
    draw_circle(pos.x, pos.y - 14.0, 6.5, PAL_BLUE_3);
    draw_circle(pos.x, pos.y - 14.0, 5.5, PAL_BLUE_2);
    draw_circle(pos.x, pos.y - 15.0, 3.5, PAL_BLUE_1);

    // Golden Royal Mukuta (Crown)
    draw_triangle(
        vec2(pos.x - 6.5, pos.y - 17.0),
        vec2(pos.x + 6.5, pos.y - 17.0),
        vec2(pos.x, pos.y - 27.0),
        PAL_GOLD_2,
    );
    draw_triangle(
        vec2(pos.x - 4.0, pos.y - 17.0),
        vec2(pos.x + 4.0, pos.y - 17.0),
        vec2(pos.x, pos.y - 25.0),
        PAL_GOLD_0,
    );
    draw_circle(pos.x, pos.y - 22.0, 1.8, PAL_RED_0); // Crown ruby

    // Fierce Fiery Eyes
    draw_circle(pos.x - 2.5, pos.y - 15.0, 1.5, PAL_RED_0);
    draw_circle(pos.x + 2.5, pos.y - 15.0, 1.5, PAL_RED_0);
    draw_circle(pos.x - 2.5, pos.y - 15.0, 0.7, WHITE);
    draw_circle(pos.x + 2.5, pos.y - 15.0, 0.7, WHITE);

    // Dark Sorcery Sigils rotating around hands
    let sigil_rot = time * 3.5;
    for i in 0..4 {
        let a = sigil_rot + i as f32 * (std::f32::consts::PI * 0.5);
        let sx = pos.x + a.cos() * 18.0;
        let sy = pos.y + a.sin() * 9.0;
        draw_circle(sx, sy, 2.2, PAL_BLUE_0);
        draw_circle(sx, sy, 1.0, WHITE);
    }
}

// ============================================================================
// 16-BIT RETRO SPRITES: SUBAHU (ARMORED BRUTE OF DEFILEMENT)
// ============================================================================
pub fn draw_subahu(pos: Vec2, hit_flash: f32, has_shield: bool, time: f32) {
    if hit_flash > 0.0 {
        draw_rectangle(pos.x - 16.0, pos.y - 22.0, 32.0, 44.0, WHITE);
        return;
    }

    // Heavy Ground Drop Shadow
    draw_ellipse(pos.x, pos.y + 18.0, 18.0, 6.5, 0.0, Color::new(0.04, 0.02, 0.03, 0.55));

    // Spiked Iron Pauldrons (Shoulders in 4-tone Rakshasa Red / Iron)
    draw_rectangle(pos.x - 17.0, pos.y - 14.0, 34.0, 10.0, PAL_RED_3);
    draw_rectangle(pos.x - 15.0, pos.y - 13.0, 30.0, 8.0, PAL_RED_2);
    // Shoulder spikes
    draw_triangle(
        vec2(pos.x - 17.0, pos.y - 14.0),
        vec2(pos.x - 12.0, pos.y - 14.0),
        vec2(pos.x - 20.0, pos.y - 22.0),
        PAL_RED_0,
    );
    draw_triangle(
        vec2(pos.x + 12.0, pos.y - 14.0),
        vec2(pos.x + 17.0, pos.y - 14.0),
        vec2(pos.x + 20.0, pos.y - 22.0),
        PAL_RED_0,
    );

    // Torso (Heavy Bronze / Iron Breastplate)
    draw_rectangle(pos.x - 11.0, pos.y - 6.0, 22.0, 23.0, PAL_RED_3);
    draw_rectangle(pos.x - 9.0, pos.y - 5.0, 18.0, 20.0, PAL_RED_2);
    draw_rectangle(pos.x - 6.0, pos.y - 4.0, 12.0, 8.0, PAL_RED_1);
    draw_rectangle_lines(pos.x - 9.0, pos.y - 5.0, 18.0, 20.0, 1.5, PAL_GOLD_3);

    // Massive Horned Demonic War Helm
    draw_circle(pos.x, pos.y - 16.0, 8.0, PAL_RED_3);
    draw_circle(pos.x, pos.y - 16.0, 6.5, PAL_RED_2);
    // Curved Iron Horns
    draw_line(pos.x - 5.0, pos.y - 20.0, pos.x - 14.0, pos.y - 29.0, 3.0, PAL_RED_1);
    draw_line(pos.x - 14.0, pos.y - 29.0, pos.x - 11.0, pos.y - 32.0, 2.0, PAL_RED_0);
    draw_line(pos.x + 5.0, pos.y - 20.0, pos.x + 14.0, pos.y - 29.0, 3.0, PAL_RED_1);
    draw_line(pos.x + 14.0, pos.y - 29.0, pos.x + 11.0, pos.y - 32.0, 2.0, PAL_RED_0);

    // Blood-Red Glowing Slit Visor
    draw_line(pos.x - 4.0, pos.y - 16.0, pos.x + 4.0, pos.y - 16.0, 2.5, PAL_FIRE_3);
    draw_line(pos.x - 2.0, pos.y - 16.0, pos.x + 2.0, pos.y - 16.0, 1.5, PAL_FIRE_0);

    // Massive Bronze Defilement Cauldron carried in hands
    draw_circle(pos.x, pos.y + 6.0, 7.5, PAL_SAND_4);
    draw_circle(pos.x, pos.y + 6.0, 6.0, PAL_RED_3);
    draw_circle(pos.x, pos.y + 4.0, 4.5, PAL_FIRE_4); // Boiling sacrificial defilement gore

    // Crackling Miasma Shield (Vulnerable to Agneyastra!)
    if has_shield {
        let shield_pulse = (time * 4.0).sin() * 2.5;
        let shield_r = 26.0 + shield_pulse;
        draw_circle(pos.x, pos.y, shield_r, Color::new(0.55, 0.12, 0.75, 0.22));
        draw_circle_lines(pos.x, pos.y, shield_r, 2.0, PAL_PURPLE_0);

        // Orbiting Hex Shield Sigils
        let shield_rot = time * 2.8;
        for i in 0..4 {
            let a = shield_rot + i as f32 * (std::f32::consts::PI * 0.5);
            let hx = pos.x + a.cos() * shield_r;
            let hy = pos.y + a.sin() * shield_r;
            draw_circle(hx, hy, 3.0, PAL_PURPLE_1);
            draw_circle(hx, hy, 1.5, WHITE);
        }
    }
}

// Draw Astra Projectile based on its Divine Type
pub fn draw_astra_projectile(pos: Vec2, vel: Vec2, p_type: AstraProjectileType, time: f32) {
    let dir = vel.normalize_or_zero();
    let _angle = vel.y.atan2(vel.x);

    match p_type {
        AstraProjectileType::Kodanda => {
            // Standard Golden Kodanda Arrow
            let p1 = pos - dir * 10.0;
            let p2 = pos + dir * 10.0;
            draw_line(p1.x, p1.y, p2.x, p2.y, 2.0, COLOR_RAMA_GOLD);
            // Arrowhead
            draw_circle(p2.x, p2.y, 2.0, WHITE);
            // Fletchings
            let perp = vec2(-dir.y, dir.x) * 2.5;
            draw_line(p1.x, p1.y, p1.x + perp.x, p1.y + perp.y, 1.5, COLOR_RAMA_GOLD);
            draw_line(p1.x, p1.y, p1.x - perp.x, p1.y - perp.y, 1.5, COLOR_RAMA_GOLD);
        }
        AstraProjectileType::Vayavya => {
            // Vayavyastra: Wind Gale Vortex Arrow
            let p1 = pos - dir * 12.0;
            let p2 = pos + dir * 12.0;
            draw_line(p1.x, p1.y, p2.x, p2.y, 2.5, Color::new(0.7, 0.95, 1.0, 0.9));

            // Rotating Wind Rings around the shaft
            for i in 0..3 {
                let ring_phase = time * 24.0 + i as f32 * 2.0;
                let offset_dist = (i as f32 - 1.0) * 6.0;
                let ring_c = pos + dir * offset_dist;
                let ring_h = 6.0 * ring_phase.sin();
                let perp = vec2(-dir.y, dir.x) * ring_h;
                draw_line(
                    ring_c.x - perp.x,
                    ring_c.y - perp.y,
                    ring_c.x + perp.x,
                    ring_c.y + perp.y,
                    1.8,
                    Color::new(0.8, 1.0, 1.0, 0.75),
                );
            }
        }
        AstraProjectileType::Manava => {
            // Manavastra: Celestial Seeker of Vishnu / Manu
            let p1 = pos - dir * 14.0;
            let p2 = pos + dir * 14.0;
            draw_line(p1.x, p1.y, p2.x, p2.y, 3.0, Color::new(0.3, 0.7, 1.0, 1.0));
            draw_line(p1.x, p1.y, p2.x, p2.y, 1.2, WHITE);

            // Radiant Starlight Tip
            draw_circle(p2.x, p2.y, 3.5, Color::new(0.4, 0.85, 1.0, 0.95));
            draw_circle(p2.x, p2.y, 1.8, WHITE);

            // Trailing Diamond Glyph Cross
            let cross_sz = 3.0;
            draw_line(pos.x - cross_sz, pos.y, pos.x + cross_sz, pos.y, 1.2, Color::new(0.6, 0.9, 1.0, 0.8));
            draw_line(pos.x, pos.y - cross_sz, pos.x, pos.y + cross_sz, 1.2, Color::new(0.6, 0.9, 1.0, 0.8));
        }
        AstraProjectileType::Agneya => {
            // Agneyastra: Sacred Solar Fire of Agni
            let p1 = pos - dir * 14.0;
            let p2 = pos + dir * 14.0;
            draw_line(p1.x, p1.y, p2.x, p2.y, 3.5, Color::new(1.0, 0.4, 0.05, 0.95));
            draw_line(p1.x, p1.y, p2.x, p2.y, 1.5, Color::new(1.0, 0.9, 0.3, 1.0));

            // Solar Flame Burst around head
            let flame_flicker = (time * 28.0).sin() * 2.0;
            draw_circle(p2.x, p2.y, 4.5 + flame_flicker, Color::new(1.0, 0.5, 0.1, 0.8));
            draw_circle(p2.x, p2.y, 2.2, Color::new(1.0, 0.95, 0.6, 1.0));

            // Flaming ember sparks
            for i in 0..2 {
                let sx = pos.x + ((time * 18.0 + i as f32).sin() * 4.0);
                let sy = pos.y + ((time * 22.0 + i as f32).cos() * 4.0);
                draw_circle(sx, sy, 1.5, Color::new(1.0, 0.7, 0.2, 0.8));
            }
        }
    }
}

// Draw Blood Bomb thrown by Winged Asuras
pub fn draw_blood_bomb(pos: Vec2, target_pos: Vec2, altitude: f32) {
    // Target landing shadow on ground
    let shadow_radius = 8.0 * (1.0 - altitude * 0.4);
    draw_ellipse(
        target_pos.x,
        target_pos.y,
        shadow_radius,
        shadow_radius * 0.45,
        0.0,
        Color::new(0.5, 0.05, 0.1, 0.45),
    );
    draw_circle_lines(
        target_pos.x,
        target_pos.y,
        shadow_radius,
        1.0,
        Color::new(0.8, 0.1, 0.15, 0.6),
    );

    // Flying Clay Pot of Unholy Blood (elevated by altitude)
    let bomb_pos = pos + vec2(0.0, -altitude * 48.0);
    draw_circle(bomb_pos.x, bomb_pos.y, 4.5, Color::new(0.35, 0.15, 0.10, 1.0));
    draw_circle(bomb_pos.x, bomb_pos.y - 1.0, 3.0, Color::new(0.65, 0.05, 0.12, 1.0));

    // Blood droplets trailing
    draw_circle(bomb_pos.x, bomb_pos.y + 4.0, 1.5, Color::new(0.65, 0.05, 0.12, 0.7));
}

// Draw Dark Spike shot by Maricha
pub fn draw_dark_spike(pos: Vec2, vel: Vec2) {
    let dir = vel.normalize_or_zero();
    let p1 = pos - dir * 8.0;
    let p2 = pos + dir * 8.0;
    draw_line(p1.x, p1.y, p2.x, p2.y, 2.5, Color::new(0.45, 0.15, 0.65, 0.95));
    draw_circle(p2.x, p2.y, 2.0, Color::new(0.7, 0.3, 0.9, 1.0));
}

// Level 2 HUD for 16:9 Widescreen (Yajna Purity, Waves, Boss HP, Astra Selector)
pub fn draw_hud_l2(
    font: &Font,
    stats: &PlayerStats,
    yajna_purity: f32,
    wave: i32,
    maricha_hp: Option<(f32, f32)>,
    subahu_hp: Option<(f32, f32)>,
) {
    // Top HUD Bar Panel
    draw_rectangle(0.0, 0.0, SCREEN_W, 34.0, Color::new(0.06, 0.08, 0.10, 0.95));
    draw_line(0.0, 34.0, SCREEN_W, 34.0, 2.0, Color::new(0.35, 0.30, 0.20, 1.0));

    // Left: Prana (HP)
    let hp_ratio = (stats.prana as f32 / stats.max_prana as f32).clamp(0.0, 1.0);
    draw_pixel_text(font, "HP", 10.0, 12.0, 8, Color::new(0.9, 0.8, 0.4, 1.0));
    draw_rectangle(32.0, 6.0, 70.0, 8.0, Color::new(0.18, 0.06, 0.06, 1.0));
    let hp_color = if hp_ratio > 0.4 {
        Color::new(0.2, 0.85, 0.3, 1.0)
    } else {
        Color::new(0.9, 0.2, 0.2, 1.0)
    };
    draw_rectangle(32.0, 6.0, 70.0 * hp_ratio, 8.0, hp_color);
    draw_rectangle_lines(32.0, 6.0, 70.0, 8.0, 1.0, COLOR_RAMA_GOLD);
    draw_pixel_text(font, &format!("{}/{}", stats.prana, stats.max_prana), 36.0, 24.0, 8, WHITE);

    // Tejas (MP / Astra Energy)
    let mp_ratio = (stats.tejas as f32 / stats.max_tejas as f32).clamp(0.0, 1.0);
    draw_pixel_text(font, "MP", 120.0, 12.0, 8, Color::new(0.4, 0.8, 1.0, 1.0));
    draw_rectangle(142.0, 6.0, 65.0, 8.0, Color::new(0.06, 0.12, 0.20, 1.0));
    draw_rectangle(142.0, 6.0, 65.0 * mp_ratio, 8.0, Color::new(0.2, 0.6, 1.0, 1.0));
    draw_rectangle_lines(142.0, 6.0, 65.0, 8.0, 1.0, Color::new(0.5, 0.8, 1.0, 1.0));
    draw_pixel_text(font, &format!("{}/{}", stats.tejas, stats.max_tejas), 146.0, 24.0, 8, WHITE);

    // Center: Central Yajna Altar Purity Meter
    let purity_w = 140.0;
    let purity_x = (SCREEN_W - purity_w) * 0.5;
    let purity_pct = (yajna_purity as i32).clamp(0, 100);

    let (purity_col, label_text) = if yajna_purity >= 60.0 {
        (COLOR_RAMA_GOLD, "YAJNA PURITY")
    } else if yajna_purity >= 30.0 {
        (Color::new(1.0, 0.6, 0.1, 1.0), "PURITY DEFILING!")
    } else {
        (Color::new(1.0, 0.15, 0.15, 1.0), "CRITICAL DEFILEMENT!")
    };

    draw_pixel_text(font, label_text, purity_x + 8.0, 12.0, 8, purity_col);
    draw_rectangle(purity_x, 16.0, purity_w, 10.0, Color::new(0.12, 0.10, 0.14, 1.0));
    let p_ratio = (yajna_purity / 100.0).clamp(0.0, 1.0);
    draw_rectangle(purity_x, 16.0, purity_w * p_ratio, 10.0, purity_col);
    draw_rectangle_lines(purity_x, 16.0, purity_w, 10.0, 1.5, COLOR_RAMA_GOLD);

    // Flame icon before bar
    draw_circle(purity_x - 9.0, 21.0, 4.0, Color::new(1.0, 0.5, 0.1, 1.0));
    draw_pixel_text(font, &format!("{}%", purity_pct), purity_x + purity_w + 6.0, 24.0, 8, WHITE);

    // Right: Wave Counter
    let wave_str = match wave {
        1 => "WAVE 1/3 (HORDE)",
        2 => "WAVE 2/3 (AIR)",
        _ => "WAVE 3/3 (BOSSES)",
    };
    draw_pixel_text(font, wave_str, 480.0, 16.0, 9, Color::new(0.9, 0.85, 0.6, 1.0));

    // Boss Bars if active (Rendered right beneath top bar)
    if let Some((m_hp, m_max)) = maricha_hp {
        let bar_x = 40.0;
        let bar_y = 38.0;
        let bar_w = 160.0;
        draw_rectangle(bar_x, bar_y, bar_w, 8.0, Color::new(0.12, 0.08, 0.18, 0.95));
        let m_ratio = (m_hp / m_max).clamp(0.0, 1.0);
        draw_rectangle(bar_x, bar_y, bar_w * m_ratio, 8.0, Color::new(0.3, 0.7, 1.0, 1.0));
        draw_rectangle_lines(bar_x, bar_y, bar_w, 8.0, 1.0, Color::new(0.5, 0.85, 1.0, 1.0));
        draw_pixel_text(font, "MARICHA (MANAVA COUNTERS)", bar_x, bar_y + 16.0, 7, Color::new(0.4, 0.85, 1.0, 1.0));
    }

    if let Some((s_hp, s_max)) = subahu_hp {
        let bar_x = SCREEN_W - 200.0;
        let bar_y = 38.0;
        let bar_w = 160.0;
        draw_rectangle(bar_x, bar_y, bar_w, 8.0, Color::new(0.20, 0.08, 0.08, 0.95));
        let s_ratio = (s_hp / s_max).clamp(0.0, 1.0);
        draw_rectangle(bar_x, bar_y, bar_w * s_ratio, 8.0, Color::new(0.9, 0.25, 0.1, 1.0));
        draw_rectangle_lines(bar_x, bar_y, bar_w, 8.0, 1.0, Color::new(1.0, 0.5, 0.2, 1.0));
        draw_pixel_text(font, "SUBAHU (AGNEYA COUNTERS)", bar_x, bar_y + 16.0, 7, Color::new(1.0, 0.6, 0.2, 1.0));
    }

    // Bottom Toolbar & Astra Weapon Selector
    draw_rectangle(0.0, SCREEN_H - 22.0, SCREEN_W, 22.0, Color::new(0.05, 0.06, 0.08, 0.95));
    draw_line(0.0, SCREEN_H - 22.0, SCREEN_W, SCREEN_H - 22.0, 1.0, Color::new(0.25, 0.22, 0.16, 1.0));

    // Astra Slots with clean 110px spacing
    let astra_slots = [
        (Astra::KodandaBow, "[1] KODANDA (0)", 8.0),
        (Astra::Vayavyastra, "[2] VAYAVYA (20)", 122.0),
        (Astra::Manavastra, "[3] MANAVA (25)", 236.0),
        (Astra::Agneyastra, "[4] AGNEYA (25)", 350.0),
    ];

    for &(astra_type, label, slot_x) in &astra_slots {
        let is_selected = stats.selected_astra == astra_type;
        if is_selected {
            draw_rectangle(slot_x - 3.0, SCREEN_H - 20.0, 110.0, 18.0, Color::new(0.22, 0.18, 0.08, 0.9));
            draw_rectangle_lines(slot_x - 3.0, SCREEN_H - 20.0, 110.0, 18.0, 1.5, COLOR_RAMA_GOLD);
            draw_pixel_text(font, label, slot_x + 2.0, SCREEN_H - 8.0, 7, COLOR_RAMA_GOLD);
        } else {
            draw_pixel_text(font, label, slot_x + 2.0, SCREEN_H - 8.0, 7, Color::new(0.65, 0.65, 0.65, 1.0));
        }
    }

    draw_pixel_text(font, "WASD:Move | Mouse:Aim/Fire", 466.0, SCREEN_H - 8.0, 7, Color::new(0.85, 0.85, 0.9, 1.0));
}

// Victory Screen for Level 2 (Siddhashrama)
pub fn draw_victory_l2(font: &Font, time: f32) {
    draw_rectangle(0.0, 0.0, SCREEN_W, SCREEN_H, Color::new(0.04, 0.07, 0.09, 0.95));

    let sway = (time * 5.0).sin() * 3.0;
    draw_pixel_text(font, "VICTORY AT SIDDHASHRAMA!", 130.0, 60.0 + sway, 20, COLOR_RAMA_GOLD);

    let sub = "THE SACRED YAJNA IS DEFENDED AND SANCTIFIED";
    draw_pixel_text(font, sub, 125.0, 88.0, 9, Color::new(0.4, 0.9, 0.6, 1.0));

    let rw_x = 55.0;
    let rw_y = 110.0;
    let rw_w = 530.0;
    let rw_h = 160.0;
    draw_rectangle(rw_x, rw_y, rw_w, rw_h, Color::new(0.09, 0.13, 0.16, 0.95));
    draw_rectangle_lines(rw_x, rw_y, rw_w, rw_h, 1.5, COLOR_RAMA_GOLD);

    draw_pixel_text(font, "VALMIKI RAMAYANA - BALA KANDA ACHIEVEMENTS:", rw_x + 16.0, rw_y + 22.0, 9, COLOR_RAMA_GOLD);
    draw_pixel_text(font, "✓ Manavastra struck Maricha, flinging him 100 yojanas into the ocean!", rw_x + 16.0, rw_y + 46.0, 8, WHITE);
    draw_pixel_text(font, "✓ Agneyastra incinerated Subahu and his unholy gore cauldrons to ash!", rw_x + 16.0, rw_y + 68.0, 8, WHITE);
    draw_pixel_text(font, "✓ Vayavyastra scattered the Rakshasa horde across the horizon!", rw_x + 16.0, rw_y + 90.0, 8, WHITE);
    draw_pixel_text(font, "✓ Sage Vishwamitra concludes the 6-day Yajna in blissful peace!", rw_x + 16.0, rw_y + 112.0, 8, Color::new(1.0, 0.85, 0.3, 1.0));
    draw_pixel_text(font, "✓ Next Destination: Chapter 3 - Mithila & Shiva Dhanush!", rw_x + 16.0, rw_y + 136.0, 9, Color::new(0.4, 0.85, 1.0, 1.0));

    let blink = (time * 4.0).sin() > 0.0;
    if blink {
        draw_pixel_text(font, "PRESS SPACE OR CLICK TO PROCEED TO TITLE", 120.0, 298.0, 10, COLOR_RAMA_GOLD);
    }
    draw_pixel_text(font, "Press [R] to Replay Level 2 (Siddhashrama)", 165.0, 324.0, 8, Color::new(0.7, 0.7, 0.7, 1.0));
}

// Victory Screen for Level 3 (Mithila & Bala Kanda Finale)
pub fn draw_victory_l3(font: &Font, time: f32) {
    draw_rectangle(0.0, 0.0, SCREEN_W, SCREEN_H, Color::new(0.06, 0.05, 0.10, 0.96));

    let sway = (time * 5.0).sin() * 3.0;
    draw_pixel_text(font, "BALA KANDA CONCLUDED!", 145.0, 56.0 + sway, 20, PAL_GOLD_0);

    let sub = "SITA KALYANAM CELEBRATED & PARASHURAMA RECONCILED";
    draw_pixel_text(font, sub, 105.0, 84.0, 9, PAL_TURMERIC_1);

    let rw_x = 55.0;
    let rw_y = 104.0;
    let rw_w = 530.0;
    let rw_h = 168.0;
    draw_rectangle(rw_x, rw_y, rw_w, rw_h, Color::new(0.10, 0.09, 0.16, 0.95));
    draw_rectangle_lines(rw_x, rw_y, rw_w, rw_h, 1.5, PAL_GOLD_1);

    draw_pixel_text(font, "EPIC BALA KANDA MILESTONES COMPLETED:", rw_x + 16.0, rw_y + 22.0, 9, PAL_GOLD_0);
    draw_pixel_text(font, "✓ Pinaka Shattered : Cosmic bow broken in King Janaka's court!", rw_x + 16.0, rw_y + 44.0, 8, WHITE);
    draw_pixel_text(font, "✓ Lineages Honored : Divine births of Rama and Sita revealed in media res!", rw_x + 16.0, rw_y + 66.0, 8, WHITE);
    draw_pixel_text(font, "✓ Sita Kalyanam    : Sacred wedding solemnized amidst celestial flower shower!", rw_x + 16.0, rw_y + 88.0, 8, PAL_GOLD_1);
    draw_pixel_text(font, "✓ Sharanga Strung  : Rama strings Vaishnava bow, pacifying Sage Parashurama!", rw_x + 16.0, rw_y + 110.0, 8, WHITE);
    draw_pixel_text(font, "★ BALA KANDA COMPLETE — ONWARD TO AYODHYA KANDA! ★", rw_x + 16.0, rw_y + 138.0, 9, PAL_FIRE_1);

    let blink = (time * 4.0).sin() > 0.0;
    if blink {
        draw_pixel_text(font, "PRESS SPACE OR CLICK TO RETURN TO TITLE", 130.0, 298.0, 10, PAL_GOLD_0);
    }
    draw_pixel_text(font, "Press [R] to Replay Level 3 (Mithila Swayamvara)", 140.0, 324.0, 8, Color::new(0.7, 0.7, 0.7, 1.0));
}

// Defeat Screen for Level 2 (Siddhashrama)
pub fn draw_game_over_l2(font: &Font, time: f32) {
    draw_rectangle(0.0, 0.0, SCREEN_W, SCREEN_H, Color::new(0.12, 0.03, 0.04, 0.95));

    let sway = (time * 2.0).sin() * 2.5;
    draw_pixel_text(font, "THE YAJNA IS DEFILED", 180.0, 70.0 + sway, 20, Color::new(0.95, 0.2, 0.2, 1.0));

    let quote = "\"The holy fire is extinguished under a deluge of darkness.\"";
    draw_pixel_text(font, quote, 90.0, 100.0, 9, Color::new(0.8, 0.6, 0.6, 1.0));

    let tip_x = 60.0;
    let tip_y = 125.0;
    let tip_w = 520.0;
    let tip_h = 130.0;
    draw_rectangle(tip_x, tip_y, tip_w, tip_h, Color::new(0.18, 0.05, 0.08, 0.92));
    draw_rectangle_lines(tip_x, tip_y, tip_w, tip_h, 1.5, Color::new(0.6, 0.2, 0.2, 0.85));

    draw_pixel_text(font, "TACTICAL ASTRA STRATEGY:", tip_x + 16.0, tip_y + 22.0, 9, COLOR_RAMA_GOLD);
    draw_pixel_text(font, "- [2] Vayavyastra: Sweeps minion swarms far away from the altar!", tip_x + 16.0, tip_y + 44.0, 8, Color::new(0.6, 0.9, 1.0, 1.0));
    draw_pixel_text(font, "- [3] Manavastra: Homes in on Maricha & knocks him 180px away!", tip_x + 16.0, tip_y + 66.0, 8, Color::new(0.4, 0.8, 1.0, 1.0));
    draw_pixel_text(font, "- [4] Agneyastra: Shatters Subahu's miasma shield & burns him!", tip_x + 16.0, tip_y + 88.0, 8, Color::new(1.0, 0.6, 0.2, 1.0));
    draw_pixel_text(font, "- Shoot down Winged Asuras before their blood bombs land!", tip_x + 16.0, tip_y + 110.0, 8, WHITE);

    let blink = (time * 4.0).sin() > 0.0;
    if blink {
        draw_pixel_text(font, "PRESS [R] OR CLICK TO RETRY LEVEL 2", 140.0, 285.0, 11, COLOR_RAMA_GOLD);
    }
    draw_pixel_text(font, "Press [ESC] to Return to Title", 195.0, 315.0, 8, Color::new(0.7, 0.7, 0.7, 1.0));
}

// ============================================================================
// 16-BIT RETRO CUTSCENE SCREEN (IN MEDIA RES LINEAGE FLASHBACKS)
// ============================================================================
pub fn draw_cutscene_screen(
    font: &Font,
    cutscene: CutsceneType,
    slide_idx: usize,
    time: f32,
    virtual_mouse: Vec2,
) {
    clear_background(Color::new(0.05, 0.04, 0.08, 1.0));

    // Top Story Title & Decorative Trim
    let (chapter_tag, story_title) = match cutscene {
        CutsceneType::RamaBirth => ("BALA KANDA: LINEAGE RECITAL (PART I)", "THE BIRTH OF PRINCE RAMA"),
        CutsceneType::SitaBirth => ("BALA KANDA: LINEAGE RECITAL (PART II)", "THE MIRACULOUS BIRTH OF SITA"),
        CutsceneType::SitaKalyanam => ("BALA KANDA: THE SACRED UNION", "SITA KALYANAM — THE ROYAL WEDDING"),
        CutsceneType::ParashuramaEncounter => ("BALA KANDA: THE FINALE", "STANDOFF WITH SAGE PARASHURAMA"),
        CutsceneType::AyodhyaExile => ("AYODHYA KANDA: THE EXILE DECREE", "THE TWO BOONS & THE SACRED VALKALA ROBES"),
    };

    draw_pixel_text(font, chapter_tag, 40.0, 20.0, 7, PAL_GOLD_2);
    draw_pixel_text(font, story_title, 40.0, 34.0, 10, PAL_GOLD_0);
    draw_line(40.0, 42.0, 600.0, 42.0, 1.5, PAL_GOLD_3);

    // 16-Bit Pixel Art Vignette Window (560 x 175)
    let vig_x = 40.0;
    let vig_y = 50.0;
    let vig_w = 560.0;
    let vig_h = 180.0;

    draw_rectangle(vig_x, vig_y, vig_w, vig_h, Color::new(0.08, 0.07, 0.12, 1.0));

    match cutscene {
        CutsceneType::RamaBirth => draw_vignette_rama_birth(vig_x, vig_y, vig_w, vig_h, time),
        CutsceneType::SitaBirth => draw_vignette_sita_birth(vig_x, vig_y, vig_w, vig_h, time),
        CutsceneType::SitaKalyanam => draw_vignette_sita_kalyanam(vig_x, vig_y, vig_w, vig_h, time),
        CutsceneType::ParashuramaEncounter => draw_vignette_parashurama(vig_x, vig_y, vig_w, vig_h, slide_idx, time),
        CutsceneType::AyodhyaExile => draw_vignette_ayodhya_exile(vig_x, vig_y, vig_w, vig_h, slide_idx, time),
    }

    draw_rectangle_lines(vig_x, vig_y, vig_w, vig_h, 2.0, PAL_GOLD_1);
    // Ornamental corners
    let c = 6.0;
    draw_rectangle(vig_x, vig_y, c, 2.0, PAL_GOLD_0);
    draw_rectangle(vig_x, vig_y, 2.0, c, PAL_GOLD_0);
    draw_rectangle(vig_x + vig_w - c, vig_y, c, 2.0, PAL_GOLD_0);
    draw_rectangle(vig_x + vig_w - 2.0, vig_y, 2.0, c, PAL_GOLD_0);
    draw_rectangle(vig_x, vig_y + vig_h - 2.0, c, 2.0, PAL_GOLD_0);
    draw_rectangle(vig_x, vig_y + vig_h - c, 2.0, c, PAL_GOLD_0);
    draw_rectangle(vig_x + vig_w - c, vig_y + vig_h - 2.0, c, 2.0, PAL_GOLD_0);
    draw_rectangle(vig_x + vig_w - 2.0, vig_y + vig_h - c, 2.0, c, PAL_GOLD_0);

    // Narrative Box below (580 x 100)
    let nar_x = 30.0;
    let nar_y = 236.0;
    let nar_w = SCREEN_W - 60.0;
    let nar_h = 100.0;

    draw_rectangle(nar_x, nar_y, nar_w, nar_h, Color::new(0.09, 0.08, 0.14, 0.95));
    draw_rectangle_lines(nar_x, nar_y, nar_w, nar_h, 1.2, Color::new(0.4, 0.35, 0.45, 0.9));

    let (text_lines, total_slides) = get_cutscene_lines(cutscene, slide_idx);

    let max_text_w = nar_w - 32.0;
    let full_narrative = text_lines.join(" ");
    let wrapped = wrap_pixel_text(font, &full_narrative, 7, max_text_w);

    let mut ty = nar_y + 16.0;
    for (i, line) in wrapped.iter().enumerate().take(5) {
        let col = if i == wrapped.len() - 1 && i > 1 {
            PAL_TURMERIC_1
        } else {
            WHITE
        };
        draw_pixel_text(font, line, nar_x + 16.0, ty, 7, col);
        ty += 12.0;
    }

    // Re-Read / Prev Button if slide_idx > 0
    if slide_idx > 0 {
        let hovered = is_cutscene_prev_button_hovered(virtual_mouse);
        let btn_x = nar_x + 16.0;
        let btn_y = nar_y + nar_h - 22.0;
        let btn_w = 95.0;
        let btn_h = 16.0;
        let bg = if hovered {
            Color::new(0.28, 0.20, 0.12, 1.0)
        } else {
            Color::new(0.15, 0.12, 0.18, 0.9)
        };
        let border = if hovered { PAL_GOLD_0 } else { PAL_GOLD_2 };
        draw_rectangle(btn_x, btn_y, btn_w, btn_h, bg);
        draw_rectangle_lines(btn_x, btn_y, btn_w, btn_h, 1.0, border);
        draw_pixel_text(font, "◀ PREV [LEFT]", btn_x + 6.0, btn_y + 11.0, 7, border);
    }

    // Bottom Navigation Indicator
    let slide_tag = format!("PAGE {} / {}", slide_idx + 1, total_slides);
    let tag_x = if slide_idx > 0 { nar_x + 122.0 } else { nar_x + 16.0 };
    draw_pixel_text(font, &slide_tag, tag_x, nar_y + nar_h - 8.0, 7, PAL_GOLD_2);

    let prompt = if slide_idx + 1 < total_slides {
        "PRESS [SPACE / CLICK] TO CONTINUE ▶"
    } else {
        match cutscene {
            CutsceneType::RamaBirth => "PROCEED TO SITA'S BIRTH FLASHBACK ▶",
            CutsceneType::SitaBirth => "PROCEED TO THE ROYAL WEDDING ▶",
            CutsceneType::SitaKalyanam => "BEGIN THE JOURNEY BACK TO AYODHYA ▶",
            CutsceneType::ParashuramaEncounter => "COMPLETE BALA KANDA ★",
            CutsceneType::AyodhyaExile => "BEGIN CHAPTER 5: CHITRAKUTA EXILE ▶",
        }
    };
    let blink = (time * 4.0).sin() > 0.0;
    let prompt_col = if blink { PAL_GOLD_0 } else { PAL_GOLD_1 };
    draw_pixel_text(font, prompt, nar_x + nar_w - 260.0, nar_y + nar_h - 8.0, 7, prompt_col);

    draw_reticle(virtual_mouse, false);
}

pub fn get_cutscene_total_slides(cutscene: CutsceneType) -> usize {
    match cutscene {
        CutsceneType::RamaBirth => 4,
        CutsceneType::SitaBirth => 4,
        CutsceneType::SitaKalyanam => 3,
        CutsceneType::ParashuramaEncounter => 4,
        CutsceneType::AyodhyaExile => 3,
    }
}

fn get_cutscene_lines(cutscene: CutsceneType, slide: usize) -> (Vec<&'static str>, usize) {
    match cutscene {
        CutsceneType::AyodhyaExile => {
            let total = 3;
            let lines = match slide {
                0 => vec![
                    "In the Chamber of Wrath, Queen Kaikeyi, poisoned by Manthara's deceit,",
                    "demands two fatal boons: Prince Bharata shall be consecrated King of Ayodhya,",
                    "and Prince Rama must be exiled to the perils of Dandaka forest for 14 years!",
                    "King Dasharatha collapses to the floor in agony, weeping piteously for Rama.",
                ],
                1 => vec![
                    "Rama enters with serene composure: 'The decree of my father is my highest Dharma.'",
                    "Princess Sita and Prince Lakshmana adamantly refuse to stay behind in the palace.",
                    "Casting off royal silks and jewels, the three don rough ascetic bark robes (Valkala),",
                    "binding their hair in forest ascetics' jata, ready to step into the wilderness.",
                ],
                _ => vec![
                    "Sumantra drives the royal chariot past weeping citizens who follow them in anguish.",
                    "Sparing them grief, Rama slips across the Tamasa river while the citizens sleep.",
                    "At Sringaverapura on the sacred Ganga, tribal King Guha welcomes Rama with deep love.",
                    "Bidding farewell to Sumantra, Rama, Sita, and Lakshmana cross the Ganga toward Chitrakuta.",
                ],
            };
            (lines, total)
        }
        CutsceneType::RamaBirth => {
            let total = 4;
            let lines = match slide {
                0 => vec![
                    "Before the wedding rites begin in Mithila, both royal families recite",
                    "their ancient lineages as was customary in Vedic tradition.",
                    "Sage Vasishtha proudly recounts the lineage of the solar dynasty.",
                ],
                1 => vec![
                    "In the great city of Ayodhya, King Dasharatha ruled with justice and grace.",
                    "Yet despite all his glory, his heart remained heavy with sorrow,",
                    "for he had no sons to continue the sacred line of Ikshvaku.",
                ],
                2 => vec![
                    "Under Sage Rishyashringa, the king held the Putrakameshti sacrifice.",
                    "From the blazing flames, a radiant divine being stepped forth,",
                    "holding a golden bowl of sweet celestial nectar (Payasam).",
                ],
                _ => vec![
                    "Dasharatha offered the holy nectar to his three queens.",
                    "In time, Queen Kausalya gave birth to the divine Prince Rama,",
                    "while Kaikeyi gave birth to Bharata, and Sumitra to Lakshmana & Shatrughna.",
                ],
            };
            (lines, total)
        }
        CutsceneType::SitaBirth => {
            let total = 4;
            let lines = match slide {
                0 => vec![
                    "King Janaka now speaks, reciting the wondrous arrival of his daughter,",
                    "Princess Sita, whose birth was not of mortal womb, but of Mother Earth.",
                    "The court listens in quiet reverence.",
                ],
                1 => vec![
                    "Years ago, a great drought troubled the kingdom of Mithila.",
                    "To purify the sacred grounds for a yajna, King Janaka took hold",
                    "of a golden plow and began furrowing the sacred earth himself.",
                ],
                2 => vec![
                    "Suddenly, the tip of the golden plow struck an ornate chest.",
                    "Buried beneath the fertile soil, the casket was encrusted with jewels.",
                    "When Janaka lifted the lid, he found a glowing baby girl, smiling like the sun.",
                ],
                _ => vec![
                    "Because she had emerged from the furrow of the plow, he named her 'Sita'.",
                    "King Janaka raised her as his most beloved princess,",
                    "vowing she would wed only the warrior who could string Shiva's divine Pinaka bow.",
                ],
            };
            (lines, total)
        }
        CutsceneType::SitaKalyanam => {
            let total = 3;
            let lines = match slide {
                0 => vec![
                    "With their divine lineages honored, the wedding ceremony commences.",
                    "The royal houses of Ayodhya and Mithila gather in the grand pavilion,",
                    "adorned with flower garlands, Vedic lamps, and auspicious songs.",
                ],
                1 => vec![
                    "Rama and Sita take their sacred vows, walking seven steps around Agni.",
                    "King Janaka places Sita's hand into Rama's: 'This is Sita, my daughter;'",
                    "'she shall walk beside you in dharma as your devoted companion.'",
                ],
                _ => vec![
                    "The prince and princess exchange fragrant garlands of lotus and jasmine.",
                    "Celestial drums thunder from the skies as the Devas rain sweet blossoms,",
                    "blessing the sacred union of Rama and Sita.",
                ],
            };
            (lines, total)
        }
        CutsceneType::ParashuramaEncounter => {
            let total = 4;
            let lines = match slide {
                0 => vec![
                    "As the joyful wedding procession departs Mithila toward Ayodhya,",
                    "dark storm clouds suddenly blot out the sun, and the earth shudders.",
                    "The furious warrior-sage Parashurama appears, blocking their path!",
                ],
                1 => vec![
                    "Enraged by the snap of his guru Shiva's bow, Parashurama roars:",
                    "'Who dared break Pinaka? If you possess true divine blood, Prince Rama,'",
                    "'lift and string this bow of Lord Vishnu—the great Sharanga!'",
                ],
                2 => vec![
                    "While King Dasharatha trembles in fear for his son, Rama steps forward.",
                    "With gentle calm and immense grace, Rama takes the Vaishnava bow.",
                    "With one effortless motion, he notches the bowstring and draws it taut!",
                ],
                _ => vec![
                    "Blinding golden light fills the sky. Parashurama bows in reverent humility,",
                    "recognizing Rama as the supreme protector of dharma.",
                    "With blessings bestowed, the sage departs, and Bala Kanda is triumphantly complete!",
                ],
            };
            (lines, total)
        }
    }
}

// ============================================================================
// 16-BIT VIGNETTES
// ============================================================================
fn draw_vignette_rama_birth(x: f32, y: f32, w: f32, h: f32, time: f32) {
    // Palace Courtyard & Night Sky in Ayodhya
    draw_rectangle(x, y, w, h * 0.45, Color::new(0.08, 0.08, 0.16, 1.0));
    // Punarvasu Nakshatra stars twinkling
    for i in 0..12 {
        let sx = x + 30.0 + i as f32 * 45.0;
        let sy = y + 15.0 + (i as f32 * 17.0) % 45.0;
        let blink = ((time * 3.0 + i as f32).sin() * 0.3 + 0.7).clamp(0.0, 1.0);
        draw_circle(sx, sy, 1.5, Color::new(1.0, 1.0, 1.0, blink));
    }
    // Ayodhya Palace Domes in background
    draw_circle(x + 120.0, y + 60.0, 35.0, Color::new(0.20, 0.15, 0.28, 1.0));
    draw_circle(x + 440.0, y + 60.0, 35.0, Color::new(0.20, 0.15, 0.28, 1.0));

    // Marble Terrace Floor
    draw_rectangle(x, y + h * 0.45, w, h * 0.55, Color::new(0.22, 0.18, 0.24, 1.0));
    draw_line(x, y + h * 0.45, x + w, y + h * 0.45, 1.5, PAL_GOLD_3);

    // Sacred Yajna Fire Altar in center (x + 280.0, y + 140.0)
    let fire_x = x + 280.0;
    let fire_y = y + 140.0;
    draw_rectangle(fire_x - 30.0, fire_y, 60.0, 16.0, PAL_SAND_4);
    draw_rectangle(fire_x - 22.0, fire_y - 8.0, 44.0, 10.0, PAL_SAND_2);

    // Radiant Yajna Fire
    let f_h = 24.0 + (time * 12.0).sin() * 4.0;
    draw_triangle(
        vec2(fire_x - 18.0, fire_y - 6.0),
        vec2(fire_x + 18.0, fire_y - 6.0),
        vec2(fire_x, fire_y - 6.0 - f_h),
        PAL_FIRE_3,
    );
    draw_triangle(
        vec2(fire_x - 10.0, fire_y - 6.0),
        vec2(fire_x + 10.0, fire_y - 6.0),
        vec2(fire_x, fire_y - 6.0 - f_h * 0.75),
        PAL_FIRE_1,
    );
    draw_circle(fire_x, fire_y - 12.0, 6.0, PAL_FIRE_0);

    // Divine Yajnapurusha Messenger emerging from flames holding Golden Payasam Vessel
    let m_y = fire_y - f_h - 12.0;
    draw_circle(fire_x, m_y - 8.0, 8.0, PAL_GOLD_0);
    draw_triangle(
        vec2(fire_x - 12.0, m_y),
        vec2(fire_x + 12.0, m_y),
        vec2(fire_x, m_y + 16.0),
        PAL_GOLD_1,
    );
    // Golden bowl of Payasam
    draw_circle(fire_x, m_y - 14.0, 5.0, PAL_GOLD_1);
    draw_circle(fire_x, m_y - 15.0, 3.0, WHITE);

    // King Dasharatha (Left) bowing in reverence
    let d_x = x + 180.0;
    let d_y = y + 130.0;
    draw_circle(d_x, d_y - 12.0, 6.5, Color::new(0.92, 0.78, 0.62, 1.0));
    draw_triangle(vec2(d_x - 5.0, d_y - 17.0), vec2(d_x + 5.0, d_y - 17.0), vec2(d_x, d_y - 25.0), PAL_GOLD_0);
    draw_rectangle(d_x - 7.0, d_y - 4.0, 14.0, 18.0, PAL_TURMERIC_1);
    // Folded hands (Anjali Mudra)
    draw_circle(d_x + 7.0, d_y, 2.5, PAL_GOLD_1);

    // Sage Rishyashringa (Right) with sacrificial ladle
    let r_x = x + 380.0;
    let r_y = y + 130.0;
    draw_circle(r_x, r_y - 12.0, 6.0, Color::new(0.85, 0.70, 0.55, 1.0));
    draw_circle(r_x, r_y - 16.0, 4.0, Color::new(0.2, 0.15, 0.1, 1.0)); // Topknot
    draw_rectangle(r_x - 6.0, r_y - 4.0, 12.0, 18.0, PAL_RED_1); // Saffron robe
    draw_line(r_x - 5.0, r_y, fire_x + 15.0, fire_y - 4.0, 1.5, PAL_SAND_3); // Sacrificial ladle

    // Infant Rama Halo in upper right
    let b_x = x + 490.0;
    let b_y = y + 60.0;
    let glow = (time * 4.0).sin() * 2.0;
    draw_circle(b_x, b_y, 20.0 + glow, Color::new(1.0, 0.85, 0.3, 0.35));
    draw_circle(b_x, b_y, 14.0, PAL_RAMA_1); // Shyamavarna baby
    draw_circle(b_x, b_y - 4.0, 4.0, Color::new(0.1, 0.1, 0.15, 1.0)); // Hair
    draw_circle(b_x, b_y + 1.0, 1.0, PAL_RED_0); // Tilaka
}

fn draw_vignette_sita_birth(x: f32, y: f32, w: f32, h: f32, time: f32) {
    // Golden Dawn Sky & Mithila Farmland
    draw_rectangle(x, y, w, h * 0.40, Color::new(0.95, 0.70, 0.45, 1.0)); // Dawn orange
    draw_circle(x + 100.0, y + 45.0, 26.0, PAL_FIRE_1); // Morning sun

    // Rich Brown Soil & Furrow Field
    draw_rectangle(x, y + h * 0.40, w, h * 0.60, Color::new(0.35, 0.22, 0.15, 1.0));
    // Ploughed furrow lines
    for i in 0..6 {
        let fy = y + h * 0.45 + i as f32 * 16.0;
        draw_line(x, fy, x + w, fy, 2.0, Color::new(0.25, 0.15, 0.10, 1.0));
    }

    // King Janaka with Golden Plow (Suvarna Hala)
    let j_x = x + 210.0;
    let j_y = y + 115.0;
    // Janaka
    draw_circle(j_x, j_y - 14.0, 7.0, Color::new(0.92, 0.78, 0.62, 1.0));
    draw_rectangle(j_x - 3.0, j_y - 10.0, 6.0, 5.0, WHITE); // White beard
    draw_triangle(vec2(j_x - 5.0, j_y - 19.0), vec2(j_x + 5.0, j_y - 19.0), vec2(j_x, j_y - 27.0), PAL_GOLD_0);
    draw_rectangle(j_x - 8.0, j_y - 5.0, 16.0, 22.0, PAL_TURMERIC_1); // Yellow dhoti

    // Golden Plow held in hands
    draw_line(j_x + 4.0, j_y + 2.0, j_x + 35.0, j_y + 35.0, 3.5, PAL_GOLD_1);
    draw_triangle(
        vec2(j_x + 35.0, j_y + 35.0),
        vec2(j_x + 50.0, j_y + 38.0),
        vec2(j_x + 38.0, j_y + 44.0),
        PAL_GOLD_0,
    );

    // Deep Furrow Opening in Earth (Center Right)
    let f_x = x + 350.0;
    let f_y = y + 130.0;
    draw_ellipse(f_x, f_y + 10.0, 48.0, 16.0, 0.0, Color::new(0.18, 0.10, 0.06, 1.0));

    // Golden Casket in Furrow
    draw_rectangle(f_x - 26.0, f_y - 4.0, 52.0, 20.0, PAL_GOLD_2);
    draw_rectangle_lines(f_x - 26.0, f_y - 4.0, 52.0, 20.0, 1.5, PAL_GOLD_0);
    // Open Lid leaning back
    draw_line(f_x - 26.0, f_y - 4.0, f_x - 20.0, f_y - 18.0, 3.0, PAL_GOLD_1);
    draw_line(f_x + 26.0, f_y - 4.0, f_x + 32.0, f_y - 18.0, 3.0, PAL_GOLD_1);
    draw_line(f_x - 20.0, f_y - 18.0, f_x + 32.0, f_y - 18.0, 3.0, PAL_GOLD_0);

    // Radiant Infant Baby Sita inside the casket
    let glow = (time * 5.0).sin() * 2.5;
    draw_circle(f_x, f_y + 4.0, 14.0 + glow, Color::new(1.0, 0.9, 0.4, 0.4));
    draw_circle(f_x, f_y + 2.0, 6.0, Color::new(0.98, 0.86, 0.72, 1.0)); // Face
    draw_circle(f_x, f_y - 2.0, 4.0, Color::new(0.1, 0.08, 0.12, 1.0)); // Hair
    draw_circle(f_x, f_y - 3.0, 1.2, PAL_GOLD_0); // Tilaka
    draw_rectangle(f_x - 5.0, f_y + 6.0, 10.0, 7.0, PAL_RED_1); // Red silk wrapping
}

fn draw_vignette_sita_kalyanam(x: f32, y: f32, w: f32, h: f32, time: f32) {
    // Royal Wedding Mandapa Pavilion
    draw_rectangle(x, y, w, h * 0.35, Color::new(0.20, 0.08, 0.14, 1.0));
    // Saffron and Floral Draperies
    for i in 0..9 {
        let dx = x + 20.0 + i as f32 * 60.0;
        draw_triangle(
            vec2(dx, y + 20.0),
            vec2(dx + 35.0, y + 20.0),
            vec2(dx + 17.5, y + 45.0),
            PAL_RED_1,
        );
        draw_circle(dx + 17.5, y + 45.0, 2.5, PAL_GOLD_0);
    }

    // Marble Platform
    draw_rectangle(x, y + h * 0.35, w, h * 0.65, Color::new(0.90, 0.85, 0.80, 1.0));
    draw_line(x, y + h * 0.35, x + w, y + h * 0.35, 2.0, PAL_GOLD_2);

    // Sacred Marriage Agni Fire Kunda in Center
    let agni_x = x + 280.0;
    let agni_y = y + 135.0;
    draw_rectangle(agni_x - 22.0, agni_y, 44.0, 14.0, PAL_SAND_4);
    draw_rectangle_lines(agni_x - 22.0, agni_y, 44.0, 14.0, 1.0, PAL_GOLD_1);
    let f_h = 16.0 + (time * 10.0).sin() * 3.0;
    draw_triangle(vec2(agni_x - 12.0, agni_y), vec2(agni_x + 12.0, agni_y), vec2(agni_x, agni_y - f_h), PAL_FIRE_3);
    draw_circle(agni_x, agni_y - 6.0, 4.0, PAL_FIRE_0);

    // Prince Rama (Left of Agni) in Wedding Attire
    let r_x = agni_x - 55.0;
    let r_y = agni_y - 5.0;
    draw_circle(r_x, r_y - 14.0, 6.5, PAL_RAMA_1);
    draw_triangle(vec2(r_x - 5.0, r_y - 19.0), vec2(r_x + 5.0, r_y - 19.0), vec2(r_x, r_y - 27.0), PAL_GOLD_0);
    draw_rectangle(r_x - 8.0, r_y - 6.0, 16.0, 22.0, PAL_TURMERIC_1); // Golden silk
    draw_circle(r_x, r_y - 1.0, 1.5, PAL_RED_0);

    // Princess Sita (Right of Agni) in Bridal Silk
    let s_x = agni_x + 55.0;
    let s_y = agni_y - 5.0;
    draw_circle(s_x, s_y - 14.0, 6.0, Color::new(0.96, 0.82, 0.68, 1.0));
    draw_circle(s_x, s_y - 18.0, 4.0, Color::new(0.1, 0.08, 0.12, 1.0));
    draw_circle(s_x, s_y - 19.0, 1.5, PAL_GOLD_0);
    draw_rectangle(s_x - 8.0, s_y - 6.0, 16.0, 22.0, PAL_RED_1); // Bridal crimson silk
    draw_rectangle_lines(s_x - 8.0, s_y - 6.0, 16.0, 22.0, 1.0, PAL_GOLD_0);

    // Garland Exchange (Jaimala)
    draw_line(r_x + 6.0, r_y, s_x - 6.0, s_y, 2.5, PAL_GOLD_1);
    draw_circle(agni_x, agni_y - 18.0, 3.0, Color::new(1.0, 0.7, 0.8, 1.0));

    // King Dasharatha & King Janaka standing behind blessing
    let jd_x = x + 110.0;
    draw_circle(jd_x, y + 85.0, 6.0, Color::new(0.92, 0.78, 0.62, 1.0));
    draw_rectangle(jd_x - 6.0, y + 92.0, 12.0, 18.0, PAL_TURMERIC_2);
    let jj_x = x + 450.0;
    draw_circle(jj_x, y + 85.0, 6.0, Color::new(0.92, 0.78, 0.62, 1.0));
    draw_rectangle(jj_x - 6.0, y + 92.0, 12.0, 18.0, PAL_SAND_2);

    // Falling Flower Shower (Pushpa-Vrishti)
    for i in 0..25 {
        let px = x + 40.0 + (i as f32 * 21.0) % (w - 80.0);
        let py = y + 15.0 + ((i as f32 * 37.0 + time * 35.0) % (h - 30.0));
        let col = if i % 2 == 0 { Color::new(1.0, 0.75, 0.85, 0.9) } else { PAL_GOLD_0 };
        draw_circle(px, py, 2.0, col);
    }
}

fn draw_vignette_parashurama(x: f32, y: f32, w: f32, h: f32, slide: usize, time: f32) {
    // Stormy Thunder Skies & Forest Mountain Road
    draw_rectangle(x, y, w, h * 0.50, Color::new(0.12, 0.10, 0.18, 1.0));
    // Jagged Lightning bolt flashing
    if (time * 8.0) as i32 % 7 == 0 {
        draw_line(x + 180.0, y + 10.0, x + 210.0, y + 45.0, 2.0, Color::new(0.9, 0.9, 1.0, 0.95));
        draw_line(x + 210.0, y + 45.0, x + 200.0, y + 65.0, 2.0, Color::new(0.9, 0.9, 1.0, 0.95));
        draw_line(x + 200.0, y + 65.0, x + 230.0, y + 90.0, 2.5, WHITE);
    }

    // Rocky Mountain Path
    draw_rectangle(x, y + h * 0.50, w, h * 0.50, Color::new(0.28, 0.24, 0.22, 1.0));
    draw_line(x, y + h * 0.50, x + w, y + h * 0.50, 1.5, PAL_SAND_4);

    // Sage Parashurama (Left) - Fierce Warrior Sage with Battle Axe
    let p_x = x + 160.0;
    let p_y = y + 125.0;
    draw_circle(p_x, p_y - 16.0, 7.5, Color::new(0.85, 0.70, 0.55, 1.0));
    draw_circle(p_x, p_y - 23.0, 6.0, Color::new(0.18, 0.12, 0.10, 1.0)); // Wild Jata dreadlocks
    draw_line(p_x - 3.0, p_y - 18.0, p_x + 3.0, p_y - 18.0, 1.2, WHITE); // Bhasma tripundra
    draw_rectangle(p_x - 9.0, p_y - 8.0, 18.0, 24.0, Color::new(0.55, 0.35, 0.18, 1.0)); // Tiger skin

    // Parashu (Battle Axe on shoulder)
    draw_line(p_x - 12.0, p_y + 12.0, p_x - 22.0, p_y - 22.0, 3.0, Color::new(0.4, 0.3, 0.2, 1.0));
    draw_triangle(
        vec2(p_x - 22.0, p_y - 22.0),
        vec2(p_x - 34.0, p_y - 30.0),
        vec2(p_x - 28.0, p_y - 14.0),
        Color::new(0.75, 0.78, 0.82, 1.0),
    );

    // Prince Rama (Right)
    let r_x = x + 380.0;
    let r_y = y + 125.0;
    draw_circle(r_x, r_y - 16.0, 7.0, PAL_RAMA_1);
    draw_triangle(vec2(r_x - 5.0, r_y - 21.0), vec2(r_x + 5.0, r_y - 21.0), vec2(r_x, r_y - 29.0), PAL_GOLD_0);
    draw_rectangle(r_x - 8.0, r_y - 8.0, 16.0, 24.0, PAL_TURMERIC_1);

    // The Vaishnava Bow (Sharanga)
    let bow_x = if slide >= 2 { r_x - 8.0 } else { p_x + 18.0 };
    let bow_y = y + 115.0;

    // Glowing Vaishnava Bow (Double curved solar gold)
    let bow_glow = (time * 6.0).sin() * 0.2 + 0.8;
    draw_ellipse(bow_x, bow_y, 14.0, 34.0, 0.0, Color::new(1.0, 0.85, 0.2, 0.3 * bow_glow));

    draw_line(bow_x, bow_y - 28.0, bow_x + 6.0, bow_y, 4.0, PAL_GOLD_3);
    draw_line(bow_x, bow_y - 28.0, bow_x + 6.0, bow_y, 2.5, PAL_GOLD_0);
    draw_line(bow_x + 6.0, bow_y, bow_x, bow_y + 28.0, 4.0, PAL_GOLD_3);
    draw_line(bow_x + 6.0, bow_y, bow_x, bow_y + 28.0, 2.5, PAL_GOLD_0);

    // Taut glowing string when Rama strings it
    if slide >= 2 {
        draw_line(bow_x, bow_y - 28.0, bow_x - 4.0, bow_y, 2.0, WHITE);
        draw_line(bow_x - 4.0, bow_y, bow_x, bow_y + 28.0, 2.0, WHITE);
        draw_circle(bow_x - 4.0, bow_y, 3.0, PAL_FIRE_0); // Solar arrowhead
    } else {
        draw_line(bow_x, bow_y - 28.0, bow_x, bow_y + 28.0, 1.2, Color::new(0.9, 0.9, 1.0, 0.5));
    }
}

fn draw_vignette_ayodhya_exile(x: f32, y: f32, w: f32, h: f32, slide: usize, time: f32) {
    match slide {
        0 => {
            // SLIDE 0: The Chamber of Wrath (Krodhagara)
            // Dark palace chamber with marble pillars
            draw_rectangle(x, y, w, h, Color::new(0.08, 0.05, 0.09, 1.0));

            // Palace Pillars
            for i in 0..6 {
                let px = x + 35.0 + i as f32 * 95.0;
                draw_rectangle(px, y + 10.0, 14.0, h - 20.0, Color::new(0.20, 0.16, 0.22, 1.0));
                draw_line(px, y + 10.0, px + 14.0, y + 10.0, 2.0, PAL_GOLD_3);
            }

            // Flickering torches
            for i in 0..2 {
                let tx = x + 100.0 + i as f32 * 360.0;
                let ty = y + 45.0;
                draw_rectangle(tx - 3.0, ty, 6.0, 25.0, Color::new(0.3, 0.2, 0.1, 1.0));
                let tb = (time * 12.0 + i as f32 * 3.0).sin() * 2.5;
                draw_circle(tx, ty - 6.0, 6.0 + tb * 0.5, Color::new(1.0, 0.5, 0.1, 0.4));
                draw_circle(tx, ty - 6.0, 3.5, PAL_FIRE_0);
            }

            // Queen Kaikeyi (Left, unadorned, stern, dark robes)
            let k_x = x + 170.0;
            let k_y = y + 130.0;
            draw_circle(k_x, k_y - 24.0, 7.0, Color::new(0.95, 0.80, 0.68, 1.0));
            draw_circle(k_x, k_y - 29.0, 5.0, Color::new(0.12, 0.08, 0.10, 1.0));
            // Stern purple dress, no ornaments
            draw_triangle(vec2(k_x - 10.0, k_y), vec2(k_x + 10.0, k_y), vec2(k_x, k_y - 22.0), Color::new(0.35, 0.12, 0.25, 1.0));
            draw_rectangle(k_x - 6.0, k_y - 20.0, 12.0, 14.0, Color::new(0.28, 0.10, 0.20, 1.0));

            // Discarded ornaments scattered on the marble floor
            for i in 0..8 {
                let ox = k_x + 15.0 + (i as f32 * 14.0);
                let oy = k_y - 2.0 + ((i as f32 * 7.0) % 6.0);
                let col = if i % 2 == 0 { PAL_GOLD_0 } else { PAL_RED_1 };
                draw_circle(ox, oy, 2.0, col);
            }

            // King Dasharatha prostrate on the carpet in grief
            let d_x = x + 310.0;
            let d_y = y + 140.0;
            // Ornate Persian rug
            draw_rectangle(d_x - 45.0, d_y - 6.0, 90.0, 18.0, Color::new(0.42, 0.12, 0.16, 0.95));
            draw_rectangle_lines(d_x - 45.0, d_y - 6.0, 90.0, 18.0, 1.0, PAL_GOLD_1);
            // Prostrate Dasharatha
            draw_rectangle(d_x - 25.0, d_y - 8.0, 48.0, 12.0, PAL_GOLD_2);
            draw_circle(d_x + 24.0, d_y - 2.0, 6.0, Color::new(0.90, 0.75, 0.60, 1.0));
            draw_triangle(vec2(d_x + 20.0, d_y - 1.0), vec2(d_x + 28.0, d_y - 1.0), vec2(d_x + 24.0, d_y + 6.0), WHITE); // White beard

            // Manthara's crooked silhouette in the doorway
            let m_x = x + 480.0;
            let m_y = y + 125.0;
            draw_rectangle(m_x - 16.0, y + 25.0, 32.0, 110.0, Color::new(0.04, 0.03, 0.05, 1.0)); // Archway
            draw_rectangle_lines(m_x - 16.0, y + 25.0, 32.0, 110.0, 1.0, Color::new(0.2, 0.15, 0.1, 0.8));
            // Hunchback silhouette
            draw_ellipse(m_x - 3.0, m_y - 12.0, 8.0, 12.0, 0.4, Color::new(0.08, 0.08, 0.10, 1.0));
            draw_circle(m_x + 2.0, m_y - 22.0, 5.0, Color::new(0.08, 0.08, 0.10, 1.0));
            draw_line(m_x + 6.0, m_y - 15.0, m_x + 10.0, m_y + 8.0, 1.5, Color::new(0.3, 0.2, 0.15, 1.0)); // Staff
        }
        1 => {
            // SLIDE 1: Donning Valkala Robes (Rama, Sita, Lakshmana)
            // Palace courtyard to forest threshold
            draw_rectangle(x, y, w * 0.4, h, Color::new(0.18, 0.16, 0.24, 1.0));
            draw_rectangle(x + w * 0.4, y, w * 0.6, h, Color::new(0.25, 0.45, 0.28, 1.0));
            draw_line(x + w * 0.4, y, x + w * 0.4, y + h, 2.0, PAL_GOLD_2);

            // Discarded golden royal crowns on marble steps
            let step_x = x + 100.0;
            let step_y = y + 140.0;
            draw_rectangle(step_x - 30.0, step_y - 4.0, 60.0, 10.0, Color::new(0.85, 0.82, 0.78, 1.0));
            draw_triangle(vec2(step_x - 12.0, step_y - 4.0), vec2(step_x, step_y - 4.0), vec2(step_x - 6.0, step_y - 16.0), PAL_GOLD_0);
            draw_circle(step_x - 6.0, step_y - 16.0, 2.0, PAL_RED_1);

            // Prince Rama in Forest Valkala (Center)
            let r_x = x + 300.0;
            let r_y = y + 130.0;
            // Golden halo (Tejas)
            draw_circle(r_x, r_y - 24.0, 14.0, Color::new(1.0, 0.85, 0.3, 0.25));
            draw_circle(r_x, r_y - 25.0, 7.0, PAL_RAMA_1);
            draw_circle(r_x, r_y - 32.0, 4.5, Color::new(0.12, 0.10, 0.15, 1.0)); // Ascetic jata
            draw_circle(r_x, r_y - 32.0, 2.0, PAL_GOLD_0); // Kusha grass knot
            // Valkala bark robe
            draw_rectangle(r_x - 8.0, r_y - 18.0, 16.0, 24.0, Color::new(0.88, 0.50, 0.18, 1.0));
            draw_line(r_x - 7.0, r_y - 18.0, r_x + 7.0, r_y, 2.0, PAL_GOLD_0);
            // Kodanda bow on shoulder
            draw_line(r_x - 11.0, r_y - 32.0, r_x - 11.0, r_y + 10.0, 2.5, PAL_GOLD_1);

            // Princess Sita in Gentle Valkala (Left of Rama)
            let s_x = x + 235.0;
            let s_y = y + 132.0;
            draw_circle(s_x, s_y - 22.0, 6.0, Color::new(0.98, 0.88, 0.75, 1.0));
            draw_circle(s_x, s_y - 27.0, 4.0, Color::new(0.18, 0.12, 0.10, 1.0));
            draw_circle(s_x - 3.0, s_y - 26.0, 2.0, Color::new(1.0, 0.75, 0.85, 1.0)); // Lotus in hair
            // Pale tree-bark sari
            draw_triangle(vec2(s_x - 8.0, s_y + 6.0), vec2(s_x + 8.0, s_y + 6.0), vec2(s_x, s_y - 18.0), Color::new(0.92, 0.82, 0.65, 1.0));
            // Lotus in hands
            draw_circle(s_x + 4.0, s_y - 8.0, 3.5, Color::new(1.0, 0.75, 0.85, 1.0));

            // Prince Lakshmana in Valkala with quiver (Right of Rama)
            let l_x = x + 370.0;
            let l_y = y + 130.0;
            draw_circle(l_x, l_y - 24.0, 6.5, Color::new(0.96, 0.78, 0.58, 1.0));
            draw_circle(l_x, l_y - 31.0, 4.0, Color::new(0.2, 0.15, 0.1, 1.0));
            // Valkala loincloth & cord
            draw_rectangle(l_x - 8.0, l_y - 18.0, 16.0, 24.0, Color::new(0.85, 0.55, 0.20, 1.0));
            // Bow in hand
            draw_line(l_x + 10.0, l_y - 30.0, l_x + 10.0, l_y + 8.0, 2.0, PAL_GOLD_2);
            // Quiver
            draw_rectangle(l_x - 11.0, l_y - 22.0, 4.0, 16.0, Color::new(0.4, 0.25, 0.15, 1.0));
        }
        _ => {
            // SLIDE 2: Sacred Ganga & Tribal King Guha at Sringaverapura
            // Dawn sky over holy river Ganga
            draw_rectangle(x, y, w, h * 0.45, Color::new(0.55, 0.72, 0.85, 1.0));
            // Sun rising on horizon
            draw_circle(x + w * 0.75, y + h * 0.45, 22.0, Color::new(1.0, 0.85, 0.4, 0.7));

            // Flowing sacred waters of Ganga
            let river_y = y + h * 0.45;
            draw_rectangle(x, river_y, w, h * 0.55, Color::new(0.22, 0.52, 0.70, 1.0));
            for i in 0..6 {
                let wy = river_y + 8.0 + i as f32 * 12.0;
                let offset = (time * 2.0 + i as f32 * 1.5).sin() * 6.0;
                draw_line(x, wy, x + w, wy, 1.2, Color::new(0.4, 0.7, 0.9, 0.7));
                draw_line(x + 100.0 + offset, wy - 1.0, x + 150.0 + offset, wy - 1.0, 1.2, WHITE);
                draw_line(x + 320.0 + offset, wy - 1.0, x + 380.0 + offset, wy - 1.0, 1.2, WHITE);
            }

            // Near shoreline
            draw_triangle(vec2(x, y + h), vec2(x + 220.0, y + h), vec2(x, y + h * 0.55), Color::new(0.35, 0.60, 0.30, 1.0));

            // Nishada King Guha bowing low with folded hands before Rama
            let g_x = x + 120.0;
            let g_y = y + 138.0;
            draw_rectangle(g_x - 6.0, g_y - 12.0, 12.0, 14.0, Color::new(0.65, 0.35, 0.15, 1.0));
            draw_circle(g_x, g_y - 18.0, 6.0, Color::new(0.70, 0.52, 0.38, 1.0)); // Dark tribal hero complexion
            draw_circle(g_x - 3.0, g_y - 14.0, 2.5, Color::new(0.70, 0.52, 0.38, 1.0)); // Folded hands

            // Rama with blessing hand
            let r_x = x + 165.0;
            let r_y = y + 128.0;
            draw_circle(r_x, r_y - 20.0, 6.5, PAL_RAMA_1);
            draw_rectangle(r_x - 6.0, r_y - 14.0, 12.0, 18.0, Color::new(0.88, 0.50, 0.18, 1.0));
            // Blessing hand raised
            draw_circle(r_x - 5.0, r_y - 12.0, 2.5, PAL_RAMA_1);

            // Wooden ferry boat moored in reeds
            let b_x = x + 280.0;
            let b_y = y + 120.0;
            draw_rectangle(b_x - 30.0, b_y, 60.0, 12.0, Color::new(0.45, 0.28, 0.16, 1.0));
            draw_line(b_x - 30.0, b_y, b_x - 38.0, b_y - 8.0, 2.5, Color::new(0.45, 0.28, 0.16, 1.0));
            draw_line(b_x + 30.0, b_y, b_x + 38.0, b_y - 8.0, 2.5, Color::new(0.45, 0.28, 0.16, 1.0));
            // Oar in boat
            draw_line(b_x - 10.0, b_y - 12.0, b_x + 15.0, b_y + 14.0, 2.0, Color::new(0.3, 0.18, 0.1, 1.0));

            // Royal chariot of Sumantra on distant Ayodhya shore (far left)
            let c_x = x + 35.0;
            let c_y = y + 80.0;
            draw_rectangle(c_x - 12.0, c_y - 8.0, 24.0, 10.0, PAL_GOLD_2);
            draw_circle(c_x - 8.0, c_y + 4.0, 4.0, PAL_GOLD_1);
            draw_circle(c_x + 8.0, c_y + 4.0, 4.0, PAL_GOLD_1);
            // Sumantra with bowed head
            draw_circle(c_x - 14.0, c_y - 4.0, 3.5, Color::new(0.85, 0.72, 0.60, 1.0));
        }
    }
}
