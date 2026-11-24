use macroquad::prelude::*;
use crate::audio::SoundEffects;
use crate::render::{
    draw_dialogue_box, draw_pixel_text, draw_reticle, is_dialogue_prev_button_hovered,
    COLOR_RAMA_GOLD, PAL_FIRE_1, PAL_GOLD_0, PAL_GOLD_1, PAL_GOLD_2, PAL_GOLD_3,
    PAL_GRASS_2, PAL_RED_1, PAL_TURMERIC_1, SCREEN_H, SCREEN_W,
};
use crate::types::{Astra, PlayerStats};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Level6Phase {
    IntroDialogue,
    ViradhaAmbush,
    Phase1Trident,
    Phase2HandToHand,
    Phase3Excavation,
    BurialAndAscension,
    Complete,
}

#[derive(Clone, Debug)]
pub struct ViradhaArrow {
    pub pos: Vec2,
    pub vel: Vec2,
    pub is_player: bool,
    pub astra: Astra,
    pub active: bool,
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct ForestPhantom {
    pub pos: Vec2,
    pub hp: f32,
    pub active: bool,
    pub spawn_time: f32,
}

#[derive(Clone, Debug)]
pub struct Shockwave {
    pub x: f32,
    pub y: f32,
    pub speed: f32,
    pub radius: f32,
    pub max_radius: f32,
    pub active: bool,
}

#[derive(Clone, Debug)]
pub struct FloatingText {
    pub text: String,
    pub pos: Vec2,
    pub color: Color,
    pub timer: f32,
    pub max_timer: f32,
}

#[derive(Clone, Debug)]
pub struct ViradhaParticle {
    pub pos: Vec2,
    pub vel: Vec2,
    pub color: Color,
    pub size: f32,
    pub timer: f32,
    pub max_timer: f32,
}

#[allow(dead_code)]
pub enum FightResultL6 {
    Continue,
    Victory,
    Defeat,
}

#[allow(dead_code)]
pub struct Level6Fight {
    pub phase: Level6Phase,
    pub rama_pos: Vec2,
    pub rama_vel: Vec2,
    pub viradha_pos: Vec2,
    pub viradha_facing_left: bool,
    pub trident_hp: f32,
    pub max_trident_hp: f32,
    pub left_arm_hp: f32,
    pub right_arm_hp: f32,
    pub max_arm_hp: f32,
    pub left_arm_broken: bool,
    pub right_arm_broken: bool,
    pub trench_progress: f32, // 0.0 to 100.0
    pub trench_x: f32,
    pub lakshmana_pos: Vec2,
    pub sita_pos: Vec2,

    // Timers & cooldowns
    pub attack_timer: f32,
    pub attack_state: u8, // 0 = idle, 1 = windup, 2 = striking, 3 = recover
    pub shake_timer: f32,
    pub ascension_timer: f32,
    pub bow_tension: f32,
    pub is_drawing: bool,

    // Entities
    pub arrows: Vec<ViradhaArrow>,
    pub phantoms: Vec<ForestPhantom>,
    pub shockwaves: Vec<Shockwave>,
    pub floating_texts: Vec<FloatingText>,
    pub particles: Vec<ViradhaParticle>,

    // Dialogue & Messages
    pub dialogue: Option<(String, Vec<String>, usize)>,
    pub action_message: Option<(String, f32)>,
    pub camera_x: f32,
}

impl Level6Fight {
    pub fn new() -> Self {
        let mut fight = Self {
            phase: Level6Phase::IntroDialogue,
            rama_pos: vec2(140.0, 240.0),
            rama_vel: Vec2::ZERO,
            viradha_pos: vec2(500.0, 225.0),
            viradha_facing_left: true,
            trident_hp: 100.0,
            max_trident_hp: 100.0,
            left_arm_hp: 80.0,
            right_arm_hp: 80.0,
            max_arm_hp: 80.0,
            left_arm_broken: false,
            right_arm_broken: false,
            trench_progress: 0.0,
            trench_x: 440.0,
            lakshmana_pos: vec2(200.0, 240.0),
            sita_pos: vec2(70.0, 240.0),

            attack_timer: 0.0,
            attack_state: 0,
            shake_timer: 0.0,
            ascension_timer: 0.0,
            bow_tension: 0.0,
            is_drawing: false,

            arrows: Vec::new(),
            phantoms: Vec::new(),
            shockwaves: Vec::new(),
            floating_texts: Vec::new(),
            particles: Vec::new(),

            dialogue: None,
            action_message: Some((
                "Dandakaranya Wilderness — The Primordial Forest".to_string(),
                4.0,
            )),
            camera_x: 0.0,
        };

        // Initial Intro Dialogue
        fight.dialogue = Some((
            "Dandakaranya Trail".to_string(),
            vec![
                "Prince Rama: 'Be watchful, Lakshmana. The air here hangs heavy with malice... not a single bird calls in the canopy.'".to_string(),
                "Princess Sita: 'Look at the ancient sal trees, My Lord—crushed as if by the foot of a mountain!'".to_string(),
                "Prince Lakshmana: 'A foul stench of blood and brimstone approaches from the eastern glade! Draw your Kodanda bow!'".to_string(),
                "A thunderous tremor shudders through the earth as branches snap above!".to_string(),
            ],
            0,
        ));

        fight
    }

    pub fn update(
        &mut self,
        dt: f32,
        sounds: &SoundEffects,
        stats: &mut PlayerStats,
        virtual_mouse: Vec2,
        interact: bool,
    ) -> FightResultL6 {
        // Update Action message
        if let Some((_, ref mut timer)) = self.action_message {
            *timer -= dt;
            if *timer <= 0.0 {
                self.action_message = None;
            }
        }

        // Screen shake decay
        if self.shake_timer > 0.0 {
            self.shake_timer = (self.shake_timer - dt).max(0.0);
        }

        // Handle Active Dialogue
        if let Some((_, ref lines, ref mut idx)) = self.dialogue {
            let prev_clicked = is_mouse_button_pressed(MouseButton::Left)
                && is_dialogue_prev_button_hovered(virtual_mouse);
            let prev_key = is_key_pressed(KeyCode::Left) || is_key_pressed(KeyCode::Backspace);

            if (prev_clicked || prev_key) && *idx > 0 {
                *idx -= 1;
                sounds.play(&sounds.shoot, 0.4);
                return FightResultL6::Continue;
            }

            if interact {
                *idx += 1;
                sounds.play(&sounds.shoot, 0.4);
                if *idx >= lines.len() {
                    self.dialogue = None;
                    self.on_dialogue_complete(sounds);
                }
            }
            return FightResultL6::Continue;
        }

        // Developer / Test quick-advance shortcut
        if is_key_pressed(KeyCode::K) {
            match self.phase {
                Level6Phase::Phase1Trident => {
                    self.trident_hp = 0.0;
                    self.on_trident_shattered(sounds);
                    return FightResultL6::Continue;
                }
                Level6Phase::Phase2HandToHand => {
                    self.left_arm_broken = true;
                    self.right_arm_broken = true;
                    self.on_both_arms_broken(sounds);
                    return FightResultL6::Continue;
                }
                Level6Phase::Phase3Excavation => {
                    self.trench_progress = 100.0;
                    self.on_trench_complete(sounds);
                    return FightResultL6::Continue;
                }
                Level6Phase::BurialAndAscension => {
                    self.ascension_timer = 8.0;
                    self.dialogue = None;
                    self.phase = Level6Phase::Complete;
                    return FightResultL6::Victory;
                }
                _ => {}
            }
        }

        // Phase specific updates
        match self.phase {
            Level6Phase::IntroDialogue => {
                // Handled in dialogue
            }
            Level6Phase::ViradhaAmbush => {
                // Viradha drops down with shockwave
                self.shake_timer = 1.5;
                sounds.play(&sounds.thunder, 0.9);
                self.spawn_ground_dust(self.viradha_pos.x, 240.0, 30);
                self.phase = Level6Phase::Phase1Trident;
                self.action_message = Some((
                    "TARGET THE IRON TRIDENT JOINT! BRAHMA'S BOON PROTECTS HIS FLESH!".to_string(),
                    5.0,
                ));
            }
            Level6Phase::Phase1Trident => {
                self.update_combat(dt, sounds, stats, virtual_mouse);
                self.update_viradha_trident(dt, sounds, stats);
                if self.trident_hp <= 0.0 {
                    self.on_trident_shattered(sounds);
                }
            }
            Level6Phase::Phase2HandToHand => {
                self.update_combat(dt, sounds, stats, virtual_mouse);
                self.update_viradha_brawler(dt, sounds, stats);
                if self.left_arm_broken && self.right_arm_broken {
                    self.on_both_arms_broken(sounds);
                }
            }
            Level6Phase::Phase3Excavation => {
                self.update_combat(dt, sounds, stats, virtual_mouse);
                self.update_excavation(dt, sounds, interact);
                if self.trench_progress >= 100.0 {
                    self.on_trench_complete(sounds);
                }
            }
            Level6Phase::BurialAndAscension => {
                self.ascension_timer += dt;
                self.update_ascension_particles(dt);
                if self.ascension_timer >= 7.0 && self.dialogue.is_none() {
                    self.phase = Level6Phase::Complete;
                    return FightResultL6::Victory;
                }
            }
            Level6Phase::Complete => {
                return FightResultL6::Victory;
            }
        }

        // Entity updates
        self.update_entities(dt, sounds, stats);

        FightResultL6::Continue
    }

