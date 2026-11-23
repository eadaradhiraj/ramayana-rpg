use macroquad::prelude::*;
use crate::audio::SoundEffects;
use crate::render::*;
use crate::types::{Astra, AstraProjectileType, PlayerStats};

#[derive(Clone, Copy)]
pub struct SimpleRng(pub u32);

impl SimpleRng {
    pub fn next(&mut self) -> f32 {
        self.0 = self.0.wrapping_mul(1664525).wrapping_add(1013904223);
        ((self.0 >> 16) as f32) / 65536.0
    }
}

pub const ALTAR_POS: Vec2 = vec2(320.0, 180.0);

#[allow(dead_code)]
pub struct AstraProjectile {
    pub pos: Vec2,
    pub vel: Vec2,
    pub p_type: AstraProjectileType,
    pub lifetime: f32,
    pub max_lifetime: f32,
    pub pierce_count: i32,
}

#[allow(dead_code)]
pub struct BloodBomb {
    pub pos: Vec2,
    pub target_pos: Vec2,
    pub altitude: f32,
    pub max_altitude: f32,
    pub timer: f32,
    pub max_time: f32,
}

#[allow(dead_code)]
pub struct DarkSpike {
    pub pos: Vec2,
    pub vel: Vec2,
    pub lifetime: f32,
}

#[allow(dead_code)]
pub struct RakshasaMinion {
    pub pos: Vec2,
    pub speed: f32,
    pub hp: f32,
    pub hit_flash: f32,
    pub is_alive: bool,
}

#[allow(dead_code)]
pub struct WingedAsura {
    pub pos: Vec2,
    pub angle: f32,
    pub hp: f32,
    pub hit_flash: f32,
    pub bomb_timer: f32,
    pub is_alive: bool,
}

#[allow(dead_code)]
pub struct MarichaBoss {
    pub pos: Vec2,
    pub vel: Vec2,
    pub hp: f32,
    pub max_hp: f32,
    pub hit_flash: f32,
    pub attack_timer: f32,
    pub teleport_timer: f32,
    pub is_active: bool,
    pub is_defeated: bool,
}

#[allow(dead_code)]
pub struct SubahuBoss {
    pub pos: Vec2,
    pub speed: f32,
    pub hp: f32,
    pub max_hp: f32,
    pub hit_flash: f32,
    pub has_miasma_shield: bool,
    pub shield_regen_timer: f32,
    pub is_active: bool,
    pub is_defeated: bool,
}

pub struct Level2Fight {
    pub rama_pos: Vec2,
    pub rama_speed: f32,
    pub rama_bow_cooldown: f32,
    pub rama_invuln_timer: f32,
    pub yajna_purity: f32,
    pub wave: i32,
    pub wave_timer: f32,
    pub spawn_timer: f32,
    pub minions_to_spawn: i32,
    pub asuras_to_spawn: i32,
    pub minions: Vec<RakshasaMinion>,
    pub asuras: Vec<WingedAsura>,
    pub maricha: MarichaBoss,
    pub subahu: SubahuBoss,
    pub astra_projectiles: Vec<AstraProjectile>,
    pub blood_bombs: Vec<BloodBomb>,
    pub dark_spikes: Vec<DarkSpike>,
    pub particles: Vec<Particle>,
    pub floating_texts: Vec<FloatingText>,
    pub screen_shake_time: f32,
    pub screen_shake_intensity: f32,
    pub screen_flash_timer: f32,
    pub victory_delay: f32,
    pub is_victory: bool,
    pub time_elapsed: f32,
    pub rng: SimpleRng,
}

impl Level2Fight {
    pub fn new() -> Self {
        Self {
            rama_pos: vec2(320.0, 240.0),
            rama_speed: 185.0,
            rama_bow_cooldown: 0.0,
            rama_invuln_timer: 0.0,
            yajna_purity: 100.0,
            wave: 1,
            wave_timer: 0.0,
            spawn_timer: 1.0,
            minions_to_spawn: 14,
            asuras_to_spawn: 0,
            minions: Vec::new(),
            asuras: Vec::new(),
            maricha: MarichaBoss {
                pos: vec2(150.0, 100.0),
                vel: vec2(40.0, 20.0),
                hp: 130.0,
                max_hp: 130.0,
                hit_flash: 0.0,
                attack_timer: 2.0,
                teleport_timer: 5.0,
                is_active: false,
                is_defeated: false,
            },
            subahu: SubahuBoss {
                pos: vec2(490.0, 90.0),
                speed: 42.0,
                hp: 150.0,
                max_hp: 150.0,
                hit_flash: 0.0,
                has_miasma_shield: true,
                shield_regen_timer: 0.0,
                is_active: false,
                is_defeated: false,
            },
            astra_projectiles: Vec::new(),
            blood_bombs: Vec::new(),
            dark_spikes: Vec::new(),
            particles: Vec::new(),
            floating_texts: Vec::new(),
            screen_shake_time: 0.0,
            screen_shake_intensity: 0.0,
            screen_flash_timer: 0.0,
            victory_delay: 0.0,
            is_victory: false,
            time_elapsed: 0.0,
            rng: SimpleRng(987654),
        }
    }

