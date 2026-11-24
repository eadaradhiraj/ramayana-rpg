use macroquad::prelude::*;
use crate::audio::SoundEffects;
use crate::render::*;
use crate::types::PlayerStats;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Level3Phase {
    Approach,
    Lifting,
    Stringing,
    Dhanurbhanga,
    Complete,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum FightResultL3 {
    Victory,
}

struct Petal {
    pos: Vec2,
    vel: Vec2,
    color: Color,
    rot: f32,
    rot_speed: f32,
    size: f32,
}

pub struct Level3Fight {
    pub phase: Level3Phase,
    pub rama_pos: Vec2,
    pub aim_dir: Vec2,
    pub lift_progress: f32,
    pub balance_pos: f32,
    pub balance_speed: f32,
    pub string_progress: f32,
    pub bow_break_timer: f32,
    pub shake_timer: f32,
    pub shake_intensity: f32,
    pub court_dialogue: Option<(&'static str, f32)>,
    petals: Vec<Petal>,
}

impl Level3Fight {
    pub fn new() -> Self {
        Self {
            phase: Level3Phase::Approach,
            rama_pos: vec2(120.0, 220.0),
            aim_dir: vec2(1.0, 0.0),
            lift_progress: 0.0,
            balance_pos: 0.0,
            balance_speed: 2.2,
            string_progress: 0.0,
            bow_break_timer: 0.0,
            shake_timer: 0.0,
            shake_intensity: 0.0,
            court_dialogue: None,
            petals: Vec::new(),
        }
    }

    pub fn update(
        &mut self,
        dt: f32,
        virtual_mouse: Vec2,
        _stats: &mut PlayerStats,
        sounds: &SoundEffects,
    ) -> Option<FightResultL3> {
        // Update screen shake
        if self.shake_timer > 0.0 {
            self.shake_timer -= dt;
            if self.shake_timer <= 0.0 {
                self.shake_intensity = 0.0;
            }
        }

        // Update court reaction dialogue fade
        if let Some((_, ref mut timer)) = self.court_dialogue {
            *timer -= dt;
            if *timer <= 0.0 {
                self.court_dialogue = None;
            }
        }

        // Update falling petals
        for p in &mut self.petals {
            p.pos += p.vel * dt;
            p.rot += p.rot_speed * dt;
            if p.pos.y > SCREEN_H + 10.0 {
                p.pos.y = -10.0;
                p.pos.x = ((p.pos.x * 1.3) % (SCREEN_W - 20.0)) + 10.0;
            }
        }

        // Aim direction
        let to_mouse = (virtual_mouse - self.rama_pos).normalize_or_zero();
        if to_mouse.length_squared() > 0.01 {
            self.aim_dir = to_mouse;
        }

        match self.phase {
            Level3Phase::Approach => {
                let mut move_dir = Vec2::ZERO;
                if is_key_down(KeyCode::W) || is_key_down(KeyCode::Up) {
                    move_dir.y -= 1.0;
                }
                if is_key_down(KeyCode::S) || is_key_down(KeyCode::Down) {
                    move_dir.y += 1.0;
                }
                if is_key_down(KeyCode::A) || is_key_down(KeyCode::Left) {
                    move_dir.x -= 1.0;
                }
                if is_key_down(KeyCode::D) || is_key_down(KeyCode::Right) {
                    move_dir.x += 1.0;
                }

                if move_dir != Vec2::ZERO {
                    self.rama_pos += move_dir.normalize() * 110.0 * dt;
                    self.rama_pos.x = self.rama_pos.x.clamp(60.0, 560.0);
                    self.rama_pos.y = self.rama_pos.y.clamp(170.0, 270.0);
                }

                // Check distance to Pinaka carriage (centered at x=320, y=210)
                let carriage_pos = vec2(320.0, 210.0);
                if self.rama_pos.distance(carriage_pos) < 55.0 {
                    if is_key_pressed(KeyCode::Space)
                        || is_key_pressed(KeyCode::Enter)
                        || is_mouse_button_pressed(MouseButton::Left)
                    {
                        sounds.play(&sounds.bell, 0.7);
                        self.phase = Level3Phase::Lifting;
                        self.court_dialogue = Some(("King Janaka: Behold, Prince Rama approaches Pinaka!", 3.0));
                    }
                }
            }

            Level3Phase::Lifting => {
                // Keep Rama near carriage
                self.rama_pos = vec2(295.0, 210.0);
                self.aim_dir = vec2(1.0, 0.0);

                // Balance meter oscillation
                self.balance_pos += self.balance_speed * dt;
                let balance_indicator = self.balance_pos.sin(); // -1.0 to 1.0

                // Player holds Space or Left Mouse inside the green balance sweetspot
                let is_holding = is_key_down(KeyCode::Space) || is_mouse_button_down(MouseButton::Left);
                let in_sweetspot = balance_indicator.abs() < 0.45;

                if is_holding && in_sweetspot {
                    self.lift_progress += dt * 0.55;
                    if (self.lift_progress * 10.0) as i32 % 3 == 0 {
                        self.shake_timer = 0.05;
                        self.shake_intensity = 1.5;
                    }
                } else if !is_holding {
                    self.lift_progress = (self.lift_progress - dt * 0.25).max(0.0);
                }

                if self.lift_progress >= 1.0 {
                    self.lift_progress = 1.0;
                    self.phase = Level3Phase::Stringing;
                    sounds.play(&sounds.bell, 0.8);
                    sounds.play(&sounds.victory, 0.6);
                    self.court_dialogue = Some(("The Court: By the gods! He lifted Shiva's bow with one hand!", 3.5));
                }
            }

            Level3Phase::Stringing => {
                self.rama_pos = vec2(300.0, 210.0);

                // String tension mechanic: hold & pull
                let is_pulling = is_key_down(KeyCode::Space)
                    || is_mouse_button_down(MouseButton::Left)
                    || is_key_down(KeyCode::Enter);

                if is_pulling {
                    self.string_progress += dt * 0.45;
                    self.shake_timer = 0.08;
                    self.shake_intensity = 2.0 + self.string_progress * 5.0;
                } else {
                    self.string_progress = (self.string_progress - dt * 0.35).max(0.0);
                }

                if self.string_progress >= 1.0 {
                    self.string_progress = 1.0;
                    self.phase = Level3Phase::Dhanurbhanga;
                    self.bow_break_timer = 3.5;

                    // Play cosmic snap and rolling thunder
                    sounds.play(&sounds.snap, 1.0);
                    sounds.play(&sounds.thunder, 0.95);
                    self.shake_timer = 1.5;
                    self.shake_intensity = 16.0;

                    // Spawn celebratory flower petals
                    let mut rng: u32 = 42195;
                    for _ in 0..70 {
                        rng = rng.wrapping_mul(1664525).wrapping_add(1013904223);
                        let px = (rng % SCREEN_W as u32) as f32;
                        rng = rng.wrapping_mul(1664525).wrapping_add(1013904223);
                        let py = (rng % 180) as f32 - 100.0;
                        rng = rng.wrapping_mul(1664525).wrapping_add(1013904223);
                        let vx = (rng % 60) as f32 - 30.0;
                        rng = rng.wrapping_mul(1664525).wrapping_add(1013904223);
                        let vy = 40.0 + (rng % 40) as f32;

                        let color = match rng % 3 {
                            0 => Color::new(1.0, 0.75, 0.85, 0.95), // Pink lotus petal
                            1 => PAL_GOLD_0,                        // Golden flower
                            _ => Color::new(1.0, 0.95, 0.98, 0.9),  // White jasmine
                        };

                        self.petals.push(Petal {
                            pos: vec2(px, py),
                            vel: vec2(vx, vy),
                            color,
                            rot: (rng % 360) as f32,
                            rot_speed: (rng % 4) as f32 - 2.0,
                            size: 3.0 + (rng % 3) as f32,
                        });
                    }

                    self.court_dialogue = Some(("DHANURBHANGA! The cosmic bow has snapped in two!", 4.0));
                }
            }

            Level3Phase::Dhanurbhanga => {
                self.bow_break_timer -= dt;
                if self.bow_break_timer <= 0.0 {
                    self.phase = Level3Phase::Complete;
                    return Some(FightResultL3::Victory);
                }
            }

            Level3Phase::Complete => {
                return Some(FightResultL3::Victory);
            }
        }

        None
    }

    pub fn draw(&self, font: &Font, virtual_mouse: Vec2, time: f32) {
        // Apply camera shake if active
        let shake_offset = if self.shake_timer > 0.0 {
            vec2(
                (time * 65.0).sin() * self.shake_intensity,
                (time * 77.0).cos() * self.shake_intensity,
            )
        } else {
            Vec2::ZERO
        };

        // 1. Draw Royal Mithila Palace Hall
        draw_mithila_hall(time, shake_offset);

        // 2. Draw Assembled Courtiers, King Janaka, Princess Sita
        draw_mithila_royalty(time, shake_offset, font);

        // 3. Draw Ashtachakra Ratha (8-Wheeled Cart) & Pinaka
        let cart_c = vec2(320.0, 210.0) + shake_offset;
        draw_ashtachakra_cart(cart_c, self.phase, self.string_progress, self.bow_break_timer, time);

        // 4. Draw Prince Rama
        draw_rama(
            self.rama_pos + shake_offset,
            self.aim_dir,
            0.0,
            if self.phase == Level3Phase::Stringing { 0.5 } else { 0.0 },
            time,
        );

        // 5. Draw Falling Petals (Pushpa-Vrishti)
        for p in &self.petals {
            draw_ellipse(p.pos.x, p.pos.y, p.size, p.size * 0.55, p.rot, p.color);
        }

        // 6. Draw Shockwaves during Dhanurbhanga
        if self.phase == Level3Phase::Dhanurbhanga {
            let snap_progress = 1.0 - (self.bow_break_timer / 3.5).clamp(0.0, 1.0);
            let ring1 = snap_progress * 380.0;
            let ring2 = (snap_progress * 460.0 - 50.0).max(0.0);
            let alpha = (1.0 - snap_progress).clamp(0.0, 1.0);

            draw_circle_lines(cart_c.x, cart_c.y - 12.0, ring1, 3.0, Color::new(1.0, 0.9, 0.4, alpha));
            draw_circle_lines(cart_c.x, cart_c.y - 12.0, ring2, 2.0, Color::new(1.0, 0.3, 0.1, alpha * 0.8));

            // Dramatic Flash
            if self.bow_break_timer > 3.2 {
                draw_rectangle(0.0, 0.0, SCREEN_W, SCREEN_H, Color::new(1.0, 1.0, 1.0, 0.65));
            }
        }

        // 7. Draw HUD & Phase Minigame Prompts
        draw_level3_hud(self, font, time);

        // 8. Custom targeting reticle
        draw_reticle(virtual_mouse, false);
    }
}

// ============================================================================
// MITHILA PALACE COURT RENDERING
// ============================================================================
fn draw_mithila_hall(time: f32, shake: Vec2) {
    // Marble floor with lotus checker paver stones
    let tile_w = 40.0;
    let tile_h = 24.0;
    let cols = (SCREEN_W / tile_w) as usize + 1;
    let rows = (SCREEN_H / tile_h) as usize + 1;

    for y in 0..rows {
        for x in 0..cols {
            let px = x as f32 * tile_w + shake.x;
            let py = y as f32 * tile_h + shake.y;

            if py < 120.0 {
                // Palace Upper Walls (Terracotta Sandstone & Gold Filigree)
                let wall_col = Color::new(0.24, 0.12, 0.14, 1.0);
                draw_rectangle(px, py, tile_w, tile_h, wall_col);
                if (x + y) % 2 == 0 {
                    draw_rectangle(px + 4.0, py + 4.0, tile_w - 8.0, tile_h - 8.0, Color::new(0.32, 0.16, 0.18, 1.0));
                }
            } else {
                // Polished Marble Floor Tiles
                let floor_hash = (x * 13 + y * 29) % 3;
                let floor_col = match floor_hash {
                    0 => Color::new(0.88, 0.84, 0.78, 1.0),
                    1 => Color::new(0.84, 0.80, 0.74, 1.0),
                    _ => Color::new(0.92, 0.88, 0.82, 1.0),
                };
                draw_rectangle(px, py, tile_w, tile_h, floor_col);
                draw_rectangle_lines(px, py, tile_w, tile_h, 0.8, Color::new(0.68, 0.62, 0.55, 0.6));
            }
        }
    }

    // Carved Golden Temple Pillars
    let pillar_xs = [60.0, 160.0, 480.0, 580.0];
    for &px in &pillar_xs {
        let x = px + shake.x;
        // Shadow
        draw_ellipse(x, 290.0 + shake.y, 16.0, 6.0, 0.0, Color::new(0.0, 0.0, 0.0, 0.3));
        // Base
        draw_rectangle(x - 14.0, 275.0 + shake.y, 28.0, 15.0, PAL_SAND_4);
        draw_rectangle(x - 12.0, 265.0 + shake.y, 24.0, 10.0, PAL_SAND_3);
        // Column shaft
        draw_rectangle(x - 9.0, 80.0 + shake.y, 18.0, 185.0, PAL_SAND_2);
        draw_rectangle_lines(x - 9.0, 80.0 + shake.y, 18.0, 185.0, 1.2, PAL_GOLD_2);
        // Golden filigree ring bands
        for b in 0..4 {
            let by = 110.0 + b as f32 * 45.0 + shake.y;
            draw_rectangle(x - 11.0, by, 22.0, 5.0, PAL_GOLD_1);
        }
        // Capital
        draw_triangle(
            vec2(x - 16.0, 80.0 + shake.y),
            vec2(x + 16.0, 80.0 + shake.y),
            vec2(x, 60.0 + shake.y),
            PAL_GOLD_1,
        );
    }

    // Royal Silk Banners & Garland Drapes
    for i in 0..8 {
        let bx = 30.0 + i as f32 * 75.0 + shake.x;
        let drape = (time * 3.0 + i as f32).sin() * 2.0;
        draw_triangle(
            vec2(bx, 120.0 + shake.y),
            vec2(bx + 40.0, 120.0 + shake.y),
            vec2(bx + 20.0, 140.0 + drape + shake.y),
            PAL_RED_1,
        );
        draw_circle(bx + 20.0, 140.0 + drape + shake.y, 2.5, PAL_GOLD_0);
    }
}

// ============================================================================
// ROYAL BALCONY, KING JANAKA & PRINCESS SITA
// ============================================================================
fn draw_mithila_royalty(time: f32, shake: Vec2, font: &Font) {
    // 1. Center Royal Balcony (Princess Sita & Maids)
    let b_x = 320.0 + shake.x;
    let b_y = 52.0 + shake.y;

    // Balcony structure
    draw_rectangle(b_x - 55.0, b_y - 20.0, 110.0, 32.0, PAL_SAND_3);
    draw_rectangle_lines(b_x - 55.0, b_y - 20.0, 110.0, 32.0, 1.5, PAL_GOLD_1);
    draw_rectangle(b_x - 60.0, b_y + 12.0, 120.0, 8.0, PAL_SAND_4);

    // Princess Sita (Royal Vermillion & Gold)
    draw_circle(b_x, b_y - 4.0, 5.5, Color::new(0.96, 0.82, 0.68, 1.0)); // Face
    draw_circle(b_x, b_y - 8.0, 4.0, Color::new(0.1, 0.08, 0.12, 1.0)); // Black hair
    draw_circle(b_x, b_y - 9.0, 1.5, PAL_GOLD_0); // Golden Maang Tikka
    draw_rectangle(b_x - 6.0, b_y + 2.0, 12.0, 14.0, PAL_RED_1); // Red Silk Sari
    draw_rectangle_lines(b_x - 6.0, b_y + 2.0, 12.0, 14.0, 1.0, PAL_GOLD_0);
    // Flower Garland in hand
    let g_sway = (time * 4.0).sin() * 2.0;
    draw_circle(b_x + 8.0, b_y + 8.0 + g_sway, 2.5, PAL_GOLD_1);
    draw_circle(b_x - 8.0, b_y + 8.0 - g_sway, 2.5, PAL_GOLD_1);

    draw_pixel_text(font, "PRINCESS SITA", b_x - 38.0, b_y - 26.0, 7, PAL_GOLD_0);

    // 2. Left Throne: King Janaka
    let j_x = 100.0 + shake.x;
    let j_y = 120.0 + shake.y;
    // Throne
    draw_rectangle(j_x - 22.0, j_y - 15.0, 44.0, 35.0, PAL_GOLD_2);
    draw_rectangle(j_x - 18.0, j_y - 12.0, 36.0, 30.0, PAL_RED_2);
    // King Janaka Sprite
    draw_circle(j_x, j_y - 6.0, 6.0, Color::new(0.92, 0.78, 0.62, 1.0)); // Face
    draw_rectangle(j_x - 3.0, j_y - 2.0, 6.0, 5.0, WHITE); // White ascetic beard
    // Mukuta Crown
    draw_triangle(
        vec2(j_x - 5.0, j_y - 10.0),
        vec2(j_x + 5.0, j_y - 10.0),
        vec2(j_x, j_y - 19.0),
        PAL_GOLD_0,
    );
    draw_rectangle(j_x - 8.0, j_y + 2.0, 16.0, 14.0, PAL_TURMERIC_1); // Saffron silk robes
    draw_pixel_text(font, "KING JANAKA", j_x - 32.0, j_y - 26.0, 7, PAL_GOLD_1);

    // 3. Right Throne: Sage Vishwamitra
    let v_x = 540.0 + shake.x;
    let v_y = 120.0 + shake.y;
    // Asana Seat
    draw_rectangle(v_x - 18.0, v_y + 4.0, 36.0, 12.0, PAL_SAND_3);
    // Vishwamitra Sprite
    draw_circle(v_x, v_y - 6.0, 5.5, Color::new(0.85, 0.70, 0.55, 1.0));
    draw_circle(v_x, v_y - 11.0, 4.0, Color::new(0.3, 0.2, 0.1, 1.0)); // Jata topknot
    draw_rectangle(v_x - 4.0, v_y - 2.0, 8.0, 6.0, Color::new(0.9, 0.9, 0.9, 1.0)); // Grey beard
    draw_rectangle(v_x - 7.0, v_y + 2.0, 14.0, 14.0, PAL_RED_0); // Saffron robes
    draw_circle(v_x + 10.0, v_y + 10.0, 2.5, PAL_GOLD_2); // Kamandalu
    draw_pixel_text(font, "VISHWAMITRA", v_x - 30.0, v_y - 26.0, 7, PAL_GOLD_1);
}

// ============================================================================
// ASHTACHAKRA RATHA (8-WHEELED IRON CART) & PINAKA BOW
// ============================================================================
fn draw_ashtachakra_cart(pos: Vec2, phase: Level3Phase, string_prog: f32, break_timer: f32, time: f32) {
    let x = pos.x;
    let y = pos.y;

    // Cart Drop Shadow
    draw_ellipse(x, y + 24.0, 75.0, 16.0, 0.0, Color::new(0.0, 0.0, 0.0, 0.45));

    // 8 Heavy Iron Wheels (Ashtachakra)
    let wheel_xs = [-58.0, -40.0, -22.0, -4.0, 14.0, 32.0, 50.0, 68.0];
    for &wx in &wheel_xs {
        let wheel_c = vec2(x + wx, y + 18.0);
        draw_circle(wheel_c.x, wheel_c.y, 8.5, Color::new(0.18, 0.16, 0.20, 1.0));
        draw_circle(wheel_c.x, wheel_c.y, 7.0, Color::new(0.30, 0.28, 0.32, 1.0));
        draw_circle(wheel_c.x, wheel_c.y, 2.5, PAL_GOLD_2);
    }

    // Heavy Iron & Bronze Platform Base
    draw_rectangle(x - 70.0, y + 4.0, 140.0, 14.0, Color::new(0.24, 0.20, 0.22, 1.0));
    draw_rectangle_lines(x - 70.0, y + 4.0, 140.0, 14.0, 1.5, PAL_GOLD_3);
    // Inlaid Vedic Lotus Carvings on Iron Bed
    for i in 0..5 {
        draw_circle(x - 50.0 + i as f32 * 25.0, y + 11.0, 2.5, PAL_GOLD_1);
    }

    // Iron Bow Cradle Mounts
    draw_rectangle(x - 45.0, y - 8.0, 12.0, 12.0, Color::new(0.32, 0.28, 0.30, 1.0));
    draw_rectangle(x + 33.0, y - 8.0, 12.0, 12.0, Color::new(0.32, 0.28, 0.30, 1.0));

    // ------------------------------------------------------------------------
    // THE CELESTIAL BOW PINAKA (LORD SHIVA'S COSMIC BOW)
    // ------------------------------------------------------------------------
    match phase {
        Level3Phase::Approach | Level3Phase::Lifting => {
            // Resting on the carriage cradle with divine aura
            let bow_glow = (time * 4.0).sin() * 0.15 + 0.85;
            let aura_col = Color::new(1.0, 0.8, 0.2, 0.35 * bow_glow);
            draw_ellipse(x, y - 8.0, 62.0, 18.0, 0.0, aura_col);

            // Double recurve bow limbs in celestial deep gold & cosmic ruby
            let bow_left = vec2(x - 56.0, y - 18.0);
            let bow_center = vec2(x, y - 8.0);
            let bow_right = vec2(x + 56.0, y - 18.0);

            draw_line(bow_left.x, bow_left.y, bow_center.x, bow_center.y, 4.0, PAL_GOLD_3);
            draw_line(bow_left.x, bow_left.y, bow_center.x, bow_center.y, 2.5, PAL_GOLD_0);
            draw_line(bow_center.x, bow_center.y, bow_right.x, bow_right.y, 4.0, PAL_GOLD_3);
            draw_line(bow_center.x, bow_center.y, bow_right.x, bow_right.y, 2.5, PAL_GOLD_0);

            // Central Grip Gem (Lord Shiva's Trishula emblem)
            draw_circle(bow_center.x, bow_center.y, 4.0, PAL_RED_0);
            draw_circle(bow_center.x, bow_center.y, 2.0, PAL_FIRE_0);

            // Unstrung celestial string resting slack
            draw_line(bow_left.x, bow_left.y, bow_right.x, bow_right.y, 1.2, Color::new(0.9, 0.9, 1.0, 0.6));
        }

        Level3Phase::Stringing => {
            // Held up by Prince Rama, celestial string pulled taut!
            let bow_center = vec2(x - 5.0, y - 16.0);
            let bow_left = vec2(x - 50.0, y - 32.0);
            let bow_right = vec2(x + 50.0, y - 32.0);

            // Limbs flex inward as tension builds
            let flex = string_prog * 8.0;
            let flexed_left = vec2(bow_left.x + flex * 0.4, bow_left.y + flex);
            let flexed_right = vec2(bow_right.x - flex * 0.4, bow_right.y + flex);

            draw_line(flexed_left.x, flexed_left.y, bow_center.x, bow_center.y, 4.5, PAL_GOLD_3);
            draw_line(flexed_left.x, flexed_left.y, bow_center.x, bow_center.y, 3.0, PAL_GOLD_0);
            draw_line(bow_center.x, bow_center.y, flexed_right.x, flexed_right.y, 4.5, PAL_GOLD_3);
            draw_line(bow_center.x, bow_center.y, flexed_right.x, flexed_right.y, 3.0, PAL_GOLD_0);

            // Taut string drawn back towards Rama's chest
            let nock_point = bow_center + vec2(-string_prog * 18.0, 4.0);
            draw_line(flexed_left.x, flexed_left.y, nock_point.x, nock_point.y, 2.0, WHITE);
            draw_line(flexed_right.x, flexed_right.y, nock_point.x, nock_point.y, 2.0, WHITE);
            draw_circle(nock_point.x, nock_point.y, 3.0, PAL_FIRE_0);
        }

        Level3Phase::Dhanurbhanga | Level3Phase::Complete => {
            // The Bow is Broken in two! Shards flying outwards
            let t = (3.5 - break_timer).clamp(0.0, 3.5);
            let fly_dist = t * 24.0;
            let _rot = t * 2.0;

            // Left broken half
            let left_c = vec2(x - 30.0 - fly_dist, y - 10.0 + fly_dist * 0.5);
            draw_line(left_c.x - 22.0, left_c.y - 12.0, left_c.x + 8.0, left_c.y, 4.0, PAL_GOLD_3);
            draw_line(left_c.x - 22.0, left_c.y - 12.0, left_c.x + 8.0, left_c.y, 2.2, PAL_GOLD_1);
            // Snapped jagged end
            draw_circle(left_c.x + 8.0, left_c.y, 3.0, PAL_FIRE_1);

            // Right broken half
            let right_c = vec2(x + 30.0 + fly_dist, y - 10.0 + fly_dist * 0.5);
            draw_line(right_c.x - 8.0, right_c.y, right_c.x + 22.0, right_c.y - 12.0, 4.0, PAL_GOLD_3);
            draw_line(right_c.x - 8.0, right_c.y, right_c.x + 22.0, right_c.y - 12.0, 2.2, PAL_GOLD_1);
            // Snapped jagged end
            draw_circle(right_c.x - 8.0, right_c.y, 3.0, PAL_FIRE_1);

            // Snapped string coils
            draw_circle(left_c.x, left_c.y - 4.0, 2.0, WHITE);
            draw_circle(right_c.x, right_c.y - 4.0, 2.0, WHITE);
        }
    }
}

// ============================================================================
// HUD & INTERACTIVE MINIGAME OVERLAYS
// ============================================================================
fn draw_level3_hud(trial: &Level3Fight, font: &Font, time: f32) {
    // Top Chapter Banner
    draw_rectangle(140.0, 6.0, 360.0, 22.0, Color::new(0.08, 0.07, 0.12, 0.9));
    draw_rectangle_lines(140.0, 6.0, 360.0, 22.0, 1.2, PAL_GOLD_2);
    draw_pixel_text(font, "BALA KANDA: MITHILA — PINAKA SWAYAMVARA", 152.0, 21.0, 8, PAL_GOLD_0);

    // Court dialogue banner if active
    if let Some((msg, _)) = trial.court_dialogue {
        let msg_w = 480.0;
        let msg_x = (SCREEN_W - msg_w) * 0.5;
        draw_rectangle(msg_x, 32.0, msg_w, 22.0, Color::new(0.12, 0.08, 0.16, 0.95));
        draw_rectangle_lines(msg_x, 32.0, msg_w, 22.0, 1.0, PAL_GOLD_1);
        draw_pixel_text(font, msg, msg_x + 16.0, 47.0, 8, PAL_GOLD_0);
    }

    // Phase Instructions & Gauges
    match trial.phase {
        Level3Phase::Approach => {
            let dist_to_cart = trial.rama_pos.distance(vec2(320.0, 210.0));
            if dist_to_cart > 55.0 {
                let prompt = "▶ MOVE WITH [WASD] TO REACH SHIVA'S CARRIAGE ◀";
                let blink = (time * 5.0).sin() > 0.0;
                let text_col = if blink { PAL_GOLD_0 } else { WHITE };
                draw_pixel_text(font, prompt, 130.0, 335.0, 8, text_col);
            } else {
                let prompt = "▶ PRESS [SPACE] OR [LMB] TO LIFT PINAKA ◀";
                let blink = (time * 6.0).sin() > 0.0;
                let text_col = if blink { PAL_FIRE_1 } else { PAL_GOLD_0 };
                draw_pixel_text(font, prompt, 150.0, 335.0, 9, text_col);
            }
        }

        Level3Phase::Lifting => {
            // Balance gauge box
            let gw = 260.0;
            let gh = 34.0;
            let gx = (SCREEN_W - gw) * 0.5;
            let gy = 295.0;

            draw_rectangle(gx, gy, gw, gh, Color::new(0.08, 0.07, 0.12, 0.95));
            draw_rectangle_lines(gx, gy, gw, gh, 1.5, PAL_GOLD_2);

            // Green sweetspot zone in center
            let sweet_w = gw * 0.45;
            let sweet_x = gx + (gw - sweet_w) * 0.5;
            draw_rectangle(sweet_x, gy + 4.0, sweet_w, gh - 8.0, Color::new(0.2, 0.6, 0.25, 0.45));
            draw_rectangle_lines(sweet_x, gy + 4.0, sweet_w, gh - 8.0, 1.0, Color::new(0.4, 0.9, 0.4, 0.8));

            // Oscillating balance pin
            let ind_val = trial.balance_pos.sin(); // -1.0 to 1.0
            let pin_x = gx + gw * 0.5 + ind_val * (gw * 0.42);
            draw_line(pin_x, gy + 2.0, pin_x, gy + gh - 2.0, 3.0, PAL_GOLD_0);
            draw_circle(pin_x, gy + gh * 0.5, 4.0, PAL_RED_0);

            // Lift progress bar below
            let bar_y = gy + gh + 4.0;
            draw_rectangle(gx, bar_y, gw, 8.0, Color::new(0.15, 0.12, 0.18, 1.0));
            draw_rectangle(gx, bar_y, gw * trial.lift_progress, 8.0, PAL_GOLD_1);
            draw_rectangle_lines(gx, bar_y, gw, 8.0, 1.0, PAL_GOLD_3);

            draw_pixel_text(font, "HOLD [SPACE / LMB] INSIDE THE GREEN ZONE TO LIFT", gx + 6.0, gy - 8.0, 7, PAL_GOLD_0);
        }

        Level3Phase::Stringing => {
            // String tension bar
            let gw = 260.0;
            let gh = 18.0;
            let gx = (SCREEN_W - gw) * 0.5;
            let gy = 310.0;

            draw_rectangle(gx, gy, gw, gh, Color::new(0.08, 0.07, 0.12, 0.95));
            draw_rectangle_lines(gx, gy, gw, gh, 1.5, PAL_GOLD_2);

            // Tension fill
            let fill_col = if trial.string_progress > 0.8 { PAL_FIRE_3 } else { PAL_GOLD_1 };
            draw_rectangle(gx + 2.0, gy + 2.0, (gw - 4.0) * trial.string_progress, gh - 4.0, fill_col);

            let prompt = "HOLD [SPACE / PULL MOUSE] TO DRAW STRING TO PEAK TENSION!";
            draw_pixel_text(font, prompt, gx - 10.0, gy - 8.0, 7, PAL_FIRE_1);
        }

        Level3Phase::Dhanurbhanga => {
            let msg = "★ THE COSMIC BOW IS BROKEN! ★";
            draw_pixel_text(font, msg, 140.0, 330.0, 12, PAL_GOLD_0);
        }

        Level3Phase::Complete => {
            let msg = "VICTORY! SITA SWAYAMVARA FULFILLED";
            draw_pixel_text(font, msg, 130.0, 330.0, 10, PAL_GOLD_1);
        }
    }
}