    fn on_dialogue_complete(&mut self, _sounds: &SoundEffects) {
        match self.phase {
            Level6Phase::IntroDialogue => {
                self.phase = Level6Phase::ViradhaAmbush;
            }
            Level6Phase::BurialAndAscension => {
                // Finished Tumburu's dialogue
            }
            _ => {}
        }
    }

    fn on_trident_shattered(&mut self, sounds: &SoundEffects) {
        sounds.play(&sounds.smash, 1.0);
        sounds.play(&sounds.thunder, 0.8);
        self.shake_timer = 1.0;
        self.spawn_iron_shards(self.viradha_pos.x - 30.0, self.viradha_pos.y - 30.0, 40);
        self.floating_texts.push(FloatingText {
            text: "TRIDENT SHATTERED! DISARMED!".to_string(),
            pos: vec2(self.viradha_pos.x - 60.0, self.viradha_pos.y - 70.0),
            color: PAL_FIRE_1,
            timer: 2.5,
            max_timer: 2.5,
        });

        self.phase = Level6Phase::Phase2HandToHand;
        self.action_message = Some((
            "VIRADHA IS ENRAGED! TARGET HIS EXPOSED ARM JOINTS TO SHATTER HIS LIMBS!".to_string(),
            5.0,
        ));
    }

    fn on_both_arms_broken(&mut self, sounds: &SoundEffects) {
        sounds.play(&sounds.smash, 1.0);
        sounds.play(&sounds.victory, 0.7);
        self.shake_timer = 0.8;
        self.floating_texts.push(FloatingText {
            text: "BOTH ARMS BROKEN! GIANT COLLAPSED!".to_string(),
            pos: vec2(self.viradha_pos.x - 70.0, self.viradha_pos.y - 60.0),
            color: PAL_GOLD_0,
            timer: 3.0,
            max_timer: 3.0,
        });

        self.phase = Level6Phase::Phase3Excavation;
        self.lakshmana_pos = vec2(self.trench_x - 30.0, 240.0);
        self.action_message = Some((
            "WEAPONS CANNOT SLAY HIM! DEFEND LAKSHMANA & PRESS [SPACE] AT THE PIT TO DIG!".to_string(),
            5.0,
        ));
    }

    fn on_trench_complete(&mut self, sounds: &SoundEffects) {
        sounds.play(&sounds.bell, 1.0);
        sounds.play(&sounds.victory, 1.0);
        self.shake_timer = 2.0;
        self.phase = Level6Phase::BurialAndAscension;
        self.ascension_timer = 0.0;

        self.dialogue = Some((
            "Celestial Gandharva Tumburu".to_string(),
            vec![
                "From the consecrated pit, a blinding column of pure golden light bursts forth!".to_string(),
                "The grotesque demon carcass dissolves, and a radiant celestial being holding a golden veena ascends!".to_string(),
                "Gandharva Tumburu: 'O Prince Rama! In a former age, Kubera cursed me to walk as the vile Rakshasa Viradha.'".to_string(),
                "Tumburu: 'Kubera decreed: only when Lord Rama shatters your limbs and buries you in sacred earth shall you be free!'".to_string(),
                "Tumburu: 'The curse is undone! Seek now the hermitage of Sage Sharabhanga and Sage Agastya at holy Panchavati!'".to_string(),
                "With blessings showered upon Rama, Sita, and Lakshmana, the radiant Gandharva ascends into heaven.".to_string(),
            ],
            0,
        ));
    }

    // Player controls & combat
    fn update_combat(
        &mut self,
        dt: f32,
        sounds: &SoundEffects,
        stats: &mut PlayerStats,
        virtual_mouse: Vec2,
    ) {
        // Rama Movement (A/D)
        let mut move_x = 0.0;
        if is_key_down(KeyCode::A) || is_key_down(KeyCode::Left) {
            move_x -= 1.0;
        }
        if is_key_down(KeyCode::D) || is_key_down(KeyCode::Right) {
            move_x += 1.0;
        }

        self.rama_pos.x += move_x * 140.0 * dt;
        self.rama_pos.x = self.rama_pos.x.clamp(60.0, 580.0);

        // Aiming & Firing
        let is_left_down = is_mouse_button_down(MouseButton::Left);
        if is_left_down && !self.is_drawing {
            self.is_drawing = true;
            self.bow_tension = 0.0;
        } else if is_left_down && self.is_drawing {
            self.bow_tension = (self.bow_tension + dt * 2.5).min(1.0);
        } else if !is_left_down && self.is_drawing {
            self.is_drawing = false;
            if self.bow_tension > 0.2 {
                self.fire_arrow(sounds, stats, virtual_mouse);
            }
            self.bow_tension = 0.0;
        }

        // Camera follow
        let target_cam = (self.rama_pos.x - SCREEN_W * 0.45).clamp(0.0, 100.0);
        self.camera_x += (target_cam - self.camera_x) * dt * 4.0;
    }

    fn fire_arrow(&mut self, sounds: &SoundEffects, stats: &mut PlayerStats, target: Vec2) {
        let spawn_pos = self.rama_pos + vec2(10.0, -25.0);
        let dir = (target - spawn_pos).normalize_or_zero();
        let speed = 360.0 + self.bow_tension * 280.0;

        self.arrows.push(ViradhaArrow {
            pos: spawn_pos,
            vel: dir * speed,
            is_player: true,
            astra: stats.selected_astra,
            active: true,
        });

        sounds.play(&sounds.shoot, 0.7);
    }

