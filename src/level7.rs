use macroquad::prelude::*;
use crate::audio::SoundEffects;
use crate::render::{
    draw_ancient_forest_tree, draw_astra_projectile, draw_dialogue_box, draw_hermitage_kutir,
    draw_hero_exile, draw_pixel_text, draw_reticle, draw_sita_hermit,
    is_dialogue_prev_button_hovered, PAL_BLUE_0, PAL_BLUE_2, PAL_FIRE_0,
    PAL_FIRE_1, PAL_FIRE_3, PAL_GOLD_0, PAL_GOLD_1, PAL_GOLD_2, PAL_GOLD_3, PAL_GRASS_2,
    PAL_GRASS_3, PAL_PURPLE_0, PAL_PURPLE_1, PAL_RED_1, PAL_RED_2, PAL_RED_3,
    PAL_SAFFRON_0, PAL_SAND_1, PAL_SAND_4, PAL_TURMERIC_1, SCREEN_H, SCREEN_W,
};
use crate::types::{Astra, AstraProjectileType, PlayerStats};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Level7Phase {
    IntroDialogue,
    Vanguard14,
    DushanaAndArmy,
    GandharvaAwakening,
    TrishirasChariot,
    KharaShowdown,
    EkaViraTriumph,
    Complete,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum DemonKind {
    Vanguard,
    Infantry,
    FlyingArcher,
}

#[derive(Clone, Debug)]
pub struct HostDemon {
    pub pos: Vec2,
    pub vel: Vec2,
    pub hp: f32,
    pub max_hp: f32,
    pub kind: DemonKind,
    pub host_value: i32,
    pub confused_timer: f32,
    pub attack_cd: f32,
    pub hit_flash: f32,
    pub active: bool,
}

#[derive(Clone, Debug)]
pub struct BossEntity {
    pub name: &'static str,
    pub pos: Vec2,
    pub hp: f32,
    pub max_hp: f32,
    pub chariot_intact: bool,
    pub attack_timer: f32,
    pub hit_flash: f32,
}

#[derive(Clone, Debug)]
pub struct L7Projectile {
    pub pos: Vec2,
    pub vel: Vec2,
    pub is_player: bool,
    pub astra: Astra,
    pub damage: f32,
    pub active: bool,
}

#[derive(Clone, Debug)]
pub struct L7Particle {
    pub pos: Vec2,
    pub vel: Vec2,
    pub color: Color,
    pub size: f32,
    pub timer: f32,
    pub max_timer: f32,
}

#[derive(Clone, Debug)]
pub struct L7FloatingText {
    pub text: String,
    pub pos: Vec2,
    pub color: Color,
    pub timer: f32,
    pub max_timer: f32,
}

#[allow(dead_code)]
pub enum FightResultL7 {
    Continue,
    Victory,
    Defeat,
}

pub struct Level7Fight {
    pub phase: Level7Phase,
    pub host_remaining: i32,
    pub rama_pos: Vec2,
    pub sita_pos: Vec2,
    pub lakshmana_pos: Vec2,
    pub aim_dir: Vec2,
    pub bow_tension: f32,
    pub is_drawing: bool,
    pub tejas_regen_timer: f32,
    pub invuln_timer: f32,
    pub shake_timer: f32,
    pub gandharva_wave_timer: f32,

    pub vanguard_spawned: i32,
    pub wave_spawn_timer: f32,
    pub demons: Vec<HostDemon>,
    pub boss: Option<BossEntity>,
    pub projectiles: Vec<L7Projectile>,
    pub particles: Vec<L7Particle>,
    pub floating_texts: Vec<L7FloatingText>,

    pub dialogue: Option<(String, Vec<String>, usize)>,
    pub action_message: Option<(String, f32)>,
    pub triumph_timer: f32,
}

impl Level7Fight {
    pub fn new(stats: &mut PlayerStats) -> Self {
        stats.prana = stats.max_prana;
        stats.tejas = stats.max_tejas;
        stats.unlocked_astras = vec![
            Astra::KodandaBow,
            Astra::Vayavyastra,
            Astra::Manavastra,
            Astra::Agneyastra,
        ];
        stats.selected_astra = Astra::KodandaBow;

        let mut fight = Self {
            phase: Level7Phase::IntroDialogue,
            host_remaining: 14000,
            rama_pos: vec2(170.0, 236.0),
            sita_pos: vec2(58.0, 236.0),
            lakshmana_pos: vec2(92.0, 236.0),
            aim_dir: vec2(1.0, 0.0),
            bow_tension: 0.0,
            is_drawing: false,
            tejas_regen_timer: 0.0,
            invuln_timer: 0.0,
            shake_timer: 0.0,
            gandharva_wave_timer: 0.0,

            vanguard_spawned: 0,
            wave_spawn_timer: 0.0,
            demons: Vec::new(),
            boss: None,
            projectiles: Vec::new(),
            particles: Vec::new(),
            floating_texts: Vec::new(),

            dialogue: None,
            action_message: Some((
                "PANCHAVATI ON THE GODAVARI — 14,000 RAKSHASAS APPROACH!".to_string(),
                5.0,
            )),
            triumph_timer: 0.0,
        };

        fight.dialogue = Some((
            "Panchavati — Banks of River Godavari".to_string(),
            vec![
                "Prince Lakshmana: 'The horizon darkens! Khara marches from Janasthana with fourteen thousand Rakshasas!'".to_string(),
                "Prince Rama: 'Brother, guard Sita inside the mountain cave. Today I stand alone as Eka-Vira to shield Panchavati!'".to_string(),
                "Here come the 14 fierce Rakshasa champions leading Khara's vanguard! Draw Kodanda and hold the riverbank!".to_string(),
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
    ) -> FightResultL7 {
        if self.shake_timer > 0.0 {
            self.shake_timer = (self.shake_timer - dt).max(0.0);
        }
        if self.invuln_timer > 0.0 {
            self.invuln_timer = (self.invuln_timer - dt).max(0.0);
        }
        if self.gandharva_wave_timer > 0.0 {
            self.gandharva_wave_timer = (self.gandharva_wave_timer - dt).max(0.0);
        }
        if let Some((_, ref mut timer)) = self.action_message {
            *timer -= dt;
            if *timer <= 0.0 {
                self.action_message = None;
            }
        }

        // Regenerate Tejas steadily so Astras can be woven continuously in epic siege
        self.tejas_regen_timer += dt;
        if self.tejas_regen_timer >= 0.35 {
            self.tejas_regen_timer = 0.0;
            stats.tejas = (stats.tejas + 2).min(stats.max_tejas);
        }

        // Handle Active Dialogue
        if let Some((_, ref lines, ref mut idx)) = self.dialogue {
            let prev_clicked =
                is_mouse_button_pressed(MouseButton::Left) && is_dialogue_prev_button_hovered(virtual_mouse);
            let prev_key = is_key_pressed(KeyCode::Left) || is_key_pressed(KeyCode::Backspace);

            if (prev_clicked || prev_key) && *idx > 0 {
                *idx -= 1;
                sounds.play(&sounds.shoot, 0.4);
                return FightResultL7::Continue;
            }

            if interact {
                *idx += 1;
                sounds.play(&sounds.shoot, 0.4);
                if *idx >= lines.len() {
                    self.dialogue = None;
                    self.on_dialogue_complete(sounds, stats);
                }
            }
            return FightResultL7::Continue;
        }

        // Quick-advance developer test key [K]
        if is_key_pressed(KeyCode::K) {
            match self.phase {
                Level7Phase::Vanguard14 => {
                    self.demons.clear();
                    self.host_remaining = 13986;
                    self.start_dushana_phase(sounds);
                    return FightResultL7::Continue;
                }
                Level7Phase::DushanaAndArmy => {
                    self.demons.clear();
                    self.boss = None;
                    self.host_remaining = 6500;
                    self.start_gandharva_awakening(sounds, stats);
                    return FightResultL7::Continue;
                }
                Level7Phase::TrishirasChariot => {
                    self.demons.clear();
                    self.boss = None;
                    self.host_remaining = 2000;
                    self.start_khara_phase(sounds);
                    return FightResultL7::Continue;
                }
                Level7Phase::KharaShowdown => {
                    self.demons.clear();
                    self.boss = None;
                    self.host_remaining = 0;
                    self.start_triumph(sounds);
                    return FightResultL7::Continue;
                }
                Level7Phase::EkaViraTriumph => {
                    self.phase = Level7Phase::Complete;
                    return FightResultL7::Victory;
                }
                _ => {}
            }
        }

        // Player Movement, Aiming & Astra Selection (all active combat phases)
        match self.phase {
            Level7Phase::Vanguard14
            | Level7Phase::DushanaAndArmy
            | Level7Phase::TrishirasChariot
            | Level7Phase::KharaShowdown => {
                self.update_player_controls(dt, sounds, stats, virtual_mouse);
            }
            _ => {}
        }

        // Phase Progression
        match self.phase {
            Level7Phase::Vanguard14 => {
                self.wave_spawn_timer -= dt;
                if self.vanguard_spawned < 14 && self.wave_spawn_timer <= 0.0 && self.demons.len() < 5 {
                    self.wave_spawn_timer = 0.75;
                    self.vanguard_spawned += 1;
                    self.demons.push(HostDemon {
                        pos: vec2(620.0, 236.0 - (self.vanguard_spawned % 3) as f32 * 8.0),
                        vel: vec2(-68.0 - (self.vanguard_spawned % 3) as f32 * 12.0, 0.0),
                        hp: 30.0,
                        max_hp: 30.0,
                        kind: DemonKind::Vanguard,
                        host_value: 1,
                        confused_timer: 0.0,
                        attack_cd: 1.5,
                        hit_flash: 0.0,
                        active: true,
                    });
                }
                if self.vanguard_spawned >= 14 && self.demons.is_empty() {
                    self.start_dushana_phase(sounds);
                }
            }
            Level7Phase::DushanaAndArmy => {
                self.wave_spawn_timer -= dt;
                if self.wave_spawn_timer <= 0.0 && self.demons.len() < 12 && self.host_remaining > 6600 {
                    self.wave_spawn_timer = 1.0;
                    for lane in 0..2 {
                        let flying = (self.host_remaining + lane) % 3 == 0;
                        self.demons.push(HostDemon {
                            pos: vec2(625.0 + lane as f32 * 18.0, if flying { 145.0 + lane as f32 * 22.0 } else { 236.0 }),
                            vel: vec2(-52.0 - (lane as f32 * 10.0), 0.0),
                            hp: if flying { 35.0 } else { 45.0 },
                            max_hp: if flying { 35.0 } else { 45.0 },
                            kind: if flying { DemonKind::FlyingArcher } else { DemonKind::Infantry },
                            host_value: 350,
                            confused_timer: 0.0,
                            attack_cd: 2.0 + lane as f32 * 0.5,
                            hit_flash: 0.0,
                            active: true,
                        });
                    }
                }
                self.update_boss_ai(dt, sounds);
                if self.boss.as_ref().map(|b| b.hp <= 0.0).unwrap_or(false) {
                    self.host_remaining = self.host_remaining.min(6500);
                    self.demons.clear();
                    self.boss = None;
                    self.start_gandharva_awakening(sounds, stats);
                }
            }
            Level7Phase::TrishirasChariot => {
                self.wave_spawn_timer -= dt;
                if self.wave_spawn_timer <= 0.0 && self.demons.len() < 14 && self.host_remaining > 2200 {
                    self.wave_spawn_timer = 0.85;
                    for lane in 0..2 {
                        let flying = lane == 1;
                        self.demons.push(HostDemon {
                            pos: vec2(625.0 + lane as f32 * 14.0, if flying { 136.0 } else { 236.0 }),
                            vel: vec2(-60.0, 0.0),
                            hp: 40.0,
                            max_hp: 40.0,
                            kind: if flying { DemonKind::FlyingArcher } else { DemonKind::Infantry },
                            host_value: 450,
                            confused_timer: 0.0,
                            attack_cd: 1.8,
                            hit_flash: 0.0,
                            active: true,
                        });
                    }
                }
                self.update_boss_ai(dt, sounds);
                if self.boss.as_ref().map(|b| b.hp <= 0.0).unwrap_or(false) {
                    self.host_remaining = self.host_remaining.min(2000);
                    self.demons.clear();
                    self.boss = None;
                    self.start_khara_phase(sounds);
                }
            }
            Level7Phase::KharaShowdown => {
                self.wave_spawn_timer -= dt;
                if self.wave_spawn_timer <= 0.0 && self.demons.len() < 8 && self.host_remaining > 200 {
                    self.wave_spawn_timer = 1.3;
                    self.demons.push(HostDemon {
                        pos: vec2(620.0, 236.0),
                        vel: vec2(-64.0, 0.0),
                        hp: 45.0,
                        max_hp: 45.0,
                        kind: DemonKind::Infantry,
                        host_value: 300,
                        confused_timer: 0.0,
                        attack_cd: 2.0,
                        hit_flash: 0.0,
                        active: true,
                    });
                }
                self.update_boss_ai(dt, sounds);
                if self.boss.as_ref().map(|b| b.hp <= 0.0).unwrap_or(false) {
                    self.host_remaining = 0;
                    self.demons.clear();
                    self.boss = None;
                    self.start_triumph(sounds);
                }
            }
            Level7Phase::EkaViraTriumph => {
                self.triumph_timer += dt;
                if self.particles.len() < 50 {
                    let px = ((self.triumph_timer * 197.0) as i32 % 600 + 20) as f32;
                    self.particles.push(L7Particle {
                        pos: vec2(px, 10.0),
                        vel: vec2((self.triumph_timer * 3.0).sin() * 14.0, 48.0),
                        color: if self.particles.len() % 2 == 0 { PAL_GOLD_0 } else { PAL_SAFFRON_0 },
                        size: 3.0,
                        timer: 2.4,
                        max_timer: 2.4,
                    });
                }
                if self.triumph_timer >= 4.5 && self.dialogue.is_none() {
                    self.phase = Level7Phase::Complete;
                    return FightResultL7::Victory;
                }
            }
            Level7Phase::Complete => return FightResultL7::Victory,
            _ => {}
        }

        // Entities Update (Demons, Projectiles, Collisions, Particles)
        if let Some(res) = self.update_entities(dt, sounds, stats) {
            return res;
        }

        FightResultL7::Continue
    }

    fn on_dialogue_complete(&mut self, sounds: &SoundEffects, _stats: &mut PlayerStats) {
        match self.phase {
            Level7Phase::IntroDialogue => {
                self.phase = Level7Phase::Vanguard14;
                sounds.play(&sounds.conch, 0.8);
                self.action_message = Some((
                    "WAVE 1: SLAY THE 14 VANGUARD CHAMPIONS! [CLICK TO DRAW & LOOSE]".to_string(),
                    5.0,
                ));
            }
            Level7Phase::GandharvaAwakening => {
                self.phase = Level7Phase::TrishirasChariot;
                self.boss = Some(BossEntity {
                    name: "THREE-HEADED TRISHIRAS",
                    pos: vec2(525.0, 224.0),
                    hp: 260.0,
                    max_hp: 260.0,
                    chariot_intact: true,
                    attack_timer: 0.0,
                    hit_flash: 0.0,
                });
                sounds.play(&sounds.roar, 0.9);
                self.action_message = Some((
                    "PRESS [5] GANDHARVASTRA TO TURN THE HOST AGAINST ITSELF!".to_string(),
                    6.0,
                ));
            }
            _ => {}
        }
    }

    fn start_dushana_phase(&mut self, sounds: &SoundEffects) {
        self.phase = Level7Phase::DushanaAndArmy;
        self.shake_timer = 1.0;
        sounds.play(&sounds.roar, 0.9);
        self.boss = Some(BossEntity {
            name: "GENERAL DUSHANA (MACE LORD)",
            pos: vec2(520.0, 228.0),
            hp: 220.0,
            max_hp: 220.0,
            chariot_intact: false,
            attack_timer: 0.0,
            hit_flash: 0.0,
        });
        self.action_message = Some((
            "GENERAL DUSHANA MARCHES WITH THOUSANDS! USE [1-4] ASTRAS!".to_string(),
            5.0,
        ));
    }

    fn start_gandharva_awakening(&mut self, sounds: &SoundEffects, stats: &mut PlayerStats) {
        self.phase = Level7Phase::GandharvaAwakening;
        sounds.play(&sounds.bell, 0.9);
        if !stats.unlocked_astras.contains(&Astra::Gandharvastra) {
            stats.unlocked_astras.push(Astra::Gandharvastra);
        }
        stats.selected_astra = Astra::Gandharvastra;
        self.dialogue = Some((
            "Celestial Awakening — Gandharvastra".to_string(),
            vec![
                "General Dushana falls beside shattered war drums, yet thousands more swarm the banks of the Godavari!".to_string(),
                "Rama invokes [5] GANDHARVASTRA! Illusionary illusions multiply Rama's form across the forest!".to_string(),
                "Three-headed General Trishiras charges on his spiked chariot! Strike with Gandharvastra!".to_string(),
            ],
            0,
        ));
    }

    fn start_khara_phase(&mut self, sounds: &SoundEffects) {
        self.phase = Level7Phase::KharaShowdown;
        self.shake_timer = 1.4;
        sounds.play(&sounds.thunder, 1.0);
        self.boss = Some(BossEntity {
            name: "GENERAL KHARA (CHARIOT DUEL)",
            pos: vec2(525.0, 222.0),
            hp: 340.0,
            max_hp: 340.0,
            chariot_intact: true,
            attack_timer: 0.0,
            hit_flash: 0.0,
        });
        self.action_message = Some((
            "FINAL STAND: SHATTER KHARA'S WAR CHARIOT & END THE 14,000 HOST!".to_string(),
            6.0,
        ));
    }

    fn start_triumph(&mut self, sounds: &SoundEffects) {
        self.phase = Level7Phase::EkaViraTriumph;
        self.triumph_timer = 0.0;
        sounds.play(&sounds.victory, 1.0);
        self.action_message = Some((
            "★ EKA-VIRA VICTORY: 14,000 RAKSHASAS SLAIN SINGLE-HANDEDLY! ★".to_string(),
            5.0,
        ));
        self.dialogue = Some((
            "Eka-Vira — The Lone Victor of Janasthana".to_string(),
            vec![
                "In less than one and a half muhurtas, Prince Rama slew all 14,000 Rakshasas and their three invincible generals!".to_string(),
                "Princess Sita rushes from the cave to embrace Rama, while celestial flowers rain over sacred Panchavati!".to_string(),
                "Akampana flees in terror to Lanka—warning Ravana of the unconquerable warrior of Ayodhya!".to_string(),
            ],
            0,
        ));
    }

    fn update_player_controls(
        &mut self,
        dt: f32,
        sounds: &SoundEffects,
        stats: &mut PlayerStats,
        virtual_mouse: Vec2,
    ) {
        let mut move_x = 0.0;
        if is_key_down(KeyCode::A) || is_key_down(KeyCode::Left) {
            move_x -= 1.0;
        }
        if is_key_down(KeyCode::D) || is_key_down(KeyCode::Right) {
            move_x += 1.0;
        }
        self.rama_pos.x = (self.rama_pos.x + move_x * 150.0 * dt).clamp(115.0, 460.0);

        // Astra Selection [1]-[5]
        if is_key_pressed(KeyCode::Key1) {
            stats.selected_astra = Astra::KodandaBow;
        } else if is_key_pressed(KeyCode::Key2) {
            stats.selected_astra = Astra::Vayavyastra;
        } else if is_key_pressed(KeyCode::Key3) {
            stats.selected_astra = Astra::Manavastra;
        } else if is_key_pressed(KeyCode::Key4) {
            stats.selected_astra = Astra::Agneyastra;
        } else if is_key_pressed(KeyCode::Key5)
            && stats.unlocked_astras.contains(&Astra::Gandharvastra)
        {
            stats.selected_astra = Astra::Gandharvastra;
        }

        let origin = self.rama_pos + vec2(0.0, -24.0);
        let dir = (virtual_mouse - origin).normalize_or_zero();
        if dir.length_squared() > 0.01 {
            self.aim_dir = dir;
        }

        let is_down = is_mouse_button_down(MouseButton::Left);
        if is_down && !self.is_drawing {
            self.is_drawing = true;
            self.bow_tension = 0.0;
        } else if is_down && self.is_drawing {
            self.bow_tension = (self.bow_tension + dt * 3.0).min(1.0);
        } else if !is_down && self.is_drawing {
            self.is_drawing = false;
            if self.bow_tension > 0.15 {
                self.fire_astra(sounds, stats);
            }
            self.bow_tension = 0.0;
        }
    }

    fn fire_astra(&mut self, sounds: &SoundEffects, stats: &mut PlayerStats) {
        let mut astra = stats.selected_astra;
        let cost = match astra {
            Astra::KodandaBow => 0,
            Astra::Vayavyastra => 16,
            Astra::Manavastra => 18,
            Astra::Agneyastra => 20,
            Astra::Gandharvastra => 24,
            _ => 0,
        };
        if stats.tejas < cost {
            astra = Astra::KodandaBow;
        } else {
            stats.tejas -= cost;
        }

        let spawn = self.rama_pos + vec2(self.aim_dir.x * 12.0, -24.0);
        let speed = 400.0 + self.bow_tension * 220.0;

        match astra {
            Astra::Vayavyastra => {
                sounds.play(&sounds.wind, 0.8);
                for angle_ofs in [-0.22f32, 0.0, 0.22] {
                    let c = angle_ofs.cos();
                    let s = angle_ofs.sin();
                    let d = vec2(self.aim_dir.x * c - self.aim_dir.y * s, self.aim_dir.x * s + self.aim_dir.y * c);
                    self.projectiles.push(L7Projectile {
                        pos: spawn,
                        vel: d * speed,
                        is_player: true,
                        astra,
                        damage: 34.0,
                        active: true,
                    });
                }
            }
            Astra::Agneyastra => {
                sounds.play(&sounds.fire, 0.9);
                self.projectiles.push(L7Projectile {
                    pos: spawn,
                    vel: self.aim_dir * speed,
                    is_player: true,
                    astra,
                    damage: 55.0,
                    active: true,
                });
            }
            Astra::Gandharvastra => {
                sounds.play(&sounds.bell, 1.0);
                self.gandharva_wave_timer = 1.2;
                self.projectiles.push(L7Projectile {
                    pos: spawn,
                    vel: self.aim_dir * (speed * 0.9),
                    is_player: true,
                    astra,
                    damage: 40.0,
                    active: true,
                });
                // Apply illusionary confusion across nearby army demons immediately!
                for d in &mut self.demons {
                    if d.active && d.pos.x < 610.0 {
                        d.confused_timer = 5.5;
                    }
                }
                self.floating_texts.push(L7FloatingText {
                    text: "GANDHARVASTRA! THEY SEE RAMA EVERYWHERE!".to_string(),
                    pos: vec2(210.0, 120.0),
                    color: PAL_PURPLE_0,
                    timer: 1.8,
                    max_timer: 1.8,
                });
            }
            _ => {
                sounds.play(&sounds.shoot, 0.7);
                self.projectiles.push(L7Projectile {
                    pos: spawn,
                    vel: self.aim_dir * speed,
                    is_player: true,
                    astra,
                    damage: if astra == Astra::Manavastra { 42.0 } else { 28.0 },
                    active: true,
                });
            }
        }
    }

    fn update_boss_ai(&mut self, dt: f32, sounds: &SoundEffects) {
        let mut new_projs: Vec<L7Projectile> = Vec::new();
        if let Some(ref mut boss) = self.boss {
            if boss.hit_flash > 0.0 {
                boss.hit_flash = (boss.hit_flash - dt).max(0.0);
            }
            // Khara loses his chariot at 50% HP
            if boss.name.contains("KHARA") && boss.chariot_intact && boss.hp <= boss.max_hp * 0.5 {
                boss.chariot_intact = false;
                self.shake_timer = 0.8;
                sounds.play(&sounds.smash, 0.9);
                self.floating_texts.push(L7FloatingText {
                    text: "KHARA'S CHARIOT SHATTERED!".to_string(),
                    pos: boss.pos + vec2(-50.0, -70.0),
                    color: PAL_GOLD_0,
                    timer: 1.8,
                    max_timer: 1.8,
                });
            }

            // Gentle oscillating advance/retreat
            let target_x = if boss.chariot_intact {
                510.0 + (boss.attack_timer * 1.6).sin() * 45.0
            } else {
                475.0 + (boss.attack_timer * 2.2).sin() * 55.0
            };
            boss.pos.x += (target_x - boss.pos.x) * dt * 2.0;

            boss.attack_timer += dt;
            let interval = if boss.name.contains("KHARA") { 2.1 } else { 2.6 };
            if boss.attack_timer >= interval {
                boss.attack_timer = 0.0;
                let dir = (self.rama_pos + vec2(0.0, -20.0) - (boss.pos + vec2(-10.0, -25.0)))
                    .normalize_or_zero();
                if boss.name.contains("TRISHIRAS") {
                    sounds.play(&sounds.fire, 0.6);
                    for dy in [-0.14f32, 0.0, 0.14] {
                        new_projs.push(L7Projectile {
                            pos: boss.pos + vec2(-18.0, -28.0),
                            vel: vec2(dir.x, dir.y + dy).normalize() * 210.0,
                            is_player: false,
                            astra: Astra::KodandaBow,
                            damage: 10.0,
                            active: true,
                        });
                    }
                } else {
                    sounds.play(&sounds.roar, 0.5);
                    new_projs.push(L7Projectile {
                        pos: boss.pos + vec2(-18.0, -26.0),
                        vel: dir * 235.0,
                        is_player: false,
                        astra: Astra::Agneyastra,
                        damage: 12.0,
                        active: true,
                    });
                }
            }
        }
        self.projectiles.extend(new_projs);
    }

    fn update_entities(
        &mut self,
        dt: f32,
        sounds: &SoundEffects,
        stats: &mut PlayerStats,
    ) -> Option<FightResultL7> {
        let mut new_particles: Vec<L7Particle> = Vec::new();
        let mut new_texts: Vec<L7FloatingText> = Vec::new();
        let mut new_projs: Vec<L7Projectile> = Vec::new();

        // 1. Projectiles
        for p in self.projectiles.iter_mut().filter(|p| p.active) {
            // Manavastra slight homing toward nearest active demon or boss
            if p.is_player && p.astra == Astra::Manavastra {
                if let Some(target) = self.demons.iter().find(|d| d.active).map(|d| d.pos + vec2(0.0, -14.0))
                    .or_else(|| self.boss.as_ref().map(|b| b.pos + vec2(0.0, -26.0)))
                {
                    let want = (target - p.pos).normalize_or_zero() * p.vel.length();
                    p.vel = p.vel.lerp(want, dt * 6.0);
                }
            }
            p.pos += p.vel * dt;

            if p.is_player {
                // Check demon hits
                for d in self.demons.iter_mut().filter(|d| d.active) {
                    let center = d.pos + vec2(0.0, -14.0);
                    if (p.pos - center).length() < 20.0 {
                        p.active = false;
                        d.hp -= p.damage;
                        d.hit_flash = 0.14;
                        if p.astra == Astra::Gandharvastra {
                            d.confused_timer = 6.0;
                        }
                        if d.hp <= 0.0 {
                            d.active = false;
                            self.host_remaining = (self.host_remaining - d.host_value).max(0);
                            sounds.play(&sounds.smash, 0.5);
                            new_texts.push(L7FloatingText {
                                text: format!("-{}", d.host_value),
                                pos: center + vec2(-10.0, -16.0),
                                color: PAL_GOLD_0,
                                timer: 0.9,
                                max_timer: 0.9,
                            });
                            for k in 0..8 {
                                let a = (k as f32 / 8.0) * std::f32::consts::TAU;
                                new_particles.push(L7Particle {
                                    pos: center,
                                    vel: vec2(a.cos() * 55.0, a.sin() * 55.0),
                                    color: PAL_FIRE_1,
                                    size: 2.5,
                                    timer: 0.45,
                                    max_timer: 0.45,
                                });
                            }
                        } else {
                            sounds.play(&sounds.hit, 0.5);
                        }
                        break;
                    }
                }
                if !p.active {
                    continue;
                }
                // Check Boss hits
                if let Some(ref mut boss) = self.boss {
                    let b_center = boss.pos + vec2(0.0, -24.0);
                    if (p.pos - b_center).length() < 34.0 {
                        p.active = false;
                        boss.hp = (boss.hp - p.damage).max(0.0);
                        boss.hit_flash = 0.15;
                        sounds.play(&sounds.hit, 0.75);
                        new_texts.push(L7FloatingText {
                            text: format!("-{:.0}", p.damage),
                            pos: p.pos + vec2(-8.0, -14.0),
                            color: if p.astra == Astra::Agneyastra { PAL_FIRE_1 } else { PAL_GOLD_0 },
                            timer: 0.9,
                            max_timer: 0.9,
                        });
                    }
                }
            } else {
                // Enemy projectile vs Rama
                if self.invuln_timer <= 0.0 && (p.pos - (self.rama_pos + vec2(0.0, -20.0))).length() < 18.0 {
                    p.active = false;
                    stats.prana = (stats.prana - p.damage as i32).max(0);
                    self.invuln_timer = 0.65;
                    self.shake_timer = 0.35;
                    sounds.play(&sounds.hurt, 0.8);
                    if stats.prana <= 0 {
                        return Some(FightResultL7::Defeat);
                    }
                }
            }

            if p.pos.x < -20.0 || p.pos.x > SCREEN_W + 20.0 || p.pos.y < -20.0 || p.pos.y > SCREEN_H + 20.0 {
                p.active = false;
            }
        }
        self.projectiles.retain(|p| p.active);

        // 2. Demons movement, Gandharvastra friendly-fire infighting, & attacks
        let demon_count = self.demons.len();
        for i in 0..demon_count {
            if !self.demons[i].active {
                continue;
            }
            if self.demons[i].hit_flash > 0.0 {
                self.demons[i].hit_flash = (self.demons[i].hit_flash - dt).max(0.0);
            }

            if self.demons[i].confused_timer > 0.0 {
                self.demons[i].confused_timer -= dt;
                // Under Gandharvastra Maya: demons mistake allies for Rama and strike each other!
                let mut target_j: Option<usize> = None;
                for j in 0..demon_count {
                    if i != j && self.demons[j].active {
                        target_j = Some(j);
                        break;
                    }
                }
                if let Some(j) = target_j {
                    let dir = (self.demons[j].pos - self.demons[i].pos).normalize_or_zero();
                    self.demons[i].pos += dir * 75.0 * dt;
                    if (self.demons[i].pos - self.demons[j].pos).length() < 22.0 {
                        self.demons[j].hp -= 45.0 * dt;
                        self.demons[j].hit_flash = 0.1;
                        if self.demons[j].hp <= 0.0 {
                            self.demons[j].active = false;
                            self.host_remaining = (self.host_remaining - self.demons[j].host_value).max(0);
                            new_texts.push(L7FloatingText {
                                text: format!("MAYA! -{}", self.demons[j].host_value),
                                pos: self.demons[j].pos + vec2(-12.0, -24.0),
                                color: PAL_PURPLE_0,
                                timer: 0.9,
                                max_timer: 0.9,
                            });
                        }
                    }
                }
            } else {
                // Normal hostile advance toward Rama
                let d_kind = self.demons[i].kind;
                let spd = self.demons[i].vel.x;
                self.demons[i].pos.x += spd * dt;
                self.demons[i].attack_cd -= dt;

                if d_kind == DemonKind::FlyingArcher && self.demons[i].attack_cd <= 0.0 {
                    self.demons[i].attack_cd = 2.8;
                    let dir = (self.rama_pos + vec2(0.0, -20.0) - self.demons[i].pos).normalize_or_zero();
                    new_projs.push(L7Projectile {
                        pos: self.demons[i].pos,
                        vel: dir * 185.0,
                        is_player: false,
                        astra: Astra::KodandaBow,
                        damage: 8.0,
                        active: true,
                    });
                }

                // Melee collision with Rama
                if self.invuln_timer <= 0.0
                    && (self.demons[i].pos - self.rama_pos).length() < 24.0
                {
                    stats.prana = (stats.prana - 10).max(0);
                    self.invuln_timer = 0.7;
                    self.shake_timer = 0.3;
                    self.demons[i].pos.x += 28.0; // knockback recoil
                    sounds.play(&sounds.hurt, 0.8);
                    if stats.prana <= 0 {
                        return Some(FightResultL7::Defeat);
                    }
                }

                if self.demons[i].pos.x < 110.0 {
                    // Lakshmana intercepts anyone approaching Sita's cave!
                    self.demons[i].active = false;
                    self.host_remaining = (self.host_remaining - self.demons[i].host_value).max(0);
                    new_texts.push(L7FloatingText {
                        text: "LAKSHMANA GUARD!".to_string(),
                        pos: vec2(96.0, 200.0),
                        color: PAL_GOLD_0,
                        timer: 0.8,
                        max_timer: 0.8,
                    });
                }
            }
        }
        self.demons.retain(|d| d.active);
        self.projectiles.extend(new_projs);
        self.particles.extend(new_particles);
        self.floating_texts.extend(new_texts);

        // 3. Particles & Floating Text decay
        for pt in &mut self.particles {
            pt.timer -= dt;
            pt.pos += pt.vel * dt;
        }
        self.particles.retain(|pt| pt.timer > 0.0);

        for ft in &mut self.floating_texts {
            ft.timer -= dt;
            ft.pos.y -= 14.0 * dt;
        }
        self.floating_texts.retain(|ft| ft.timer > 0.0);

        None
    }

    // ========================================================================
    // RENDERING (720p CRISP 16-BIT VEDIC RETRO ART)
    // ========================================================================
    pub fn draw(&self, font: &Font, time: f32, stats: &PlayerStats, virtual_mouse: Vec2) {
        let shake = if self.shake_timer > 0.0 {
            vec2((time * 42.0).sin() * 3.0, (time * 52.0).cos() * 3.0)
        } else {
            Vec2::ZERO
        };

        // 1. Panchavati Environment & Shimmering Godavari River
        self.draw_panchavati_arena(time, shake);

        // 2. Sita sheltered at Cave Sanctuary & Lakshmana Standing Guard
        draw_sita_hermit(self.sita_pos, time);
        draw_hero_exile(self.lakshmana_pos, false, vec2(1.0, 0.0), 0.0, false, time);

        // 3. Eka-Vira Prince Rama
        if !(self.invuln_timer > 0.0 && ((self.invuln_timer * 18.0) as i32 % 2 == 0)) {
            draw_hero_exile(
                self.rama_pos,
                true,
                self.aim_dir,
                self.bow_tension,
                self.is_drawing,
                time,
            );
        }

        // Gandharvastra Maya Flash Ring
        if self.gandharva_wave_timer > 0.0 {
            let r = (1.2 - self.gandharva_wave_timer) * 420.0;
            draw_circle_lines(self.rama_pos.x, self.rama_pos.y - 20.0, r, 2.5, PAL_PURPLE_0);
            draw_circle_lines(self.rama_pos.x, self.rama_pos.y - 20.0, r * 0.75, 1.5, PAL_GOLD_0);
        }

        // 4. Demon Squads & Generals
        for d in &self.demons {
            if d.active {
                self.draw_host_demon(d, time);
            }
        }
        if let Some(ref boss) = self.boss {
            self.draw_boss(boss, time);
        }

        // 5. Projectiles & Particles
        for p in &self.projectiles {
            if p.active {
                if p.is_player {
                    let pt = match p.astra {
                        Astra::Vayavyastra => AstraProjectileType::Vayavya,
                        Astra::Manavastra => AstraProjectileType::Manava,
                        Astra::Agneyastra => AstraProjectileType::Agneya,
                        Astra::Gandharvastra => AstraProjectileType::Gandharva,
                        _ => AstraProjectileType::Kodanda,
                    };
                    draw_astra_projectile(p.pos, p.vel, pt, time);
                } else {
                    let tail = p.pos - p.vel.normalize_or_zero() * 12.0;
                    draw_line(tail.x, tail.y, p.pos.x, p.pos.y, 2.2, PAL_FIRE_3);
                    draw_circle(p.pos.x, p.pos.y, 2.2, PAL_FIRE_0);
                }
            }
        }

        for pt in &self.particles {
            let alpha = (pt.timer / pt.max_timer).clamp(0.0, 1.0);
            let mut c = pt.color;
            c.a *= alpha;
            draw_rectangle(pt.pos.x - pt.size * 0.5, pt.pos.y - pt.size * 0.5, pt.size, pt.size, c);
        }

        for ft in &self.floating_texts {
            let alpha = (ft.timer / ft.max_timer).clamp(0.0, 1.0);
            let mut c = ft.color;
            c.a *= alpha;
            draw_pixel_text(font, &ft.text, ft.pos.x, ft.pos.y, 8, c);
        }

        // 6. Clean Non-Overlapping HUD + Bottom Astra Dock
        self.draw_level7_hud(font, stats, time);

        if let Some((msg, _)) = &self.action_message {
            let tw = measure_text(msg, Some(font), 8, 1.0).width + 24.0;
            let bw = tw.clamp(220.0, SCREEN_W - 24.0);
            let bx = (SCREEN_W - bw) * 0.5;
            draw_rectangle(bx, 42.0, bw, 22.0, Color::new(0.08, 0.06, 0.10, 0.92));
            draw_rectangle_lines(bx, 42.0, bw, 22.0, 1.2, PAL_GOLD_1);
            draw_pixel_text(font, msg, bx + 12.0, 57.0, 8, PAL_GOLD_0);
        }

        // 7. Dialogue Overlay
        if let Some((speaker, lines, idx)) = &self.dialogue {
            if *idx < lines.len() {
                draw_dialogue_box(font, speaker, &lines[*idx], time, *idx, lines.len(), virtual_mouse);
            }
        }

        draw_reticle(virtual_mouse, false);
    }

    fn draw_panchavati_arena(&self, time: f32, shake: Vec2) {
        // Sky color escalates with battle intensity
        let is_crimson = matches!(
            self.phase,
            Level7Phase::TrishirasChariot | Level7Phase::KharaShowdown
        );
        if is_crimson {
            draw_rectangle(0.0, 0.0, SCREEN_W, 85.0, Color::new(0.18, 0.05, 0.08, 1.0));
            draw_rectangle(0.0, 85.0, SCREEN_W, 85.0, Color::new(0.28, 0.08, 0.10, 1.0));
            draw_rectangle(0.0, 170.0, SCREEN_W, 70.0, Color::new(0.38, 0.13, 0.12, 1.0));
        } else {
            draw_rectangle(0.0, 0.0, SCREEN_W, 85.0, Color::new(0.14, 0.16, 0.28, 1.0));
            draw_rectangle(0.0, 85.0, SCREEN_W, 85.0, Color::new(0.24, 0.22, 0.34, 1.0));
            draw_rectangle(0.0, 170.0, SCREEN_W, 70.0, Color::new(0.36, 0.28, 0.32, 1.0));
        }

        // Sun disc veiled behind battle haze
        draw_circle(510.0, 74.0, 20.0, if is_crimson { PAL_FIRE_3 } else { PAL_GOLD_1 });
        draw_circle(510.0, 74.0, 14.0, if is_crimson { PAL_FIRE_1 } else { PAL_GOLD_0 });

        // Distant Western Ghats Hills
        for i in 0..6 {
            let hx = i as f32 * 130.0 - 20.0;
            draw_triangle(
                vec2(hx, 236.0),
                vec2(hx + 65.0, 128.0 + (i % 2) as f32 * 18.0),
                vec2(hx + 140.0, 236.0),
                Color::new(0.14, 0.12, 0.18, 0.92),
            );
        }

        // Pancha-Vata: The 5 Sacred Banyan Trees of Panchavati
        let banyans = [
            (130.0, 172.0, 18.0, 0),
            (245.0, 180.0, 20.0, 1),
            (360.0, 168.0, 18.0, 2),
            (475.0, 176.0, 19.0, 3),
            (595.0, 172.0, 18.0, 4),
        ];
        for (tx, th, tw, var) in banyans {
            draw_ancient_forest_tree(tx + shake.x * 0.25, 236.0, th, tw, var, time);
        }

        // Hermitage Shelter Sanctuary on far left (around Sita)
        draw_hermitage_kutir(42.0, 238.0, time);

        // Riverbank Grass & Sandy Trail
        draw_rectangle(0.0, 236.0, SCREEN_W, 20.0, PAL_GRASS_3);
        draw_rectangle(0.0, 236.0, SCREEN_W, 4.0, PAL_GRASS_2);
        draw_rectangle(0.0, 254.0, SCREEN_W, 8.0, PAL_SAND_4);

        // Sacred Godavari River in foreground (y = 260..360)
        draw_rectangle(0.0, 260.0, SCREEN_W, 45.0, Color::new(0.12, 0.34, 0.56, 1.0));
        draw_rectangle(0.0, 305.0, SCREEN_W, 55.0, Color::new(0.08, 0.22, 0.40, 1.0));
        for i in 0..10 {
            let rx = ((i as f32 * 75.0) + time * 16.0) % (SCREEN_W + 40.0) - 20.0;
            let ry = 268.0 + (i % 4) as f32 * 12.0;
            draw_line(rx, ry, rx + 28.0, ry, 1.2, Color::new(0.55, 0.85, 1.0, 0.45));
        }
        // Floating Lotus blossoms on Godavari
        for lx in [150.0, 310.0, 490.0] {
            let bob = (time * 2.5 + lx).sin() * 1.5;
            draw_ellipse(lx, 282.0 + bob, 8.0, 3.0, 0.0, PAL_GRASS_2);
            draw_circle(lx, 280.0 + bob, 3.2, Color::new(1.0, 0.70, 0.82, 0.95));
        }
    }

    fn draw_host_demon(&self, d: &HostDemon, time: f32) {
        let x = d.pos.x;
        let y = d.pos.y;
        let is_confused = d.confused_timer > 0.0;

        if d.hit_flash > 0.0 {
            draw_rectangle(x - 10.0, y - 26.0, 20.0, 26.0, WHITE);
            return;
        }

        let (body_col, head_col) = if is_confused {
            (PAL_PURPLE_1, PAL_PURPLE_0)
        } else {
            match d.kind {
                DemonKind::Vanguard => (PAL_RED_2, PAL_RED_1),
                DemonKind::FlyingArcher => (PAL_BLUE_2, PAL_BLUE_0),
                DemonKind::Infantry => (PAL_RED_3, PAL_RED_2),
            }
        };

        // Bat wings for flying archers
        if d.kind == DemonKind::FlyingArcher {
            let flap = (time * 14.0 + x).sin() * 6.0;
            draw_triangle(vec2(x, y - 16.0), vec2(x - 16.0, y - 24.0 + flap), vec2(x - 10.0, y - 10.0), PAL_BLUE_2);
            draw_triangle(vec2(x, y - 16.0), vec2(x + 16.0, y - 24.0 + flap), vec2(x + 10.0, y - 10.0), PAL_BLUE_2);
        } else {
            draw_ellipse(x, y + 1.0, 9.0, 3.5, 0.0, Color::new(0.0, 0.0, 0.0, 0.35));
        }

        // Body & Armor
        draw_rectangle(x - 6.0, y - 18.0, 12.0, 18.0, body_col);
        draw_rectangle_lines(x - 6.0, y - 18.0, 12.0, 18.0, 1.0, PAL_GOLD_3);
        // Head & horns
        draw_circle(x, y - 22.0, 5.0, head_col);
        draw_line(x - 3.0, y - 26.0, x - 6.0, y - 31.0, 1.5, PAL_SAND_1);
        draw_line(x + 3.0, y - 26.0, x + 6.0, y - 31.0, 1.5, PAL_SAND_1);
        draw_circle(x - 2.0, y - 22.0, 1.0, PAL_FIRE_0);

        // Weapon (Spear / Club)
        draw_line(x - 7.0, y - 10.0, x - 14.0, y - 24.0, 2.0, PAL_GOLD_2);

        if is_confused {
            draw_circle_lines(x, y - 32.0, 5.0, 1.2, PAL_GOLD_0);
        }
        if d.hp < d.max_hp {
            let pct = (d.hp / d.max_hp).clamp(0.0, 1.0);
            draw_rectangle(x - 7.0, y - 35.0, 14.0, 2.5, Color::new(0.1, 0.05, 0.05, 0.85));
            draw_rectangle(x - 7.0, y - 35.0, 14.0 * pct, 2.5, PAL_FIRE_1);
        }
    }

    fn draw_boss(&self, boss: &BossEntity, time: f32) {
        let x = boss.pos.x;
        let y = boss.pos.y;
        let bob = (time * 4.0).sin() * 2.0;

        if boss.hit_flash > 0.0 {
            draw_rectangle(x - 28.0, y - 56.0, 56.0, 62.0, WHITE);
            return;
        }

        // Chariot Chassis (for Trishiras & Khara when chariot_intact)
        if boss.chariot_intact {
            draw_rectangle(x - 36.0, y - 6.0, 72.0, 16.0, PAL_GOLD_3);
            draw_rectangle(x - 32.0, y - 4.0, 64.0, 12.0, PAL_RED_3);
            draw_rectangle_lines(x - 36.0, y - 6.0, 72.0, 16.0, 1.5, PAL_GOLD_1);
            // Blazing Ratha Wheels
            for wx in [x - 22.0, x + 22.0] {
                draw_circle(wx, y + 10.0, 11.0, PAL_GOLD_2);
                draw_circle(wx, y + 10.0, 7.0, Color::new(0.18, 0.12, 0.12, 1.0));
                draw_circle(wx, y + 10.0, 2.5, PAL_GOLD_0);
            }
            // Demonic Banner Staff
            draw_line(x + 28.0, y - 6.0, x + 28.0, y - 58.0, 2.0, PAL_GOLD_2);
            draw_triangle(vec2(x + 28.0, y - 58.0), vec2(x + 28.0, y - 44.0), vec2(x + 48.0, y - 51.0), PAL_RED_1);
        } else {
            draw_ellipse(x, y + 12.0, 28.0, 7.0, 0.0, Color::new(0.0, 0.0, 0.0, 0.45));
        }

        let stand_y = if boss.chariot_intact { y - 6.0 } else { y - 2.0 };

        // Muscular Legs & Bronze Greaves when on foot (Dushana / Dismounted Khara)
        if !boss.chariot_intact {
            draw_rectangle(x - 16.0, stand_y - 6.0, 11.0, 18.0, PAL_RED_3);
            draw_rectangle(x + 5.0, stand_y - 6.0, 11.0, 18.0, PAL_RED_3);
            draw_rectangle(x - 15.0, stand_y + 4.0, 9.0, 8.0, PAL_GOLD_3);
            draw_rectangle(x + 6.0, stand_y + 4.0, 9.0, 8.0, PAL_GOLD_3);
        }

        // Towering Armored Boss Torso + Spiked Pauldrons
        draw_rectangle(x - 20.0, stand_y - 38.0 + bob, 40.0, 32.0, PAL_RED_3);
        draw_rectangle(x - 16.0, stand_y - 34.0 + bob, 32.0, 26.0, PAL_RED_2);
        draw_rectangle(x - 10.0, stand_y - 31.0 + bob, 20.0, 12.0, PAL_RED_1);
        draw_rectangle_lines(x - 20.0, stand_y - 38.0 + bob, 40.0, 32.0, 1.5, PAL_GOLD_2);
        draw_circle(x, stand_y - 25.0 + bob, 3.5, PAL_GOLD_1); // Chest medallion
        // Shoulder Pauldron Spikes
        draw_triangle(vec2(x - 22.0, stand_y - 36.0 + bob), vec2(x - 14.0, stand_y - 38.0 + bob), vec2(x - 26.0, stand_y - 46.0 + bob), PAL_GOLD_1);
        draw_triangle(vec2(x + 14.0, stand_y - 38.0 + bob), vec2(x + 22.0, stand_y - 36.0 + bob), vec2(x + 26.0, stand_y - 46.0 + bob), PAL_GOLD_1);

        if boss.name.contains("TRISHIRAS") {
            // THREE DISTINCT HEADS WITH GOLDEN CROWNS!
            for (idx, hdx) in [-13.0f32, 0.0, 13.0].iter().enumerate() {
                let hx = x + hdx;
                let hy = stand_y - 46.0 + bob - if idx == 1 { 4.0 } else { 0.0 };
                draw_circle(hx, hy, 7.0, PAL_RED_1);
                draw_triangle(vec2(hx - 5.0, hy - 5.0), vec2(hx + 5.0, hy - 5.0), vec2(hx, hy - 14.0), PAL_GOLD_1);
                draw_circle(hx - 2.0, hy - 1.0, 1.3, PAL_FIRE_0);
                draw_circle(hx + 2.0, hy - 1.0, 1.3, PAL_FIRE_0);
            }
        } else {
            // Single Horned General Crowned Head (Dushana / Khara)
            let hy = stand_y - 48.0 + bob;
            draw_circle(x, hy, 10.0, PAL_RED_1);
            draw_triangle(vec2(x - 8.0, hy - 7.0), vec2(x + 8.0, hy - 7.0), vec2(x, hy - 20.0), PAL_GOLD_0);
            draw_line(x - 8.0, hy - 4.0, x - 16.0, hy - 14.0, 2.5, PAL_SAND_1);
            draw_line(x + 8.0, hy - 4.0, x + 16.0, hy - 14.0, 2.5, PAL_SAND_1);
            draw_circle(x - 3.5, hy - 1.0, 1.6, PAL_FIRE_0);
            draw_circle(x + 3.5, hy - 1.0, 1.6, PAL_FIRE_0);
        }

        // Boss Weapon (Giant Mace or Blazing Bow)
        if boss.name.contains("DUSHANA") {
            draw_line(x - 20.0, stand_y - 24.0, x - 34.0, stand_y - 48.0, 3.0, PAL_GOLD_2);
            draw_circle(x - 35.0, stand_y - 50.0, 7.5, PAL_GOLD_1);
        } else {
            // Great Rakshasa Bow
            draw_line(x - 22.0, stand_y - 44.0, x - 26.0, stand_y - 26.0, 2.5, PAL_GOLD_1);
            draw_line(x - 26.0, stand_y - 26.0, x - 22.0, stand_y - 8.0, 2.5, PAL_GOLD_1);
        }
    }

    fn draw_level7_hud(&self, font: &Font, stats: &PlayerStats, _time: f32) {
        // 1. Top-Left Prana & Tejas
        draw_pixel_text(font, "PRANA", 12.0, 12.0, 8, PAL_GOLD_1);
        draw_rectangle(12.0, 14.0, 106.0, 8.0, Color::new(0.12, 0.08, 0.10, 0.9));
        let pw = (stats.prana as f32 / stats.max_prana as f32).clamp(0.0, 1.0) * 104.0;
        draw_rectangle(13.0, 15.0, pw, 6.0, Color::new(0.85, 0.25, 0.25, 1.0));
        draw_rectangle_lines(12.0, 14.0, 106.0, 8.0, 1.0, PAL_GOLD_2);

        draw_pixel_text(font, "TEJAS", 12.0, 31.0, 8, Color::new(0.45, 0.85, 1.0, 1.0));
        draw_rectangle(12.0, 33.0, 106.0, 8.0, Color::new(0.08, 0.10, 0.14, 0.9));
        let tw = (stats.tejas as f32 / stats.max_tejas as f32).clamp(0.0, 1.0) * 104.0;
        draw_rectangle(13.0, 34.0, tw, 6.0, Color::new(0.25, 0.65, 0.95, 1.0));
        draw_rectangle_lines(12.0, 33.0, 106.0, 8.0, 1.0, PAL_GOLD_2);

        // 2. Top-Center Boss / Wave Health
        let bx = 160.0;
        let bw = 240.0;
        if let Some(ref boss) = self.boss {
            draw_pixel_text(font, boss.name, bx + 4.0, 12.0, 8, PAL_GOLD_0);
            draw_rectangle(bx, 15.0, bw, 10.0, Color::new(0.15, 0.08, 0.10, 0.92));
            let pct = (boss.hp / boss.max_hp).clamp(0.0, 1.0);
            draw_rectangle(bx + 1.0, 16.0, (bw - 2.0) * pct, 8.0, PAL_FIRE_1);
            draw_rectangle_lines(bx, 15.0, bw, 10.0, 1.2, PAL_GOLD_1);
        } else if self.phase == Level7Phase::Vanguard14 {
            let rem = (14 - self.vanguard_spawned + self.demons.len() as i32).max(0);
            draw_pixel_text(font, &format!("VANGUARD CHAMPIONS: {}", rem), bx + 20.0, 16.0, 8, PAL_GOLD_0);
        }

        // 3. Top-Right 14,000 Host Counter
        let hx = 430.0;
        draw_rectangle(hx, 8.0, 198.0, 26.0, Color::new(0.18, 0.06, 0.08, 0.92));
        draw_rectangle_lines(hx, 8.0, 198.0, 26.0, 1.2, PAL_FIRE_1);
        draw_pixel_text(
            font,
            &format!("HOST: {:>5} / 14000", self.host_remaining),
            hx + 10.0,
            25.0,
            8,
            if self.host_remaining == 0 { Color::new(0.45, 0.92, 0.55, 1.0) } else { PAL_GOLD_0 },
        );

        // 4. Bottom Astra Selection Bar (y = 326..352)
        let slots = [
            (Astra::KodandaBow, "[1] KODANDA", 14.0),
            (Astra::Vayavyastra, "[2] VAYAVYA", 128.0),
            (Astra::Manavastra, "[3] MANAVA", 242.0),
            (Astra::Agneyastra, "[4] AGNEYA", 350.0),
            (Astra::Gandharvastra, "[5] GANDHARVA", 458.0),
        ];
        for (astra, label, sx) in slots {
            let unlocked = stats.unlocked_astras.contains(&astra);
            let selected = stats.selected_astra == astra;
            let sw = if astra == Astra::Gandharvastra { 166.0 } else { 104.0 };
            let bg = if selected {
                Color::new(0.24, 0.18, 0.34, 0.95)
            } else if unlocked {
                Color::new(0.10, 0.09, 0.15, 0.88)
            } else {
                Color::new(0.06, 0.06, 0.08, 0.7)
            };
            draw_rectangle(sx, 326.0, sw, 24.0, bg);
            draw_rectangle_lines(
                sx,
                326.0,
                sw,
                24.0,
                if selected { 1.5 } else { 1.0 },
                if selected { PAL_GOLD_0 } else { Color::new(0.4, 0.35, 0.25, 0.8) },
            );
            let col = if !unlocked {
                Color::new(0.45, 0.45, 0.50, 0.8)
            } else if selected {
                PAL_GOLD_0
            } else {
                WHITE
            };
            draw_pixel_text(font, if unlocked { label } else { "[5] LOCKED" }, sx + 8.0, 342.0, 8, col);
        }
    }
}

// ============================================================================
// LEVEL 7 VICTORY SCREEN
// ============================================================================
pub fn draw_victory_l7(font: &Font, time: f32) {
    draw_rectangle(0.0, 0.0, SCREEN_W, SCREEN_H, Color::new(0.06, 0.05, 0.10, 0.96));

    let sway = (time * 5.0).sin() * 2.5;
    draw_pixel_text(font, "EKA-VIRA: 14,000 HOST VANQUISHED!", 64.0, 40.0 + sway, 16, PAL_GOLD_0);

    let sub = "PANCHAVATI ON THE GODAVARI — ARANYA KANDA VICTORY";
    draw_pixel_text(font, sub, 118.0, 60.0, 8, PAL_TURMERIC_1);

    let rw_x = 34.0;
    let rw_y = 76.0;
    let rw_w = 572.0;
    let rw_h = 212.0;
    draw_rectangle(rw_x, rw_y, rw_w, rw_h, Color::new(0.10, 0.09, 0.16, 0.95));
    draw_rectangle_lines(rw_x, rw_y, rw_w, rw_h, 1.5, PAL_GOLD_1);

    draw_pixel_text(font, "CHAPTER 7 FEATS ACCOMPLISHED (EKA-VIRA):", rw_x + 16.0, rw_y + 22.0, 8, PAL_GOLD_0);
    draw_pixel_text(font, "[+] Vanguard Slain  : Defeated Khara's 14 elite champions.", rw_x + 16.0, rw_y + 44.0, 8, WHITE);
    draw_pixel_text(font, "[+] General Dushana : Felled the mace-wielding commander.", rw_x + 16.0, rw_y + 64.0, 8, WHITE);
    draw_pixel_text(font, "[+] Trishiras Maya  : Host slew itself under Gandharvastra!", rw_x + 16.0, rw_y + 84.0, 8, PAL_PURPLE_0);
    draw_pixel_text(font, "[+] General Khara   : Shattered chariot & won < 1.5 muhurtas!", rw_x + 16.0, rw_y + 104.0, 8, Color::new(0.45, 0.92, 0.55, 1.0));

    draw_line(rw_x + 16.0, rw_y + 122.0, rw_x + rw_w - 16.0, rw_y + 122.0, 1.0, PAL_GOLD_3);
    draw_pixel_text(font, "★ NEXT IN RAMAYANA: GOLDEN DEER MARICHA & KISHKINDHA ★", rw_x + 16.0, rw_y + 142.0, 8, PAL_FIRE_1);
    draw_pixel_text(font, "Akampana carries word of Rama's valor to Lanka—Ravana plots", rw_x + 16.0, rw_y + 162.0, 8, Color::new(0.92, 0.85, 0.85, 1.0));
    draw_pixel_text(font, "the golden deer Maya before Sugriva's alliance in Kishkindha!", rw_x + 16.0, rw_y + 180.0, 8, Color::new(0.92, 0.85, 0.85, 1.0));

    let blink = (time * 4.0).sin() > 0.0;
    if blink {
        draw_pixel_text(font, "PRESS [SPACE] OR CLICK TO RETURN TO CHAPTER SELECT ▶", 92.0, 312.0, 8, PAL_GOLD_0);
    }
    draw_pixel_text(font, "Press [R] to Replay Chapter 7 (14,000 Host Siege)", 128.0, 336.0, 8, Color::new(0.75, 0.75, 0.80, 1.0));
}
