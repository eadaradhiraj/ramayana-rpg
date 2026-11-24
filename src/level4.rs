use macroquad::prelude::*;
use crate::audio::SoundEffects;
use crate::render::*;
use crate::types::PlayerStats;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Level4Phase {
    FestiveStreets,
    RoyalCouncil,
    InnerShrine,
    MantharaTerrace,
    Complete,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum FightResultL4 {
    Victory,
}

struct Petal {
    pos: Vec2,
    vel: Vec2,
    color: Color,
    rot: f32,
    size: f32,
}

pub struct Level4Fight {
    pub phase: Level4Phase,
    pub rama_pos: Vec2,
    pub camera_x: f32,
    pub facing_right: bool,
    pub walk_cycle: f32,
    pub dialogue: Option<(String, Vec<String>, usize)>,
    pub visited_dharmadasa: bool,
    pub visited_malati: bool,
    pub visited_sumantra: bool,
    pub visited_dasharatha: bool,
    pub visited_kausalya: bool,
    pub manthara_slide: usize,
    pub terrace_timer: f32,
    pub tilak_fx: f32,
    petals: Vec<Petal>,
}

impl Level4Fight {
    pub fn new() -> Self {
        // Spawn floating festive petals
        let mut petals = Vec::new();
        let mut rng: u32 = 84729;
        for _ in 0..50 {
            rng = rng.wrapping_mul(1664525).wrapping_add(1013904223);
            let px = (rng % 1200) as f32;
            rng = rng.wrapping_mul(1664525).wrapping_add(1013904223);
            let py = (rng % 300) as f32;
            rng = rng.wrapping_mul(1664525).wrapping_add(1013904223);
            let vx = (rng % 40) as f32 - 20.0;
            rng = rng.wrapping_mul(1664525).wrapping_add(1013904223);
            let vy = 20.0 + (rng % 30) as f32;

            let color = match rng % 3 {
                0 => Color::new(1.0, 0.72, 0.20, 0.9), // Marigold orange
                1 => PAL_GOLD_0,                       // Yellow flower
                _ => Color::new(1.0, 0.40, 0.55, 0.85), // Red/pink rose petal
            };

            petals.push(Petal {
                pos: vec2(px, py),
                vel: vec2(vx, vy),
                color,
                rot: (rng % 360) as f32,
                size: 2.5 + (rng % 4) as f32 * 0.5,
            });
        }

        Self {
            phase: Level4Phase::FestiveStreets,
            rama_pos: vec2(80.0, 235.0),
            camera_x: 0.0,
            facing_right: true,
            walk_cycle: 0.0,
            dialogue: None,
            visited_dharmadasa: false,
            visited_malati: false,
            visited_sumantra: false,
            visited_dasharatha: false,
            visited_kausalya: false,
            manthara_slide: 0,
            terrace_timer: 0.0,
            tilak_fx: 0.0,
            petals,
        }
    }

    pub fn update(
        &mut self,
        dt: f32,
        virtual_mouse: Vec2,
        _stats: &mut PlayerStats,
        sounds: &SoundEffects,
    ) -> Option<FightResultL4> {
        // Update flower petals
        for p in &mut self.petals {
            p.pos.x += p.vel.x * dt;
            p.pos.y += p.vel.y * dt;
            p.rot += 1.5 * dt;

            let max_w = if self.phase == Level4Phase::FestiveStreets { 1200.0 } else { SCREEN_W };
            if p.pos.y > 330.0 {
                p.pos.y = -10.0;
                p.pos.x = (p.pos.x + 80.0) % max_w;
            }
        }

        if self.tilak_fx > 0.0 {
            self.tilak_fx = (self.tilak_fx - dt).max(0.0);
        }

        // Active Dialogue handling with Re-Read Support
        if let Some((_speaker, lines, idx)) = &mut self.dialogue {
            let is_prev_click = is_mouse_button_pressed(MouseButton::Left)
                && is_dialogue_prev_button_hovered(virtual_mouse);

            if is_key_pressed(KeyCode::Left) || is_key_pressed(KeyCode::Backspace) || is_prev_click {
                if *idx > 0 {
                    *idx -= 1;
                    sounds.play(&sounds.shoot, 0.4);
                }
                return None;
            }

            let advance = is_key_pressed(KeyCode::Space)
                || is_key_pressed(KeyCode::Enter)
                || is_key_pressed(KeyCode::E)
                || is_mouse_button_pressed(MouseButton::Left);

            if advance {
                sounds.play(&sounds.shoot, 0.4);
                *idx += 1;
                if *idx >= lines.len() {
                    self.dialogue = None;
                }
            }
            return None;
        }

        // Phase-specific input and flow
        match self.phase {
            Level4Phase::FestiveStreets => {
                let speed = 120.0;
                let mut move_x = 0.0;

                if is_key_down(KeyCode::A) || is_key_down(KeyCode::Left) {
                    move_x -= 1.0;
                    self.facing_right = false;
                }
                if is_key_down(KeyCode::D) || is_key_down(KeyCode::Right) {
                    move_x += 1.0;
                    self.facing_right = true;
                }

                if move_x != 0.0 {
                    self.rama_pos.x += move_x * speed * dt;
                    self.walk_cycle += dt * 10.0;
                } else {
                    self.walk_cycle = 0.0;
                }

                // Bounds in street
                self.rama_pos.x = self.rama_pos.x.clamp(40.0, 1120.0);

                // Smooth camera scrolling
                let target_cam = (self.rama_pos.x - SCREEN_W * 0.5).clamp(0.0, 1140.0 - SCREEN_W);
                self.camera_x += (target_cam - self.camera_x) * 8.0 * dt;

                // Interactions:
                let interact = is_key_pressed(KeyCode::E)
                    || is_key_pressed(KeyCode::Space)
                    || is_key_pressed(KeyCode::Enter);

                // 1. Citizen Dharmadasa at x = 240
                if (self.rama_pos.x - 240.0).abs() < 45.0 && interact {
                    sounds.play(&sounds.bell, 0.6);
                    self.visited_dharmadasa = true;
                    self.dialogue = Some((
                        "Citizen Dharmadasa".to_string(),
                        vec![
                            "Pranams, Prince Rama! The whole city of Ayodhya is in a frenzy of pure joy!".to_string(),
                            "All thirty-two avenues are washed with cool sandalwood water and strewn with fresh flowers!".to_string(),
                            "King Dasharatha has decreed your coronation at dawn under the sacred Pushya star!".to_string(),
                        ],
                        0,
                    ));
                }
                // 2. Citizen Malati at x = 480
                else if (self.rama_pos.x - 480.0).abs() < 45.0 && interact {
                    sounds.play(&sounds.bell, 0.6);
                    self.visited_malati = true;
                    self.dialogue = Some((
                        "Malati (Flower Merchant)".to_string(),
                        vec![
                            "Glory to Prince Rama, embodiment of Dharma!".to_string(),
                            "We have woven fifty thousand marigold garlands to drape upon every pillar of Ayodhya!".to_string(),
                            "May you reign over Kosala for a hundred glorious autumns!".to_string(),
                        ],
                        0,
                    ));
                }
                // 3. Charioteer Sumantra at x = 750
                else if (self.rama_pos.x - 750.0).abs() < 55.0 && interact {
                    sounds.play(&sounds.conch, 0.85);
                    self.visited_sumantra = true;
                    self.dialogue = Some((
                        "Minister Sumantra".to_string(),
                        vec![
                            "Prince Rama! The golden chariot with four white steeds is prepared for tomorrow's procession.".to_string(),
                            "His Majesty King Dasharatha anxiously awaits you in the Royal Sabha Council Hall.".to_string(),
                            "Proceed through the golden Simhadvara portal just ahead to meet the King and Sage Vasishtha!".to_string(),
                        ],
                        0,
                    ));
                }
                // 4. Palace Entrance at x >= 1020
                else if self.rama_pos.x >= 1020.0 {
                    if interact || self.rama_pos.x >= 1080.0 {
                        sounds.play(&sounds.conch, 0.9);
                        self.phase = Level4Phase::RoyalCouncil;
                        self.rama_pos = vec2(60.0, 235.0);
                        self.camera_x = 0.0;
                    }
                }
            }

            Level4Phase::RoyalCouncil => {
                let speed = 100.0;
                let mut move_x = 0.0;

                if is_key_down(KeyCode::A) || is_key_down(KeyCode::Left) {
                    move_x -= 1.0;
                    self.facing_right = false;
                }
                if is_key_down(KeyCode::D) || is_key_down(KeyCode::Right) {
                    move_x += 1.0;
                    self.facing_right = true;
                }

                if move_x != 0.0 {
                    self.rama_pos.x += move_x * speed * dt;
                    self.walk_cycle += dt * 10.0;
                } else {
                    self.walk_cycle = 0.0;
                }

                self.rama_pos.x = self.rama_pos.x.clamp(40.0, 580.0);

                let interact = is_key_pressed(KeyCode::E)
                    || is_key_pressed(KeyCode::Space)
                    || is_key_pressed(KeyCode::Enter);

                // Near Dasharatha at x = 320
                if (self.rama_pos.x - 300.0).abs() < 50.0 && interact && !self.visited_dasharatha {
                    sounds.play(&sounds.conch, 0.9);
                    self.visited_dasharatha = true;
                    self.dialogue = Some((
                        "King Dasharatha".to_string(),
                        vec![
                            "Rama, my dearest child! Come near to your aging father.".to_string(),
                            "The royal council, our revered sages, and all the citizens have unanimously chosen you to be crowned Yuvaraja!".to_string(),
                            "Rule with humble devotion, restrain all pride, and protect our people as a father cherishes his children.".to_string(),
                            "Sage Vasishtha: 'Rama, proceed to Queen Kausalya's shrine. Observe the sacred Upavasa fast with Princess Sita tonight!'".to_string(),
                        ],
                        0,
                    ));
                }
                // Doorway to Inner Shrine on the right (x > 530)
                else if self.rama_pos.x >= 530.0 && self.visited_dasharatha {
                    sounds.play(&sounds.bell, 0.7);
                    self.phase = Level4Phase::InnerShrine;
                    self.rama_pos = vec2(70.0, 235.0);
                }
            }

            Level4Phase::InnerShrine => {
                let speed = 100.0;
                let mut move_x = 0.0;

                if is_key_down(KeyCode::A) || is_key_down(KeyCode::Left) {
                    move_x -= 1.0;
                    self.facing_right = false;
                }
                if is_key_down(KeyCode::D) || is_key_down(KeyCode::Right) {
                    move_x += 1.0;
                    self.facing_right = true;
                }

                if move_x != 0.0 {
                    self.rama_pos.x += move_x * speed * dt;
                    self.walk_cycle += dt * 10.0;
                } else {
                    self.walk_cycle = 0.0;
                }

                self.rama_pos.x = self.rama_pos.x.clamp(40.0, 580.0);

                let interact = is_key_pressed(KeyCode::E)
                    || is_key_pressed(KeyCode::Space)
                    || is_key_pressed(KeyCode::Enter);

                // Near Kausalya & Sita at x = 310
                if (self.rama_pos.x - 310.0).abs() < 50.0 && interact && !self.visited_kausalya {
                    sounds.play(&sounds.bell, 0.8);
                    self.visited_kausalya = true;
                    self.tilak_fx = 4.0;
                    self.dialogue = Some((
                        "Queen Kausalya & Sita".to_string(),
                        vec![
                            "Queen Kausalya: 'May the devas protect you, Rama! Here, receive the golden tilak of coronation blessings upon your brow!'".to_string(),
                            "Princess Sita: 'My Lord, I have prepared the sacred kusha grass bed and fragrant offerings for our holy Upavasa fast tonight.'".to_string(),
                            "Prince Lakshmana: 'Brother! My bow Kodanda and my life shall ever stand guard beside your righteous throne!'".to_string(),
                            "Proceed to the palace balcony to view the grand illuminated night sky of Ayodhya...".to_string(),
                        ],
                        0,
                    ));
                }
                // Doorway to balcony at x > 530
                else if self.rama_pos.x >= 530.0 && self.visited_kausalya {
                    sounds.play(&sounds.thunder, 0.6);
                    self.phase = Level4Phase::MantharaTerrace;
                    self.manthara_slide = 0;
                    self.terrace_timer = 0.0;
                }
            }

            Level4Phase::MantharaTerrace => {
                self.terrace_timer += dt;

                let advance = is_key_pressed(KeyCode::Space)
                    || is_key_pressed(KeyCode::Enter)
                    || is_mouse_button_pressed(MouseButton::Left);

                if advance && self.terrace_timer > 0.5 {
                    self.terrace_timer = 0.0;
                    self.manthara_slide += 1;
                    if self.manthara_slide >= 4 {
                        sounds.play(&sounds.victory, 0.85);
                        self.phase = Level4Phase::Complete;
                        return Some(FightResultL4::Victory);
                    } else if self.manthara_slide == 2 {
                        sounds.play(&sounds.thunder, 0.7);
                    } else {
                        sounds.play(&sounds.shoot, 0.5);
                    }
                }
            }

            Level4Phase::Complete => {
                return Some(FightResultL4::Victory);
            }
        }

        None
    }

    pub fn draw(&self, font: &Font, virtual_mouse: Vec2, time: f32) {
        match self.phase {
            Level4Phase::FestiveStreets => {
                self.draw_festive_streets(font, time);
            }
            Level4Phase::RoyalCouncil => {
                self.draw_royal_council(font, time);
            }
            Level4Phase::InnerShrine => {
                self.draw_inner_shrine(font, time);
            }
            Level4Phase::MantharaTerrace => {
                self.draw_manthara_terrace(font, time);
            }
            Level4Phase::Complete => {
                draw_victory_l4(font, time);
            }
        }

        // Draw Active Dialogue Box
        if let Some((speaker, lines, idx)) = &self.dialogue {
            if *idx < lines.len() {
                draw_dialogue_box(font, speaker, &lines[*idx], time, *idx, lines.len(), virtual_mouse);
            }
        }

        draw_reticle(virtual_mouse, false);
    }

    // ========================================================================
    // SCENE 1: FESTIVE STREETS OF AYODHYA (RAJA MARGA)
    // ========================================================================
    fn draw_festive_streets(&self, font: &Font, time: f32) {
        // Bright festive morning sky of Ayodhya
        clear_background(Color::new(0.42, 0.68, 0.92, 1.0));

        let cx = self.camera_x;

        // Distant grand temple Shikharas (Parallax background)
        for i in 0..12 {
            let sx = i as f32 * 120.0 - cx * 0.3;
            // Golden Shikhara towers
            draw_triangle(
                vec2(sx, 160.0),
                vec2(sx + 70.0, 160.0),
                vec2(sx + 35.0, 50.0),
                Color::new(0.85, 0.75, 0.45, 0.6),
            );
            draw_circle(sx + 35.0, 48.0, 4.0, PAL_GOLD_0);
        }

        // Midground: Marble Palace Facades & Festive Pillars
        for i in 0..15 {
            let bx = i as f32 * 90.0 - cx;
            // White marble building block
            draw_rectangle(bx, 120.0, 84.0, 120.0, Color::new(0.95, 0.94, 0.98, 1.0));
            draw_rectangle_lines(bx, 120.0, 84.0, 120.0, 1.5, PAL_GOLD_3);

            // Arched Windows
            draw_ellipse(bx + 42.0, 150.0, 14.0, 22.0, 0.0, Color::new(0.18, 0.16, 0.28, 1.0));
            draw_circle(bx + 42.0, 140.0, 14.0, Color::new(0.95, 0.94, 0.98, 1.0));

            // Saffron festival flags (Dhwaja) fluttering
            let flag_wave = (time * 6.0 + i as f32).sin() * 5.0;
            draw_line(bx + 42.0, 120.0, bx + 42.0, 80.0, 2.0, PAL_SAND_2);
            draw_triangle(
                vec2(bx + 42.0, 82.0),
                vec2(bx + 42.0, 102.0),
                vec2(bx + 72.0 + flag_wave, 92.0),
                PAL_TURMERIC_1,
            );
        }

        // Mango-leaf & Marigold Garlands strung across buildings
        for i in 0..14 {
            let gx1 = i as f32 * 90.0 + 42.0 - cx;
            let gx2 = gx1 + 90.0;
            let dip = 15.0 + (time * 2.0).sin() * 2.0;
            draw_line(gx1, 120.0, (gx1 + gx2) * 0.5, 120.0 + dip, 1.5, PAL_GRASS_2);
            draw_line((gx1 + gx2) * 0.5, 120.0 + dip, gx2, 120.0, 1.5, PAL_GRASS_2);

            // Marigold bead clusters
            for b in 1..4 {
                let bx = gx1 + b as f32 * 22.0;
                let by = 120.0 + dip * 0.7;
                draw_circle(bx, by, 3.0, PAL_TURMERIC_1);
                draw_circle(bx, by, 1.5, PAL_GOLD_0);
            }
        }

        // Paved Sandalwood-washed Royal Avenue (Raja Marga)
        let road_y = 240.0;
        draw_rectangle(0.0, road_y, SCREEN_W, SCREEN_H - road_y, Color::new(0.88, 0.82, 0.72, 1.0));
        draw_line(0.0, road_y, SCREEN_W, road_y, 2.0, PAL_GOLD_2);

        // Rangoli sacred decorative patterns on road
        for i in 0..12 {
            let rx = i as f32 * 110.0 + 30.0 - cx;
            if rx >= -50.0 && rx <= SCREEN_W + 50.0 {
                draw_circle(rx, road_y + 25.0, 8.0, Color::new(0.95, 0.45, 0.50, 0.8));
                draw_circle(rx, road_y + 25.0, 4.0, PAL_GOLD_0);
                draw_circle_lines(rx, road_y + 25.0, 14.0, 1.0, Color::new(0.98, 0.98, 0.98, 0.7));
            }
        }

        // Banana Trees (Mangala Torana) at doorways
        let tree_spots = [180.0, 360.0, 600.0, 880.0];
        for tx in tree_spots {
            let scr_x = tx - cx;
            if scr_x >= -40.0 && scr_x <= SCREEN_W + 40.0 {
                // Green trunk
                draw_rectangle(scr_x - 3.0, road_y - 40.0, 6.0, 40.0, PAL_GRASS_2);
                // Broad leaves
                draw_ellipse(scr_x - 12.0, road_y - 35.0, 16.0, 6.0, -0.4, PAL_GRASS_1);
                draw_ellipse(scr_x + 12.0, road_y - 35.0, 16.0, 6.0, 0.4, PAL_GRASS_1);
                // Golden Purna Kumbha pot
                draw_circle(scr_x, road_y - 2.0, 7.0, PAL_GOLD_1);
                draw_circle(scr_x, road_y - 7.0, 4.0, Color::new(0.4, 0.8, 0.3, 1.0));
            }
        }

        // NPCs:
        // 1. Citizen Dharmadasa at x = 240
        let dh_x = 240.0 - cx;
        if dh_x >= -40.0 && dh_x <= SCREEN_W + 40.0 {
            draw_circle(dh_x, road_y - 22.0, 6.0, PAL_RED_2);
            draw_rectangle(dh_x - 6.0, road_y - 16.0, 12.0, 20.0, PAL_TURMERIC_1);
            draw_pixel_text(font, "Dharmadasa", dh_x - 30.0, road_y - 32.0, 7, PAL_GOLD_0);
            if (self.rama_pos.x - 240.0).abs() < 45.0 {
                let pulse = (time * 6.0).sin() * 2.0;
                draw_pixel_text(font, "[SPACE / E]", dh_x - 26.0, road_y - 44.0 + pulse, 7, WHITE);
            }
        }

        // 2. Citizen Malati (Flower seller) at x = 480
        let ml_x = 480.0 - cx;
        if ml_x >= -40.0 && ml_x <= SCREEN_W + 40.0 {
            draw_circle(ml_x, road_y - 20.0, 6.0, PAL_SAND_2);
            draw_rectangle(ml_x - 6.0, road_y - 14.0, 12.0, 18.0, Color::new(0.9, 0.3, 0.5, 1.0)); // Saree
            // Flower basket
            draw_circle(ml_x + 9.0, road_y - 4.0, 6.0, PAL_SAND_3);
            draw_circle(ml_x + 9.0, road_y - 7.0, 3.0, PAL_GOLD_0);
            draw_pixel_text(font, "Malati", ml_x - 18.0, road_y - 30.0, 7, PAL_GOLD_0);
            if (self.rama_pos.x - 480.0).abs() < 45.0 {
                let pulse = (time * 6.0).sin() * 2.0;
                draw_pixel_text(font, "[SPACE / E]", ml_x - 26.0, road_y - 42.0 + pulse, 7, WHITE);
            }
        }

        // 3. Minister Sumantra & Golden Chariot at x = 750
        let ch_x = 750.0 - cx;
        if ch_x >= -120.0 && ch_x <= SCREEN_W + 120.0 {
            // Chariot Body
            draw_rectangle(ch_x - 35.0, road_y - 30.0, 55.0, 26.0, PAL_GOLD_1);
            draw_rectangle_lines(ch_x - 35.0, road_y - 30.0, 55.0, 26.0, 2.0, PAL_GOLD_3);
            // Wheels
            draw_circle(ch_x - 20.0, road_y - 2.0, 12.0, PAL_GOLD_3);
            draw_circle(ch_x - 20.0, road_y - 2.0, 7.0, PAL_GOLD_0);
            draw_circle(ch_x + 8.0, road_y - 2.0, 12.0, PAL_GOLD_3);
            draw_circle(ch_x + 8.0, road_y - 2.0, 7.0, PAL_GOLD_0);
            // Royal Chariot Canopy
            draw_triangle(
                vec2(ch_x - 40.0, road_y - 30.0),
                vec2(ch_x + 25.0, road_y - 30.0),
                vec2(ch_x - 8.0, road_y - 58.0),
                Color::new(0.85, 0.15, 0.18, 1.0),
            );
            // White Horses
            draw_rectangle(ch_x + 35.0, road_y - 22.0, 38.0, 18.0, WHITE);
            draw_circle(ch_x + 72.0, road_y - 24.0, 8.0, WHITE);
            draw_rectangle(ch_x + 42.0, road_y - 4.0, 5.0, 14.0, Color::new(0.9, 0.9, 0.9, 1.0));
            draw_rectangle(ch_x + 65.0, road_y - 4.0, 5.0, 14.0, Color::new(0.9, 0.9, 0.9, 1.0));

            // Sumantra
            draw_circle(ch_x - 10.0, road_y - 36.0, 6.0, PAL_SAND_2);
            draw_rectangle(ch_x - 16.0, road_y - 30.0, 12.0, 18.0, PAL_RED_1);
            draw_pixel_text(font, "Sumantra", ch_x - 26.0, road_y - 66.0, 7, PAL_GOLD_0);

            if (self.rama_pos.x - 750.0).abs() < 55.0 {
                let pulse = (time * 6.0).sin() * 2.0;
                draw_pixel_text(font, "[SPACE / E]", ch_x - 28.0, road_y - 78.0 + pulse, 7, WHITE);
            }
        }

        // 4. Grand Palace Gateway (Simhadvara) at x = 1040
        let gate_x = 1040.0 - cx;
        if gate_x >= -80.0 && gate_x <= SCREEN_W + 120.0 {
            // Tall golden pillars
            draw_rectangle(gate_x - 25.0, 80.0, 16.0, 160.0, PAL_GOLD_1);
            draw_rectangle(gate_x + 45.0, 80.0, 16.0, 160.0, PAL_GOLD_1);
            // Archway Header
            draw_rectangle(gate_x - 35.0, 70.0, 106.0, 24.0, PAL_GOLD_2);
            draw_pixel_text(font, "ROYAL SABHA", gate_x - 28.0, 84.0, 7, PAL_GOLD_0);

            if self.rama_pos.x >= 980.0 {
                let pulse = (time * 6.0).sin() * 2.0;
                draw_pixel_text(font, "ENTER SABHA ▶", gate_x - 30.0, 140.0 + pulse, 8, PAL_GOLD_0);
            }
        }

        // Draw Prince Rama
        let rama_scr_x = self.rama_pos.x - cx;
        let aim_vec = if self.facing_right { vec2(1.0, 0.0) } else { vec2(-1.0, 0.0) };
        draw_rama(vec2(rama_scr_x, self.rama_pos.y), aim_vec, 0.0, 0.0, time);

        // Falling Flower Petals
        for p in &self.petals {
            let px = p.pos.x - cx;
            if px >= -10.0 && px <= SCREEN_W + 10.0 {
                draw_ellipse(px, p.pos.y, p.size, p.size * 0.55, p.rot, p.color);
            }
        }

        // Street HUD Banner
        draw_rectangle(10.0, 10.0, 380.0, 28.0, Color::new(0.08, 0.07, 0.12, 0.85));
        draw_rectangle_lines(10.0, 10.0, 380.0, 28.0, 1.0, PAL_GOLD_2);
        draw_pixel_text(font, "AYODHYA: RAJA MARGA (EVE OF CORONATION)", 18.0, 24.0, 7, PAL_GOLD_0);
        draw_pixel_text(font, "[WASD] Walk | [SPACE/E] Converse | Explore to Royal Sabha", 18.0, 34.0, 6, WHITE);
    }

    // ========================================================================
    // SCENE 2: THE ROYAL COUNCIL SABHA
    // ========================================================================
    fn draw_royal_council(&self, font: &Font, time: f32) {
        clear_background(Color::new(0.08, 0.06, 0.10, 1.0));

        // Ornate Marble Pillars
        for i in 0..7 {
            let px = i as f32 * 96.0 + 30.0;
            draw_rectangle(px - 10.0, 40.0, 20.0, 200.0, Color::new(0.92, 0.90, 0.96, 1.0));
            draw_rectangle_lines(px - 10.0, 40.0, 20.0, 200.0, 1.0, PAL_GOLD_2);
            // Gold Capitals
            draw_rectangle(px - 15.0, 36.0, 30.0, 10.0, PAL_GOLD_1);
            draw_rectangle(px - 15.0, 235.0, 30.0, 10.0, PAL_GOLD_1);
        }

        // Red Royal Carpet
        draw_rectangle(0.0, 240.0, SCREEN_W, 120.0, Color::new(0.14, 0.12, 0.16, 1.0));
        draw_rectangle(40.0, 240.0, 560.0, 24.0, Color::new(0.65, 0.15, 0.20, 1.0));
        draw_line(40.0, 240.0, 600.0, 240.0, 2.0, PAL_GOLD_1);
        draw_line(40.0, 264.0, 600.0, 264.0, 2.0, PAL_GOLD_1);

        // Golden Simhasana (Lion Throne) at x = 320
        let th_x = 320.0;
        let th_y = 200.0;
        // Canopy
        draw_triangle(vec2(th_x - 45.0, th_y - 70.0), vec2(th_x + 45.0, th_y - 70.0), vec2(th_x, th_y - 95.0), PAL_GOLD_1);
        draw_circle(th_x, th_y - 95.0, 4.0, PAL_RED_1);
        // Backrest
        draw_rectangle(th_x - 30.0, th_y - 65.0, 60.0, 65.0, PAL_GOLD_2);
        draw_rectangle_lines(th_x - 30.0, th_y - 65.0, 60.0, 65.0, 2.0, PAL_GOLD_0);
        draw_circle(th_x, th_y - 35.0, 16.0, Color::new(0.7, 0.15, 0.2, 1.0)); // Velvet cushion

        // King Dasharatha seated on Throne
        draw_circle(th_x, th_y - 42.0, 8.0, PAL_SAND_2);
        draw_triangle(vec2(th_x - 6.0, th_y - 48.0), vec2(th_x + 6.0, th_y - 48.0), vec2(th_x, th_y - 58.0), PAL_GOLD_0); // Crown
        draw_circle(th_x, th_y - 34.0, 4.0, WHITE); // White beard
        draw_rectangle(th_x - 12.0, th_y - 32.0, 24.0, 28.0, Color::new(0.85, 0.15, 0.25, 1.0)); // Royal Robe
        draw_pixel_text(font, "King Dasharatha", th_x - 48.0, th_y - 75.0, 7, PAL_GOLD_0);

        // Sage Vasishtha (Left of Throne at x = 240)
        let v_x = 230.0;
        draw_circle(v_x, th_y - 36.0, 7.0, PAL_SAND_3);
        draw_circle(v_x, v_x * 0.0 + th_y - 42.0, 4.0, Color::new(0.4, 0.25, 0.15, 1.0)); // Jata topknot
        draw_rectangle(v_x - 8.0, th_y - 28.0, 16.0, 32.0, PAL_TURMERIC_1); // Saffron robe
        draw_pixel_text(font, "Vasishtha", v_x - 25.0, th_y - 50.0, 7, PAL_GOLD_1);

        // Courtiers holding Chamara Whisks (Right of Throne at x = 400)
        let c_x = 410.0;
        draw_circle(c_x, th_y - 36.0, 6.0, PAL_SAND_2);
        draw_rectangle(c_x - 6.0, th_y - 28.0, 12.0, 30.0, Color::new(0.85, 0.85, 0.9, 1.0));
        let chamara_wave = (time * 5.0).sin() * 6.0;
        draw_line(c_x + 8.0, th_y - 10.0, c_x + 16.0, th_y - 40.0, 2.0, PAL_GOLD_2);
        draw_ellipse(c_x + 16.0 + chamara_wave, th_y - 45.0, 8.0, 14.0, 0.2, WHITE);

        // Gateway to Inner Shrine at x = 580
        draw_rectangle(560.0, 120.0, 20.0, 120.0, PAL_GOLD_2);
        draw_pixel_text(font, "TO SHRINE ▶", 530.0, 105.0, 6, PAL_GOLD_0);

        // Draw Prince Rama
        let aim_vec = if self.facing_right { vec2(1.0, 0.0) } else { vec2(-1.0, 0.0) };
        draw_rama(self.rama_pos, aim_vec, 0.0, 0.0, time);

        // Prompt
        if !self.visited_dasharatha && (self.rama_pos.x - 300.0).abs() < 50.0 {
            let pulse = (time * 6.0).sin() * 2.0;
            draw_pixel_text(font, "[SPACE / E] ACCEPT ROYAL DECREE", 210.0, 150.0 + pulse, 8, PAL_GOLD_0);
        } else if self.visited_dasharatha && self.rama_pos.x < 500.0 {
            draw_pixel_text(font, "Proceed Right into the Inner Shrine ▶", 220.0, 150.0, 7, PAL_GOLD_1);
        }

        // HUD Banner
        draw_rectangle(10.0, 10.0, 360.0, 28.0, Color::new(0.08, 0.07, 0.12, 0.85));
        draw_rectangle_lines(10.0, 10.0, 360.0, 28.0, 1.0, PAL_GOLD_2);
        draw_pixel_text(font, "THE ROYAL SABHA OF AYODHYA", 18.0, 24.0, 7, PAL_GOLD_0);
        draw_pixel_text(font, "Approach King Dasharatha to receive the Yuvaraja Decree", 18.0, 34.0, 6, WHITE);
    }

    // ========================================================================
    // SCENE 3: THE INNER SHRINE (KAUSALYA & SITA)
    // ========================================================================
    fn draw_inner_shrine(&self, font: &Font, time: f32) {
        clear_background(Color::new(0.07, 0.05, 0.09, 1.0));

        // Sacred Sanctum Wall Panels
        for i in 0..5 {
            let wx = i as f32 * 125.0 + 15.0;
            draw_rectangle(wx, 50.0, 110.0, 190.0, Color::new(0.12, 0.10, 0.16, 1.0));
            draw_rectangle_lines(wx, 50.0, 110.0, 190.0, 1.0, PAL_GOLD_3);
            draw_circle(wx + 55.0, 90.0, 24.0, Color::new(0.20, 0.15, 0.28, 0.6));
            draw_circle(wx + 55.0, 90.0, 12.0, PAL_GOLD_2);
        }

        // Floor
        let floor_y = 240.0;
        draw_rectangle(0.0, floor_y, SCREEN_W, SCREEN_H - floor_y, Color::new(0.16, 0.13, 0.20, 1.0));
        draw_line(0.0, floor_y, SCREEN_W, floor_y, 2.0, PAL_GOLD_1);

        // Sacred Agni Kunda (Holy Fire Altar) at x = 320
        let a_x = 320.0;
        let a_y = 230.0;
        draw_rectangle(a_x - 30.0, a_y - 12.0, 60.0, 20.0, PAL_SAND_4);
        draw_rectangle(a_x - 22.0, a_y - 18.0, 44.0, 10.0, PAL_SAND_2);

        // Flaming Fire
        let f_h = 16.0 + (time * 9.0).sin() * 5.0;
        draw_triangle(
            vec2(a_x - 14.0, a_y - 18.0),
            vec2(a_x + 14.0, a_y - 18.0),
            vec2(a_x, a_y - 18.0 - f_h),
            PAL_FIRE_2,
        );
        draw_circle(a_x, a_y - 18.0 - f_h * 0.4, 4.0, PAL_GOLD_0);
        // Soft fire glow
        draw_circle(a_x, a_y - 20.0, 36.0, Color::new(1.0, 0.6, 0.1, 0.15));

        // Queen Kausalya (Left of Fire at x = 250)
        let k_x = 250.0;
        draw_circle(k_x, floor_y - 26.0, 7.0, PAL_SAND_2);
        draw_rectangle(k_x - 7.0, floor_y - 18.0, 14.0, 26.0, WHITE); // Auspicious white silk
        draw_line(k_x - 7.0, floor_y - 18.0, k_x + 7.0, floor_y - 18.0, 2.0, PAL_GOLD_1);
        draw_pixel_text(font, "Queen Kausalya", k_x - 35.0, floor_y - 36.0, 7, PAL_GOLD_0);

        // Princess Sita (Right of Fire at x = 390)
        let s_x = 390.0;
        draw_circle(s_x, floor_y - 24.0, 6.5, PAL_SAND_2);
        draw_rectangle(s_x - 6.0, floor_y - 17.0, 12.0, 25.0, PAL_TURMERIC_1); // Golden yellow saree
        draw_circle(s_x, floor_y - 28.0, 3.0, Color::new(0.3, 0.18, 0.1, 1.0)); // Hair bun
        draw_pixel_text(font, "Princess Sita", s_x - 30.0, floor_y - 34.0, 7, PAL_GOLD_0);

        // Lakshmana (Standing guard at x = 460)
        let l_x = 460.0;
        draw_circle(l_x, floor_y - 28.0, 7.0, PAL_SAND_2);
        draw_triangle(vec2(l_x - 5.0, floor_y - 34.0), vec2(l_x + 5.0, floor_y - 34.0), vec2(l_x, floor_y - 42.0), PAL_GOLD_0);
        draw_rectangle(l_x - 7.0, floor_y - 20.0, 14.0, 28.0, PAL_TURMERIC_1);
        // Bow on shoulder
        draw_line(l_x - 8.0, floor_y - 32.0, l_x - 8.0, floor_y + 4.0, 2.0, PAL_GOLD_2);
        draw_pixel_text(font, "Lakshmana", l_x - 22.0, floor_y - 46.0, 7, PAL_GOLD_1);

        // Portal to Balcony at x = 580
        draw_rectangle(560.0, 120.0, 20.0, 120.0, PAL_GOLD_1);
        draw_pixel_text(font, "TO TERRACE ▶", 525.0, 105.0, 6, PAL_GOLD_0);

        // Draw Prince Rama
        let aim_vec = if self.facing_right { vec2(1.0, 0.0) } else { vec2(-1.0, 0.0) };
        draw_rama(self.rama_pos, aim_vec, 0.0, 0.0, time);

        // Divine Tilak Glowing FX
        if self.tilak_fx > 0.0 {
            let glow = (time * 8.0).sin().abs() * 0.4 + 0.6;
            draw_circle(self.rama_pos.x, self.rama_pos.y - 18.0, 6.0 * glow, PAL_GOLD_0);
            draw_circle(self.rama_pos.x, self.rama_pos.y - 18.0, 2.5, PAL_FIRE_0);
        }

        // Interaction Prompt
        if !self.visited_kausalya && (self.rama_pos.x - 310.0).abs() < 50.0 {
            let pulse = (time * 6.0).sin() * 2.0;
            draw_pixel_text(font, "[SPACE / E] RECEIVE UPAVASA BLESSINGS", 190.0, 150.0 + pulse, 8, PAL_GOLD_0);
        } else if self.visited_kausalya && self.rama_pos.x < 500.0 {
            draw_pixel_text(font, "Step Out to the Moonlit Palace Terrace ▶", 195.0, 150.0, 7, PAL_GOLD_1);
        }

        // HUD Banner
        draw_rectangle(10.0, 10.0, 360.0, 28.0, Color::new(0.08, 0.07, 0.12, 0.85));
        draw_rectangle_lines(10.0, 10.0, 360.0, 28.0, 1.0, PAL_GOLD_2);
        draw_pixel_text(font, "QUEEN KAUSALYA'S INNER SHRINE", 18.0, 24.0, 7, PAL_GOLD_0);
        draw_pixel_text(font, "Meet Queen Kausalya & Princess Sita for Coronation Vows", 18.0, 34.0, 6, WHITE);
    }

    // ========================================================================
    // SCENE 4: MANTHARA'S MOONLIT TERRACE (THE SHADOW OF EXILE)
    // ========================================================================
    fn draw_manthara_terrace(&self, font: &Font, time: f32) {
        // Night sky of Ayodhya: Deep indigo / midnight blue
        clear_background(Color::new(0.04, 0.04, 0.12, 1.0));

        // Twinkling Stars
        let mut rng: u32 = 91823;
        for _ in 0..45 {
            rng = rng.wrapping_mul(1664525).wrapping_add(1013904223);
            let sx = (rng % SCREEN_W as u32) as f32;
            rng = rng.wrapping_mul(1664525).wrapping_add(1013904223);
            let sy = (rng % 140) as f32;
            let star_blink = ((time * 3.0 + sx * 0.1).sin() > 0.3) as i32;
            if star_blink == 1 {
                draw_circle(sx, sy, 1.2, Color::new(0.9, 0.9, 1.0, 0.8));
            }
        }

        // Glowing Crescent Moon
        draw_circle(540.0, 55.0, 22.0, Color::new(0.98, 0.96, 0.82, 0.95));
        draw_circle(548.0, 51.0, 20.0, Color::new(0.04, 0.04, 0.12, 1.0)); // Shadow cut

        // Distant illuminated Ayodhya Cityscape (Rows of golden oil lamps - Deepas)
        for i in 0..16 {
            let tx = i as f32 * 42.0;
            // Distant palace rooftops
            draw_rectangle(tx, 160.0, 38.0, 80.0, Color::new(0.08, 0.07, 0.18, 1.0));
            draw_triangle(vec2(tx, 160.0), vec2(tx + 38.0, 160.0), vec2(tx + 19.0, 130.0), Color::new(0.12, 0.10, 0.22, 1.0));

            // Glowing Deepas on rooftops
            let lamp_flicker = (time * 7.0 + i as f32).sin() * 0.2 + 0.8;
            draw_circle(tx + 10.0, 160.0, 2.5 * lamp_flicker, PAL_GOLD_0);
            draw_circle(tx + 28.0, 160.0, 2.5 * lamp_flicker, PAL_GOLD_0);
        }

        // High Palace Balustrade (Marble Parapet)
        let wall_y = 230.0;
        draw_rectangle(0.0, wall_y, SCREEN_W, SCREEN_H - wall_y, Color::new(0.12, 0.11, 0.18, 1.0));
        draw_line(0.0, wall_y, SCREEN_W, wall_y, 2.0, PAL_GOLD_2);

        // Balusters
        for i in 0..20 {
            let bx = i as f32 * 32.0 + 8.0;
            draw_rectangle(bx, wall_y - 24.0, 6.0, 24.0, Color::new(0.75, 0.72, 0.82, 1.0));
            draw_circle(bx + 3.0, wall_y - 24.0, 4.0, PAL_GOLD_1);
        }
        draw_rectangle(0.0, wall_y - 30.0, SCREEN_W, 6.0, Color::new(0.85, 0.82, 0.90, 1.0));

        // Hunchbacked Maid MANTHARA (Silhouette with sinister aura)
        let m_x = 220.0;
        let m_y = wall_y - 28.0;

        // Malevolent Green/Violet Jealousy Aura
        let aura_pulse = (time * 4.0).sin() * 4.0;
        draw_ellipse(m_x, m_y - 18.0, 26.0 + aura_pulse, 34.0 + aura_pulse, 0.0, Color::new(0.2, 0.6, 0.3, 0.2));
        draw_ellipse(m_x, m_y - 18.0, 18.0, 26.0, 0.0, Color::new(0.4, 0.1, 0.5, 0.35));

        // Manthara's bent, stooped figure
        draw_circle(m_x + 6.0, m_y - 32.0, 7.0, Color::new(0.35, 0.28, 0.25, 1.0)); // Head bent forward
        draw_ellipse(m_x - 6.0, m_y - 22.0, 10.0, 14.0, 0.4, Color::new(0.25, 0.18, 0.22, 1.0)); // Hunchback
        draw_rectangle(m_x - 10.0, m_y - 18.0, 18.0, 24.0, Color::new(0.20, 0.14, 0.22, 1.0)); // Dark gown
        // Crooked Walking Stick
        draw_line(m_x + 14.0, m_y - 28.0, m_x + 16.0, m_y + 4.0, 2.5, Color::new(0.45, 0.30, 0.18, 1.0));

        // Floating Title Banner
        draw_pixel_text(font, "MIDNIGHT OVER AYODHYA — THE SEED OF EXILE", 130.0, 26.0, 8, PAL_GOLD_1);

        // Manthara Dialogue Carousel
        let dialogue_slides = [
            (
                "Manthara (Muttering in malice):",
                "Sing and dance, fools of Ayodhya! Celebrate while Queen Kaikeyi sleeps and Prince Bharata is far away in Kekaya!",
            ),
            (
                "Manthara (Clutching her crooked staff):",
                "Tomorrow morning Rama will be crowned Yuvaraja? Never! Not while Manthara draws breath!",
            ),
            (
                "Manthara (Eyes burning with venom):",
                "Long ago upon the celestial battlefield, King Dasharatha granted Queen Kaikeyi two irrevocable boons!",
            ),
            (
                "Manthara (The fatal scheme):",
                "Tonight I enter Kaikeyi's chamber of wrath! Bharata shall have the throne—and Rama shall be cast out for fourteen years into the deadly Dandaka forest!",
            ),
        ];

        let (speaker, line) = dialogue_slides[self.manthara_slide.min(3)];

        // Dialogue Box at bottom
        let box_x = 35.0;
        let box_y = 265.0;
        let box_w = 570.0;
        let box_h = 82.0;

        draw_rectangle(box_x, box_y, box_w, box_h, Color::new(0.08, 0.06, 0.12, 0.96));
        draw_rectangle_lines(box_x, box_y, box_w, box_h, 1.5, Color::new(0.4, 0.8, 0.4, 1.0));

        draw_pixel_text(font, speaker, box_x + 16.0, box_y + 18.0, 8, Color::new(0.4, 0.9, 0.5, 1.0));
        draw_pixel_text(font, line, box_x + 16.0, box_y + 36.0, 7, WHITE);

        let blink = (time * 5.0).sin() > 0.0;
        if blink {
            let prompt = if self.manthara_slide < 3 {
                "▶ PRESS SPACE / CLICK TO CONTINUE ◀"
            } else {
                "▶ PRESS SPACE TO COMPLETE AYODHYA KANDA EVE ◀"
            };
            draw_pixel_text(font, prompt, box_x + 140.0, box_y + 70.0, 7, PAL_GOLD_0);
        }
    }
}

// ============================================================================
// LEVEL 4 VICTORY SCREEN
// ============================================================================
pub fn draw_victory_l4(font: &Font, time: f32) {
    draw_rectangle(0.0, 0.0, SCREEN_W, SCREEN_H, Color::new(0.06, 0.05, 0.10, 0.96));

    let sway = (time * 5.0).sin() * 3.0;
    draw_pixel_text(font, "AYODHYA KANDA: PROCLAMATION COMPLETE!", 80.0, 56.0 + sway, 16, PAL_GOLD_0);

    let sub = "THE CELEBRATION OF YUVARAJA & THE SHADOW OF THE TWO BOONS";
    draw_pixel_text(font, sub, 75.0, 84.0, 8, PAL_TURMERIC_1);

    let rw_x = 55.0;
    let rw_y = 104.0;
    let rw_w = 530.0;
    let rw_h = 168.0;
    draw_rectangle(rw_x, rw_y, rw_w, rw_h, Color::new(0.10, 0.09, 0.16, 0.95));
    draw_rectangle_lines(rw_x, rw_y, rw_w, rw_h, 1.5, PAL_GOLD_1);

    draw_pixel_text(font, "CHAPTER 4 MILESTONES COMPLETED:", rw_x + 16.0, rw_y + 22.0, 9, PAL_GOLD_0);
    draw_pixel_text(font, "✓ Raja Marga Festivities : Ayodhya's avenues washed with sandalwood and flowers!", rw_x + 16.0, rw_y + 44.0, 7, WHITE);
    draw_pixel_text(font, "✓ Royal Council Sabha   : King Dasharatha & Sage Vasishtha decree Rama Yuvaraja!", rw_x + 16.0, rw_y + 66.0, 7, WHITE);
    draw_pixel_text(font, "✓ Sacred Upavasa Vow    : Queen Kausalya & Princess Sita bestow coronation blessings!", rw_x + 16.0, rw_y + 88.0, 7, PAL_GOLD_1);
    draw_pixel_text(font, "✓ The Seed of Exile     : Manthara plots to demand the 14-year exile to Dandaka!", rw_x + 16.0, rw_y + 110.0, 7, Color::new(0.9, 0.4, 0.4, 1.0));
    draw_pixel_text(font, "★ NEXT: THE EXILE DECREE & CHITRAKUTA (LEVEL 5) ★", rw_x + 16.0, rw_y + 138.0, 8, PAL_FIRE_1);

    let blink = (time * 4.0).sin() > 0.0;
    if blink {
        draw_pixel_text(font, "PRESS SPACE OR CLICK TO PROCEED TO FOREST EXILE ▶", 90.0, 298.0, 9, PAL_GOLD_0);
    }
    draw_pixel_text(font, "Press [R] to Replay Chapter 4 (Ayodhya Coronation Eve)", 130.0, 324.0, 8, Color::new(0.7, 0.7, 0.7, 1.0));
}
