use macroquad::prelude::*;
use crate::audio::SoundEffects;
use crate::render::*;
use crate::types::{Astra, PlayerStats};

#[derive(Clone, Copy)]
pub struct SimpleRng(pub u32);

impl SimpleRng {
    pub fn next(&mut self) -> f32 {
        self.0 = self.0.wrapping_mul(1664525).wrapping_add(1013904223);
        ((self.0 >> 16) as f32) / 65536.0
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum TatakaPhase {
    Hunt,      // Phase 1: Shabdavedhi invisible hunt
    FinalRush, // Phase 2: Visible charge and melee/stomp
}

pub struct Rama {
    pub pos: Vec2,
    pub speed: f32,
    pub bow_cooldown: f32,
    pub invuln_timer: f32,
    pub step_particle_timer: f32,
}

pub struct Tataka {
    pub pos: Vec2,
    pub vel: Vec2,
    pub phase: TatakaPhase,
    pub visible: bool,
    pub hp: f32,
    pub max_hp: f32,
    pub pin_bar: f32,
    pub shimmer_timer: f32,
    pub hit_flash: f32,
    pub ripple_timer: f32,
    pub boulder_timer: f32,
    pub stomp_timer: f32,
    pub change_dir_timer: f32,
    pub melee_cooldown: f32,
}

pub struct Arrow {
    pub pos: Vec2,
    pub vel: Vec2,
    pub lifetime: f32,
}

pub struct Boulder {
    pub pos: Vec2,
    pub vel: Vec2,
    pub angle: f32,
    pub rot_speed: f32,
    pub lifetime: f32,
}

pub struct SoundRipple {
    pub pos: Vec2,
    pub radius: f32,
    pub max_radius: f32,
    pub lifetime: f32,
    pub max_lifetime: f32,
}

pub struct Shockwave {
    pub pos: Vec2,
    pub radius: f32,
    pub max_radius: f32,
    pub lifetime: f32,
    pub max_lifetime: f32,
    pub has_hit_player: bool,
}

pub struct Level1Fight {
    pub rama: Rama,
    pub tataka: Tataka,
    pub sunset_timer: f32,
    pub is_frenzy: bool,
    pub arrows: Vec<Arrow>,
    pub boulders: Vec<Boulder>,
    pub ripples: Vec<SoundRipple>,
    pub shockwaves: Vec<Shockwave>,
    pub particles: Vec<Particle>,
    pub floating_texts: Vec<FloatingText>,
    pub screen_shake_time: f32,
    pub screen_shake_intensity: f32,
    pub screen_flash_timer: f32,
    pub victory_delay: f32,
    pub is_boss_dead: bool,
    pub time_elapsed: f32,
    pub rng: SimpleRng,
}

impl Level1Fight {
    pub fn new() -> Self {
        Self {
            rama: Rama {
                pos: vec2(120.0, 180.0),
                speed: 185.0,
                bow_cooldown: 0.0,
                invuln_timer: 0.0,
                step_particle_timer: 0.0,
            },
            tataka: Tataka {
                pos: vec2(500.0, 180.0),
                vel: vec2(-55.0, 35.0),
                phase: TatakaPhase::Hunt,
                visible: false,
                hp: 100.0,
                max_hp: 100.0,
                pin_bar: 0.0,
                shimmer_timer: 0.0,
                hit_flash: 0.0,
                ripple_timer: 0.5,
                boulder_timer: 2.5,
                stomp_timer: 4.0,
                change_dir_timer: 2.0,
                melee_cooldown: 0.0,
            },
            sunset_timer: 90.0,
            is_frenzy: false,
            arrows: Vec::new(),
            boulders: Vec::new(),
            ripples: Vec::new(),
            shockwaves: Vec::new(),
            particles: Vec::new(),
            floating_texts: Vec::new(),
            screen_shake_time: 0.0,
            screen_shake_intensity: 0.0,
            screen_flash_timer: 0.0,
            victory_delay: 0.0,
            is_boss_dead: false,
            time_elapsed: 0.0,
            rng: SimpleRng(54321),
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
    ) -> Option<FightResult> {
        let mut rng = self.rng;
        self.time_elapsed += dt;

        // Screen shake & flash decay
        if self.screen_shake_time > 0.0 {
            self.screen_shake_time -= dt;
        }
        if self.screen_flash_timer > 0.0 {
            self.screen_flash_timer -= dt;
        }

        // 1. Sunset Timer & Dusk Frenzy
        if self.sunset_timer > 0.0 {
            self.sunset_timer -= dt;
            if self.sunset_timer <= 0.0 {
                self.sunset_timer = 0.0;
                if !self.is_frenzy {
                    self.is_frenzy = true;
                    self.trigger_shake(8.0, 0.6);
                    self.screen_flash_timer = 0.3;
                    sounds.play(&sounds.roar, 1.0);
                    self.floating_texts.push(FloatingText {
                        pos: vec2(SCREEN_W * 0.5 - 130.0, SCREEN_H * 0.5 - 20.0),
                        text: "DUSK HAS FALLEN! FRENZY!".to_string(),
                        color: Color::new(1.0, 0.15, 0.15, 1.0),
                        lifetime: 2.5,
                        max_lifetime: 2.5,
                        font_size: 11,
                    });
                }
            }
        }

        // 2. Victory delay check
        if self.is_boss_dead {
            self.victory_delay += dt;
            if self.victory_delay >= 1.2 {
                self.rng = rng;
                return Some(FightResult::Victory);
            }
        }

        // 3. Rama Movement (WASD + Arrow keys)
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
            self.rama.pos += move_dir * self.rama.speed * dt;

            // Step dust particles
            self.rama.step_particle_timer -= dt;
            if self.rama.step_particle_timer <= 0.0 {
                self.rama.step_particle_timer = 0.14;
                let vel = -move_dir * 14.0 + vec2((rng.next() - 0.5) * 8.0, -4.0);
                self.particles.push(Particle {
                    pos: self.rama.pos + vec2(0.0, 8.0),
                    vel,
                    color: Color::new(0.40, 0.35, 0.25, 0.5),
                    size: 2.5,
                    lifetime: 0.22,
                    max_lifetime: 0.22,
                });
            }
        }

        // Clamp Rama inside Arena
        self.rama.pos.x = self.rama.pos.x.clamp(ARENA_MIN_X + 14.0, ARENA_MAX_X - 14.0);
        self.rama.pos.y = self.rama.pos.y.clamp(ARENA_MIN_Y + 14.0, ARENA_MAX_Y - 14.0);

        // Cooldowns
        if self.rama.invuln_timer > 0.0 {
            self.rama.invuln_timer -= dt;
        }
        if self.rama.bow_cooldown > 0.0 {
            self.rama.bow_cooldown -= dt;
        }

        // Astra selection
        if is_key_pressed(KeyCode::Key1) {
            stats.selected_astra = Astra::KodandaBow;
        }
        if is_key_pressed(KeyCode::Key2) || is_key_pressed(KeyCode::Key3) || is_key_pressed(KeyCode::Key4) {
            self.floating_texts.push(FloatingText {
                pos: self.rama.pos + vec2(-45.0, -28.0),
                text: "ASTRAS UNLOCK IN L2!".to_string(),
                color: Color::new(1.0, 0.8, 0.2, 1.0),
                lifetime: 1.0,
                max_lifetime: 1.0,
                font_size: 8,
            });
        }

        // Aiming & Bow Fire
        let aim_vec = virtual_mouse - self.rama.pos;
        let aim_dir = if aim_vec.length_squared() > 1.0 {
            aim_vec.normalize()
        } else {
            vec2(1.0, 0.0)
        };

        let is_firing = (is_mouse_button_down(MouseButton::Left) || is_key_down(KeyCode::Space))
            && self.rama.bow_cooldown <= 0.0
            && !self.is_boss_dead;

        if is_firing {
            self.rama.bow_cooldown = 0.30;
            let arrow_pos = self.rama.pos + aim_dir * 11.0;
            self.arrows.push(Arrow {
                pos: arrow_pos,
                vel: aim_dir * 480.0,
                lifetime: 1.4,
            });
            sounds.play(&sounds.shoot, 0.7);

            self.particles.push(Particle {
                pos: arrow_pos,
                vel: aim_dir * 25.0,
                color: COLOR_RAMA_GOLD,
                size: 3.5,
                lifetime: 0.10,
                max_lifetime: 0.10,
            });
        }

        // 4. Tataka Boss AI
        if !self.is_boss_dead {
            let speed_mult = if self.is_frenzy { 2.0 } else { 1.0 };
            let timer_mult = if self.is_frenzy { 0.55 } else { 1.0 };

            if self.tataka.shimmer_timer > 0.0 {
                self.tataka.shimmer_timer -= dt;
            }
            if self.tataka.hit_flash > 0.0 {
                self.tataka.hit_flash -= dt;
            }
            if self.tataka.melee_cooldown > 0.0 {
                self.tataka.melee_cooldown -= dt;
            }

            match self.tataka.phase {
                TatakaPhase::Hunt => {
                    self.tataka.change_dir_timer -= dt;
                    if self.tataka.change_dir_timer <= 0.0 {
                        self.tataka.change_dir_timer = 1.5 + rng.next() * 1.5;
                        let angle = rng.next() * std::f32::consts::TAU;
                        let spd = 95.0 * speed_mult;
                        self.tataka.vel = vec2(angle.cos() * spd, angle.sin() * spd);
                    }

                    self.tataka.pos += self.tataka.vel * dt;

                    if self.tataka.pos.x < ARENA_MIN_X + 20.0 {
                        self.tataka.pos.x = ARENA_MIN_X + 20.0;
                        self.tataka.vel.x = self.tataka.vel.x.abs();
                    } else if self.tataka.pos.x > ARENA_MAX_X - 20.0 {
                        self.tataka.pos.x = ARENA_MAX_X - 20.0;
                        self.tataka.vel.x = -self.tataka.vel.x.abs();
                    }
                    if self.tataka.pos.y < ARENA_MIN_Y + 20.0 {
                        self.tataka.pos.y = ARENA_MIN_Y + 20.0;
                        self.tataka.vel.y = self.tataka.vel.y.abs();
                    } else if self.tataka.pos.y > ARENA_MAX_Y - 20.0 {
                        self.tataka.pos.y = ARENA_MAX_Y - 20.0;
                        self.tataka.vel.y = -self.tataka.vel.y.abs();
                    }

                    // Sound Ripple emission
                    self.tataka.ripple_timer -= dt;
                    if self.tataka.ripple_timer <= 0.0 {
                        self.tataka.ripple_timer = 1.2 * timer_mult;
                        self.ripples.push(SoundRipple {
                            pos: self.tataka.pos,
                            radius: 4.0,
                            max_radius: 65.0,
                            lifetime: 1.0,
                            max_lifetime: 1.0,
                        });
                        sounds.play(&sounds.ripple, 0.6);
                    }

                    // Boulder throw
                    self.tataka.boulder_timer -= dt;
                    if self.tataka.boulder_timer <= 0.0 {
                        self.tataka.boulder_timer = 3.2 * timer_mult;
                        let b_dir = (self.rama.pos - self.tataka.pos).normalize_or_zero();
                        let b_spd = 210.0 * speed_mult.min(1.4);
                        self.boulders.push(Boulder {
                            pos: self.tataka.pos,
                            vel: b_dir * b_spd,
                            angle: 0.0,
                            rot_speed: 6.0,
                            lifetime: 3.5,
                        });
                        sounds.play(&sounds.boulder, 0.7);
                    }
                }
                TatakaPhase::FinalRush => {
                    let to_rama = self.rama.pos - self.tataka.pos;
                    let dist = to_rama.length();
                    let charge_dir = if dist > 1.0 { to_rama / dist } else { Vec2::ZERO };
                    let charge_spd = 145.0 * speed_mult;
                    self.tataka.pos += charge_dir * charge_spd * dt;

                    self.tataka.pos.x = self.tataka.pos.x.clamp(ARENA_MIN_X + 20.0, ARENA_MAX_X - 20.0);
                    self.tataka.pos.y = self.tataka.pos.y.clamp(ARENA_MIN_Y + 20.0, ARENA_MAX_Y - 20.0);

                    // Melee claw swipe
                    if dist < 30.0 && self.tataka.melee_cooldown <= 0.0 && self.rama.invuln_timer <= 0.0 {
                        self.tataka.melee_cooldown = 1.0 * timer_mult;
                        let dmg = if self.is_frenzy { 50 } else { 25 };
                        stats.prana -= dmg;
                        self.rama.invuln_timer = 0.8;
                        self.trigger_shake(6.0, 0.25);
                        sounds.play(&sounds.hurt, 0.8);

                        self.rama.pos += charge_dir * 32.0;

                        self.floating_texts.push(FloatingText {
                            pos: self.rama.pos + vec2(-15.0, -20.0),
                            text: format!("-{} HP", dmg),
                            color: Color::new(1.0, 0.2, 0.2, 1.0),
                            lifetime: 1.0,
                            max_lifetime: 1.0,
                            font_size: 9,
                        });

                        if stats.prana <= 0 {
                            sounds.play(&sounds.game_over, 0.9);
                            self.rng = rng;
                            return Some(FightResult::Defeat);
                        }
                    }

                    // Stomp attack
                    self.tataka.stomp_timer -= dt;
                    if self.tataka.stomp_timer <= 0.0 {
                        self.tataka.stomp_timer = 4.0 * timer_mult;
                        self.shockwaves.push(Shockwave {
                            pos: self.tataka.pos,
                            radius: 10.0,
                            max_radius: 75.0,
                            lifetime: 0.7,
                            max_lifetime: 0.7,
                            has_hit_player: false,
                        });
                        self.trigger_shake(5.0, 0.2);
                        sounds.play(&sounds.smash, 0.7);

                        self.floating_texts.push(FloatingText {
                            pos: self.tataka.pos + vec2(-30.0, -25.0),
                            text: "STOMP!".to_string(),
                            color: Color::new(1.0, 0.4, 0.2, 1.0),
                            lifetime: 0.9,
                            max_lifetime: 0.9,
                            font_size: 8,
                        });
                    }
                }
            }
        }

        // 5. Update Ripples
        for ripple in &mut self.ripples {
            ripple.lifetime -= dt;
            let progress = 1.0 - (ripple.lifetime / ripple.max_lifetime);
            ripple.radius = progress * ripple.max_radius;
        }
        self.ripples.retain(|r| r.lifetime > 0.0);

        // 6. Update Shockwaves
        let mut shock_hit_player = false;
        for shock in &mut self.shockwaves {
            shock.lifetime -= dt;
            let progress = 1.0 - (shock.lifetime / shock.max_lifetime);
            shock.radius = progress * shock.max_radius;

            if !shock.has_hit_player && self.rama.invuln_timer <= 0.0 {
                let dist = (self.rama.pos - shock.pos).length();
                if (dist - shock.radius).abs() < 16.0 {
                    shock.has_hit_player = true;
                    shock_hit_player = true;
                }
            }
        }
        self.shockwaves.retain(|s| s.lifetime > 0.0);

        if shock_hit_player && self.rama.invuln_timer <= 0.0 {
            let dmg = if self.is_frenzy { 30 } else { 15 };
            stats.prana -= dmg;
            self.rama.invuln_timer = 0.8;
            self.trigger_shake(5.0, 0.2);
            sounds.play(&sounds.hurt, 0.8);

            self.floating_texts.push(FloatingText {
                pos: self.rama.pos + vec2(-20.0, -20.0),
                text: format!("-{} HP (SHOCK)", dmg),
                color: Color::new(1.0, 0.25, 0.25, 1.0),
                lifetime: 1.0,
                max_lifetime: 1.0,
                font_size: 8,
            });

            if stats.prana <= 0 {
                sounds.play(&sounds.game_over, 0.9);
                self.rng = rng;
                return Some(FightResult::Defeat);
            }
        }

        // 7. Update Boulders
        let mut boulder_hits = Vec::new();
        for b in &mut self.boulders {
            b.pos += b.vel * dt;
            b.angle += b.rot_speed * dt;
            b.lifetime -= dt;

            let dist_to_rama = (b.pos - self.rama.pos).length();
            if dist_to_rama < 17.0 && self.rama.invuln_timer <= 0.0 {
                b.lifetime = 0.0;
                boulder_hits.push(b.pos);
            }
        }
        self.boulders.retain(|b| {
            b.lifetime > 0.0
                && b.pos.x >= ARENA_MIN_X
                && b.pos.x <= ARENA_MAX_X
                && b.pos.y >= ARENA_MIN_Y
                && b.pos.y <= ARENA_MAX_Y
        });

        for hit_pos in boulder_hits {
            let dmg = if self.is_frenzy { 40 } else { 20 };
            stats.prana -= dmg;
            self.rama.invuln_timer = 0.8;
            self.trigger_shake(5.0, 0.2);
            sounds.play(&sounds.hurt, 0.8);
            sounds.play(&sounds.smash, 0.7);

            for _ in 0..12 {
                let rand_v = vec2((rng.next() - 0.5) * 85.0, (rng.next() - 0.5) * 85.0);
                self.particles.push(Particle {
                    pos: hit_pos,
                    vel: rand_v,
                    color: Color::new(0.5, 0.45, 0.4, 0.8),
                    size: 3.0,
                    lifetime: 0.32,
                    max_lifetime: 0.32,
                });
            }

            self.floating_texts.push(FloatingText {
                pos: self.rama.pos + vec2(-20.0, -20.0),
                text: format!("-{} HP (ROCK)", dmg),
                color: Color::new(1.0, 0.25, 0.25, 1.0),
                lifetime: 1.0,
                max_lifetime: 1.0,
                font_size: 8,
            });

            if stats.prana <= 0 {
                sounds.play(&sounds.game_over, 0.9);
                self.rng = rng;
                return Some(FightResult::Defeat);
            }
        }

        // 8. Update Arrows & Collisions with Tataka
        let mut pin_hits = 0;
        let mut damage_hits = 0;
        let mut last_arrow_pos = Vec2::ZERO;

        for arrow in &mut self.arrows {
            arrow.pos += arrow.vel * dt;
            arrow.lifetime -= dt;

            if !self.is_boss_dead {
                let dist_to_tataka = (arrow.pos - self.tataka.pos).length();

                match self.tataka.phase {
                    TatakaPhase::Hunt => {
                        if dist_to_tataka <= 28.0 {
                            arrow.lifetime = 0.0;
                            pin_hits += 1;
                            last_arrow_pos = arrow.pos;
                        }
                    }
                    TatakaPhase::FinalRush => {
                        if dist_to_tataka <= 30.0 {
                            arrow.lifetime = 0.0;
                            damage_hits += 1;
                            last_arrow_pos = arrow.pos;
                        }
                    }
                }
            }
        }
        self.arrows.retain(|a| {
            a.lifetime > 0.0
                && a.pos.x >= ARENA_MIN_X
                && a.pos.x <= ARENA_MAX_X
                && a.pos.y >= ARENA_MIN_Y
                && a.pos.y <= ARENA_MAX_Y
        });

        // Pin Hits (Phase 1)
        if pin_hits > 0 {
            self.tataka.pin_bar += 20.0 * pin_hits as f32;
            self.tataka.shimmer_timer = 0.35;
            self.trigger_shake(4.0, 0.15);
            sounds.play(&sounds.pin, 0.85);

            for _ in 0..14 {
                let vel = vec2((rng.next() - 0.5) * 110.0, (rng.next() - 0.5) * 110.0);
                self.particles.push(Particle {
                    pos: self.tataka.pos,
                    vel,
                    color: COLOR_RAMA_GOLD,
                    size: 3.0,
                    lifetime: 0.28,
                    max_lifetime: 0.28,
                });
            }

            let hits = (self.tataka.pin_bar / 20.0).round() as i32;
            self.floating_texts.push(FloatingText {
                pos: self.tataka.pos + vec2(-35.0, -22.0),
                text: format!("+20% PINNED! ({}/5)", hits.min(5)),
                color: Color::new(0.3, 0.85, 1.0, 1.0),
                lifetime: 1.2,
                max_lifetime: 1.2,
                font_size: 9,
            });

            if self.tataka.pin_bar >= 100.0 {
                self.tataka.phase = TatakaPhase::FinalRush;
                self.tataka.visible = true;
                self.tataka.hp = 100.0;
                self.tataka.max_hp = 100.0;
                self.trigger_shake(8.0, 0.5);
                self.screen_flash_timer = 0.25;
                sounds.play(&sounds.roar, 1.0);

                self.floating_texts.push(FloatingText {
                    pos: self.tataka.pos + vec2(-55.0, -35.0),
                    text: "TATAKA REVEALED!".to_string(),
                    color: Color::new(1.0, 0.2, 0.2, 1.0),
                    lifetime: 2.0,
                    max_lifetime: 2.0,
                    font_size: 11,
                });
            }
        }

        // Damage Hits (Phase 2)
        if damage_hits > 0 {
            let dmg = 20.0 * damage_hits as f32;
            self.tataka.hp -= dmg;
            self.tataka.hit_flash = 0.15;
            self.trigger_shake(3.5, 0.12);
            sounds.play(&sounds.hit, 0.85);

            for _ in 0..10 {
                let vel = vec2((rng.next() - 0.5) * 100.0, (rng.next() - 0.5) * 100.0);
                self.particles.push(Particle {
                    pos: last_arrow_pos,
                    vel,
                    color: Color::new(0.9, 0.1, 0.2, 0.9),
                    size: 2.5,
                    lifetime: 0.24,
                    max_lifetime: 0.24,
                });
            }

            self.floating_texts.push(FloatingText {
                pos: self.tataka.pos + vec2(-15.0, -24.0),
                text: format!("-{:.0} HP", dmg),
                color: Color::new(1.0, 0.3, 0.3, 1.0),
                lifetime: 1.0,
                max_lifetime: 1.0,
                font_size: 9,
            });

            if self.tataka.hp <= 0.0 {
                self.tataka.hp = 0.0;
                self.is_boss_dead = true;
                self.trigger_shake(10.0, 0.8);
                self.screen_flash_timer = 0.4;
                sounds.play(&sounds.victory, 1.0);

                for _ in 0..60 {
                    let vel = vec2((rng.next() - 0.5) * 190.0, (rng.next() - 0.5) * 190.0);
                    let col = if rng.next() > 0.5 {
                        COLOR_RAMA_GOLD
                    } else {
                        Color::new(0.8, 0.1, 0.2, 1.0)
                    };
                    self.particles.push(Particle {
                        pos: self.tataka.pos,
                        vel,
                        color: col,
                        size: 3.5,
                        lifetime: 0.65,
                        max_lifetime: 0.65,
                    });
                }

                self.floating_texts.push(FloatingText {
                    pos: self.tataka.pos + vec2(-60.0, -35.0),
                    text: "TATAKA VANQUISHED!".to_string(),
                    color: COLOR_RAMA_GOLD,
                    lifetime: 2.5,
                    max_lifetime: 2.5,
                    font_size: 11,
                });
            }
        }

        // 9. Update Particles & Floating Text
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

        // 1. Draw Battlefield Terrain, Relics, Altars, Paths & Forest Borders
        draw_battlefield(self.time_elapsed, self.sunset_timer);

        // 2. Draw Sound Ripples
        for ripple in &self.ripples {
            let alpha = ripple.lifetime / ripple.max_lifetime;
            draw_sound_ripple(ripple.pos + shake_offset, ripple.radius, ripple.max_radius, alpha);
        }

        // 3. Draw Shockwaves
        for shock in &self.shockwaves {
            let alpha = shock.lifetime / shock.max_lifetime;
            draw_shockwave(shock.pos + shake_offset, shock.radius, alpha);
        }

        // 4. Draw Boulders
        for b in &self.boulders {
            draw_boulder(b.pos + shake_offset, b.angle);
        }

        // 5. Draw Tataka (if alive)
        if !self.is_boss_dead {
            draw_tataka(
                self.tataka.pos + shake_offset,
                self.tataka.visible,
                self.tataka.shimmer_timer,
                self.tataka.hit_flash,
                self.is_frenzy,
                self.time_elapsed,
            );
        }

        // 6. Draw Prince Rama & Kodanda Bow
        let aim_vec = virtual_mouse - self.rama.pos;
        let aim_dir = if aim_vec.length_squared() > 1.0 {
            aim_vec.normalize()
        } else {
            vec2(1.0, 0.0)
        };
        draw_rama(
            self.rama.pos + shake_offset,
            aim_dir,
            self.rama.invuln_timer,
            self.rama.bow_cooldown,
            self.time_elapsed,
        );

        // 7. Draw Arrows
        for arrow in &self.arrows {
            draw_arrow(arrow.pos + shake_offset, arrow.vel);
        }

        // 8. Draw Particles
        for p in &self.particles {
            p.draw();
        }

        // 9. Draw Floating Combat Text
        for ft in &self.floating_texts {
            ft.draw(font);
        }

        // 10. Screen Flash Overlay
        if self.screen_flash_timer > 0.0 {
            let alpha = (self.screen_flash_timer / 0.4).clamp(0.0, 0.7);
            draw_rectangle(0.0, 0.0, SCREEN_W, SCREEN_H, Color::new(1.0, 1.0, 1.0, alpha));
        }

        // 11. HUD: Prana, Tejas, Sunset Dial, Pin / Boss HP Bar, Astra Slots
        draw_hud(
            font,
            stats,
            self.sunset_timer,
            self.is_frenzy,
            self.tataka.phase == TatakaPhase::FinalRush,
            self.tataka.pin_bar,
            self.tataka.hp,
            self.tataka.max_hp,
        );

        // 12. Mouse Targeting Crosshair / Reticle
        draw_reticle(virtual_mouse, self.rama.bow_cooldown > 0.0);
    }
}

pub enum FightResult {
    Victory,
    Defeat,
}
