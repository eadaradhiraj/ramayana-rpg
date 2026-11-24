#![allow(dead_code)]
use macroquad::prelude::*;
use crate::types::{Astra, AstraProjectileType, PlayerStats};

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

// Retro RPG Dialogue Box for 16:9
pub fn draw_dialogue_box(font: &Font, speaker: &str, line: &str, time: f32) {
    let box_x = 30.0;
    let box_y = SCREEN_H - 110.0;
    let box_w = SCREEN_W - 60.0;
    let box_h = 92.0;

    draw_rectangle(box_x, box_y, box_w, box_h, Color::new(0.08, 0.09, 0.12, 0.96));
    draw_rectangle_lines(box_x, box_y, box_w, box_h, 2.5, COLOR_RAMA_GOLD);
    draw_rectangle_lines(box_x + 2.0, box_y + 2.0, box_w - 4.0, box_h - 4.0, 1.0, Color::new(0.6, 0.5, 0.2, 0.8));

    // Speaker Nameplate
    let plate_w = 175.0;
    let plate_h = 20.0;
    draw_rectangle(box_x + 12.0, box_y - 10.0, plate_w, plate_h, Color::new(0.18, 0.12, 0.08, 1.0));
    draw_rectangle_lines(box_x + 12.0, box_y - 10.0, plate_w, plate_h, 1.5, COLOR_RAMA_GOLD);
    draw_pixel_text(font, speaker, box_x + 18.0, box_y + 4.0, 9, COLOR_RAMA_GOLD);

    // Word wrapped pixel dialogue text
    let words: Vec<&str> = line.split_whitespace().collect();
    let mut current_line = String::new();
    let mut line_y = box_y + 28.0;

    for word in words {
        let test_line = if current_line.is_empty() {
            word.to_string()
        } else {
            format!("{} {}", current_line, word)
        };
        // Max ~52 chars per line for 9px font in 560px width
        if test_line.len() > 52 {
            draw_pixel_text(font, &current_line, box_x + 18.0, line_y, 9, WHITE);
            current_line = word.to_string();
            line_y += 18.0;
        } else {
            current_line = test_line;
        }
    }
    if !current_line.is_empty() {
        draw_pixel_text(font, &current_line, box_x + 18.0, line_y, 9, WHITE);
    }

    let blink = (time * 5.0).sin() > 0.0;
    if blink {
        let prompt_text = "[CLICK / SPACE ▶]";
        draw_pixel_text(font, prompt_text, box_x + box_w - 145.0, box_y + box_h - 10.0, 8, COLOR_RAMA_GOLD);
    }
}