    pub fn trigger_shake(&mut self, intensity: f32, duration: f32) {
        self.screen_shake_intensity = intensity;
        self.screen_shake_time = duration;
    }

    pub fn update(
        &mut self,
        dt: f32,
        virtual_mouse: Vec2,
        stats: &mut PlayerStats,
        sounds: &SoundEffects,
    ) -> Option<FightResultL2> {
        let mut rng = self.rng;
        self.time_elapsed += dt;

        // Shake & flash decay
        if self.screen_shake_time > 0.0 {
            self.screen_shake_time -= dt;
        }
        if self.screen_flash_timer > 0.0 {
            self.screen_flash_timer -= dt;
        }

        // Tejas (MP) Passive Regeneration: +12 per second up to max
        if stats.tejas < stats.max_tejas {
            let regen = (12.0 * dt) as i32;
            stats.tejas = (stats.tejas + regen.max(1)).min(stats.max_tejas);
        }

        // Check Victory
        if self.is_victory {
            self.victory_delay += dt;
            if self.victory_delay >= 1.5 {
                self.rng = rng;
                return Some(FightResultL2::Victory);
            }
        }

        // Check Defeat (Rama Dead or Altar Defiled)
        if stats.prana <= 0 || self.yajna_purity <= 0.0 {
            sounds.play(&sounds.game_over, 0.9);
            self.rng = rng;
            return Some(FightResultL2::Defeat);
        }

        // 1. Rama Movement (WASD + Arrows)
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

        if move_dir.length_squared() > 0.0 {
            move_dir = move_dir.normalize();
            self.rama_pos += move_dir * self.rama_speed * dt;
        }
        self.rama_pos.x = self.rama_pos.x.clamp(ARENA_MIN_X + 14.0, ARENA_MAX_X - 14.0);
        self.rama_pos.y = self.rama_pos.y.clamp(ARENA_MIN_Y + 14.0, ARENA_MAX_Y - 14.0);

        if self.rama_invuln_timer > 0.0 {
            self.rama_invuln_timer -= dt;
        }
        if self.rama_bow_cooldown > 0.0 {
            self.rama_bow_cooldown -= dt;
        }

        // 2. Astra Selection (Keys 1-4)
        if is_key_pressed(KeyCode::Key1) {
            stats.selected_astra = Astra::KodandaBow;
        } else if is_key_pressed(KeyCode::Key2) {
            stats.selected_astra = Astra::Vayavyastra;
        } else if is_key_pressed(KeyCode::Key3) {
            stats.selected_astra = Astra::Manavastra;
        } else if is_key_pressed(KeyCode::Key4) {
            stats.selected_astra = Astra::Agneyastra;
        }

        // 3. Aiming & Firing Selected Astra
        let aim_vec = virtual_mouse - self.rama_pos;
        let aim_dir = if aim_vec.length_squared() > 1.0 {
            aim_vec.normalize()
        } else {
            vec2(1.0, 0.0)
        };

        let is_firing = (is_mouse_button_down(MouseButton::Left) || is_key_down(KeyCode::Space))
            && self.rama_bow_cooldown <= 0.0
            && !self.is_victory;

        if is_firing {
            match stats.selected_astra {
                Astra::KodandaBow => {
                    self.rama_bow_cooldown = 0.28;
                    let spawn_pos = self.rama_pos + aim_dir * 10.0;
                    self.astra_projectiles.push(AstraProjectile {
                        pos: spawn_pos,
                        vel: aim_dir * 500.0,
                        p_type: AstraProjectileType::Kodanda,
                        lifetime: 1.4,
                        max_lifetime: 1.4,
                        pierce_count: 1,
                    });
                    sounds.play(&sounds.shoot, 0.7);
                }
                Astra::Vayavyastra => {
                    if stats.tejas >= 20 {
                        stats.tejas -= 20;
                        self.rama_bow_cooldown = 0.40;
                        sounds.play(&sounds.wind, 0.85);
                        self.trigger_shake(3.0, 0.15);

                        // Fire 3 spiraling gale whirlwinds
                        for angle_offset in [-0.22f32, 0.0, 0.22] {
                            let cos_a = angle_offset.cos();
                            let sin_a = angle_offset.sin();
                            let dir = vec2(
                                aim_dir.x * cos_a - aim_dir.y * sin_a,
                                aim_dir.x * sin_a + aim_dir.y * cos_a,
                            );
                            self.astra_projectiles.push(AstraProjectile {
                                pos: self.rama_pos + dir * 10.0,
                                vel: dir * 420.0,
                                p_type: AstraProjectileType::Vayavya,
                                lifetime: 1.3,
                                max_lifetime: 1.3,
                                pierce_count: 5, // Pierces multiple enemies
                            });
                        }
                    } else {
                        self.floating_texts.push(FloatingText {
                            pos: self.rama_pos + vec2(-30.0, -25.0),
                            text: "NEED 20 TEJAS!".to_string(),
                            color: Color::new(0.4, 0.8, 1.0, 1.0),
                            lifetime: 0.8,
                            max_lifetime: 0.8,
                            font_size: 8,
                        });
                    }
                }
                Astra::Manavastra => {
                    if stats.tejas >= 25 {
                        stats.tejas -= 25;
                        self.rama_bow_cooldown = 0.48;
                        sounds.play(&sounds.homing, 0.9);
                        self.trigger_shake(3.5, 0.15);

                        self.astra_projectiles.push(AstraProjectile {
                            pos: self.rama_pos + aim_dir * 12.0,
                            vel: aim_dir * 380.0,
                            p_type: AstraProjectileType::Manava,
                            lifetime: 2.5,
                            max_lifetime: 2.5,
                            pierce_count: 1,
                        });
                    } else {
                        self.floating_texts.push(FloatingText {
                            pos: self.rama_pos + vec2(-30.0, -25.0),
                            text: "NEED 25 TEJAS!".to_string(),
                            color: Color::new(0.4, 0.8, 1.0, 1.0),
                            lifetime: 0.8,
                            max_lifetime: 0.8,
                            font_size: 8,
                        });
                    }
                }
                Astra::Agneyastra => {
                    if stats.tejas >= 25 {
                        stats.tejas -= 25;
                        self.rama_bow_cooldown = 0.48;
                        sounds.play(&sounds.fire, 0.85);
                        self.trigger_shake(4.0, 0.18);

                        self.astra_projectiles.push(AstraProjectile {
                            pos: self.rama_pos + aim_dir * 12.0,
                            vel: aim_dir * 440.0,
                            p_type: AstraProjectileType::Agneya,
                            lifetime: 1.5,
                            max_lifetime: 1.5,
                            pierce_count: 1,
                        });
                    } else {
                        self.floating_texts.push(FloatingText {
                            pos: self.rama_pos + vec2(-30.0, -25.0),
                            text: "NEED 25 TEJAS!".to_string(),
                            color: Color::new(1.0, 0.6, 0.2, 1.0),
                            lifetime: 0.8,
                            max_lifetime: 0.8,
                            font_size: 8,
                        });
                    }
                }
                _ => {}
            }
        }

        // 4. Wave Management & Spawning
        self.wave_timer += dt;
        self.spawn_timer -= dt;

        if self.spawn_timer <= 0.0 {
            self.spawn_timer = match self.wave {
                1 => 1.4,
                2 => 1.2,
                _ => 1.8,
            };

            // Spawn Rakshasa Minion if remaining
            if self.minions_to_spawn > 0 {
                self.minions_to_spawn -= 1;
                // Random border position
                let side = (rng.next() * 4.0) as i32;
                let spawn_p = match side {
                    0 => vec2(ARENA_MIN_X - 10.0, 60.0 + rng.next() * 240.0), // Left
                    1 => vec2(ARENA_MAX_X + 10.0, 60.0 + rng.next() * 240.0), // Right
                    2 => vec2(60.0 + rng.next() * 520.0, ARENA_MIN_Y - 10.0), // Top
                    _ => vec2(60.0 + rng.next() * 520.0, ARENA_MAX_Y + 10.0), // Bottom
                };
                self.minions.push(RakshasaMinion {
                    pos: spawn_p,
                    speed: 75.0 + rng.next() * 30.0,
                    hp: 30.0,
                    hit_flash: 0.0,
                    is_alive: true,
                });
            }

            // Spawn Winged Asura if remaining (Wave 2)
            if self.asuras_to_spawn > 0 {
                self.asuras_to_spawn -= 1;
                let ax = if rng.next() > 0.5 { ARENA_MIN_X } else { ARENA_MAX_X };
                self.asuras.push(WingedAsura {
                    pos: vec2(ax, 70.0 + rng.next() * 80.0),
                    angle: rng.next() * std::f32::consts::TAU,
                    hp: 40.0,
                    hit_flash: 0.0,
                    bomb_timer: 1.5 + rng.next() * 2.0,
                    is_alive: true,
                });
            }
        }

        // Wave Transitions
        if self.wave == 1 && self.minions_to_spawn == 0 && self.minions.is_empty() {
            self.wave = 2;
            self.minions_to_spawn = 16;
            self.asuras_to_spawn = 4;
            self.spawn_timer = 2.0;
            self.floating_texts.push(FloatingText {
                pos: vec2(SCREEN_W * 0.5 - 120.0, 100.0),
                text: "WAVE 2: WINGED BLOOD-THROWERS!".to_string(),
                color: Color::new(1.0, 0.4, 0.2, 1.0),
                lifetime: 2.2,
                max_lifetime: 2.2,
                font_size: 9,
            });
        } else if self.wave == 2 && self.minions_to_spawn == 0 && self.minions.is_empty() && self.asuras.is_empty() {
            self.wave = 3;
            self.maricha.is_active = true;
            self.subahu.is_active = true;
            self.minions_to_spawn = 12; // supporting minions
            self.trigger_shake(8.0, 0.8);
            self.screen_flash_timer = 0.3;
            sounds.play(&sounds.roar, 1.0);

            self.floating_texts.push(FloatingText {
                pos: vec2(SCREEN_W * 0.5 - 150.0, 90.0),
                text: "BOSS WAVE: MARICHA & SUBAHU ARRIVE!".to_string(),
                color: Color::new(1.0, 0.15, 0.15, 1.0),
                lifetime: 3.0,
                max_lifetime: 3.0,
                font_size: 10,
            });
        }

        // 5. Update Minions
        let mut defiled_damage = 0.0f32;
        for m in &mut self.minions {
            if !m.is_alive {
                continue;
            }
            if m.hit_flash > 0.0 {
                m.hit_flash -= dt;
            }

            // Move directly towards Yajna Altar
            let to_altar = ALTAR_POS - m.pos;
            let dist = to_altar.length();
            if dist > 18.0 {
                m.pos += (to_altar / dist) * m.speed * dt;
            } else {
                // Reached altar! Sacrifice to defile
                m.is_alive = false;
                defiled_damage += 8.0;
            }
        }
        self.minions.retain(|m| m.is_alive);

        if defiled_damage > 0.0 {
            self.yajna_purity = (self.yajna_purity - defiled_damage).max(0.0);
            self.trigger_shake(4.0, 0.2);
            sounds.play(&sounds.altar_hiss, 0.85);

            self.floating_texts.push(FloatingText {
                pos: ALTAR_POS + vec2(-45.0, -25.0),
                text: format!("-{:.0} YAJNA PURITY!", defiled_damage),
                color: Color::new(1.0, 0.2, 0.2, 1.0),
                lifetime: 1.2,
                max_lifetime: 1.2,
                font_size: 9,
            });
        }

        // 6. Update Winged Asuras
        for a in &mut self.asuras {
            if !a.is_alive {
                continue;
            }
            if a.hit_flash > 0.0 {
                a.hit_flash -= dt;
            }

            a.angle += 1.5 * dt;
            a.pos.x += a.angle.cos() * 85.0 * dt;
            a.pos.y = 80.0 + (a.angle * 1.8).sin() * 25.0;

            a.bomb_timer -= dt;
            if a.bomb_timer <= 0.0 {
                a.bomb_timer = 3.5 + rng.next() * 1.5;
                // Target either the altar or Rama
                let target = if rng.next() > 0.5 { ALTAR_POS } else { self.rama_pos };
                self.blood_bombs.push(BloodBomb {
                    pos: a.pos,
                    target_pos: target,
                    altitude: 1.0,
                    max_altitude: 1.0,
                    timer: 0.0,
                    max_time: 1.4,
                });
            }
        }
        self.asuras.retain(|a| a.is_alive);

        // 7. Update Blood Bombs
        let mut bomb_hits = Vec::new();
        for b in &mut self.blood_bombs {
            b.timer += dt;
            let t = b.timer / b.max_time;
            b.pos = b.pos.lerp(b.target_pos, t);
            b.altitude = (1.0 - t).max(0.0);

            if b.timer >= b.max_time {
                bomb_hits.push(b.target_pos);
            }
        }
        self.blood_bombs.retain(|b| b.timer < b.max_time);

        for hit_pos in bomb_hits {
            self.trigger_shake(4.0, 0.2);
            sounds.play(&sounds.altar_hiss, 0.8);

            // Blood splat particles
            for _ in 0..12 {
                let vel = vec2((rng.next() - 0.5) * 80.0, (rng.next() - 0.5) * 80.0);
                self.particles.push(Particle {
                    pos: hit_pos,
                    vel,
                    color: Color::new(0.65, 0.05, 0.12, 0.9),
                    size: 3.5,
                    lifetime: 0.4,
                    max_lifetime: 0.4,
                });
            }

            // Damage Altar if close
            if (hit_pos - ALTAR_POS).length() < 30.0 {
                self.yajna_purity = (self.yajna_purity - 10.0).max(0.0);
                self.floating_texts.push(FloatingText {
                    pos: ALTAR_POS + vec2(-40.0, -25.0),
                    text: "-10 PURITY (BOMB)!".to_string(),
                    color: Color::new(1.0, 0.2, 0.2, 1.0),
                    lifetime: 1.2,
                    max_lifetime: 1.2,
                    font_size: 9,
                });
            }

            // Damage Rama if close
            if (hit_pos - self.rama_pos).length() < 24.0 && self.rama_invuln_timer <= 0.0 {
                stats.prana -= 20;
                self.rama_invuln_timer = 0.8;
                sounds.play(&sounds.hurt, 0.8);
                self.floating_texts.push(FloatingText {
                    pos: self.rama_pos + vec2(-15.0, -20.0),
                    text: "-20 HP".to_string(),
                    color: Color::new(1.0, 0.2, 0.2, 1.0),
                    lifetime: 1.0,
                    max_lifetime: 1.0,
                    font_size: 9,
                });
            }
        }

        // 8. Update Maricha Boss AI
        if self.maricha.is_active && !self.maricha.is_defeated {
            if self.maricha.hit_flash > 0.0 {
                self.maricha.hit_flash -= dt;
            }

            // Floats and drifts
            self.maricha.pos += self.maricha.vel * dt;
            if self.maricha.pos.x < ARENA_MIN_X + 30.0 || self.maricha.pos.x > ARENA_MAX_X - 30.0 {
                self.maricha.vel.x = -self.maricha.vel.x;
            }
            if self.maricha.pos.y < ARENA_MIN_Y + 20.0 || self.maricha.pos.y > 150.0 {
                self.maricha.vel.y = -self.maricha.vel.y;
            }

            // Casts dark illusion spikes
            self.maricha.attack_timer -= dt;
            if self.maricha.attack_timer <= 0.0 {
                self.maricha.attack_timer = 2.8;
                let to_target = (self.rama_pos - self.maricha.pos).normalize_or_zero();
                self.dark_spikes.push(DarkSpike {
                    pos: self.maricha.pos,
                    vel: to_target * 220.0,
                    lifetime: 2.2,
                });
                sounds.play(&sounds.shoot, 0.5);
            }
        }

        // 9. Update Subahu Boss AI
        if self.subahu.is_active && !self.subahu.is_defeated {
            if self.subahu.hit_flash > 0.0 {
                self.subahu.hit_flash -= dt;
            }

            // Marches steadily straight towards the Yajna Altar!
            let to_altar = ALTAR_POS - self.subahu.pos;
            let dist = to_altar.length();
            if dist > 24.0 {
                self.subahu.pos += (to_altar / dist) * self.subahu.speed * dt;
            } else {
                // Reached altar with cauldron!
                self.yajna_purity = (self.yajna_purity - 25.0).max(0.0);
                self.trigger_shake(8.0, 0.4);
                sounds.play(&sounds.altar_hiss, 1.0);
                self.subahu.pos -= (to_altar / dist) * 40.0; // Pushed back slightly after dump

                self.floating_texts.push(FloatingText {
                    pos: ALTAR_POS + vec2(-60.0, -30.0),
                    text: "-25 PURITY! CAULDRON DUMP!".to_string(),
                    color: Color::new(1.0, 0.15, 0.15, 1.0),
                    lifetime: 1.5,
                    max_lifetime: 1.5,
                    font_size: 9,
                });
            }

            // Miasma shield regeneration timer
            if !self.subahu.has_miasma_shield {
                self.subahu.shield_regen_timer -= dt;
                if self.subahu.shield_regen_timer <= 0.0 {
                    self.subahu.has_miasma_shield = true;
                    self.floating_texts.push(FloatingText {
                        pos: self.subahu.pos + vec2(-45.0, -30.0),
                        text: "MIASMA SHIELD RESTORED!".to_string(),
                        color: Color::new(0.7, 0.2, 0.9, 1.0),
                        lifetime: 1.2,
                        max_lifetime: 1.2,
                        font_size: 8,
                    });
                }
            }
        }

        // 10. Update Dark Spikes (Maricha's attack)
        let mut spike_hit_rama = false;
        for spike in &mut self.dark_spikes {
            spike.pos += spike.vel * dt;
            spike.lifetime -= dt;

            // Damage Rama
            if (spike.pos - self.rama_pos).length() < 16.0 && self.rama_invuln_timer <= 0.0 {
                spike.lifetime = 0.0;
                spike_hit_rama = true;
            }
        }
        self.dark_spikes.retain(|s| s.lifetime > 0.0);

        if spike_hit_rama {
            stats.prana -= 15;
            self.rama_invuln_timer = 0.8;
            self.trigger_shake(4.0, 0.15);
            sounds.play(&sounds.hurt, 0.8);

            self.floating_texts.push(FloatingText {
                pos: self.rama_pos + vec2(-15.0, -20.0),
                text: "-15 HP (DARK SPIKE)".to_string(),
                color: Color::new(0.8, 0.3, 0.9, 1.0),
                lifetime: 1.0,
                max_lifetime: 1.0,
                font_size: 8,
            });
        }

        // 11. Update Astra Projectiles & Collision with Enemies
        let mut shake_request: Option<(f32, f32)> = None;
        for proj in &mut self.astra_projectiles {
            proj.lifetime -= dt;

            // Manavastra Homing Physics: Steers toward Maricha if active, else nearest enemy!
            if proj.p_type == AstraProjectileType::Manava {
                let target_pos = if self.maricha.is_active && !self.maricha.is_defeated {
                    Some(self.maricha.pos)
                } else if let Some(m) = self.minions.first() {
                    Some(m.pos)
                } else {
                    None
                };

                if let Some(target) = target_pos {
                    let desired_dir = (target - proj.pos).normalize_or_zero();
                    let current_dir = proj.vel.normalize_or_zero();
                    let new_dir = current_dir.lerp(desired_dir, 6.0 * dt).normalize();
                    proj.vel = new_dir * 420.0;
                }
            }

            proj.pos += proj.vel * dt;

            // Projectile trail particles
            let p_col = match proj.p_type {
                AstraProjectileType::Kodanda => COLOR_RAMA_GOLD,
                AstraProjectileType::Vayavya => Color::new(0.6, 0.95, 1.0, 0.7),
                AstraProjectileType::Manava => Color::new(0.3, 0.7, 1.0, 0.9),
                AstraProjectileType::Agneya => Color::new(1.0, 0.45, 0.1, 0.9),
            };
            self.particles.push(Particle {
                pos: proj.pos,
                vel: -proj.vel * 0.06,
                color: p_col,
                size: 2.5,
                lifetime: 0.12,
                max_lifetime: 0.12,
            });

            // Collisions with Minions
            for m in &mut self.minions {
                if !m.is_alive || proj.pierce_count <= 0 {
                    continue;
                }
                if (proj.pos - m.pos).length() < 18.0 {
                    proj.pierce_count -= 1;
                    m.hit_flash = 0.15;
                    sounds.play(&sounds.hit, 0.8);

                    let dmg = match proj.p_type {
                        AstraProjectileType::Vayavya => {
                            // Wind sweeps minions away!
                            m.pos += proj.vel.normalize_or_zero() * 45.0;
                            45.0
                        }
                        AstraProjectileType::Agneya => 50.0,
                        _ => 30.0,
                    };
                    m.hp -= dmg;
                    if m.hp <= 0.0 {
                        m.is_alive = false;
                        for _ in 0..8 {
                            let vel = vec2((rng.next() - 0.5) * 80.0, (rng.next() - 0.5) * 80.0);
                            self.particles.push(Particle {
                                pos: m.pos,
                                vel,
                                color: Color::new(0.3, 0.25, 0.35, 0.8),
                                size: 3.0,
                                lifetime: 0.3,
                                max_lifetime: 0.3,
                            });
                        }
                    }
                }
            }

            // Collisions with Winged Asuras
            for a in &mut self.asuras {
                if !a.is_alive || proj.pierce_count <= 0 {
                    continue;
                }
                if (proj.pos - a.pos).length() < 20.0 {
                    proj.pierce_count -= 1;
                    a.hit_flash = 0.15;
                    sounds.play(&sounds.hit, 0.8);
                    a.hp -= 35.0;
                    if a.hp <= 0.0 {
                        a.is_alive = false;
                        for _ in 0..10 {
                            let vel = vec2((rng.next() - 0.5) * 100.0, (rng.next() - 0.5) * 100.0);
                            self.particles.push(Particle {
                                pos: a.pos,
                                vel,
                                color: Color::new(0.5, 0.1, 0.15, 0.8),
                                size: 3.5,
                                lifetime: 0.35,
                                max_lifetime: 0.35,
                            });
                        }
                    }
                }
            }

            // Collision with Maricha Boss (Vulnerable to Manavastra!)
            if self.maricha.is_active && !self.maricha.is_defeated && proj.pierce_count > 0 {
                if (proj.pos - self.maricha.pos).length() < 26.0 {
                    proj.pierce_count = 0;
                    self.maricha.hit_flash = 0.2;

                    if proj.p_type == AstraProjectileType::Manava {
                        // MANAVASTRA COUNTERS MARICHA: Flings him 180px backwards!
                        let push_dir = proj.vel.normalize_or_zero();
                        self.maricha.pos += push_dir * 180.0;
                        self.maricha.pos.x = self.maricha.pos.x.clamp(ARENA_MIN_X + 30.0, ARENA_MAX_X - 30.0);
                        self.maricha.hp -= 70.0;
                        shake_request = Some((6.0, 0.25));
                        sounds.play(&sounds.smash, 0.9);

                        self.floating_texts.push(FloatingText {
                            pos: self.maricha.pos + vec2(-45.0, -25.0),
                            text: "MANAVASTRA FLINGS MARICHA! -70 HP".to_string(),
                            color: Color::new(0.3, 0.8, 1.0, 1.0),
                            lifetime: 1.8,
                            max_lifetime: 1.8,
                            font_size: 9,
                        });
                    } else {
                        // Normal attacks barely work & trigger teleport
                        self.maricha.hp -= 15.0;
                        self.maricha.pos = vec2(
                            ARENA_MIN_X + 40.0 + rng.next() * 500.0,
                            ARENA_MIN_Y + 30.0 + rng.next() * 100.0,
                        );
                        sounds.play(&sounds.hit, 0.7);
                        self.floating_texts.push(FloatingText {
                            pos: self.maricha.pos + vec2(-30.0, -20.0),
                            text: "TELEPORT! -15 HP".to_string(),
                            color: Color::new(0.7, 0.5, 0.9, 1.0),
                            lifetime: 1.0,
                            max_lifetime: 1.0,
                            font_size: 8,
                        });
                    }

                    if self.maricha.hp <= 0.0 {
                        self.maricha.is_defeated = true;
                        shake_request = Some((9.0, 0.6));
                        sounds.play(&sounds.roar, 1.0);

                        self.floating_texts.push(FloatingText {
                            pos: vec2(SCREEN_W * 0.5 - 160.0, 80.0),
                            text: "MARICHA FLUNG 100 YOJANAS INTO THE OCEAN!".to_string(),
                            color: Color::new(0.3, 0.85, 1.0, 1.0),
                            lifetime: 3.0,
                            max_lifetime: 3.0,
                            font_size: 9,
                        });
                    }
                }
            }

            // Collision with Subahu Boss (Vulnerable to Agneyastra!)
            if self.subahu.is_active && !self.subahu.is_defeated && proj.pierce_count > 0 {
                if (proj.pos - self.subahu.pos).length() < 28.0 {
                    proj.pierce_count = 0;
                    self.subahu.hit_flash = 0.2;

                    if proj.p_type == AstraProjectileType::Agneya {
                        // AGNEYASTRA COUNTERS SUBAHU: Shatters miasma shield & incinerates!
                        self.subahu.has_miasma_shield = false;
                        self.subahu.shield_regen_timer = 6.0;
                        self.subahu.hp -= 75.0;
                        shake_request = Some((7.0, 0.3));
                        sounds.play(&sounds.fire, 1.0);

                        for _ in 0..20 {
                            let vel = vec2((rng.next() - 0.5) * 140.0, (rng.next() - 0.5) * 140.0);
                            self.particles.push(Particle {
                                pos: self.subahu.pos,
                                vel,
                                color: Color::new(1.0, 0.4, 0.1, 1.0),
                                size: 4.0,
                                lifetime: 0.45,
                                max_lifetime: 0.45,
                            });
                        }

                        self.floating_texts.push(FloatingText {
                            pos: self.subahu.pos + vec2(-50.0, -25.0),
                            text: "HOLY FIRE! SUBAHU INCINERATED! -75 HP".to_string(),
                            color: Color::new(1.0, 0.5, 0.1, 1.0),
                            lifetime: 1.8,
                            max_lifetime: 1.8,
                            font_size: 9,
                        });
                    } else if self.subahu.has_miasma_shield {
                        // Shield blocks most damage
                        self.subahu.hp -= 6.0;
                        sounds.play(&sounds.hit, 0.6);
                        self.floating_texts.push(FloatingText {
                            pos: self.subahu.pos + vec2(-30.0, -20.0),
                            text: "SHIELDED! -6 HP".to_string(),
                            color: Color::new(0.6, 0.3, 0.8, 1.0),
                            lifetime: 0.9,
                            max_lifetime: 0.9,
                            font_size: 8,
                        });
                    } else {
                        // Shield down
                        self.subahu.hp -= 25.0;
                        sounds.play(&sounds.hit, 0.8);
                        self.floating_texts.push(FloatingText {
                            pos: self.subahu.pos + vec2(-20.0, -20.0),
                            text: "-25 HP".to_string(),
                            color: Color::new(1.0, 0.3, 0.3, 1.0),
                            lifetime: 0.9,
                            max_lifetime: 0.9,
                            font_size: 9,
                        });
                    }

                    if self.subahu.hp <= 0.0 {
                        self.subahu.is_defeated = true;
                        shake_request = Some((9.0, 0.6));
                        sounds.play(&sounds.fire, 1.0);

                        for _ in 0..40 {
                            let vel = vec2((rng.next() - 0.5) * 160.0, (rng.next() - 0.5) * 160.0);
                            self.particles.push(Particle {
                                pos: self.subahu.pos,
                                vel,
                                color: Color::new(1.0, 0.8, 0.2, 1.0),
                                size: 4.5,
                                lifetime: 0.6,
                                max_lifetime: 0.6,
                            });
                        }

                        self.floating_texts.push(FloatingText {
                            pos: vec2(SCREEN_W * 0.5 - 140.0, 110.0),
                            text: "SUBAHU REDUCED TO ASHES BY AGNI!".to_string(),
                            color: Color::new(1.0, 0.8, 0.2, 1.0),
                            lifetime: 3.0,
                            max_lifetime: 3.0,
                            font_size: 9,
                        });
                    }
                }
            }
        }
        if let Some((shk_i, shk_d)) = shake_request {
            self.trigger_shake(shk_i, shk_d);
        }
        self.astra_projectiles.retain(|p| {
            p.lifetime > 0.0
                && p.pierce_count > 0
                && p.pos.x >= ARENA_MIN_X
                && p.pos.x <= ARENA_MAX_X
                && p.pos.y >= ARENA_MIN_Y
                && p.pos.y <= ARENA_MAX_Y
        });

        // 12. Check Victory Condition: Wave 3 bosses defeated!
        if self.wave == 3
            && self.maricha.is_defeated
            && self.subahu.is_defeated
            && self.minions.is_empty()
            && !self.is_victory
        {
            self.is_victory = true;
            self.trigger_shake(10.0, 0.8);
            sounds.play(&sounds.victory, 1.0);

            self.floating_texts.push(FloatingText {
                pos: vec2(SCREEN_W * 0.5 - 140.0, 140.0),
                text: "SIDDHASHRAMA YAJNA DEFENDED!".to_string(),
                color: COLOR_RAMA_GOLD,
                lifetime: 3.0,
                max_lifetime: 3.0,
                font_size: 11,
            });
        }

        // 13. Update Particles & Texts
        for p in &mut self.particles {
            p.update(dt);
        }
        self.particles.retain(|p| p.is_alive());

        for ft in &mut self.floating_texts {
            ft.update(dt);
        }
        self.floating_texts.retain(|ft| ft.is_alive());

        self.rng = rng;
        None
    }

