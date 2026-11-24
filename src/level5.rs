use macroquad::prelude::*;
use crate::audio::SoundEffects;
use crate::render::{
    draw_dialogue_box, draw_pixel_text, draw_reticle, is_dialogue_prev_button_hovered,
    wrap_pixel_text, COLOR_RAMA_GOLD, PAL_FIRE_1, PAL_GOLD_0, PAL_GOLD_1, PAL_GOLD_2, PAL_GOLD_3,
    PAL_GRASS_0, PAL_GRASS_1, PAL_GRASS_2, PAL_RED_1, PAL_TURMERIC_1, SCREEN_H, SCREEN_W,
};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Level5Phase {
    RiverIntro,
    RiverGathering,
    RiverRaftBuilt,
    RiverCrossing,
    ChitrakutaIntro,
    ChitrakutaGathering,
    ChitrakutaBuilt,
    BharataArrival,
    BharataDialogue,
    PadukaCeremony,
    Complete,
}

pub enum FightResultL5 {
    Victory,
}

struct Particle {
    pos: Vec2,
    vel: Vec2,
    color: Color,
    rot: f32,
    size: f32,
}

pub struct Level5Fight {
    pub phase: Level5Phase,
    pub lakshmana_pos: Vec2,
    pub camera_x: f32,
    pub facing_right: bool,
    pub walk_cycle: f32,
    pub dialogue: Option<(String, Vec<String>, usize)>,

    // River Raft Gathering Items
    pub has_bamboo: bool,
    pub has_timber: bool,
    pub has_reeds: bool,

    // River Crossing Animation
    pub crossing_t: f32,

    // Chitrakuta Parnashala Gathering Items
    pub has_pillars: bool,
    pub has_rafters: bool,
    pub has_thatch: bool,

    // Consecration & Ceremony
    pub ritual_timer: f32,
    pub paduka_dialogue_idx: usize,

    // Dynamic particles (petals, river ripples, holy glow)
    particles: Vec<Particle>,
    pub action_message: Option<(String, f32)>,
}

impl Level5Fight {
    pub fn new() -> Self {
        let mut particles = Vec::with_capacity(50);
        for i in 0..40 {
            particles.push(Particle {
                pos: vec2((i * 31) as f32 % 1200.0, ((i * 19) as f32 % 280.0) + 20.0),
                vel: vec2(rand::gen_range(8.0, 22.0), rand::gen_range(12.0, 28.0)),
                color: if i % 3 == 0 {
                    Color::new(1.0, 0.75, 0.85, 0.75) // Pink lotus
                } else if i % 3 == 1 {
                    Color::new(1.0, 0.90, 0.35, 0.8)  // Saffron Priyala
                } else {
                    Color::new(0.9, 0.95, 1.0, 0.6)   // River spray
                },
                rot: rand::gen_range(0.0, 6.28),
                size: rand::gen_range(2.0, 4.0),
            });
        }

        Self {
            phase: Level5Phase::RiverIntro,
            lakshmana_pos: vec2(200.0, 255.0),
            camera_x: 0.0,
            facing_right: true,
            walk_cycle: 0.0,
            dialogue: Some((
                "Prince Rama".to_string(),
                vec![
                    "Lakshmana, behold the sacred river's swirling waters ahead.".to_string(),
                    "Princess Sita cannot traverse these deep, torrential currents on foot.".to_string(),
                    "Gather dry bamboo stalks, sturdy sal timber, and fibrous reeds from the riverbank.".to_string(),
                    "Let us fashion a buoyant raft (Plava) to bear her safely to the opposite shore!".to_string(),
                ],
                0,
            )),
            has_bamboo: false,
            has_timber: false,
            has_reeds: false,
            crossing_t: 0.0,
            has_pillars: false,
            has_rafters: false,
            has_thatch: false,
            ritual_timer: 0.0,
            paduka_dialogue_idx: 0,
            particles,
            action_message: None,
        }
    }

