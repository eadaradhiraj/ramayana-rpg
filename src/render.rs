use macroquad::prelude::*;
use crate::types::PlayerStats;

pub const SCREEN_W: f32 = 640.0;
pub const SCREEN_H: f32 = 360.0;

// Arena bounds for 16:9 widescreen (640x360)
pub const ARENA_MIN_X: f32 = 44.0;
pub const ARENA_MAX_X: f32 = 596.0;
pub const ARENA_MIN_Y: f32 = 38.0;
pub const ARENA_MAX_Y: f32 = 324.0;

// Palette constants inspired by Indian miniature painting and 16-bit retro RPGs
pub const COLOR_RAMA_SKIN: Color = Color::new(0.28, 0.48, 0.72, 1.0); // Shyamavarna
pub const COLOR_RAMA_DHOTI: Color = Color::new(0.95, 0.76, 0.15, 1.0); // Pitambara
pub const COLOR_RAMA_GOLD: Color = Color::new(1.0, 0.85, 0.25, 1.0);
pub const COLOR_TATAKA_SKIN: Color = Color::new(0.18, 0.12, 0.16, 1.0);
pub const COLOR_TATAKA_EYES: Color = Color::new(1.0, 0.15, 0.15, 1.0);

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

// Draw Prince Rama with animated Kodanda bow
pub fn draw_rama(pos: Vec2, aim_dir: Vec2, invuln_timer: f32, bow_cooldown: f32, _time: f32) {
    if invuln_timer > 0.0 && ((invuln_timer * 18.0) as i32 % 2 == 0) {
        return;
    }

    let x = pos.x;
    let y = pos.y;

    // Drop shadow
    draw_ellipse(x, y + 12.0, 11.0, 4.5, 0.0, Color::new(0.0, 0.0, 0.0, 0.4));

    // Feet
    draw_rectangle(x - 6.0, y + 8.0, 4.0, 4.0, Color::new(0.4, 0.25, 0.1, 1.0));
    draw_rectangle(x + 2.0, y + 8.0, 4.0, 4.0, Color::new(0.4, 0.25, 0.1, 1.0));

    // Dhoti (Pitambara)
    draw_rectangle(x - 7.0, y + 1.0, 14.0, 9.0, COLOR_RAMA_DHOTI);
    draw_rectangle(x - 7.0, y + 8.0, 14.0, 2.0, Color::new(0.8, 0.15, 0.15, 1.0));
    draw_line(x, y + 1.0, x, y + 9.0, 1.5, Color::new(0.85, 0.65, 0.1, 1.0));

    // Torso (Shyamavarna)
    draw_rectangle(x - 6.0, y - 9.0, 12.0, 11.0, COLOR_RAMA_SKIN);
    draw_line(x - 4.0, y - 9.0, x + 5.0, y + 1.0, 1.5, COLOR_RAMA_GOLD); // Yajnopavita
    draw_circle(x, y - 6.0, 2.0, COLOR_RAMA_GOLD);

    // Head
    draw_circle(x, y - 14.0, 6.0, COLOR_RAMA_SKIN);
    draw_circle(x - 3.5, y - 17.0, 3.0, Color::new(0.1, 0.1, 0.12, 1.0));
    draw_circle(x + 3.5, y - 17.0, 3.0, Color::new(0.1, 0.1, 0.12, 1.0));
    draw_circle(x, y - 19.0, 3.5, Color::new(0.1, 0.1, 0.12, 1.0));

    // Golden Crown & Tilaka
    draw_triangle(
        vec2(x - 4.5, y - 18.0),
        vec2(x + 4.5, y - 18.0),
        vec2(x, y - 25.0),
        COLOR_RAMA_GOLD,
    );
    draw_circle(x, y - 21.0, 1.5, Color::new(0.9, 0.1, 0.1, 1.0));
    draw_line(x, y - 15.0, x, y - 13.0, 1.2, Color::new(0.9, 0.1, 0.1, 1.0));

    // Quiver on Rama's back
    let qx = x - 7.0;
    let qy = y - 10.0;
    draw_line(qx - 2.0, qy - 5.0, qx + 2.0, qy + 6.0, 3.5, Color::new(0.5, 0.28, 0.12, 1.0));
    draw_line(qx - 3.0, qy - 6.0, qx - 5.0, qy - 10.0, 1.5, WHITE);
    draw_line(qx, qy - 6.0, qx - 1.0, qy - 11.0, 1.5, WHITE);

    // Kodanda Bow
    let bow_center = pos + vec2(aim_dir.x * 9.0, aim_dir.y * 9.0 - 3.0);
    let bow_len = 14.0;
    let perp = vec2(-aim_dir.y, aim_dir.x);
    let bow_top = bow_center + perp * bow_len;
    let bow_bot = bow_center - perp * bow_len;
    let bow_grip = bow_center + aim_dir * 3.5;

    draw_line(bow_bot.x, bow_bot.y, bow_grip.x, bow_grip.y, 2.5, COLOR_RAMA_GOLD);
    draw_line(bow_grip.x, bow_grip.y, bow_top.x, bow_top.y, 2.5, COLOR_RAMA_GOLD);

    let string_pull = if bow_cooldown > 0.0 { -aim_dir * 4.0 } else { Vec2::ZERO };
    let nock_point = bow_center + string_pull;
    draw_line(bow_bot.x, bow_bot.y, nock_point.x, nock_point.y, 1.0, Color::new(0.9, 0.9, 0.95, 0.8));
    draw_line(bow_top.x, bow_top.y, nock_point.x, nock_point.y, 1.0, Color::new(0.9, 0.9, 0.95, 0.8));

    if bow_cooldown <= 0.0 {
        let arrow_tip = nock_point + aim_dir * 16.0;
        draw_line(nock_point.x, nock_point.y, arrow_tip.x, arrow_tip.y, 2.0, COLOR_RAMA_GOLD);
        draw_circle(arrow_tip.x, arrow_tip.y, 1.8, WHITE);
    }
}