    // Viradha Trident Phase AI
    fn update_viradha_trident(&mut self, dt: f32, sounds: &SoundEffects, _stats: &mut PlayerStats) {
        self.attack_timer += dt;

        // Cycle attacks every 3.2s
        if self.attack_timer > 3.2 {
            self.attack_timer = 0.0;
            let rng = ((self.attack_timer * 133.0) as u32) % 2;

            if rng == 0 {
                // Trident Shockwave Slam
                sounds.play(&sounds.thunder, 0.7);
                self.shake_timer = 0.6;
                let ground_y = 244.0;
                self.shockwaves.push(Shockwave {
                    x: self.viradha_pos.x - 40.0,
                    y: ground_y,
                    speed: -180.0,
                    radius: 8.0,
                    max_radius: 24.0,
                    active: true,
                });
                self.spawn_ground_dust(self.viradha_pos.x - 40.0, ground_y, 15);
            } else {
                // Trident Flame Thrust (projectile)
                sounds.play(&sounds.fire, 0.7);
                let fire_dir = (self.rama_pos - self.viradha_pos).normalize_or_zero();
                self.arrows.push(ViradhaArrow {
                    pos: self.viradha_pos + vec2(-45.0, -35.0),
                    vel: fire_dir * 220.0,
                    is_player: false,
                    astra: Astra::Agneyastra,
                    active: true,
                });
            }
        }
    }

    // Viradha Brawler Phase AI
    fn update_viradha_brawler(&mut self, dt: f32, sounds: &SoundEffects, _stats: &mut PlayerStats) {
        self.attack_timer += dt;

        // Periodic seismic stomp
        if self.attack_timer > 2.8 {
            self.attack_timer = 0.0;
            sounds.play(&sounds.thunder, 0.8);
            self.shake_timer = 0.7;
            let ground_y = 244.0;
            self.shockwaves.push(Shockwave {
                x: self.viradha_pos.x,
                y: ground_y,
                speed: -190.0,
                radius: 10.0,
                max_radius: 26.0,
                active: true,
            });
            self.spawn_ground_dust(self.viradha_pos.x, ground_y, 20);
        }
    }

    // Phase 3 Excavation AI
    fn update_excavation(&mut self, dt: f32, sounds: &SoundEffects, interact: bool) {
        // Base digging speed by Lakshmana
        let mut dig_speed = 6.0;

        // If Rama is near pit and player presses interact, dig faster!
        let dist = (self.rama_pos.x - self.trench_x).abs();
        if dist < 45.0 && interact {
            dig_speed += 18.0;
            self.spawn_ground_dust(self.trench_x, 246.0, 4);
            sounds.play(&sounds.bell, 0.2);
        }

        self.trench_progress = (self.trench_progress + dig_speed * dt).min(100.0);

        // Spawn forest phantoms periodically
        self.attack_timer += dt;
        if self.attack_timer > 2.5 && self.phantoms.len() < 3 {
            self.attack_timer = 0.0;
            let spawn_x = if self.phantoms.len() % 2 == 0 { 240.0 } else { 540.0 };
            self.phantoms.push(ForestPhantom {
                pos: vec2(spawn_x, 235.0),
                hp: 30.0,
                active: true,
                spawn_time: 0.0,
            });
        }
    }

    fn update_entities(&mut self, dt: f32, sounds: &SoundEffects, stats: &mut PlayerStats) {
        let mut iron_shards_events: Vec<(f32, f32, usize)> = Vec::new();
        let mut boon_deflections: Vec<(f32, f32)> = Vec::new();
        let mut phantom_smokes: Vec<(f32, f32)> = Vec::new();

        // 1. Arrows
        for arrow in self.arrows.iter_mut().filter(|a| a.active) {
            arrow.pos += arrow.vel * dt;

            // Player arrows check
            if arrow.is_player {
                // A) Check Viradha Trident Joint in Phase 1
                if self.phase == Level6Phase::Phase1Trident {
                    let joint_pos = self.viradha_pos + vec2(-35.0, -45.0);
                    if (arrow.pos - joint_pos).length() < 24.0 {
                        arrow.active = false;
                        self.trident_hp = (self.trident_hp - 25.0).max(0.0);
                        sounds.play(&sounds.smash, 0.8);
                        iron_shards_events.push((joint_pos.x, joint_pos.y, 8));
                        self.floating_texts.push(FloatingText {
                            text: "-25 TRIDENT".to_string(),
                            pos: joint_pos + vec2(0.0, -15.0),
                            color: PAL_FIRE_1,
                            timer: 1.0,
                            max_timer: 1.0,
                        });
                        continue;
                    }
                }

                // B) Check Viradha Arms in Phase 2
                if self.phase == Level6Phase::Phase2HandToHand {
                    // Left Arm target
                    if !self.left_arm_broken {
                        let left_joint = self.viradha_pos + vec2(-42.0, -32.0);
                        if (arrow.pos - left_joint).length() < 22.0 {
                            arrow.active = false;
                            self.left_arm_hp = (self.left_arm_hp - 25.0).max(0.0);
                            sounds.play(&sounds.hit, 0.8);
                            self.floating_texts.push(FloatingText {
                                text: "-25 LEFT ARM".to_string(),
                                pos: left_joint + vec2(-10.0, -15.0),
                                color: PAL_GOLD_0,
                                timer: 1.0,
                                max_timer: 1.0,
                            });
                            if self.left_arm_hp <= 0.0 {
                                self.left_arm_broken = true;
                                sounds.play(&sounds.snap, 1.0);
                            }
                            continue;
                        }
                    }

                    // Right Arm target
                    if !self.right_arm_broken {
                        let right_joint = self.viradha_pos + vec2(15.0, -32.0);
                        if (arrow.pos - right_joint).length() < 22.0 {
                            arrow.active = false;
                            self.right_arm_hp = (self.right_arm_hp - 25.0).max(0.0);
                            sounds.play(&sounds.hit, 0.8);
                            self.floating_texts.push(FloatingText {
                                text: "-25 RIGHT ARM".to_string(),
                                pos: right_joint + vec2(-10.0, -15.0),
                                color: PAL_GOLD_0,
                                timer: 1.0,
                                max_timer: 1.0,
                            });
                            if self.right_arm_hp <= 0.0 {
                                self.right_arm_broken = true;
                                sounds.play(&sounds.snap, 1.0);
                            }
                            continue;
                        }
                    }
                }

                // C) Check Viradha Torso -> Deflection by Brahma's Boon!
                let torso_pos = self.viradha_pos + vec2(-10.0, -35.0);
                if (arrow.pos - torso_pos).length() < 35.0 {
                    arrow.active = false;
                    sounds.play(&sounds.shoot, 0.3);
                    boon_deflections.push((arrow.pos.x, arrow.pos.y));
                    self.floating_texts.push(FloatingText {
                        text: "DEFLECTED! (BRAHMA'S BOON)".to_string(),
                        pos: arrow.pos + vec2(-40.0, -18.0),
                        color: PAL_GOLD_1,
                        timer: 1.2,
                        max_timer: 1.2,
                    });
                    continue;
                }

                // D) Check Phantoms in Phase 3
                for p in self.phantoms.iter_mut().filter(|p| p.active) {
                    if (arrow.pos - p.pos).length() < 25.0 {
                        arrow.active = false;
                        p.hp -= 30.0;
                        sounds.play(&sounds.hit, 0.7);
                        if p.hp <= 0.0 {
                            p.active = false;
                            phantom_smokes.push((p.pos.x, p.pos.y));
                        }
                        break;
                    }
                }
            } else {
                // Enemy projectile hits Rama
                if (arrow.pos - self.rama_pos).length() < 20.0 {
                    arrow.active = false;
                    stats.prana = (stats.prana - 15).max(0);
                    sounds.play(&sounds.hit, 0.9);
                    self.shake_timer = 0.4;
                }
            }

            // Boundary cleanup
            if arrow.pos.x < 0.0 || arrow.pos.x > SCREEN_W + 50.0 || arrow.pos.y > SCREEN_H || arrow.pos.y < 0.0 {
                arrow.active = false;
            }
        }
        self.arrows.retain(|a| a.active);

        // Spawn deferred particle effects
        for (x, y, count) in iron_shards_events {
            self.spawn_iron_shards(x, y, count);
        }
        for (x, y) in boon_deflections {
            self.spawn_boon_deflection(x, y);
        }
        for (x, y) in phantom_smokes {
            self.spawn_phantom_smoke(x, y);
        }

        // 2. Shockwaves
        for sw in self.shockwaves.iter_mut().filter(|s| s.active) {
            sw.x += sw.speed * dt;
            sw.radius = (sw.radius + dt * 15.0).min(sw.max_radius);

            // Check hit on Rama
            if (sw.x - self.rama_pos.x).abs() < 18.0 && (sw.y - self.rama_pos.y).abs() < 15.0 {
                stats.prana = (stats.prana - 10).max(0);
                sounds.play(&sounds.hit, 0.8);
                self.shake_timer = 0.4;
                sw.active = false;
            }

            if sw.x < 20.0 || sw.x > SCREEN_W + 20.0 {
                sw.active = false;
            }
        }
        self.shockwaves.retain(|s| s.active);

        // 3. Phantoms
        for p in self.phantoms.iter_mut().filter(|p| p.active) {
            // Creep toward trench
            let dir = (self.trench_x - p.pos.x).signum();
            p.pos.x += dir * 35.0 * dt;

            // Damage Lakshmana if too close
            if (p.pos.x - self.trench_x).abs() < 20.0 {
                p.active = false;
                self.trench_progress = (self.trench_progress - 10.0).max(0.0);
                sounds.play(&sounds.smash, 0.6);
                self.floating_texts.push(FloatingText {
                    text: "-10% TRENCH!".to_string(),
                    pos: vec2(self.trench_x - 20.0, 220.0),
                    color: PAL_FIRE_1,
                    timer: 1.2,
                    max_timer: 1.2,
                });
            }
        }
        self.phantoms.retain(|p| p.active);

        // 4. Floating texts
        for ft in &mut self.floating_texts {
            ft.timer -= dt;
            ft.pos.y -= 12.0 * dt;
        }
        self.floating_texts.retain(|ft| ft.timer > 0.0);

        // 5. Particles
        for p in &mut self.particles {
            p.timer -= dt;
            p.pos += p.vel * dt;
        }
        self.particles.retain(|p| p.timer > 0.0);
    }