    pub fn update(
        &mut self,
        dt: f32,
        virtual_mouse: Vec2,
        sounds: &SoundEffects,
    ) -> Option<FightResultL5> {
        // Update ambient particles
        for p in &mut self.particles {
            p.pos.x += p.vel.x * dt;
            p.pos.y += p.vel.y * dt;
            p.rot += 1.2 * dt;

            let max_w = 900.0;
            if p.pos.y > 340.0 {
                p.pos.y = -10.0;
                p.pos.x = (p.pos.x + 120.0) % max_w;
            }
            if p.pos.x > max_w {
                p.pos.x = 0.0;
            }
        }

        // Action banner timer
        if let Some((_, timer)) = &mut self.action_message {
            *timer -= dt;
            if *timer <= 0.0 {
                self.action_message = None;
            }
        }

        // Active Dialogue with Re-Read / Prev support
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
                sounds.play(&sounds.shoot, 0.5);
                *idx += 1;
                if *idx >= lines.len() {
                    self.dialogue = None;
                    match self.phase {
                        Level5Phase::RiverIntro => {
                            self.phase = Level5Phase::RiverGathering;
                            self.action_message = Some((
                                "Task: Explore the riverbank and gather Bamboo, Timber & Reeds!".to_string(),
                                4.0,
                            ));
                        }
                        Level5Phase::RiverRaftBuilt => {
                            self.phase = Level5Phase::RiverCrossing;
                            self.crossing_t = 0.0;
                            sounds.play(&sounds.wind, 0.8);
                        }
                        Level5Phase::ChitrakutaIntro => {
                            self.phase = Level5Phase::ChitrakutaGathering;
                            self.action_message = Some((
                                "Task: Gather Sal Pillars, Bamboo Rafters & Thatch Leaves!".to_string(),
                                4.0,
                            ));
                        }
                        Level5Phase::ChitrakutaBuilt => {
                            self.phase = Level5Phase::BharataArrival;
                            sounds.play(&sounds.conch, 0.9);
                        }
                        Level5Phase::BharataDialogue => {
                            self.phase = Level5Phase::PadukaCeremony;
                            sounds.play(&sounds.bell, 0.8);
                        }
                        _ => {}
                    }
                }
            }
            return None;
        }

        // Phase specific logic
        match self.phase {
            Level5Phase::RiverGathering => {
                self.update_movement(dt, 850.0);

                let lx = self.lakshmana_pos.x;
                // Node 1: Bamboo Grove at x = 90.0
                if !self.has_bamboo && (lx - 90.0).abs() < 42.0 {
                    if is_key_pressed(KeyCode::Space) || is_key_pressed(KeyCode::E) {
                        self.has_bamboo = true;
                        sounds.play(&sounds.snap, 0.8);
                        self.action_message = Some((
                            "✓ Bamboo Stalks harvested! Strong & buoyant for the frame.".to_string(),
                            3.5,
                        ));
                    }
                }

                // Node 2: Sal Timber Logs at x = 480.0
                if !self.has_timber && (lx - 480.0).abs() < 45.0 {
                    if is_key_pressed(KeyCode::Space) || is_key_pressed(KeyCode::E) {
                        self.has_timber = true;
                        sounds.play(&sounds.smash, 0.7);
                        self.action_message = Some((
                            "✓ Sal Timber Logs hewn! Sturdy wood to withstand currents.".to_string(),
                            3.5,
                        ));
                    }
                }

                // Node 3: River Reeds & Vines at x = 750.0
                if !self.has_reeds && (lx - 750.0).abs() < 45.0 {
                    if is_key_pressed(KeyCode::Space) || is_key_pressed(KeyCode::E) {
                        self.has_reeds = true;
                        sounds.play(&sounds.bell, 0.7);
                        self.action_message = Some((
                            "✓ Fibrous Reeds & Vines gathered! Tough cords to lash the raft.".to_string(),
                            3.5,
                        ));
                    }
                }

                // Raft Assembly Site at x = 240.0
                let all_raft_materials = self.has_bamboo && self.has_timber && self.has_reeds;
                if all_raft_materials && (lx - 240.0).abs() < 50.0 {
                    if is_key_pressed(KeyCode::Space) || is_key_pressed(KeyCode::E) {
                        sounds.play(&sounds.snap, 0.9);
                        sounds.play(&sounds.bell, 0.8);
                        self.phase = Level5Phase::RiverRaftBuilt;
                        self.dialogue = Some((
                            "Lakshmana".to_string(),
                            vec![
                                "The raft (Plava) is fashioned, my Lord Rama!".to_string(),
                                "I have lashed the sal logs and bound bamboo outriggers with sacred grass.".to_string(),
                                "For Sita, I have woven a cushioned seat of fresh lotus blossoms and willow boughs.".to_string(),
                                "Step aboard, my revered brother and noble sister; we shall cross in peace!".to_string(),
                            ],
                            0,
                        ));
                    }
                }
            }

            Level5Phase::RiverCrossing => {
                self.crossing_t += dt * 0.28;
                if self.crossing_t >= 1.0 {
                    self.phase = Level5Phase::ChitrakutaIntro;
                    self.lakshmana_pos = vec2(200.0, 255.0);
                    self.camera_x = 0.0;
                    sounds.play(&sounds.bell, 0.8);
                    self.dialogue = Some((
                        "Prince Rama".to_string(),
                        vec![
                            "We have arrived at the blessed slopes of Mount Chitrakuta!".to_string(),
                            "Wild deer graze unafraid, the Mandakini flows clear, and Priyala blossoms bloom.".to_string(),
                            "Lakshmana, construct here an auspicious leaf-hut (Parnashala) for our sanctuary.".to_string(),
                            "Gather upright sal pillars, bamboo rafters, and thatched Kusha grass!".to_string(),
                        ],
                        0,
                    ));
                }
            }

            Level5Phase::ChitrakutaGathering => {
                self.update_movement(dt, 850.0);

                let lx = self.lakshmana_pos.x;
                // Node 1: Sal Pillars at x = 110.0
                if !self.has_pillars && (lx - 110.0).abs() < 42.0 {
                    if is_key_pressed(KeyCode::Space) || is_key_pressed(KeyCode::E) {
                        self.has_pillars = true;
                        sounds.play(&sounds.smash, 0.7);
                        self.action_message = Some((
                            "✓ 4 Sturdy Sal Trunks collected for the dwelling's upright pillars.".to_string(),
                            3.5,
                        ));
                    }
                }

                // Node 2: Bamboo Rafters at x = 500.0
                if !self.has_rafters && (lx - 500.0).abs() < 45.0 {
                    if is_key_pressed(KeyCode::Space) || is_key_pressed(KeyCode::E) {
                        self.has_rafters = true;
                        sounds.play(&sounds.snap, 0.8);
                        self.action_message = Some((
                            "✓ Bamboo Rafters cut for the peaked Vedic roof frame.".to_string(),
                            3.5,
                        ));
                    }
                }

                // Node 3: Kusha Thatch at x = 740.0
                if !self.has_thatch && (lx - 740.0).abs() < 45.0 {
                    if is_key_pressed(KeyCode::Space) || is_key_pressed(KeyCode::E) {
                        self.has_thatch = true;
                        sounds.play(&sounds.bell, 0.7);
                        self.action_message = Some((
                            "✓ Kusha grass and palm fronds gathered to weave waterproof walls.".to_string(),
                            3.5,
                        ));
                    }
                }

                // Hut Building Site at x = 280.0
                let all_hut_materials = self.has_pillars && self.has_rafters && self.has_thatch;
                if all_hut_materials && (lx - 280.0).abs() < 50.0 {
                    if is_key_pressed(KeyCode::Space) || is_key_pressed(KeyCode::E) {
                        sounds.play(&sounds.snap, 0.9);
                        sounds.play(&sounds.victory, 0.8);
                        self.phase = Level5Phase::ChitrakutaBuilt;
                        self.dialogue = Some((
                            "Prince Rama".to_string(),
                            vec![
                                "O Lakshmana, my heart overflows with love and wonder!".to_string(),
                                "With your own bare hands, you have erected a divine hermitage in the wilderness.".to_string(),
                                "Seeing your filial devotion, I feel our father King Dasharatha lives on in you!".to_string(),
                                "Come, let us kindle the sacred sacrificial fire and offer oblations to the Vastu devas.".to_string(),
                            ],
                            0,
                        ));
                    }
                }
            }

            Level5Phase::BharataArrival => {
                self.ritual_timer += dt;
                let advance = is_key_pressed(KeyCode::Space)
                    || is_key_pressed(KeyCode::Enter)
                    || is_mouse_button_pressed(MouseButton::Left);
                if advance {
                    self.phase = Level5Phase::BharataDialogue;
                    self.dialogue = Some((
                        "Prince Bharata".to_string(),
                        vec![
                            "O Brother Rama! Alas, grief has struck down our royal house!".to_string(),
                            "Our father King Dasharatha, broken-hearted from your parting, has left this mortal realm!".to_string(),
                            "Ayodhya is an orphan! The ministers, mothers, and all citizens implore you: RETURN!".to_string(),
                            "Take back the crown of the Ikshvakus that belongs only to you, my elder brother!".to_string(),
                        ],
                        0,
                    ));
                }
            }

            Level5Phase::PadukaCeremony => {
                let is_prev_click = is_mouse_button_pressed(MouseButton::Left)
                    && is_dialogue_prev_button_hovered(virtual_mouse);

                if is_key_pressed(KeyCode::Left) || is_key_pressed(KeyCode::Backspace) || is_prev_click {
                    if self.paduka_dialogue_idx > 0 {
                        self.paduka_dialogue_idx -= 1;
                        sounds.play(&sounds.shoot, 0.4);
                    }
                    return None;
                }

                let advance = is_key_pressed(KeyCode::Space)
                    || is_key_pressed(KeyCode::Enter)
                    || is_mouse_button_pressed(MouseButton::Left);

                if advance {
                    sounds.play(&sounds.bell, 0.7);
                    if self.paduka_dialogue_idx < 3 {
                        self.paduka_dialogue_idx += 1;
                    } else {
                        sounds.play(&sounds.victory, 0.9);
                        self.phase = Level5Phase::Complete;
                    }
                }
            }

            Level5Phase::Complete => {
                let advance = is_key_pressed(KeyCode::Space)
                    || is_key_pressed(KeyCode::Enter)
                    || is_mouse_button_pressed(MouseButton::Left);
                if advance {
                    return Some(FightResultL5::Victory);
                }
            }

            _ => {}
        }

        None
    }

    fn update_movement(&mut self, dt: f32, max_x: f32) {
        let mut moving = false;
        let speed = 120.0;

        if is_key_down(KeyCode::Left) || is_key_down(KeyCode::A) {
            self.lakshmana_pos.x = (self.lakshmana_pos.x - speed * dt).max(40.0);
            self.facing_right = false;
            moving = true;
        }
        if is_key_down(KeyCode::Right) || is_key_down(KeyCode::D) {
            self.lakshmana_pos.x = (self.lakshmana_pos.x + speed * dt).min(max_x);
            self.facing_right = true;
            moving = true;
        }

        if moving {
            self.walk_cycle += dt * 10.0;
        } else {
            self.walk_cycle = 0.0;
        }

        // Camera follow
        let target_cam = self.lakshmana_pos.x - SCREEN_W * 0.45;
        self.camera_x = self.camera_x + (target_cam.clamp(0.0, max_x - SCREEN_W) - self.camera_x) * dt * 5.0;
    }

    pub fn draw(&self, font: &Font, virtual_mouse: Vec2, time: f32) {
        match self.phase {
            Level5Phase::RiverIntro
            | Level5Phase::RiverGathering
            | Level5Phase::RiverRaftBuilt
            | Level5Phase::RiverCrossing => {
                self.draw_river_scene(font, time);
            }
            Level5Phase::ChitrakutaIntro
            | Level5Phase::ChitrakutaGathering
            | Level5Phase::ChitrakutaBuilt
            | Level5Phase::BharataArrival
            | Level5Phase::BharataDialogue
            | Level5Phase::PadukaCeremony => {
                self.draw_chitrakuta_scene(font, virtual_mouse, time);
            }
            Level5Phase::Complete => {
                draw_victory_l5(font, time);
            }
        }

        // Action banner prompt
        if let Some((msg, _)) = &self.action_message {
            let bw = 460.0;
            let bx = (SCREEN_W - bw) * 0.5;
            draw_rectangle(bx, 14.0, bw, 22.0, Color::new(0.08, 0.07, 0.12, 0.92));
            draw_rectangle_lines(bx, 14.0, bw, 22.0, 1.2, PAL_GOLD_1);
            draw_pixel_text(font, msg, bx + 12.0, 29.0, 7, PAL_GOLD_0);
        }

        // Active Dialogue
        if let Some((speaker, lines, idx)) = &self.dialogue {
            if *idx < lines.len() {
                draw_dialogue_box(font, speaker, &lines[*idx], time, *idx, lines.len(), virtual_mouse);
            }
        }

        draw_reticle(virtual_mouse, false);
    }

    // ========================================================================
    // SCENE A: SACRED RIVER RAFT CRAFTING (GANGA / YAMUNA)
    // ========================================================================
    fn draw_river_scene(&self, font: &Font, time: f32) {
        // Dawn River Sky (Soft pink, gold, and turquoise)
        clear_background(Color::new(0.48, 0.72, 0.88, 1.0));

        let cx = self.camera_x;

        // Distant forest canopy on far shore
        for i in 0..14 {
            let fx = i as f32 * 80.0 - cx * 0.2;
            draw_circle(fx, 130.0, 50.0, Color::new(0.18, 0.38, 0.24, 0.85));
            draw_circle(fx + 25.0, 125.0, 42.0, Color::new(0.24, 0.46, 0.28, 0.9));
        }

        // Shimmering Sacred River Waters
        let river_y = 150.0;
        let river_h = 95.0;
        draw_rectangle(0.0, river_y, SCREEN_W, river_h, Color::new(0.20, 0.52, 0.72, 1.0));

        // Water Ripple Bands
        for i in 0..6 {
            let wy = river_y + 12.0 + i as f32 * 14.0;
            let offset = (time * 1.8 + i as f32 * 1.5).sin() * 8.0;
            draw_line(0.0, wy, SCREEN_W, wy, 1.5, Color::new(0.35, 0.68, 0.88, 0.7));
            for j in 0..8 {
                let rx = j as f32 * 90.0 + offset;
                draw_line(rx, wy - 2.0, rx + 24.0, wy - 2.0, 1.2, Color::new(0.85, 0.95, 1.0, 0.8));
            }
        }

        // Riverbank Shore (Sandy grassy foreground)
        let shore_y = 235.0;
        draw_rectangle(0.0, shore_y, SCREEN_W, SCREEN_H - shore_y, Color::new(0.32, 0.58, 0.28, 1.0));
        // River sand edge
        draw_rectangle(0.0, shore_y - 8.0, SCREEN_W, 10.0, Color::new(0.78, 0.68, 0.44, 1.0));
        draw_line(0.0, shore_y - 8.0, SCREEN_W, shore_y - 8.0, 1.5, Color::new(0.55, 0.48, 0.30, 1.0));

        // Render ambient particles (river spray & lotus petals)
        for p in &self.particles {
            let px = p.pos.x - cx;
            if px >= -20.0 && px <= SCREEN_W + 20.0 {
                draw_circle(px, p.pos.y, p.size, p.color);
            }
        }

        // If crossing animation active
        if self.phase == Level5Phase::RiverCrossing {
            let rx = 100.0 + self.crossing_t * (SCREEN_W - 180.0);
            let ry = river_y + 25.0 + (self.crossing_t * 6.28 * 2.0).sin() * 4.0;
            self.draw_raft(rx, ry, time);

            // Crossing status text
            draw_rectangle(SCREEN_W * 0.5 - 130.0, 310.0, 260.0, 22.0, Color::new(0.08, 0.07, 0.12, 0.9));
            draw_rectangle_lines(SCREEN_W * 0.5 - 130.0, 310.0, 260.0, 22.0, 1.2, PAL_GOLD_1);
            draw_pixel_text(font, "CROSSING THE SACRED WATERS TO CHITRAKUTA...", SCREEN_W * 0.5 - 120.0, 325.0, 7, PAL_GOLD_0);
            return;
        }

        // Resource Nodes on Riverbank
        // Node 1: Bamboo Grove at x = 90.0
        let b_screen_x = 90.0 - cx;
        self.draw_bamboo_cluster(b_screen_x, shore_y - 35.0);
        if !self.has_bamboo {
            let dist = (self.lakshmana_pos.x - 90.0).abs();
            if dist < 45.0 {
                let blink = (time * 6.0).sin() > 0.0;
                let col = if blink { PAL_GOLD_0 } else { WHITE };
                draw_pixel_text(font, "[SPACE / E] HARVEST BAMBOO", b_screen_x - 35.0, shore_y - 45.0, 7, col);
            } else {
                draw_pixel_text(font, "BAMBOO GROVE [1/3]", b_screen_x - 25.0, shore_y - 42.0, 6, PAL_GOLD_2);
            }
        } else {
            draw_pixel_text(font, "✓ BAMBOO HARVESTED", b_screen_x - 25.0, shore_y - 42.0, 6, Color::new(0.5, 0.9, 0.5, 0.9));
        }

        // Node 2: Sal Timber Logs at x = 480.0
        let t_screen_x = 480.0 - cx;
        self.draw_timber_logs(t_screen_x, shore_y - 8.0);
        if !self.has_timber {
            let dist = (self.lakshmana_pos.x - 480.0).abs();
            if dist < 45.0 {
                let blink = (time * 6.0).sin() > 0.0;
                let col = if blink { PAL_GOLD_0 } else { WHITE };
                draw_pixel_text(font, "[SPACE / E] HEW SAL TIMBER", t_screen_x - 30.0, shore_y - 30.0, 7, col);
            } else {
                draw_pixel_text(font, "SAL TIMBER LOGS [2/3]", t_screen_x - 30.0, shore_y - 28.0, 6, PAL_GOLD_2);
            }
        } else {
            draw_pixel_text(font, "✓ TIMBER HEWN", t_screen_x - 20.0, shore_y - 28.0, 6, Color::new(0.5, 0.9, 0.5, 0.9));
        }

        // Node 3: River Reeds & Vines at x = 750.0
        let r_screen_x = 750.0 - cx;
        self.draw_reeds_cluster(r_screen_x, shore_y - 15.0);
        if !self.has_reeds {
            let dist = (self.lakshmana_pos.x - 750.0).abs();
            if dist < 45.0 {
                let blink = (time * 6.0).sin() > 0.0;
                let col = if blink { PAL_GOLD_0 } else { WHITE };
                draw_pixel_text(font, "[SPACE / E] GATHER REEDS", r_screen_x - 30.0, shore_y - 32.0, 7, col);
            } else {
                draw_pixel_text(font, "RIVER REEDS [3/3]", r_screen_x - 25.0, shore_y - 30.0, 6, PAL_GOLD_2);
            }
        } else {
            draw_pixel_text(font, "✓ REEDS GATHERED", r_screen_x - 25.0, shore_y - 30.0, 6, Color::new(0.5, 0.9, 0.5, 0.9));
        }

        // Raft Building Site at x = 240.0
        let site_screen_x = 240.0 - cx;
        let all_gathered = self.has_bamboo && self.has_timber && self.has_reeds;
        if self.phase == Level5Phase::RiverRaftBuilt {
            self.draw_raft(site_screen_x, river_y + 40.0, time);
        } else {
            // Outlined placement marker
            let marker_col = if all_gathered {
                let glow = ((time * 6.0).sin() * 0.3 + 0.7).clamp(0.0, 1.0);
                Color::new(PAL_GOLD_0.r * glow, PAL_GOLD_0.g * glow, PAL_GOLD_0.b * glow, 1.0)
            } else {
                Color::new(0.5, 0.5, 0.6, 0.5)
            };
            draw_rectangle_lines(site_screen_x - 35.0, river_y + 55.0, 70.0, 24.0, 1.5, marker_col);

            let dist = (self.lakshmana_pos.x - 240.0).abs();
            if all_gathered {
                if dist < 50.0 {
                    draw_pixel_text(font, "[SPACE / E] ASSEMBLE RAFT!", site_screen_x - 42.0, river_y + 45.0, 7, PAL_GOLD_0);
                } else {
                    draw_pixel_text(font, "RETURN TO ASSEMBLE RAFT ▶", site_screen_x - 45.0, river_y + 45.0, 7, PAL_TURMERIC_1);
                }
            } else {
                draw_pixel_text(font, "RAFT BUILDING SITE", site_screen_x - 32.0, river_y + 45.0, 6, Color::new(0.7, 0.7, 0.7, 0.8));
            }
        }

        // Rama and Sita standing near the site
        let rama_sx = 190.0 - cx;
        let sita_sx = 160.0 - cx;
        self.draw_rama_forest(rama_sx, shore_y + 10.0, time);
        self.draw_sita_forest(sita_sx, shore_y + 12.0, time);

        // Player: Lakshmana
        let lak_sx = self.lakshmana_pos.x - cx;
        self.draw_lakshmana(lak_sx, shore_y + 15.0, self.facing_right, self.walk_cycle, time);

        // Top Objective Tracker HUD
        self.draw_hud_quest(
            font,
            "TASK 1: CRAFTING THE SACRED RIVER RAFT",
            &[
                ("Bamboo Poles", self.has_bamboo),
                ("Sal Timber", self.has_timber),
                ("River Reeds", self.has_reeds),
            ],
        );
    }

    // ========================================================================
    // SCENE B: SACRED CHITRAKUTA PARNASHALA & BHARATA'S PADUKAS
    // ========================================================================
    fn draw_chitrakuta_scene(&self, font: &Font, virtual_mouse: Vec2, time: f32) {
        // Serene mountain sky (Golden ochre dawn with sacred mountain ridges)
        clear_background(Color::new(0.55, 0.75, 0.85, 1.0));

        let cx = self.camera_x;

        // Distant Mount Chitrakuta peaks
        for i in 0..7 {
            let px = i as f32 * 140.0 - cx * 0.15;
            draw_triangle(
                vec2(px - 40.0, 180.0),
                vec2(px + 120.0, 180.0),
                vec2(px + 40.0, 45.0),
                Color::new(0.35, 0.45, 0.55, 0.8),
            );
            // Highlight slope
            draw_triangle(
                vec2(px, 180.0),
                vec2(px + 120.0, 180.0),
                vec2(px + 40.0, 45.0),
                Color::new(0.42, 0.52, 0.62, 0.8),
            );
        }

        // Flowering Priyala, Kadamba & Ashoka forest midground
        for i in 0..12 {
            let fx = i as f32 * 85.0 - cx * 0.3;
            // Tree trunk
            draw_rectangle(fx + 22.0, 150.0, 8.0, 35.0, Color::new(0.35, 0.22, 0.15, 1.0));
            // Blossoming canopy (Yellow & Green)
            let leaf_col = if i % 2 == 0 {
                Color::new(0.85, 0.65, 0.20, 0.95) // Saffron flowering
            } else {
                Color::new(0.22, 0.52, 0.28, 0.95) // Deep green
            };
            draw_circle(fx + 26.0, 140.0, 30.0, leaf_col);
            draw_circle(fx + 18.0, 135.0, 22.0, Color::new(0.95, 0.75, 0.25, 0.9));
        }

        // Forest plateau ground
        let ground_y = 185.0;
        draw_rectangle(0.0, ground_y, SCREEN_W, SCREEN_H - ground_y, PAL_GRASS_1);
        draw_rectangle(0.0, ground_y + 40.0, SCREEN_W, SCREEN_H - ground_y - 40.0, PAL_GRASS_2);
        draw_line(0.0, ground_y, SCREEN_W, ground_y, 2.0, PAL_GRASS_0);

        // Mandakini River stream running in foreground
        let stream_y = 295.0;
        draw_rectangle(0.0, stream_y, SCREEN_W, SCREEN_H - stream_y, Color::new(0.22, 0.55, 0.72, 0.95));
        for i in 0..8 {
            let sx = i as f32 * 85.0 + (time * 2.0).sin() * 6.0;
            draw_line(sx, stream_y + 10.0, sx + 22.0, stream_y + 10.0, 1.5, Color::new(0.8, 0.95, 1.0, 0.7));
            // Lotus blossoms
            draw_circle(sx + 35.0, stream_y + 14.0, 3.5, Color::new(1.0, 0.7, 0.85, 0.9));
        }

        // Ambient petals & mountain glow
        for p in &self.particles {
            let px = p.pos.x - cx;
            if px >= -20.0 && px <= SCREEN_W + 20.0 {
                draw_circle(px, p.pos.y, p.size, p.color);
            }
        }

        let hut_site_x = 280.0 - cx;
        let is_hut_built = self.phase != Level5Phase::ChitrakutaGathering && self.phase != Level5Phase::ChitrakutaIntro;

        // Draw Parnashala if built, or foundation site
        if is_hut_built {
            self.draw_parnashala(hut_site_x, ground_y + 20.0, time);
        } else {
            // Foundation marker
            let all_gathered = self.has_pillars && self.has_rafters && self.has_thatch;
            let marker_col = if all_gathered {
                let glow = ((time * 6.0).sin() * 0.3 + 0.7).clamp(0.0, 1.0);
                Color::new(PAL_GOLD_0.r * glow, PAL_GOLD_0.g * glow, PAL_GOLD_0.b * glow, 1.0)
            } else {
                Color::new(0.5, 0.5, 0.6, 0.5)
            };
            draw_rectangle_lines(hut_site_x - 45.0, ground_y + 15.0, 90.0, 45.0, 1.5, marker_col);

            let dist = (self.lakshmana_pos.x - 280.0).abs();
            if all_gathered {
                if dist < 50.0 {
                    draw_pixel_text(font, "[SPACE / E] ERECT PARNASHALA!", hut_site_x - 52.0, ground_y + 5.0, 7, PAL_GOLD_0);
                } else {
                    draw_pixel_text(font, "RETURN TO BUILD PARNASHALA ▶", site_screen_label_x(hut_site_x), ground_y + 5.0, 7, PAL_TURMERIC_1);
                }
            } else {
                draw_pixel_text(font, "PARNASHALA SITE", hut_site_x - 30.0, ground_y + 5.0, 6, Color::new(0.7, 0.7, 0.7, 0.8));
            }
        }

        // Parnashala Gathering Nodes (if in gathering phase)
        if self.phase == Level5Phase::ChitrakutaGathering {
            // Node 1: Sal Pillars at x = 110.0
            let p_sx = 110.0 - cx;
            self.draw_sal_pillars(p_sx, ground_y + 20.0);
            if !self.has_pillars {
                let dist = (self.lakshmana_pos.x - 110.0).abs();
                if dist < 42.0 {
                    draw_pixel_text(font, "[SPACE / E] GATHER SAL PILLARS", p_sx - 40.0, ground_y + 8.0, 7, PAL_GOLD_0);
                } else {
                    draw_pixel_text(font, "SAL PILLARS [1/3]", p_sx - 25.0, ground_y + 10.0, 6, PAL_GOLD_2);
                }
            } else {
                draw_pixel_text(font, "✓ PILLARS COLLECTED", p_sx - 30.0, ground_y + 10.0, 6, Color::new(0.5, 0.9, 0.5, 0.9));
            }

            // Node 2: Bamboo Rafters at x = 500.0
            let b_sx = 500.0 - cx;
            self.draw_bamboo_cluster(b_sx, ground_y + 10.0);
            if !self.has_rafters {
                let dist = (self.lakshmana_pos.x - 500.0).abs();
                if dist < 45.0 {
                    draw_pixel_text(font, "[SPACE / E] CUT ROOF RAFTERS", b_sx - 38.0, ground_y + 5.0, 7, PAL_GOLD_0);
                } else {
                    draw_pixel_text(font, "BAMBOO RAFTERS [2/3]", b_sx - 32.0, ground_y + 8.0, 6, PAL_GOLD_2);
                }
            } else {
                draw_pixel_text(font, "✓ RAFTERS COLLECTED", b_sx - 30.0, ground_y + 8.0, 6, Color::new(0.5, 0.9, 0.5, 0.9));
            }

            // Node 3: Kusha Thatch at x = 740.0
            let k_sx = 740.0 - cx;
            self.draw_reeds_cluster(k_sx, ground_y + 20.0);
            if !self.has_thatch {
                let dist = (self.lakshmana_pos.x - 740.0).abs();
                if dist < 45.0 {
                    draw_pixel_text(font, "[SPACE / E] HARVEST KUSHA THATCH", k_sx - 44.0, ground_y + 8.0, 7, PAL_GOLD_0);
                } else {
                    draw_pixel_text(font, "KUSHA THATCH [3/3]", k_sx - 28.0, ground_y + 10.0, 6, PAL_GOLD_2);
                }
            } else {
                draw_pixel_text(font, "✓ THATCH HARVESTED", k_sx - 28.0, ground_y + 10.0, 6, Color::new(0.5, 0.9, 0.5, 0.9));
            }

            // Top Quest HUD
            self.draw_hud_quest(
                font,
                "TASK 2: ERECTING THE SACRED PARNASHALA",
                &[
                    ("Sal Pillars", self.has_pillars),
                    ("Bamboo Rafters", self.has_rafters),
                    ("Kusha Thatch", self.has_thatch),
                ],
            );
        }

        // Characters present at Chitrakuta
        let rama_x = hut_site_x - 45.0;
        let sita_x = hut_site_x - 75.0;
        self.draw_rama_forest(rama_x, ground_y + 55.0, time);
        self.draw_sita_forest(sita_x, ground_y + 57.0, time);

        // Lakshmana
        let lak_sx = self.lakshmana_pos.x - cx;
        let show_lak = self.phase == Level5Phase::ChitrakutaGathering;
        if show_lak {
            self.draw_lakshmana(lak_sx, ground_y + 60.0, self.facing_right, self.walk_cycle, time);
        } else {
            // Stationed beside Rama
            self.draw_lakshmana(hut_site_x + 55.0, ground_y + 58.0, false, 0.0, time);
        }

        // Bharata's Arrival & Paduka Ceremony Phases
        let is_bharata_here = self.phase == Level5Phase::BharataArrival
            || self.phase == Level5Phase::BharataDialogue
            || self.phase == Level5Phase::PadukaCeremony;

        if is_bharata_here {
            let bharata_x = hut_site_x - 130.0;
            let sumantra_x = hut_site_x - 170.0;
            self.draw_sumantra_sad(sumantra_x, ground_y + 54.0, time);
            self.draw_bharata(bharata_x, ground_y + 58.0, self.phase == Level5Phase::PadukaCeremony, time);

            if self.phase == Level5Phase::BharataArrival {
                let blink = (time * 5.0).sin() > 0.0;
                let col = if blink { PAL_GOLD_0 } else { PAL_GOLD_1 };
                draw_pixel_text(font, "PRINCE BHARATA HAS ARRIVED IN VALKALA!", hut_site_x - 145.0, ground_y - 25.0, 8, col);
                draw_pixel_text(font, "[PRESS SPACE OR CLICK TO SPEAK ▶]", hut_site_x - 125.0, ground_y - 12.0, 7, WHITE);
            } else if self.phase == Level5Phase::PadukaCeremony {
                self.draw_paduka_ceremony_ui(font, virtual_mouse, time);
            }
        }
    }

    // Paduka Presentation Dialogue Box & Slide Viewer
    fn draw_paduka_ceremony_ui(&self, font: &Font, virtual_mouse: Vec2, time: f32) {
        let lines = [
            (
                "Prince Rama",
                "Bharata, father's vow is sacred and immutable across all the worlds. He pledged two boons to mother Kaikeyi in honor of Dharma. I cannot return until the fourteen years are fulfilled. Govern Ayodhya with righteousness, mercy, and steadfast devotion to our people.",
            ),
            (
                "Prince Bharata",
                "Then hear my vow, O brother! I will never wear the crown of the Solar dynasty. Step out of your golden-trimmed wooden sandals (Padukas) and bestow them upon me! They alone shall preside upon the golden Simhasana of Ayodhya!",
            ),
            (
                "Prince Bharata",
                "I shall dwell outside the capital at Nandigrama in an ascetic's hut, subsisting on roots and clad in bark. I shall rule only as the servant and trustee of your holy Padukas! If you do not return the exact day your 14 years end, I shall cast myself into the blazing pyre!",
            ),
            (
                "Prince Rama",
                "Take my holy Padukas, noble Bharata! May they protect Ayodhya and sustain Dharma. Lead our subjects in harmony. We shall reunite in joy when the term of exile concludes. Go in peace, dear brother. Ayodhya is blessed to have a guardian of your purity.",
            ),
        ];

        let cur = self.paduka_dialogue_idx.min(3);
        let (speaker, content) = lines[cur];

        let box_x = 30.0;
        let box_y = SCREEN_H - 108.0;
        let box_w = SCREEN_W - 60.0;
        let box_h = 96.0;

        draw_rectangle(box_x, box_y, box_w, box_h, Color::new(0.08, 0.08, 0.14, 0.96));
        draw_rectangle_lines(box_x, box_y, box_w, box_h, 2.0, PAL_GOLD_1);
        draw_rectangle_lines(box_x + 2.0, box_y + 2.0, box_w - 4.0, box_h - 4.0, 1.0, Color::new(0.6, 0.5, 0.2, 0.8));

        // Nameplate
        draw_rectangle(box_x + 12.0, box_y - 10.0, 185.0, 20.0, Color::new(0.18, 0.12, 0.08, 1.0));
        draw_rectangle_lines(box_x + 12.0, box_y - 10.0, 185.0, 20.0, 1.5, PAL_GOLD_0);
        draw_pixel_text(font, speaker, box_x + 18.0, box_y + 4.0, 9, PAL_GOLD_0);

        // Word wrapped text guaranteed to fit within box
        let max_text_w = box_w - 36.0;
        let wrapped = wrap_pixel_text(font, content, 7, max_text_w);
        let mut ty = box_y + 22.0;
        for (i, wline) in wrapped.iter().enumerate().take(4) {
            let col = if i == wrapped.len() - 1 && i > 1 { PAL_TURMERIC_1 } else { WHITE };
            draw_pixel_text(font, wline, box_x + 18.0, ty, 7, col);
            ty += 12.5;
        }

        // Prev Button
        if cur > 0 {
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

        // Page Counter
        let page_tag = format!("{} / 4", cur + 1);
        draw_pixel_text(font, &page_tag, box_x + box_w * 0.5 - 14.0, box_y + box_h - 10.0, 7, PAL_GOLD_2);

        // Next Prompt
        let blink = (time * 5.0).sin() > 0.0;
        let col = if blink { COLOR_RAMA_GOLD } else { PAL_GOLD_2 };
        let next_txt = if cur < 3 { "[CLICK / SPACE ▶]" } else { "COMPLETE CHAPTER 5 ★" };
        draw_pixel_text(font, next_txt, box_x + box_w - 150.0, box_y + box_h - 10.0, 8, col);
    }

    // Top HUD Quest Tracker
    fn draw_hud_quest(&self, font: &Font, title: &str, items: &[(&str, bool)]) {
        let hx = 20.0;
        let hy = 16.0;
        let hw = 300.0;
        let hh = 18.0 + items.len() as f32 * 14.0;

        draw_rectangle(hx, hy, hw, hh, Color::new(0.08, 0.07, 0.12, 0.85));
        draw_rectangle_lines(hx, hy, hw, hh, 1.0, PAL_GOLD_2);

        draw_pixel_text(font, title, hx + 10.0, hy + 13.0, 7, PAL_GOLD_0);

        for (i, (name, done)) in items.iter().enumerate() {
            let iy = hy + 26.0 + i as f32 * 14.0;
            let check = if *done { "✓" } else { "○" };
            let col = if *done {
                Color::new(0.4, 0.9, 0.4, 1.0)
            } else {
                Color::new(0.8, 0.8, 0.8, 0.8)
            };
            let text = format!("{} {}", check, name);
            draw_pixel_text(font, &text, hx + 12.0, iy, 7, col);
        }
    }

    // ========================================================================
    // PIXEL ART DRAWING HELPERS
    // ========================================================================
    fn draw_bamboo_cluster(&self, x: f32, y: f32) {
        for i in 0..5 {
            let bx = x + i as f32 * 7.0;
            let bh = 36.0 + (i as f32 * 5.0).sin() * 6.0;
            draw_rectangle(bx, y - bh, 4.0, bh, Color::new(0.52, 0.72, 0.28, 1.0));
            // Nodes
            for n in 1..4 {
                let ny = y - (n as f32 * (bh / 4.0));
                draw_line(bx - 1.0, ny, bx + 5.0, ny, 1.2, Color::new(0.35, 0.50, 0.18, 1.0));
            }
        }
    }

    fn draw_timber_logs(&self, x: f32, y: f32) {
        // Stack of 3 sturdy wooden Sal logs
        draw_rectangle(x - 18.0, y - 8.0, 36.0, 8.0, Color::new(0.45, 0.28, 0.16, 1.0));
        draw_circle(x - 18.0, y - 4.0, 4.0, Color::new(0.65, 0.45, 0.25, 1.0));
        draw_rectangle(x - 14.0, y - 16.0, 32.0, 8.0, Color::new(0.50, 0.32, 0.18, 1.0));
        draw_circle(x - 14.0, y - 12.0, 4.0, Color::new(0.70, 0.50, 0.28, 1.0));
    }

    fn draw_reeds_cluster(&self, x: f32, y: f32) {
        for i in 0..7 {
            let rx = x + i as f32 * 6.0;
            let rh = 28.0 + (i as f32 * 4.0).cos() * 5.0;
            draw_line(rx, y, rx + 2.0, y - rh, 1.8, Color::new(0.38, 0.58, 0.25, 1.0));
            // Reed cattail head
            draw_rectangle(rx + 1.0, y - rh, 2.5, 7.0, Color::new(0.45, 0.25, 0.12, 1.0));
        }
    }

    fn draw_sal_pillars(&self, x: f32, y: f32) {
        for i in 0..4 {
            let px = x + i as f32 * 10.0;
            draw_rectangle(px, y - 32.0, 6.0, 32.0, Color::new(0.48, 0.30, 0.18, 1.0));
            draw_line(px, y - 32.0, px + 6.0, y - 32.0, 1.5, PAL_GOLD_2);
        }
    }

    fn draw_raft(&self, x: f32, y: f32, time: f32) {
        let bob = (time * 3.0).sin() * 2.0;
        let ry = y + bob;

        // Timber deck
        draw_rectangle(x - 38.0, ry - 6.0, 76.0, 12.0, Color::new(0.55, 0.35, 0.20, 1.0));
        draw_rectangle_lines(x - 38.0, ry - 6.0, 76.0, 12.0, 1.2, Color::new(0.35, 0.20, 0.10, 1.0));

        // Bamboo outriggers
        draw_rectangle(x - 42.0, ry + 4.0, 84.0, 4.0, Color::new(0.65, 0.80, 0.35, 1.0));

        // Sita's cushioned lotus seat in center
        draw_circle(x, ry - 7.0, 10.0, Color::new(1.0, 0.8, 0.9, 1.0));
        draw_circle(x, ry - 7.0, 6.0, Color::new(1.0, 0.95, 0.5, 1.0));

        // Sita seated
        self.draw_sita_forest(x, ry - 6.0, time);

        // Rama standing holding guiding oar
        self.draw_rama_forest(x - 22.0, ry - 4.0, time);
        // Oar
        draw_line(x - 26.0, ry - 18.0, x - 32.0, ry + 16.0, 2.0, Color::new(0.4, 0.25, 0.15, 1.0));

        // Lakshmana standing with rowing pole
        self.draw_lakshmana(x + 22.0, ry - 4.0, false, 0.0, time);
        draw_line(x + 24.0, ry - 18.0, x + 30.0, ry + 16.0, 2.0, Color::new(0.4, 0.25, 0.15, 1.0));
    }

    fn draw_parnashala(&self, x: f32, y: f32, time: f32) {
        // Wooden cottage foundation terrace
        draw_rectangle(x - 55.0, y - 6.0, 110.0, 8.0, Color::new(0.48, 0.32, 0.18, 1.0));

        // 4 Sturdy Sal Pillars
        draw_rectangle(x - 48.0, y - 48.0, 6.0, 42.0, Color::new(0.40, 0.24, 0.12, 1.0));
        draw_rectangle(x - 20.0, y - 48.0, 5.0, 42.0, Color::new(0.40, 0.24, 0.12, 1.0));
        draw_rectangle(x + 15.0, y - 48.0, 5.0, 42.0, Color::new(0.40, 0.24, 0.12, 1.0));
        draw_rectangle(x + 42.0, y - 48.0, 6.0, 42.0, Color::new(0.40, 0.24, 0.12, 1.0));

        // Thatched woven walls
        draw_rectangle(x - 44.0, y - 45.0, 88.0, 39.0, Color::new(0.72, 0.58, 0.32, 0.95));
        // Wall woven texture
        for i in 0..5 {
            let wy = y - 40.0 + i as f32 * 7.0;
            draw_line(x - 44.0, wy, x + 44.0, wy, 1.0, Color::new(0.55, 0.42, 0.22, 0.8));
        }

        // Open Doorway
        draw_rectangle(x - 12.0, y - 36.0, 24.0, 30.0, Color::new(0.18, 0.12, 0.08, 1.0));
        draw_rectangle_lines(x - 12.0, y - 36.0, 24.0, 30.0, 1.5, PAL_GOLD_2);

        // Peaked Vedic Thatched Roof (Golden Kusha Thatch)
        draw_triangle(
            vec2(x - 62.0, y - 46.0),
            vec2(x + 62.0, y - 46.0),
            vec2(x, y - 78.0),
            Color::new(0.82, 0.65, 0.25, 1.0),
        );
        // Roof ridge lines
        draw_line(x - 62.0, y - 46.0, x, y - 78.0, 2.5, PAL_GOLD_1);
        draw_line(x + 62.0, y - 46.0, x, y - 78.0, 2.5, PAL_GOLD_1);
        draw_line(x - 62.0, y - 46.0, x + 62.0, y - 46.0, 2.0, Color::new(0.60, 0.45, 0.18, 1.0));

        // Sacred Golden Pinnacle Kalasha on top of roof
        draw_circle(x, y - 82.0, 4.0, PAL_GOLD_0);
        draw_triangle(vec2(x - 2.5, y - 82.0), vec2(x + 2.5, y - 82.0), vec2(x, y - 90.0), PAL_GOLD_0);

        // Sacred Agni altar in front of veranda
        let ax = x + 35.0;
        let ay = y - 2.0;
        draw_rectangle(ax - 8.0, ay - 6.0, 16.0, 6.0, Color::new(0.65, 0.32, 0.18, 1.0));
        // Flickering fire
        let f_bob = (time * 12.0).sin() * 2.0;
        draw_triangle(vec2(ax - 5.0, ay - 6.0), vec2(ax + 5.0, ay - 6.0), vec2(ax, ay - 16.0 + f_bob), PAL_FIRE_1);
        draw_circle(ax, ay - 8.0, 3.5, PAL_GOLD_0);

        // Garland of wild flowers along eaves
        for i in 0..9 {
            let gx = x - 52.0 + i as f32 * 13.0;
            let col = if i % 2 == 0 { PAL_RED_1 } else { PAL_GOLD_0 };
            draw_circle(gx, y - 44.0, 2.5, col);
        }
    }

    fn draw_lakshmana(&self, x: f32, y: f32, facing_right: bool, walk: f32, _time: f32) {
        let leg_swing = walk.sin() * 4.0;
        let dir = if facing_right { 1.0 } else { -1.0 };

        // Legs in forest valkala loincloth
        draw_line(x - 3.0, y - 10.0, x - 3.0 - leg_swing * dir, y, 2.5, Color::new(0.85, 0.55, 0.20, 1.0));
        draw_line(x + 3.0, y - 10.0, x + 3.0 + leg_swing * dir, y, 2.5, Color::new(0.85, 0.55, 0.20, 1.0));

        // Torso in ascetic tree-bark robe (Valkala)
        draw_rectangle(x - 6.0, y - 26.0, 12.0, 16.0, Color::new(0.85, 0.55, 0.20, 1.0));
        // Saffron cord sash across shoulder
        draw_line(x - 5.0, y - 26.0, x + 5.0, y - 12.0, 1.5, PAL_GOLD_0);

        // Golden complexion head
        draw_circle(x, y - 31.0, 5.5, Color::new(0.96, 0.78, 0.58, 1.0));
        // Ascetic Jata topknot
        draw_circle(x - 1.0 * dir, y - 37.0, 3.5, Color::new(0.2, 0.15, 0.1, 1.0));

        // Quiver on back with arrows
        draw_rectangle(x - 7.0 * dir, y - 28.0, 3.0, 14.0, Color::new(0.4, 0.25, 0.15, 1.0));
        draw_line(x - 7.0 * dir, y - 32.0, x - 6.0 * dir, y - 28.0, 1.5, WHITE);

        // Kodanda Bow in hand
        let bx = x + 7.0 * dir;
        draw_line(bx, y - 32.0, bx, y - 8.0, 1.5, PAL_GOLD_2);
        draw_line(bx, y - 32.0, bx - 2.0 * dir, y - 20.0, 1.0, Color::new(0.5, 0.35, 0.2, 1.0));
        draw_line(bx, y - 8.0, bx - 2.0 * dir, y - 20.0, 1.0, Color::new(0.5, 0.35, 0.2, 1.0));
    }

    fn draw_rama_forest(&self, x: f32, y: f32, time: f32) {
        // Divine golden halo (Tejas)
        let halo_r = 11.0 + (time * 4.0).sin() * 1.5;
        draw_circle(x, y - 32.0, halo_r, Color::new(1.0, 0.85, 0.3, 0.25));

        // Saffron Valkala robe
        draw_rectangle(x - 6.0, y - 28.0, 12.0, 18.0, Color::new(0.88, 0.50, 0.18, 1.0));
        draw_line(x - 5.0, y - 28.0, x + 5.0, y - 12.0, 1.5, PAL_GOLD_0);
        // Feet
        draw_rectangle(x - 5.0, y - 10.0, 4.0, 10.0, Color::new(0.35, 0.65, 0.88, 1.0));
        draw_rectangle(x + 1.0, y - 10.0, 4.0, 10.0, Color::new(0.35, 0.65, 0.88, 1.0));

        // Dark sky-blue complexion
        draw_circle(x, y - 33.0, 6.0, Color::new(0.35, 0.65, 0.88, 1.0));
        // Ascetic jata hair with Kusha grass tie
        draw_circle(x, y - 39.0, 4.0, Color::new(0.12, 0.10, 0.15, 1.0));
        draw_circle(x, y - 39.0, 2.0, PAL_GOLD_0);

        // Great Kodanda bow
        draw_line(x - 8.0, y - 38.0, x - 8.0, y - 6.0, 2.0, PAL_GOLD_1);
    }

    fn draw_sita_forest(&self, x: f32, y: f32, _time: f32) {
        // Gentle pale bark sari (Valkala) with lotus trim
        draw_triangle(vec2(x - 6.0, y), vec2(x + 6.0, y), vec2(x, y - 26.0), Color::new(0.92, 0.82, 0.65, 1.0));
        draw_line(x - 6.0, y, x + 6.0, y, 1.5, PAL_GOLD_2);

        // Fair complexion
        draw_circle(x, y - 29.0, 5.0, Color::new(0.98, 0.88, 0.75, 1.0));
        // Elegant forest hair knot
        draw_circle(x, y - 34.0, 3.5, Color::new(0.18, 0.12, 0.10, 1.0));
        draw_circle(x - 3.0, y - 33.0, 2.0, Color::new(1.0, 0.75, 0.85, 1.0)); // Lotus flower in hair

        // Hands holding lotus
        draw_circle(x + 3.0, y - 18.0, 3.0, Color::new(1.0, 0.75, 0.85, 1.0));
    }

    fn draw_sumantra_sad(&self, x: f32, y: f32, _time: f32) {
        // Minister Sumantra with grey beard and sorrowful posture
        draw_rectangle(x - 5.0, y - 26.0, 10.0, 26.0, Color::new(0.40, 0.35, 0.30, 1.0));
        draw_circle(x, y - 30.0, 5.0, Color::new(0.85, 0.72, 0.60, 1.0));
        // Grey beard
        draw_triangle(vec2(x - 3.0, y - 29.0), vec2(x + 3.0, y - 29.0), vec2(x, y - 23.0), Color::new(0.85, 0.85, 0.85, 1.0));
        // Folded hands
        draw_circle(x + 2.0, y - 20.0, 2.5, Color::new(0.85, 0.72, 0.60, 1.0));
    }

    fn draw_bharata(&self, x: f32, y: f32, holds_padukas: bool, time: f32) {
        // Prince Bharata in ascetic valkala robes
        draw_rectangle(x - 6.0, y - 26.0, 12.0, 26.0, Color::new(0.78, 0.48, 0.18, 1.0));
        draw_circle(x, y - 30.0, 5.5, Color::new(0.92, 0.76, 0.58, 1.0));
        // Simple ascetic hair
        draw_circle(x, y - 35.0, 3.5, Color::new(0.15, 0.12, 0.10, 1.0));

        if holds_padukas {
            // Bharata holds Rama's divine golden Padukas upon his head!
            let py = y - 41.0 + (time * 3.0).sin() * 1.5;
            // Divine Padukas (golden sandals)
            draw_rectangle(x - 7.0, py, 14.0, 4.0, PAL_GOLD_0);
            draw_circle(x - 4.0, py - 2.0, 2.0, PAL_RED_1); // Golden knob
            draw_circle(x + 4.0, py - 2.0, 2.0, PAL_RED_1);

            // Radiant golden rays above Bharata's head
            for i in 0..5 {
                let angle = -2.2 + i as f32 * 0.55;
                let rx1 = x + angle.cos() * 8.0;
                let ry1 = py + angle.sin() * 8.0;
                let rx2 = x + angle.cos() * 18.0;
                let ry2 = py + angle.sin() * 18.0;
                draw_line(rx1, ry1, rx2, ry2, 1.5, PAL_GOLD_0);
            }
        }
    }
}