// Draw Tataka Boss
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
            let alpha = (shimmer_timer / 0.35).clamp(0.0, 1.0) * 0.75;
            let aura_col = if is_frenzy {
                Color::new(1.0, 0.1, 0.1, alpha)
            } else {
                Color::new(0.7, 0.15, 0.45, alpha)
            };
            draw_circle(x, y - 6.0, 24.0, aura_col);
            draw_circle(x, y - 22.0, 14.0, aura_col);
            draw_circle(x - 5.0, y - 22.0, 3.0, Color::new(1.0, 0.1, 0.1, alpha));
            draw_circle(x + 5.0, y - 22.0, 3.0, Color::new(1.0, 0.1, 0.1, alpha));
        }
        return;
    }

    let main_col = if hit_flash > 0.0 {
        WHITE
    } else if is_frenzy {
        Color::new(0.35, 0.08, 0.12, 1.0)
    } else {
        COLOR_TATAKA_SKIN
    };

    let pulse = (time * 6.0).sin() * 4.0;
    let aura_col = if is_frenzy {
        Color::new(0.9, 0.05, 0.1, 0.35)
    } else {
        Color::new(0.6, 0.1, 0.4, 0.25)
    };
    draw_circle(x, y - 8.0, 30.0 + pulse, aura_col);
    draw_ellipse(x, y + 15.0, 18.0, 6.0, 0.0, Color::new(0.0, 0.0, 0.0, 0.5));

    // Talons / Feet
    draw_rectangle(x - 13.0, y + 7.0, 8.0, 8.0, main_col);
    draw_rectangle(x + 5.0, y + 7.0, 8.0, 8.0, main_col);
    draw_triangle(vec2(x - 13.0, y + 15.0), vec2(x - 5.0, y + 15.0), vec2(x - 9.0, y + 19.0), Color::new(0.1, 0.1, 0.1, 1.0));
    draw_triangle(vec2(x + 5.0, y + 15.0), vec2(x + 13.0, y + 15.0), vec2(x + 9.0, y + 19.0), Color::new(0.1, 0.1, 0.1, 1.0));

    // Torso
    draw_rectangle(x - 15.0, y - 13.0, 30.0, 22.0, main_col);

    // Bone necklace
    for i in -2..=2 {
        let bx = x + i as f32 * 5.5;
        let by = y - 9.0 + (i as f32).abs() * 1.8;
        draw_circle(bx, by, 2.2, Color::new(0.9, 0.88, 0.8, 1.0));
    }

    // Arms & Claws
    let arm_swing = (time * 8.0).sin() * 4.5;
    draw_rectangle(x - 22.0, y - 11.0 + arm_swing, 8.0, 18.0, main_col);
    draw_triangle(vec2(x - 22.0, y + 7.0 + arm_swing), vec2(x - 14.0, y + 7.0 + arm_swing), vec2(x - 20.0, y + 14.0 + arm_swing), Color::new(0.15, 0.15, 0.15, 1.0));
    draw_rectangle(x + 14.0, y - 11.0 - arm_swing, 8.0, 18.0, main_col);
    draw_triangle(vec2(x + 14.0, y + 7.0 - arm_swing), vec2(x + 22.0, y + 7.0 - arm_swing), vec2(x + 20.0, y + 14.0 - arm_swing), Color::new(0.15, 0.15, 0.15, 1.0));

    // Head
    draw_circle(x, y - 22.0, 13.0, main_col);

    // Wild Hair
    for i in -3..=3 {
        let hx = x + i as f32 * 4.5;
        let hy = y - 29.0 - (time * 6.0 + i as f32).sin() * 2.5;
        draw_circle(hx, hy, 4.5, Color::new(0.08, 0.05, 0.08, 1.0));
    }

    // Eyes
    draw_circle(x - 4.5, y - 23.0, 3.0, COLOR_TATAKA_EYES);
    draw_circle(x + 4.5, y - 23.0, 3.0, COLOR_TATAKA_EYES);
    draw_circle(x - 4.5, y - 23.0, 1.0, Color::new(1.0, 0.9, 0.4, 1.0));
    draw_circle(x + 4.5, y - 23.0, 1.0, Color::new(1.0, 0.9, 0.4, 1.0));

    // Fangs
    draw_line(x - 6.0, y - 17.0, x - 4.5, y - 13.0, 2.2, WHITE);
    draw_line(x + 6.0, y - 17.0, x + 4.5, y - 13.0, 2.2, WHITE);
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
pub fn draw_title_screen(font: &Font, time: f32) {
    clear_background(Color::new(0.07, 0.06, 0.10, 1.0));

    // Background silhouettes
    let sway = (time * 0.8).sin() * 4.0;
    for i in 0..10 {
        let x = i as f32 * 70.0 + sway;
        draw_circle(x, 190.0, 42.0, Color::new(0.12, 0.15, 0.11, 1.0));
    }

    // Epic Title in Press Start 2P font
    let title = "RAMAYANA: THE RETRO EPIC";
    draw_pixel_text(font, title, 70.0, 75.0, 20, COLOR_RAMA_GOLD);

    let sub = "BALA KANDA — LEVEL 1: TATAKA'S FOREST";
    draw_pixel_text(font, sub, 130.0, 105.0, 10, Color::new(0.9, 0.7, 0.3, 1.0));

    // Decorative Bow & Arrow
    draw_line(180.0, 130.0, 460.0, 130.0, 2.0, COLOR_RAMA_GOLD);
    draw_circle(320.0, 130.0, 5.0, Color::new(0.9, 0.2, 0.2, 1.0));

    // Lore preview box
    let box_x = 70.0;
    let box_y = 150.0;
    let box_w = 500.0;
    let box_h = 135.0;
    draw_rectangle(box_x, box_y, box_w, box_h, Color::new(0.12, 0.13, 0.17, 0.92));
    draw_rectangle_lines(box_x, box_y, box_w, box_h, 1.5, Color::new(0.6, 0.5, 0.25, 0.85));

    draw_pixel_text(font, "MISSION: DEFEAT THE YAKSHINI TATAKA", box_x + 16.0, box_y + 24.0, 9, COLOR_RAMA_GOLD);
    draw_pixel_text(font, "- Phase 1: Track unseen sound ripples with Shabdavedhi", box_x + 16.0, box_y + 48.0, 8, Color::new(0.4, 0.85, 1.0, 1.0));
    draw_pixel_text(font, "- Phase 2: Dodge her final frenzy charge & strike her down", box_x + 16.0, box_y + 70.0, 8, Color::new(1.0, 0.6, 0.6, 1.0));
    draw_pixel_text(font, "- Defeat her before the 90s Dusk Countdown hits zero!", box_x + 16.0, box_y + 92.0, 8, Color::new(1.0, 0.85, 0.3, 1.0));
    draw_pixel_text(font, "- Controls: WASD to Move | Mouse to Aim & Shoot", box_x + 16.0, box_y + 114.0, 8, WHITE);

    let blink = (time * 4.0).sin() > 0.0;
    if blink {
        let prompt = "▶ PRESS SPACE OR CLICK TO BEGIN ◀";
        draw_pixel_text(font, prompt, 135.0, 318.0, 12, COLOR_RAMA_GOLD);
    }

    draw_pixel_text(font, "16:9 Widescreen | Keyboard & Mouse Archery", 150.0, 345.0, 8, Color::new(0.6, 0.6, 0.6, 1.0));
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