    fn spawn_ground_dust(&mut self, x: f32, y: f32, count: usize) {
        for i in 0..count {
            let angle = (i as f32 / count as f32) * std::f32::consts::PI;
            let speed = 40.0 + ((i * 17) % 60) as f32;
            self.particles.push(ViradhaParticle {
                pos: vec2(x, y),
                vel: vec2(angle.cos() * speed, -angle.sin() * speed * 0.7),
                color: Color::new(0.45, 0.38, 0.28, 0.8),
                size: 3.5,
                timer: 0.6,
                max_timer: 0.6,
            });
        }
    }

    fn spawn_iron_shards(&mut self, x: f32, y: f32, count: usize) {
        for i in 0..count {
            let angle = (i as f32 / count as f32) * std::f32::consts::TAU;
            let speed = 60.0 + ((i * 23) % 90) as f32;
            self.particles.push(ViradhaParticle {
                pos: vec2(x, y),
                vel: vec2(angle.cos() * speed, angle.sin() * speed),
                color: Color::new(0.85, 0.85, 0.95, 1.0),
                size: 2.5,
                timer: 0.8,
                max_timer: 0.8,
            });
        }
    }

    fn spawn_boon_deflection(&mut self, x: f32, y: f32) {
        for i in 0..12 {
            let angle = (i as f32 / 12.0) * std::f32::consts::TAU;
            self.particles.push(ViradhaParticle {
                pos: vec2(x, y),
                vel: vec2(angle.cos() * 50.0, angle.sin() * 50.0),
                color: PAL_GOLD_0,
                size: 2.0,
                timer: 0.4,
                max_timer: 0.4,
            });
        }
    }

    fn spawn_phantom_smoke(&mut self, x: f32, y: f32) {
        for i in 0..15 {
            let angle = (i as f32 / 15.0) * std::f32::consts::TAU;
            self.particles.push(ViradhaParticle {
                pos: vec2(x, y),
                vel: vec2(angle.cos() * 30.0, angle.sin() * 30.0),
                color: Color::new(0.25, 0.12, 0.35, 0.7),
                size: 4.0,
                timer: 0.7,
                max_timer: 0.7,
            });
        }
    }

    fn update_ascension_particles(&mut self, _dt: f32) {
        if self.particles.len() < 40 {
            let px = self.trench_x + (((self.ascension_timer * 137.0) as i32 % 50) - 25) as f32;
            self.particles.push(ViradhaParticle {
                pos: vec2(px, 240.0),
                vel: vec2(0.0, -70.0 - ((self.particles.len() * 11) % 40) as f32),
                color: if self.particles.len() % 2 == 0 { PAL_GOLD_0 } else { WHITE },
                size: 2.5,
                timer: 1.5,
                max_timer: 1.5,
            });
        }
    }