    pub fn draw(&self, stats: &PlayerStats, virtual_mouse: Vec2, font: &Font) {
        let shake_offset = if self.screen_shake_time > 0.0 {
            let offset_x = ((self.time_elapsed * 35.0).sin()) * self.screen_shake_intensity;
            let offset_y = ((self.time_elapsed * 45.0).cos()) * self.screen_shake_intensity;
            vec2(offset_x, offset_y)
        } else {
            Vec2::ZERO
        };

        // 1. Draw Siddhashrama Hermitage Grove & Central Yajna Altar
        draw_siddhashrama_arena(self.time_elapsed, self.yajna_purity);

        // 2. Draw Blood Bombs falling
        for b in &self.blood_bombs {
            draw_blood_bomb(b.pos + shake_offset, b.target_pos + shake_offset, b.altitude);
        }

        // 3. Draw Dark Spikes
        for s in &self.dark_spikes {
            draw_dark_spike(s.pos + shake_offset, s.vel);
        }

        // 4. Draw Minions
        for m in &self.minions {
            draw_rakshasa_minion(m.pos + shake_offset, m.hit_flash, self.time_elapsed);
        }

        // 5. Draw Winged Asuras
        for a in &self.asuras {
            draw_winged_asura(a.pos + shake_offset, a.hit_flash, self.time_elapsed);
        }

        // 6. Draw Maricha Boss
        if self.maricha.is_active && !self.maricha.is_defeated {
            draw_maricha(self.maricha.pos + shake_offset, self.maricha.hit_flash, self.time_elapsed);
        }

        // 7. Draw Subahu Boss
        if self.subahu.is_active && !self.subahu.is_defeated {
            draw_subahu(
                self.subahu.pos + shake_offset,
                self.subahu.hit_flash,
                self.subahu.has_miasma_shield,
                self.time_elapsed,
            );
        }

        // 8. Draw Prince Rama & Kodanda Bow
        let aim_vec = virtual_mouse - self.rama_pos;
        let aim_dir = if aim_vec.length_squared() > 1.0 {
            aim_vec.normalize()
        } else {
            vec2(1.0, 0.0)
        };
        draw_rama(
            self.rama_pos + shake_offset,
            aim_dir,
            self.rama_invuln_timer,
            self.rama_bow_cooldown,
            self.time_elapsed,
        );

        // 9. Draw Astra Projectiles
        for proj in &self.astra_projectiles {
            draw_astra_projectile(proj.pos + shake_offset, proj.vel, proj.p_type, self.time_elapsed);
        }

        // 10. Draw Particles
        for p in &self.particles {
            p.draw();
        }

        // 11. Draw Floating Texts
        for ft in &self.floating_texts {
            ft.draw(font);
        }

        // 12. Screen Flash
        if self.screen_flash_timer > 0.0 {
            let alpha = (self.screen_flash_timer / 0.4).clamp(0.0, 0.7);
            draw_rectangle(0.0, 0.0, SCREEN_W, SCREEN_H, Color::new(1.0, 1.0, 1.0, alpha));
        }

        // 13. Level 2 HUD (Yajna Purity, Waves, Boss HP, Astra Bar)
        draw_hud_l2(
            font,
            stats,
            self.yajna_purity,
            self.wave,
            if self.maricha.is_active && !self.maricha.is_defeated { Some((self.maricha.hp, self.maricha.max_hp)) } else { None },
            if self.subahu.is_active && !self.subahu.is_defeated { Some((self.subahu.hp, self.subahu.max_hp)) } else { None },
        );

        // 14. Reticle
        draw_reticle(virtual_mouse, self.rama_bow_cooldown > 0.0);
    }
}

pub enum FightResultL2 {
    Victory,
    Defeat,
}