fn site_screen_label_x(x: f32) -> f32 {
    x - 48.0
}

// ============================================================================
// LEVEL 5 VICTORY & CONCLUSION SCREEN (AYODHYA KANDA COMPLETE)
// ============================================================================
pub fn draw_victory_l5(font: &Font, time: f32) {
    draw_rectangle(0.0, 0.0, SCREEN_W, SCREEN_H, Color::new(0.06, 0.05, 0.10, 0.96));

    let sway = (time * 5.0).sin() * 3.0;
    draw_pixel_text(font, "AYODHYA KANDA CONCLUDED: THE HERMITAGE OF CHITRAKUTA", 48.0, 45.0 + sway, 13, PAL_GOLD_0);

    let sub = "THE SACRED RAFT, THE SANCTIFIED PARNASHALA, AND THE REIGN OF THE PADUKAS";
    draw_pixel_text(font, sub, 50.0, 68.0, 7, PAL_TURMERIC_1);

    let rw_x = 45.0;
    let rw_y = 86.0;
    let rw_w = 550.0;
    let rw_h = 195.0;
    draw_rectangle(rw_x, rw_y, rw_w, rw_h, Color::new(0.10, 0.09, 0.16, 0.95));
    draw_rectangle_lines(rw_x, rw_y, rw_w, rw_h, 1.5, PAL_GOLD_1);

    draw_pixel_text(font, "CHAPTER 5 ACCOMPLISHMENTS (AYODHYA KANDA FINALE):", rw_x + 16.0, rw_y + 20.0, 9, PAL_GOLD_0);
    draw_pixel_text(font, "✓ Sacred Raft (Plava)  : Lakshmana bound timber & reeds to ferry Sita.", rw_x + 16.0, rw_y + 40.0, 7, WHITE);
    draw_pixel_text(font, "✓ Chitrakuta Hermitage : Sal pillars & Kusha thatch sanctified by Agni.", rw_x + 16.0, rw_y + 60.0, 7, WHITE);
    draw_pixel_text(font, "✓ Paduka Pattabhisheka : Bharata received holy sandals to reign over Ayodhya.", rw_x + 16.0, rw_y + 80.0, 7, PAL_GOLD_1);
    draw_pixel_text(font, "✓ Pitru-Vakya-Palana   : Rama upheld eternal Truth (Satya) & filial Dharma.", rw_x + 16.0, rw_y + 100.0, 7, Color::new(0.5, 0.9, 0.5, 1.0));

    // Next Epic Rakshasa Battle Announcement
    draw_line(rw_x + 16.0, rw_y + 118.0, rw_x + rw_w - 16.0, rw_y + 118.0, 1.0, PAL_GOLD_3);
    draw_pixel_text(font, "★ NEXT: ARANYA KANDA — THE AMBUSH OF RAKSHASA VIRADHA ★", rw_x + 16.0, rw_y + 135.0, 8, PAL_FIRE_1);
    draw_pixel_text(font, "In Dandakaranya, the towering man-eating demon Viradha ambushes the path,", rw_x + 16.0, rw_y + 152.0, 7, Color::new(0.9, 0.8, 0.8, 1.0));
    draw_pixel_text(font, "invulnerable to blades! Rama & Lakshmana shatter his arms and bury him!", rw_x + 16.0, rw_y + 167.0, 7, Color::new(0.9, 0.8, 0.8, 1.0));

    let blink = (time * 4.0).sin() > 0.0;
    if blink {
        draw_pixel_text(font, "PRESS SPACE OR CLICK TO RETURN TO CHAPTER SELECT ▶", 90.0, 305.0, 9, PAL_GOLD_0);
    }
    draw_pixel_text(font, "Press [R] to Replay Chapter 5 (Chitrakuta Hermitage)", 140.0, 330.0, 8, Color::new(0.7, 0.7, 0.7, 1.0));
}