    // ========================================================================
    // DRAW ROUTINES (720p COMPLIANT RETRO ART)
    // ========================================================================
    pub fn draw(&self, font: &Font, time: f32, stats: &PlayerStats, virtual_mouse: Vec2) {
        let shake = if self.shake_timer > 0.0 {
            vec2((time * 40.0).sin() * 3.0, (time * 50.0).cos() * 3.0)
        } else {
            Vec2::ZERO
        };

        // 1. Primordial Dandakaranya Forest Backdrop
        self.draw_dandaka_forest(time, shake);

        // 2. Excavation Trench Site
        if self.phase == Level6Phase::Phase3Excavation || self.phase == Level6Phase::BurialAndAscension {
            self.draw_trench_site(font, time);
        }

        // 3. Characters & Entities
        // Sita in safety
        self.draw_sita_safe(self.sita_pos.x, self.sita_pos.y);

        // Lakshmana
        if self.phase == Level6Phase::Phase3Excavation {
            self.draw_lakshmana_digging(self.lakshmana_pos.x, self.lakshmana_pos.y, time);
        } else {
            self.draw_lakshmana(self.lakshmana_pos.x, self.lakshmana_pos.y);
        }

        // Prince Rama
        self.draw_rama(self.rama_pos.x, self.rama_pos.y, time);

        // Viradha / Tumburu
        if self.phase == Level6Phase::BurialAndAscension {
            self.draw_tumburu_ascension(self.trench_x, time);
        } else {
            self.draw_viradha(self.viradha_pos.x + shake.x, self.viradha_pos.y + shake.y, time);
        }

        // Phantoms
        for p in &self.phantoms {
            if p.active {
                self.draw_phantom(p.pos.x, p.pos.y, time);
            }
        }

        // Shockwaves
        for sw in &self.shockwaves {
            if sw.active {
                draw_circle_lines(sw.x, sw.y, sw.radius, 2.0, PAL_FIRE_1);
                draw_circle(sw.x, sw.y, sw.radius * 0.5, Color::new(1.0, 0.4, 0.1, 0.4));
            }
        }

        // Arrows & Astras
        for arrow in &self.arrows {
            if arrow.active {
                self.draw_arrow(arrow);
            }
        }

        // Particles
        for p in &self.particles {
            let alpha = (p.timer / p.max_timer).clamp(0.0, 1.0);
            let mut col = p.color;
            col.a *= alpha;
            draw_rectangle(p.pos.x - p.size * 0.5, p.pos.y - p.size * 0.5, p.size, p.size, col);
        }

        // Floating texts
        for ft in &self.floating_texts {
            let alpha = (ft.timer / ft.max_timer).clamp(0.0, 1.0);
            let mut col = ft.color;
            col.a *= alpha;
            draw_pixel_text(font, &ft.text, ft.pos.x, ft.pos.y, 7, col);
        }

        // 4. Boss HUD & Quest Objectives
        self.draw_level6_hud(font, stats, time);

        // Top Action Message
        if let Some((msg, _)) = &self.action_message {
            let bw = 480.0;
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

    // Dandakaranya Forest Environment
    fn draw_dandaka_forest(&self, time: f32, shake: Vec2) {
        // Twilight violet/indigo sky
        draw_rectangle(0.0, 0.0, SCREEN_W, 240.0, Color::new(0.08, 0.06, 0.14, 1.0));

        // Spores in forest air
        for i in 0..20 {
            let sx = (i as f32 * 34.0 + time * 12.0) % SCREEN_W;
            let sy = 50.0 + ((i * 17) as f32 % 140.0) + (time * 2.0 + i as f32).sin() * 5.0;
            draw_circle(sx, sy, 1.2, Color::new(0.4, 0.85, 0.7, 0.6));
        }

        // Mountain silhouettes in background
        for i in 0..6 {
            let mx = i as f32 * 120.0 - 20.0;
            draw_triangle(
                vec2(mx, 240.0),
                vec2(mx + 60.0, 110.0),
                vec2(mx + 120.0, 240.0),
                Color::new(0.12, 0.09, 0.18, 0.9),
            );
        }

        // Giant Ancient Sal & Banyan Tree Trunks
        let trunks = [35.0, 170.0, 310.0, 480.0, 600.0];
        for (i, tx) in trunks.iter().enumerate() {
            let sway = (time * 0.8 + i as f32).sin() * 2.0;
            let trunk_x = *tx + sway + shake.x * 0.3;
            // Trunk
            draw_rectangle(trunk_x - 14.0, 30.0, 28.0, 210.0, Color::new(0.18, 0.14, 0.12, 1.0));
            draw_rectangle(trunk_x - 10.0, 30.0, 8.0, 210.0, Color::new(0.14, 0.10, 0.08, 1.0));
            // Twisted roots
            draw_line(trunk_x - 14.0, 230.0, trunk_x - 28.0, 244.0, 3.5, Color::new(0.18, 0.14, 0.12, 1.0));
            draw_line(trunk_x + 14.0, 230.0, trunk_x + 28.0, 244.0, 3.5, Color::new(0.18, 0.14, 0.12, 1.0));

            // Hanging lianas / vines
            draw_line(trunk_x - 8.0, 60.0, trunk_x - 12.0, 150.0, 1.5, Color::new(0.24, 0.40, 0.22, 0.8));
            draw_line(trunk_x + 8.0, 80.0, trunk_x + 10.0, 170.0, 1.5, Color::new(0.24, 0.40, 0.22, 0.8));

            // Canopy foliage
            draw_circle(trunk_x, 45.0, 42.0, Color::new(0.10, 0.22, 0.14, 0.95));
            draw_circle(trunk_x - 12.0, 40.0, 34.0, Color::new(0.14, 0.28, 0.18, 0.95));
        }

        // Forest Floor (Moss & Roots)
        draw_rectangle(0.0, 240.0, SCREEN_W, SCREEN_H - 240.0, Color::new(0.16, 0.26, 0.18, 1.0));
        draw_rectangle(0.0, 244.0, SCREEN_W, 6.0, Color::new(0.12, 0.20, 0.14, 1.0));
        draw_line(0.0, 240.0, SCREEN_W, 240.0, 2.0, PAL_GRASS_2);

        // Ancient Forest Hermitage Altar (behind Sita at x = 70.0)
        draw_rectangle(45.0, 210.0, 50.0, 30.0, Color::new(0.22, 0.20, 0.24, 1.0));
        draw_rectangle_lines(45.0, 210.0, 50.0, 30.0, 1.5, PAL_GOLD_3);
        draw_circle(70.0, 205.0, 8.0, Color::new(0.9, 0.5, 0.2, 0.8)); // Sacred deepa lamp
    }

    fn draw_trench_site(&self, font: &Font, time: f32) {
        let tx = self.trench_x;
        let tw = 70.0;
        let th = 18.0 + (self.trench_progress / 100.0) * 22.0;

        // Pit hole in earth
        draw_rectangle(tx - tw * 0.5, 240.0, tw, th, Color::new(0.08, 0.05, 0.04, 1.0));
        draw_rectangle_lines(tx - tw * 0.5, 240.0, tw, th, 1.5, Color::new(0.35, 0.25, 0.15, 1.0));

        // Piles of excavated earth on sides
        draw_rectangle(tx - tw * 0.5 - 16.0, 235.0, 16.0, 8.0, Color::new(0.30, 0.22, 0.15, 1.0));
        draw_rectangle(tx + tw * 0.5, 235.0, 16.0, 8.0, Color::new(0.30, 0.22, 0.15, 1.0));

        // Digging prompt
        if self.phase == Level6Phase::Phase3Excavation {
            let dist = (self.rama_pos.x - tx).abs();
            if dist < 50.0 {
                let blink = (time * 6.0).sin() > 0.0;
                let col = if blink { PAL_GOLD_0 } else { WHITE };
                draw_pixel_text(font, "[SPACE / E] DIG SACRED PIT!", tx - 52.0, 225.0, 7, col);
            }
        }
    }

    // Viradha Titan Sprite
    fn draw_viradha(&self, x: f32, y: f32, time: f32) {
        let breathe = (time * 3.0).sin() * 2.0;

        // Giant Shadow
        draw_ellipse(x, y + 15.0, 38.0, 10.0, 0.0, Color::new(0.0, 0.0, 0.0, 0.45));

        // Divine Golden Shield Halo when Brahma's Boon protects body
        if self.phase == Level6Phase::Phase1Trident || self.phase == Level6Phase::Phase2HandToHand {
            let shield_pulse = ((time * 4.0).sin() * 0.15 + 0.35).clamp(0.0, 1.0);
            draw_circle_lines(x - 10.0, y - 35.0, 48.0, 1.5, Color::new(1.0, 0.85, 0.3, shield_pulse));
        }

        // Feet & Claws
        draw_rectangle(x - 24.0, y + 5.0, 14.0, 10.0, Color::new(0.28, 0.22, 0.32, 1.0));
        draw_rectangle(x + 6.0, y + 5.0, 14.0, 10.0, Color::new(0.28, 0.22, 0.32, 1.0));

        // Massive Muscular Legs
        draw_rectangle(x - 22.0, y - 20.0, 12.0, 26.0, Color::new(0.32, 0.25, 0.38, 1.0));
        draw_rectangle(x + 8.0, y - 20.0, 12.0, 26.0, Color::new(0.32, 0.25, 0.38, 1.0));

        // Dark Hide Loincloth & Bronze Belt
        draw_rectangle(x - 24.0, y - 24.0, 46.0, 16.0, Color::new(0.18, 0.12, 0.10, 1.0));
        draw_rectangle(x - 24.0, y - 26.0, 46.0, 4.0, PAL_GOLD_3);

        // Towering Muscular Torso (Broad shoulders)
        draw_rectangle(x - 30.0, y - 56.0 + breathe, 58.0, 34.0, Color::new(0.36, 0.28, 0.42, 1.0));
        draw_rectangle(x - 22.0, y - 52.0 + breathe, 42.0, 26.0, Color::new(0.40, 0.32, 0.46, 1.0));

        // Bone Necklace
        for i in 0..5 {
            let bx = x - 20.0 + i as f32 * 10.0;
            draw_rectangle(bx, y - 56.0 + breathe, 6.0, 4.0, Color::new(0.9, 0.85, 0.75, 1.0));
        }

        // Arms
        // Left Arm (holding trident in Phase 1, brawler in Phase 2)
        if self.left_arm_broken {
            // Dangling limp broken arm
            draw_line(x - 30.0, y - 48.0 + breathe, x - 38.0, y - 20.0, 6.0, Color::new(0.30, 0.22, 0.35, 1.0));
            draw_line(x - 38.0, y - 20.0, x - 32.0, y + 2.0, 5.0, Color::new(0.30, 0.22, 0.35, 1.0));
            draw_circle(x - 38.0, y - 20.0, 4.0, PAL_RED_1); // Bone fracture
        } else {
            // Strong muscular arm
            draw_rectangle(x - 42.0, y - 52.0 + breathe, 14.0, 28.0, Color::new(0.36, 0.28, 0.42, 1.0));
            draw_rectangle(x - 46.0, y - 28.0 + breathe, 12.0, 24.0, Color::new(0.32, 0.25, 0.38, 1.0));
            if self.phase == Level6Phase::Phase2HandToHand {
                // Weak point target indicator at left elbow
                let blink = (time * 8.0).sin() > 0.0;
                let col = if blink { PAL_GOLD_0 } else { PAL_RED_1 };
                draw_circle_lines(x - 42.0, y - 32.0, 7.0, 1.5, col);
            }
        }

        // Right Arm
        if self.right_arm_broken {
            // Dangling limp broken arm
            draw_line(x + 28.0, y - 48.0 + breathe, x + 34.0, y - 20.0, 6.0, Color::new(0.30, 0.22, 0.35, 1.0));
            draw_line(x + 34.0, y - 20.0, x + 28.0, y + 2.0, 5.0, Color::new(0.30, 0.22, 0.35, 1.0));
            draw_circle(x + 34.0, y - 20.0, 4.0, PAL_RED_1); // Bone fracture
        } else {
            draw_rectangle(x + 26.0, y - 52.0 + breathe, 14.0, 28.0, Color::new(0.36, 0.28, 0.42, 1.0));
            draw_rectangle(x + 28.0, y - 28.0 + breathe, 12.0, 24.0, Color::new(0.32, 0.25, 0.38, 1.0));
            if self.phase == Level6Phase::Phase2HandToHand {
                let blink = (time * 8.0).sin() > 0.0;
                let col = if blink { PAL_GOLD_0 } else { PAL_RED_1 };
                draw_circle_lines(x + 32.0, y - 32.0, 7.0, 1.5, col);
            }
        }

        // Colossal Three-Pronged Iron Trident (Phase 1 only)
        if self.phase == Level6Phase::Phase1Trident {
            let tr_x = x - 40.0;
            let tr_y = y - 40.0 + breathe;
            // Long iron shaft
            draw_line(tr_x, tr_y + 40.0, tr_x - 15.0, tr_y - 45.0, 4.0, Color::new(0.25, 0.25, 0.30, 1.0));
            // 3 Fiery Prongs
            let tip_x = tr_x - 15.0;
            let tip_y = tr_y - 45.0;
            draw_line(tip_x, tip_y, tip_x - 8.0, tip_y - 25.0, 3.0, Color::new(0.8, 0.8, 0.9, 1.0));
            draw_line(tip_x, tip_y, tip_x, tip_y - 30.0, 3.5, Color::new(0.9, 0.9, 1.0, 1.0));
            draw_line(tip_x, tip_y, tip_x + 8.0, tip_y - 25.0, 3.0, Color::new(0.8, 0.8, 0.9, 1.0));

            // Glowing Trident Joint (Weak point target!)
            let joint_pulse = ((time * 6.0).sin() * 0.3 + 0.7).clamp(0.0, 1.0);
            draw_circle(tip_x, tip_y, 6.0, Color::new(1.0, 0.3, 0.1, joint_pulse));
            draw_circle_lines(tip_x, tip_y, 9.0, 1.5, PAL_GOLD_0);
        }

        // Head & Ferocious Demon Face
        let head_y = y - 72.0 + breathe;
        draw_rectangle(x - 16.0, head_y, 30.0, 24.0, Color::new(0.36, 0.28, 0.42, 1.0));

        // Wild Matted Black & Blood-Red Hair
        for i in 0..7 {
            let hx = x - 22.0 + i as f32 * 7.0;
            draw_line(hx, head_y + 2.0, hx - 4.0, head_y - 14.0, 3.5, Color::new(0.12, 0.08, 0.10, 1.0));
        }

        // Glowing Red Demon Eyes
        let eye_glow = ((time * 6.0).sin() * 0.2 + 0.8).clamp(0.0, 1.0);
        draw_rectangle(x - 10.0, head_y + 8.0, 5.0, 3.0, Color::new(1.0, 0.15, 0.15, eye_glow));
        draw_rectangle(x + 2.0, head_y + 8.0, 5.0, 3.0, Color::new(1.0, 0.15, 0.15, eye_glow));

        // Sharp White Tusks curving up from jaw
        draw_line(x - 8.0, head_y + 20.0, x - 10.0, head_y + 13.0, 2.5, WHITE);
        draw_line(x + 6.0, head_y + 20.0, x + 8.0, head_y + 13.0, 2.5, WHITE);
    }

    // Celestial Gandharva Tumburu (Ascension)
    fn draw_tumburu_ascension(&self, x: f32, _time: f32) {
        let rise_y = 230.0 - (self.ascension_timer * 18.0).min(90.0);

        // Divine Radiant Light Pillar from trench
        draw_rectangle(x - 30.0, 0.0, 60.0, 240.0, Color::new(1.0, 0.90, 0.50, 0.25));
        draw_line(x - 20.0, 0.0, x - 20.0, 240.0, 2.0, Color::new(1.0, 0.95, 0.70, 0.4));
        draw_line(x + 20.0, 0.0, x + 20.0, 240.0, 2.0, Color::new(1.0, 0.95, 0.70, 0.4));

        // Celestial Golden Halo
        draw_circle(x, rise_y - 20.0, 22.0, Color::new(1.0, 0.85, 0.35, 0.8));
        draw_circle_lines(x, rise_y - 20.0, 26.0, 1.5, PAL_GOLD_0);

        // Radiant Robes (Pure White and Gold)
        draw_rectangle(x - 12.0, rise_y - 12.0, 24.0, 30.0, WHITE);
        draw_rectangle(x - 10.0, rise_y + 2.0, 20.0, 16.0, PAL_GOLD_0);

        // Head & Golden Crown (Kirita)
        draw_circle(x, rise_y - 20.0, 9.0, Color::new(1.0, 0.88, 0.75, 1.0));
        draw_triangle(
            vec2(x - 7.0, rise_y - 27.0),
            vec2(x, rise_y - 37.0),
            vec2(x + 7.0, rise_y - 27.0),
            PAL_GOLD_0,
        );

        // Celestial Veena in Hand
        draw_line(x - 16.0, rise_y - 5.0, x + 16.0, rise_y - 15.0, 2.5, PAL_GOLD_1);
        draw_circle(x - 14.0, rise_y - 5.0, 5.0, PAL_TURMERIC_1);
        draw_circle(x + 14.0, rise_y - 15.0, 4.0, PAL_TURMERIC_1);
    }

    fn draw_phantom(&self, x: f32, y: f32, time: f32) {
        let hover = (time * 4.0).sin() * 4.0;
        draw_ellipse(x, y + 8.0, 16.0, 6.0, 0.0, Color::new(0.0, 0.0, 0.0, 0.3));
        // Dark spectral body
        draw_circle(x, y - 12.0 + hover, 10.0, Color::new(0.35, 0.15, 0.45, 0.85));
        draw_rectangle(x - 8.0, y - 10.0 + hover, 16.0, 16.0, Color::new(0.25, 0.10, 0.35, 0.85));
        // Glowing purple eyes
        draw_circle(x - 3.0, y - 13.0 + hover, 1.5, PAL_RED_1);
        draw_circle(x + 3.0, y - 13.0 + hover, 1.5, PAL_RED_1);
    }

    fn draw_rama(&self, x: f32, y: f32, _time: f32) {
        // Shadow
        draw_ellipse(x, y + 2.0, 14.0, 5.0, 0.0, Color::new(0.0, 0.0, 0.0, 0.3));

        // Dhoti & Valkala bark garments
        draw_rectangle(x - 5.0, y - 14.0, 10.0, 14.0, PAL_TURMERIC_1);
        draw_rectangle(x - 4.0, y - 26.0, 9.0, 13.0, Color::new(0.70, 0.52, 0.32, 1.0)); // Valkala upper sash

        // Head & Jata (Ascetic hair)
        draw_circle(x, y - 31.0, 5.5, Color::new(0.38, 0.58, 0.72, 1.0)); // Sky-blue skin
        draw_circle(x, y - 37.0, 4.0, Color::new(0.12, 0.08, 0.06, 1.0)); // Jata topknot

        // Celestial Kodanda Bow in Hand
        let bow_x = x + 10.0;
        let bow_y = y - 24.0;
        draw_line(bow_x, bow_y - 16.0, bow_x + 5.0, bow_y, 2.0, PAL_GOLD_0);
        draw_line(bow_x + 5.0, bow_y, bow_x, bow_y + 16.0, 2.0, PAL_GOLD_0);

        // Arrow drawn on bow if holding tension
        if self.is_drawing {
            let pull = self.bow_tension * 8.0;
            draw_line(bow_x - pull, bow_y, bow_x + 12.0, bow_y, 1.5, WHITE);
            draw_line(bow_x, bow_y - 16.0, bow_x - pull, bow_y, 1.0, PAL_GOLD_2);
            draw_line(bow_x, bow_y + 16.0, bow_x - pull, bow_y, 1.0, PAL_GOLD_2);
        }
    }

    fn draw_lakshmana(&self, x: f32, y: f32) {
        draw_ellipse(x, y + 2.0, 12.0, 4.5, 0.0, Color::new(0.0, 0.0, 0.0, 0.25));
        draw_rectangle(x - 4.0, y - 14.0, 8.0, 14.0, PAL_GOLD_2);
        draw_rectangle(x - 4.0, y - 25.0, 8.0, 12.0, Color::new(0.70, 0.52, 0.32, 1.0));
        draw_circle(x, y - 30.0, 5.0, Color::new(0.92, 0.78, 0.62, 1.0));
        draw_circle(x, y - 35.0, 3.5, Color::new(0.15, 0.10, 0.08, 1.0));
        // Bow on shoulder
        draw_line(x - 6.0, y - 35.0, x - 8.0, y - 10.0, 1.5, PAL_GOLD_1);
    }

    fn draw_lakshmana_digging(&self, x: f32, y: f32, time: f32) {
        let dig_cycle = (time * 8.0).sin();
        let dig_y = y + dig_cycle * 3.0;
        draw_ellipse(x, y + 2.0, 12.0, 4.5, 0.0, Color::new(0.0, 0.0, 0.0, 0.25));
        draw_rectangle(x - 4.0, dig_y - 12.0, 8.0, 12.0, PAL_GOLD_2);
        draw_circle(x, dig_y - 22.0, 5.0, Color::new(0.92, 0.78, 0.62, 1.0));
        // Digging implement / spade
        draw_line(x + 2.0, dig_y - 10.0, x + 16.0, 244.0, 2.0, Color::new(0.6, 0.6, 0.7, 1.0));
    }

    fn draw_sita_safe(&self, x: f32, y: f32) {
        draw_ellipse(x, y + 2.0, 11.0, 4.0, 0.0, Color::new(0.0, 0.0, 0.0, 0.2));
        draw_rectangle(x - 4.0, y - 16.0, 8.0, 16.0, Color::new(0.95, 0.55, 0.25, 1.0)); // Saffron ascetic sari
        draw_circle(x, y - 24.0, 4.5, Color::new(0.95, 0.82, 0.68, 1.0));
        draw_circle(x, y - 28.0, 3.5, Color::new(0.10, 0.08, 0.08, 1.0));
    }

    fn draw_arrow(&self, arrow: &ViradhaArrow) {
        let dir = arrow.vel.normalize_or_zero();
        let tail = arrow.pos - dir * 14.0;
        let col = if arrow.is_player {
            match arrow.astra {
                Astra::Agneyastra => PAL_FIRE_1,
                Astra::Vayavyastra => Color::new(0.5, 0.8, 1.0, 1.0),
                Astra::Manavastra => Color::new(0.3, 0.7, 1.0, 1.0),
                _ => COLOR_RAMA_GOLD,
            }
        } else {
            PAL_FIRE_1
        };
        draw_line(tail.x, tail.y, arrow.pos.x, arrow.pos.y, 2.0, col);
        draw_circle(arrow.pos.x, arrow.pos.y, 2.0, WHITE);
    }

    fn draw_level6_hud(&self, font: &Font, stats: &PlayerStats, _time: f32) {
        // 1. Rama Prana & Tejas (Top-left)
        let bar_x = 18.0;
        let bar_y = 12.0;
        let bar_w = 110.0;
        let bar_h = 8.0;

        // Prana (HP)
        draw_rectangle(bar_x, bar_y, bar_w, bar_h, Color::new(0.12, 0.08, 0.10, 0.9));
        let prana_w = (stats.prana as f32 / stats.max_prana as f32).clamp(0.0, 1.0) * (bar_w - 2.0);
        draw_rectangle(bar_x + 1.0, bar_y + 1.0, prana_w, bar_h - 2.0, Color::new(0.85, 0.25, 0.25, 1.0));
        draw_rectangle_lines(bar_x, bar_y, bar_w, bar_h, 1.0, PAL_GOLD_2);
        draw_pixel_text(font, "PRANA", bar_x, bar_y - 2.0, 6, PAL_GOLD_1);

        // Tejas (MP)
        draw_rectangle(bar_x, bar_y + 14.0, bar_w, bar_h, Color::new(0.08, 0.10, 0.14, 0.9));
        let tejas_w = (stats.tejas as f32 / stats.max_tejas as f32).clamp(0.0, 1.0) * (bar_w - 2.0);
        draw_rectangle(bar_x + 1.0, bar_y + 15.0, tejas_w, bar_h - 2.0, Color::new(0.25, 0.65, 0.95, 1.0));
        draw_rectangle_lines(bar_x, bar_y + 14.0, bar_w, bar_h, 1.0, PAL_GOLD_2);
        draw_pixel_text(font, "TEJAS", bar_x, bar_y + 12.0, 6, Color::new(0.4, 0.8, 1.0, 1.0));

        // 2. Boss Health / Objective Bar (Top-Center)
        let boss_x = SCREEN_W * 0.5 - 120.0;
        let boss_y = 12.0;
        let boss_w = 240.0;

        match self.phase {
            Level6Phase::Phase1Trident => {
                draw_rectangle(boss_x, boss_y, boss_w, 10.0, Color::new(0.15, 0.08, 0.10, 0.9));
                let pct = (self.trident_hp / self.max_trident_hp).clamp(0.0, 1.0);
                draw_rectangle(boss_x + 1.0, boss_y + 1.0, (boss_w - 2.0) * pct, 8.0, PAL_FIRE_1);
                draw_rectangle_lines(boss_x, boss_y, boss_w, 10.0, 1.0, PAL_GOLD_1);
                draw_pixel_text(font, "IRON TRIDENT JOINT (AIM AT WEAK POINT!)", boss_x + 12.0, boss_y - 2.0, 6, PAL_GOLD_0);
            }
            Level6Phase::Phase2HandToHand => {
                // Left Arm & Right Arm split bars
                let half_w = boss_w * 0.48;
                // Left Arm
                draw_rectangle(boss_x, boss_y, half_w, 9.0, Color::new(0.15, 0.08, 0.10, 0.9));
                let lp = (self.left_arm_hp / self.max_arm_hp).clamp(0.0, 1.0);
                draw_rectangle(boss_x + 1.0, boss_y + 1.0, (half_w - 2.0) * lp, 7.0, if self.left_arm_broken { PAL_RED_1 } else { PAL_GOLD_1 });
                draw_rectangle_lines(boss_x, boss_y, half_w, 9.0, 1.0, PAL_GOLD_2);
                let l_tag = if self.left_arm_broken { "LEFT ARM [BROKEN]" } else { "LEFT ARM JOINT" };
                draw_pixel_text(font, l_tag, boss_x + 4.0, boss_y - 2.0, 5, PAL_GOLD_0);

                // Right Arm
                let r_bx = boss_x + half_w + boss_w * 0.04;
                draw_rectangle(r_bx, boss_y, half_w, 9.0, Color::new(0.15, 0.08, 0.10, 0.9));
                let rp = (self.right_arm_hp / self.max_arm_hp).clamp(0.0, 1.0);
                draw_rectangle(r_bx + 1.0, boss_y + 1.0, (half_w - 2.0) * rp, 7.0, if self.right_arm_broken { PAL_RED_1 } else { PAL_GOLD_1 });
                draw_rectangle_lines(r_bx, boss_y, half_w, 9.0, 1.0, PAL_GOLD_2);
                let r_tag = if self.right_arm_broken { "RIGHT ARM [BROKEN]" } else { "RIGHT ARM JOINT" };
                draw_pixel_text(font, r_tag, r_bx + 4.0, boss_y - 2.0, 5, PAL_GOLD_0);
            }
            Level6Phase::Phase3Excavation => {
                draw_rectangle(boss_x, boss_y, boss_w, 10.0, Color::new(0.10, 0.15, 0.10, 0.9));
                let tp = (self.trench_progress / 100.0).clamp(0.0, 1.0);
                draw_rectangle(boss_x + 1.0, boss_y + 1.0, (boss_w - 2.0) * tp, 8.0, Color::new(0.35, 0.85, 0.45, 1.0));
                draw_rectangle_lines(boss_x, boss_y, boss_w, 10.0, 1.0, PAL_GOLD_1);
                let text = format!("SACRED EARTH TRENCH: {:.0}% (DEFEND PIT!)", self.trench_progress);
                draw_pixel_text(font, &text, boss_x + 22.0, boss_y - 2.0, 6, PAL_GOLD_0);
            }
            Level6Phase::BurialAndAscension => {
                draw_pixel_text(font, "★ GANDHARVA TUMBURU ASCENSION — CURSE LIBERATED ★", boss_x - 15.0, boss_y + 6.0, 7, PAL_GOLD_0);
            }
            _ => {}
        }
    }
}

// ============================================================================
// LEVEL 6 VICTORY SCREEN
// ============================================================================
pub fn draw_victory_l6(font: &Font, time: f32) {
    draw_rectangle(0.0, 0.0, SCREEN_W, SCREEN_H, Color::new(0.06, 0.05, 0.10, 0.96));

    let sway = (time * 5.0).sin() * 3.0;
    draw_pixel_text(font, "ARANYA KANDA: THE LIBERATION OF VIRADHA!", 65.0, 48.0 + sway, 14, PAL_GOLD_0);

    let sub = "DANDAKARANYA PRIMORDIAL FOREST TRIAL CONQUERED";
    draw_pixel_text(font, sub, 120.0, 70.0, 7, PAL_TURMERIC_1);

    let rw_x = 45.0;
    let rw_y = 88.0;
    let rw_w = 550.0;
    let rw_h = 192.0;
    draw_rectangle(rw_x, rw_y, rw_w, rw_h, Color::new(0.10, 0.09, 0.16, 0.95));
    draw_rectangle_lines(rw_x, rw_y, rw_w, rw_h, 1.5, PAL_GOLD_1);

    draw_pixel_text(font, "CHAPTER 6 MILESTONES COMPLETED (DANDAKARANYA TRIAL):", rw_x + 16.0, rw_y + 20.0, 8, PAL_GOLD_0);
    draw_pixel_text(font, "✓ Shattered Iron Trident : Severed weapon joint to disarm the towering giant.", rw_x + 16.0, rw_y + 40.0, 7, WHITE);
    draw_pixel_text(font, "✓ Broken Demon Arms     : Bypassed Brahma's weapon immunity via joint grapple.", rw_x + 16.0, rw_y + 60.0, 7, WHITE);
    draw_pixel_text(font, "✓ Sacred Earth Trench   : Lakshmana & Rama excavated the pit to bury Viradha.", rw_x + 16.0, rw_y + 80.0, 7, PAL_GOLD_1);
    draw_pixel_text(font, "✓ Redemption of Tumburu : Celestial Gandharva liberated from Kubera's curse!", rw_x + 16.0, rw_y + 100.0, 7, Color::new(0.4, 0.9, 0.5, 1.0));

    // Next Major Battle Teaser
    draw_line(rw_x + 16.0, rw_y + 118.0, rw_x + rw_w - 16.0, rw_y + 118.0, 1.0, PAL_GOLD_3);
    draw_pixel_text(font, "★ NEXT: LEVEL 7 — PANCHAVATI & THE 14,000 RAKSHASA HOST ★", rw_x + 16.0, rw_y + 135.0, 8, PAL_FIRE_1);
    draw_pixel_text(font, "At sacred Panchavati on the Godavari, demon generals Khara, Dushana,", rw_x + 16.0, rw_y + 152.0, 7, Color::new(0.9, 0.8, 0.8, 1.0));
    draw_pixel_text(font, "and Trishiras march with 14,000 Rakshasas! Rama stands alone as Eka-Vira!", rw_x + 16.0, rw_y + 167.0, 7, Color::new(0.9, 0.8, 0.8, 1.0));

    let blink = (time * 4.0).sin() > 0.0;
    if blink {
        draw_pixel_text(font, "PRESS SPACE OR CLICK TO RETURN TO CHAPTER SELECT ▶", 90.0, 305.0, 9, PAL_GOLD_0);
    }
    draw_pixel_text(font, "Press [R] to Replay Chapter 6 (Ambush of Viradha)", 150.0, 330.0, 8, Color::new(0.7, 0.7, 0.7, 1.0));
}