// Retro Title Screen for 16:9
// ============================================================================
// 16-BIT RETRO TITLE SCREEN
// ============================================================================
pub fn draw_title_screen(font: &Font, time: f32) {
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
    draw_pixel_text(font, title, 70.0, 58.0, 20, PAL_GOLD_1);

    let sub = "BALA KANDA — THE ADVENTURES OF PRINCE RAMA";
    draw_pixel_text(font, sub, 115.0, 84.0, 10, PAL_TURMERIC_1);

    // Decorative Bow & Arrow Divider
    draw_line(150.0, 102.0, 490.0, 102.0, 2.0, PAL_GOLD_2);
    draw_circle(320.0, 102.0, 5.0, PAL_RED_1);
    draw_circle(320.0, 102.0, 2.5, PAL_GOLD_0);

    // Chapter Select / Preview Box
    let box_x = 45.0;
    let box_y = 118.0;
    let box_w = 550.0;
    let box_h = 175.0;
    draw_rectangle(box_x, box_y, box_w, box_h, Color::new(0.10, 0.09, 0.15, 0.95));
    draw_rectangle_lines(box_x, box_y, box_w, box_h, 1.5, PAL_GOLD_3);

    // Level 1 Section
    draw_pixel_text(font, "[1] LEVEL 1: TATAKA'S FOREST (SHABDAVEDHI)", box_x + 16.0, box_y + 22.0, 9, PAL_GOLD_1);
    draw_pixel_text(font, "    Track sound ripples & strike unseen Yakshini before dusk (90s)", box_x + 16.0, box_y + 40.0, 8, Color::new(0.4, 0.85, 1.0, 1.0));

    // Level 2 Section
    draw_pixel_text(font, "[2] LEVEL 2: SIDDHASHRAMA (DIVINE ASTRAS)", box_x + 16.0, box_y + 68.0, 9, Color::new(1.0, 0.65, 0.25, 1.0));
    draw_pixel_text(font, "    Defend Yajna Altar against Maricha, Subahu & Defilement hordes", box_x + 16.0, box_y + 86.0, 8, Color::new(1.0, 0.85, 0.40, 1.0));
    draw_pixel_text(font, "    Astras: [1] Kodanda  [2] Vayavya  [3] Manava  [4] Agneya", box_x + 16.0, box_y + 104.0, 8, Color::new(0.65, 0.95, 0.75, 1.0));

    // Level Select Button Highlight
    draw_rectangle(box_x + 16.0, box_y + 124.0, box_w - 32.0, 24.0, Color::new(0.18, 0.14, 0.26, 1.0));
    draw_rectangle_lines(box_x + 16.0, box_y + 124.0, box_w - 32.0, 24.0, 1.0, PAL_GOLD_2);
    draw_pixel_text(font, "▶ PRESS [L] OR [TAB] TO OPEN INTERACTIVE LEVEL SELECT ◀", box_x + 36.0, box_y + 140.0, 8, PAL_GOLD_0);

    // Controls line
    draw_pixel_text(font, "Controls: WASD Move | Mouse Aim & Shoot | 1-4 Astras | ESC Pause", box_x + 16.0, box_y + 164.0, 8, WHITE);

    let blink = (time * 4.0).sin() > 0.0;
    if blink {
        let prompt = "▶ PRESS [1] OR [2] TO PLAY, [L] FOR LEVEL SELECT, OR [SPACE] ◀";
        draw_pixel_text(font, prompt, 38.0, 316.0, 10, PAL_GOLD_1);
    }

    draw_pixel_text(font, "16:9 Widescreen | 16-Bit Vedic Retro RPG | Rust + WASM", 100.0, 342.0, 8, Color::new(0.65, 0.65, 0.65, 1.0));
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
    let title = "BALA KANDA — SELECT BATTLE CHAPTER";
    draw_pixel_text(font, title, 100.0, 32.0, 13, PAL_GOLD_1);

    // Decorative bow divider line
    draw_line(80.0, 44.0, 560.0, 44.0, 2.0, PAL_GOLD_2);
    draw_circle(320.0, 44.0, 4.0, PAL_RED_1);
    draw_circle(320.0, 44.0, 2.0, PAL_GOLD_0);

    // 3 Level Cards
    let card_w = 188.0;
    let card_h = 244.0;
    let card_y = 56.0;

    let cards = [
        (
            0,
            20.0,
            "LEVEL 1",
            "TATAKA'S FOREST",
            "Shabdavedhi Archery",
            PAL_GOLD_1,
            Color::new(0.40, 0.85, 1.0, 1.0),
            vec![
                "• Dark cursed forest",
                "• Track sound ripples",
                "• Pin invisible demon",
                "• 90s Dusk Countdown",
            ],
            "[1] Kodanda Bow",
            "▶ READY ◀",
            Color::new(0.35, 0.85, 0.45, 1.0),
        ),
        (
            1,
            226.0,
            "LEVEL 2",
            "SIDDHASHRAMA",
            "Sacred Yajna Defense",
            Color::new(1.0, 0.65, 0.25, 1.0),
            Color::new(1.0, 0.85, 0.40, 1.0),
            vec![
                "• Defend holy Altar",
                "• Maricha & Subahu",
                "• Asura horde swarms",
                "• 4 Divine Astras",
            ],
            "All 4 Astras",
            "▶ READY ◀",
            Color::new(0.35, 0.85, 0.45, 1.0),
        ),
        (
            2,
            432.0,
            "LEVEL 3",
            "MITHILA PREVIEW",
            "Shiva Dhanush Swayamvar",
            Color::new(0.85, 0.65, 1.0, 1.0),
            Color::new(0.75, 0.55, 0.95, 1.0),
            vec![
                "• King Janaka's Court",
                "• Cosmic Pinaka Bow",
                "• Princess Sita Swayamvar",
                "• Chapter in Dev",
            ],
            "Pinaka Bow",
            "[ PREVIEW ]",
            PAL_GOLD_1,
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
        draw_rectangle(cx + 8.0, card_y + 8.0, 72.0, 16.0, Color::new(0.18, 0.15, 0.25, 1.0));
        draw_rectangle_lines(cx + 8.0, card_y + 8.0, 72.0, 16.0, 1.0, border_col);
        draw_pixel_text(font, badge, cx + 13.0, card_y + 20.0, 8, PAL_GOLD_1);

        // Chapter Name & Subtitle
        draw_pixel_text(font, ch_name, cx + 8.0, card_y + 42.0, 8, title_col);
        draw_pixel_text(font, sub, cx + 8.0, card_y + 55.0, 7, sub_col);

        // Mini Diorama / Visual Box
        let box_y = card_y + 64.0;
        let box_h = 58.0;
        draw_rectangle(cx + 8.0, box_y, card_w - 16.0, box_h, Color::new(0.06, 0.05, 0.09, 1.0));
        draw_rectangle_lines(cx + 8.0, box_y, card_w - 16.0, box_h, 1.0, Color::new(0.25, 0.22, 0.30, 1.0));

        // Draw thematic mini diorama inside
        match idx {
            0 => {
                // Tataka forest mini-silhouette
                draw_rectangle(cx + 8.0, box_y + box_h - 16.0, card_w - 16.0, 16.0, PAL_GRASS_4);
                draw_circle(cx + 36.0, box_y + 26.0, 14.0, PAL_GRASS_3);
                draw_circle(cx + 148.0, box_y + 28.0, 16.0, PAL_GRASS_3);
                // Sound ripple
                let rip_r = (time * 18.0) % 24.0;
                draw_circle_lines(cx + 94.0, box_y + 28.0, rip_r, 1.0, Color::new(0.4, 0.9, 1.0, 0.7));
                draw_circle(cx + 94.0, box_y + 28.0, 3.0, PAL_RED_0);
            }
            1 => {
                // Siddhashrama Yajna altar mini
                draw_rectangle(cx + 8.0, box_y + box_h - 16.0, card_w - 16.0, 16.0, Color::new(0.25, 0.40, 0.20, 1.0));
                // Stepped altar
                draw_rectangle(cx + 70.0, box_y + 30.0, 48.0, 20.0, PAL_SAND_4);
                draw_rectangle(cx + 80.0, box_y + 22.0, 28.0, 12.0, PAL_SAND_2);
                // Mini Agni flame
                let f_h = 10.0 + (time * 8.0).sin() * 3.0;
                draw_triangle(
                    vec2(cx + 86.0, box_y + 22.0),
                    vec2(cx + 102.0, box_y + 22.0),
                    vec2(cx + 94.0, box_y + 22.0 - f_h),
                    PAL_FIRE_2,
                );
                draw_circle(cx + 94.0, box_y + 20.0, 2.5, PAL_FIRE_0);
            }
            _ => {
                // Mithila Pinaka Bow mini
                draw_rectangle(cx + 8.0, box_y + box_h - 14.0, card_w - 16.0, 14.0, PAL_SAND_3);
                // Wheeled carriage
                draw_rectangle(cx + 60.0, box_y + 36.0, 68.0, 8.0, Color::new(0.3, 0.28, 0.32, 1.0));
                draw_circle(cx + 72.0, box_y + 44.0, 4.0, PAL_GOLD_2);
                draw_circle(cx + 116.0, box_y + 44.0, 4.0, PAL_GOLD_2);
                // Golden curved Pinaka bow on carriage
                draw_line(cx + 62.0, box_y + 34.0, cx + 94.0, box_y + 16.0, 2.5, PAL_GOLD_1);
                draw_line(cx + 94.0, box_y + 16.0, cx + 126.0, box_y + 34.0, 2.5, PAL_GOLD_1);
                draw_circle(cx + 94.0, box_y + 16.0, 3.0, PAL_GOLD_0);
            }
        }

        // Bullets
        let mut by = box_y + box_h + 12.0;
        for line in &bullets {
            draw_pixel_text(font, line, cx + 8.0, by, 7, Color::new(0.85, 0.85, 0.88, 1.0));
            by += 12.0;
        }

        // Astras line
        draw_pixel_text(font, "ASTRAS:", cx + 8.0, card_y + 196.0, 7, PAL_GOLD_2);
        draw_pixel_text(font, astras, cx + 58.0, card_y + 196.0, 7, PAL_GOLD_0);

        // Status Launch Button
        let btn_y = card_y + 212.0;
        let btn_h = 24.0;
        let btn_bg = if active {
            Color::new(0.22, 0.18, 0.32, 1.0)
        } else {
            Color::new(0.12, 0.10, 0.16, 1.0)
        };
        draw_rectangle(cx + 8.0, btn_y, card_w - 16.0, btn_h, btn_bg);
        draw_rectangle_lines(cx + 8.0, btn_y, card_w - 16.0, btn_h, 1.0, border_col);

        let pulse_text = if active {
            let blink = (time * 5.0).sin() > 0.0;
            if blink { status_col } else { WHITE }
        } else {
            status_col
        };
        draw_pixel_text(font, status, cx + 38.0, btn_y + 16.0, 8, pulse_text);
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
        "[1] Tataka Forest   [2] Siddhashrama   [3] Mithila   [ESC] Title",
        70.0,
        348.0,
        7,
        Color::new(0.7, 0.7, 0.7, 1.0),
    );
}

pub fn get_level_select_card_at(mouse: Vec2) -> Option<usize> {
    let card_y = 56.0;
    let card_h = 244.0;
    let card_w = 188.0;

    if mouse.y >= card_y && mouse.y <= card_y + card_h {
        if mouse.x >= 20.0 && mouse.x <= 20.0 + card_w {
            return Some(0);
        }
        if mouse.x >= 226.0 && mouse.x <= 226.0 + card_w {
            return Some(1);
        }
        if mouse.x >= 432.0 && mouse.x <= 432.0 + card_w {
            return Some(2);
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
