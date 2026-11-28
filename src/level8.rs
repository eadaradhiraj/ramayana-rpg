use macroquad::prelude::*;
use crate::audio::SoundEffects;
use crate::render::{
    draw_ancient_forest_tree, draw_astra_projectile, draw_dialogue_box, draw_hermitage_kutir,
    draw_hero_exile, draw_maricha, draw_pixel_text, draw_reticle, draw_sita_hermit,
    is_dialogue_prev_button_hovered, PAL_FIRE_0, PAL_FIRE_1, PAL_GOLD_0, PAL_GOLD_1,
    PAL_GOLD_2, PAL_GOLD_3, PAL_GRASS_2, PAL_GRASS_3, PAL_PURPLE_0, PAL_RED_0,
    PAL_RED_1, PAL_RED_2, PAL_RED_3, PAL_SAFFRON_0, PAL_SAFFRON_1, PAL_SAND_0,
    PAL_SAND_3, PAL_TURMERIC_1, SCREEN_H, SCREEN_W,
};
use crate::types::{AstraProjectileType, PlayerStats};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Level8Phase {
    IntroDialogue,
    GoldenDeerChase,
    MarichaRevealDialogue,
    LakshmanaRekha,
    RavanaAbductionDialogue,
    JatayuSkyDuel,
    JatayuFinaleDialogue,
    Complete,
}

#[derive(Clone, Debug)]
pub struct DeerTarget {
    pub pos: Vec2,
    pub vel: Vec2,
    pub is_real: bool,
    pub jump_phase: f32,
    pub hit_flash: f32,
    pub active: bool,
}

#[derive(Clone, Debug)]
pub struct L8Projectile {
    pub pos: Vec2,
    pub vel: Vec2,
    pub is_player: bool,
    pub active: bool,
}

#[derive(Clone, Debug)]
pub struct L8Particle {
    pub pos: Vec2,
    pub vel: Vec2,
    pub color: Color,
    pub size: f32,
    pub timer: f32,
    pub max_timer: f32,
}

#[derive(Clone, Debug)]
pub struct L8FloatingText {
    pub text: String,
    pub pos: Vec2,
    pub color: Color,
    pub timer: f32,
    pub max_timer: f32,
}

#[allow(dead_code)]
pub enum FightResultL8 {
    Continue,
    Victory,
    Defeat,
}

pub struct Level8Fight {
    pub phase: Level8Phase,
    // Act 1: Golden Deer Hunt
    pub rama_pos: Vec2,
    pub aim_dir: Vec2,
    pub bow_tension: f32,
    pub is_drawing: bool,
    pub deer_hits: i32,
    pub deer_targets: Vec<DeerTarget>,
    pub maricha_pos: Vec2,

    // Act 2: Lakshmana Rekha
    pub lakshmana_pos: Vec2,
    pub sita_pos: Vec2,
    pub rekha_progress: f32,

    // Act 3: Jatayu Sky Duel
    pub jatayu_pos: Vec2,
    pub jatayu_hp: i32,
    pub jatayu_max_hp: i32,
    pub jatayu_dash_cd: f32,
    pub jatayu_dash_timer: f32,
    pub jatayu_invuln: f32,
    pub vimana_pos: Vec2,
    pub vimana_hp: f32,
    pub vimana_max_hp: f32,
    pub vimana_attack_timer: f32,
    pub vimana_flash: f32,

    pub shake_timer: f32,
    pub projectiles: Vec<L8Projectile>,
    pub particles: Vec<L8Particle>,
    pub floating_texts: Vec<L8FloatingText>,
    pub dialogue: Option<(String, Vec<String>, usize)>,
    pub action_message: Option<(String, f32)>,
}

impl Level8Fight {
    pub fn new(stats: &mut PlayerStats) -> Self {
        stats.prana = stats.max_prana;
        stats.tejas = stats.max_tejas;

        let mut fight = Self {
            phase: Level8Phase::IntroDialogue,
            rama_pos: vec2(120.0, 240.0),
            aim_dir: vec2(1.0, 0.0),
            bow_tension: 0.0,
            is_drawing: false,
            deer_hits: 0,
            deer_targets: vec![
                DeerTarget {
                    pos: vec2(460.0, 240.0),
                    vel: vec2(85.0, 0.0),
                    is_real: true,
                    jump_phase: 0.0,
                    hit_flash: 0.0,
                    active: true,
                },
                DeerTarget {
                    pos: vec2(320.0, 240.0),
                    vel: vec2(-95.0, 0.0),
                    is_real: false,
                    jump_phase: 1.5,
                    hit_flash: 0.0,
                    active: true,
                },
            ],
            maricha_pos: vec2(480.0, 224.0),

            lakshmana_pos: vec2(210.0, 240.0),
            sita_pos: vec2(140.0, 240.0),
            rekha_progress: 0.0,

            jatayu_pos: vec2(140.0, 170.0),
            jatayu_hp: 100,
            jatayu_max_hp: 100,
            jatayu_dash_cd: 0.0,
            jatayu_dash_timer: 0.0,
            jatayu_invuln: 0.0,
            vimana_pos: vec2(500.0, 150.0),
            vimana_hp: 240.0,
            vimana_max_hp: 240.0,
            vimana_attack_timer: 0.0,
            vimana_flash: 0.0,

            shake_timer: 0.0,
            projectiles: Vec::new(),
            particles: Vec::new(),
            floating_texts: Vec::new(),
            dialogue: None,
            action_message: Some((
                "PANCHAVATI GLADE — THE ENCHANTED GOLDEN DEER APPEARS!".to_string(),
                5.0,
            )),
        };

        fight.dialogue = Some((
            "Panchavati Hermitage Glade".to_string(),
            vec![
                "Princess Sita: 'O Prince! Behold that wondrous deer of burnished gold and silver stars! Capture it for our ashrama!'".to_string(),
                "Prince Lakshmana: 'Brother, beware! No natural deer shines like gems—this is surely Rakshasa Maricha's Maya!'".to_string(),
                "Prince Rama: 'Guard Janaki closely, Lakshmana! Real or illusion, I will bring this deer back—dead or alive!'".to_string(),
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
    ) -> FightResultL8 {
        if self.shake_timer > 0.0 {
            self.shake_timer = (self.shake_timer - dt).max(0.0);
        }
        if self.jatayu_invuln > 0.0 {
            self.jatayu_invuln = (self.jatayu_invuln - dt).max(0.0);
        }
        if self.jatayu_dash_cd > 0.0 {
            self.jatayu_dash_cd = (self.jatayu_dash_cd - dt).max(0.0);
        }
        if self.jatayu_dash_timer > 0.0 {
            self.jatayu_dash_timer = (self.jatayu_dash_timer - dt).max(0.0);
        }
        if self.vimana_flash > 0.0 {
            self.vimana_flash = (self.vimana_flash - dt).max(0.0);
        }
        if let Some((_, ref mut timer)) = self.action_message {
            *timer -= dt;
            if *timer <= 0.0 {
                self.action_message = None;
            }
        }

        // Active Dialogue Handling
        if let Some((_, ref lines, ref mut idx)) = self.dialogue {
            let prev_clicked =
                is_mouse_button_pressed(MouseButton::Left) && is_dialogue_prev_button_hovered(virtual_mouse);
            let prev_key = is_key_pressed(KeyCode::Left) || is_key_pressed(KeyCode::Backspace);

            if (prev_clicked || prev_key) && *idx > 0 {
                *idx -= 1;
                sounds.play(&sounds.shoot, 0.4);
                return FightResultL8::Continue;
            }
            if interact {
                *idx += 1;
                sounds.play(&sounds.shoot, 0.4);
                if *idx >= lines.len() {
                    self.dialogue = None;
                    return self.on_dialogue_complete(sounds);
                }
            }
            return FightResultL8::Continue;
        }

        // Developer Quick-Advance Key [K]
        if is_key_pressed(KeyCode::K) {
            match self.phase {
                Level8Phase::GoldenDeerChase => {
                    self.deer_hits = 3;
                    self.trigger_maricha_reveal(sounds);
                    return FightResultL8::Continue;
                }
                Level8Phase::LakshmanaRekha => {
                    self.rekha_progress = 100.0;
                    self.trigger_ravana_abduction(sounds);
                    return FightResultL8::Continue;
                }
                Level8Phase::JatayuSkyDuel => {
                    self.vimana_hp = 0.0;
                    self.trigger_jatayu_finale(sounds);
                    return FightResultL8::Continue;
                }
                _ => {}
            }
        }

        // Phase Logic
        match self.phase {
            Level8Phase::GoldenDeerChase => {
                self.update_deer_chase(dt, sounds, virtual_mouse);
            }
            Level8Phase::LakshmanaRekha => {
                self.update_lakshmana_rekha(dt, sounds, interact);
            }
            Level8Phase::JatayuSkyDuel => {
                if let Some(res) = self.update_jatayu_duel(dt, sounds, stats, virtual_mouse, interact) {
                    return res;
                }
            }
            Level8Phase::Complete => return FightResultL8::Victory,
            _ => {}
        }

        // Update shared particles & floating texts
        for pt in &mut self.particles {
            pt.timer -= dt;
            pt.pos += pt.vel * dt;
        }
        self.particles.retain(|p| p.timer > 0.0);

        for ft in &mut self.floating_texts {
            ft.timer -= dt;
            ft.pos.y -= 14.0 * dt;
        }
        self.floating_texts.retain(|ft| ft.timer > 0.0);

        FightResultL8::Continue
    }

    fn on_dialogue_complete(&mut self, sounds: &SoundEffects) -> FightResultL8 {
        match self.phase {
            Level8Phase::IntroDialogue => {
                self.phase = Level8Phase::GoldenDeerChase;
                self.action_message = Some((
                    "ACT I: SHOOT THE TRUE SPARKLING GOLDEN DEER (3 HITS)!".to_string(),
                    5.0,
                ));
            }
            Level8Phase::MarichaRevealDialogue => {
                self.phase = Level8Phase::LakshmanaRekha;
                self.projectiles.clear();
                sounds.play(&sounds.bell, 0.7);
                self.action_message = Some((
                    "ACT II: MOVE NEAR SITA & HOLD/PRESS [SPACE] TO DRAW LAKSHMANA REKHA!".to_string(),
                    6.0,
                ));
            }
            Level8Phase::RavanaAbductionDialogue => {
                self.phase = Level8Phase::JatayuSkyDuel;
                self.projectiles.clear();
                sounds.play(&sounds.roar, 0.9);
                self.action_message = Some((
                    "ACT III: [WASD] FLY AS JATAYU! [SPACE/CLICK] TALON STRIKE RAVANA!".to_string(),
                    6.0,
                ));
            }
            Level8Phase::JatayuFinaleDialogue => {
                self.phase = Level8Phase::Complete;
                return FightResultL8::Victory;
            }
            _ => {}
        }
        FightResultL8::Continue
    }

    fn update_deer_chase(&mut self, dt: f32, sounds: &SoundEffects, virtual_mouse: Vec2) {
        let mut mx = 0.0;
        if is_key_down(KeyCode::A) || is_key_down(KeyCode::Left) {
            mx -= 1.0;
        }
        if is_key_down(KeyCode::D) || is_key_down(KeyCode::Right) {
            mx += 1.0;
        }
        self.rama_pos.x = (self.rama_pos.x + mx * 165.0 * dt).clamp(40.0, 560.0);

        let origin = self.rama_pos + vec2(0.0, -24.0);
        let dir = (virtual_mouse - origin).normalize_or_zero();
        if dir.length_squared() > 0.01 {
            self.aim_dir = dir;
        }

        let down = is_mouse_button_down(MouseButton::Left);
        if down && !self.is_drawing {
            self.is_drawing = true;
            self.bow_tension = 0.0;
        } else if down && self.is_drawing {
            self.bow_tension = (self.bow_tension + dt * 3.0).min(1.0);
        } else if !down && self.is_drawing {
            self.is_drawing = false;
            if self.bow_tension > 0.15 {
                sounds.play(&sounds.shoot, 0.7);
                self.projectiles.push(L8Projectile {
                    pos: origin + self.aim_dir * 12.0,
                    vel: self.aim_dir * (400.0 + self.bow_tension * 220.0),
                    is_player: true,
                    active: true,
                });
            }
            self.bow_tension = 0.0;
        }

        // Move Deer & Illusion Decoys
        let mut new_particles: Vec<L8Particle> = Vec::new();
        let mut hit_true_deer = false;
        let mut hit_decoy_pos: Option<Vec2> = None;

        for deer in &mut self.deer_targets {
            if !deer.active {
                continue;
            }
            if deer.hit_flash > 0.0 {
                deer.hit_flash = (deer.hit_flash - dt).max(0.0);
            }
            deer.jump_phase += dt * 4.5;
            deer.pos.x += deer.vel.x * dt;
            if deer.pos.x < 180.0 {
                deer.pos.x = 180.0;
                deer.vel.x = deer.vel.x.abs();
            } else if deer.pos.x > 600.0 {
                deer.pos.x = 600.0;
                deer.vel.x = -deer.vel.x.abs();
            }

            // Real golden deer sheds starlight sparks!
            if deer.is_real && new_particles.len() < 4 && (deer.jump_phase * 10.0) as i32 % 5 == 0 {
                new_particles.push(L8Particle {
                    pos: deer.pos + vec2(0.0, -10.0),
                    vel: vec2(0.0, -25.0),
                    color: PAL_GOLD_0,
                    size: 2.5,
                    timer: 0.45,
                    max_timer: 0.45,
                });
            }

            let d_center = deer.pos + vec2(0.0, -10.0 - deer.jump_phase.sin().abs() * 14.0);
            for p in self.projectiles.iter_mut().filter(|p| p.active) {
                if (p.pos - d_center).length() < 20.0 {
                    p.active = false;
                    if deer.is_real {
                        deer.hit_flash = 0.25;
                        deer.vel.x = -deer.vel.x.signum() * (115.0 + self.deer_hits as f32 * 25.0);
                        hit_true_deer = true;
                    } else {
                        deer.active = false;
                        hit_decoy_pos = Some(d_center);
                    }
                    break;
                }
            }
        }
        self.particles.extend(new_particles);

        if let Some(dpos) = hit_decoy_pos {
            sounds.play(&sounds.wind, 0.6);
            self.floating_texts.push(L8FloatingText {
                text: "ILLUSION DECOY!".to_string(),
                pos: dpos + vec2(-35.0, -15.0),
                color: PAL_PURPLE_0,
                timer: 1.0,
                max_timer: 1.0,
            });
            // Respawn another decoy to keep chase lively
            self.deer_targets.push(DeerTarget {
                pos: vec2(560.0, 240.0),
                vel: vec2(-105.0, 0.0),
                is_real: false,
                jump_phase: 0.7,
                hit_flash: 0.0,
                active: true,
            });
        }
        self.deer_targets.retain(|d| d.active);

        if hit_true_deer {
            self.deer_hits += 1;
            self.shake_timer = 0.25;
            sounds.play(&sounds.hit, 0.9);
            self.floating_texts.push(L8FloatingText {
                text: format!("TRUE HIT {}/3!", self.deer_hits),
                pos: vec2(320.0, 150.0),
                color: PAL_GOLD_0,
                timer: 1.1,
                max_timer: 1.1,
            });
            if self.deer_hits >= 3 {
                if let Some(real) = self.deer_targets.iter().find(|d| d.is_real) {
                    self.maricha_pos = vec2(real.pos.x, 224.0);
                }
                self.trigger_maricha_reveal(sounds);
            }
        }

        for p in &mut self.projectiles {
            p.pos += p.vel * dt;
            if p.pos.x < -10.0 || p.pos.x > SCREEN_W + 10.0 || p.pos.y < -10.0 || p.pos.y > SCREEN_H + 10.0 {
                p.active = false;
            }
        }
        self.projectiles.retain(|p| p.active);
    }

    fn trigger_maricha_reveal(&mut self, sounds: &SoundEffects) {
        self.phase = Level8Phase::MarichaRevealDialogue;
        self.shake_timer = 1.0;
        sounds.play(&sounds.thunder, 0.9);
        self.dialogue = Some((
            "Maricha's True Form — The Fatal Cry".to_string(),
            vec![
                "Struck cleanly by Rama's shaft, the golden deer expands into the towering Rakshasa Maricha!".to_string(),
                "Dying Maricha mimics Rama's voice across the forest: 'HA SITA! HA LAKSHMANA! SAVE ME!'".to_string(),
                "Realizing the terrible trap, Rama sprints back toward the Panchavati hermitage!".to_string(),
            ],
            0,
        ));
    }

    fn update_lakshmana_rekha(&mut self, dt: f32, sounds: &SoundEffects, interact: bool) {
        let mut mx = 0.0;
        if is_key_down(KeyCode::A) || is_key_down(KeyCode::Left) {
            mx -= 1.0;
        }
        if is_key_down(KeyCode::D) || is_key_down(KeyCode::Right) {
            mx += 1.0;
        }
        self.lakshmana_pos.x = (self.lakshmana_pos.x + mx * 140.0 * dt).clamp(90.0, 290.0);

        let near_hut = (self.lakshmana_pos.x - self.sita_pos.x).abs() < 110.0;
        if near_hut && (interact || is_key_down(KeyCode::Space) || is_key_down(KeyCode::E)) {
            let add = if interact { 25.0 } else { 55.0 * dt };
            self.rekha_progress = (self.rekha_progress + add).min(100.0);
            if self.particles.len() < 30 {
                self.particles.push(L8Particle {
                    pos: vec2(self.sita_pos.x + (self.rekha_progress - 50.0) * 0.9, 240.0),
                    vel: vec2(0.0, -35.0),
                    color: PAL_FIRE_1,
                    size: 2.5,
                    timer: 0.5,
                    max_timer: 0.5,
                });
            }
            if self.rekha_progress >= 100.0 {
                self.trigger_ravana_abduction(sounds);
            }
        }
    }

    fn trigger_ravana_abduction(&mut self, sounds: &SoundEffects) {
        self.phase = Level8Phase::RavanaAbductionDialogue;
        sounds.play(&sounds.roar, 0.9);
        self.dialogue = Some((
            "Lakshmana Rekha & Dashagriva Ravana".to_string(),
            vec![
                "Lakshmana traces seven blazing Vedic arcs: 'Sita Devi, never step beyond this sacred Lakshmana Rekha!'".to_string(),
                "Once Lakshmana departs, Ravana arrives disguised as a humble Yati beggar seeking alms beyond the fiery line.".to_string(),
                "When Sita steps across out of dharma for a guest, Ravana erupts into his ten-headed form atop Pushpaka Vimana!".to_string(),
                "High above the trees, elderly Eagle King JATAYU hears Sita's cries and dives straight into Ravana's sky-chariot!".to_string(),
            ],
            0,
        ));
    }

    fn update_jatayu_duel(
        &mut self,
        dt: f32,
        sounds: &SoundEffects,
        stats: &mut PlayerStats,
        virtual_mouse: Vec2,
        interact: bool,
    ) -> Option<FightResultL8> {
        let mut mv = Vec2::ZERO;
        if is_key_down(KeyCode::W) || is_key_down(KeyCode::Up) {
            mv.y -= 1.0;
        }
        if is_key_down(KeyCode::S) || is_key_down(KeyCode::Down) {
            mv.y += 1.0;
        }
        if is_key_down(KeyCode::A) || is_key_down(KeyCode::Left) {
            mv.x -= 1.0;
        }
        if is_key_down(KeyCode::D) || is_key_down(KeyCode::Right) {
            mv.x += 1.0;
        }

        // Talon Dive Strike (Space or Left Click) launches fierce burst toward Ravana / mouse
        if (interact || is_key_pressed(KeyCode::LeftShift)) && self.jatayu_dash_cd <= 0.0 {
            self.jatayu_dash_cd = 0.55;
            self.jatayu_dash_timer = 0.25;
            sounds.play(&sounds.wind, 0.8);
            // Fire golden feather gale forward
            let strike_dir = (self.vimana_pos - self.jatayu_pos).normalize_or_zero();
            self.projectiles.push(L8Projectile {
                pos: self.jatayu_pos + strike_dir * 16.0,
                vel: strike_dir * 420.0,
                is_player: true,
                active: true,
            });
        }

        let speed = if self.jatayu_dash_timer > 0.0 { 320.0 } else { 175.0 };
        if self.jatayu_dash_timer > 0.0 && mv.length_squared() < 0.01 {
            mv = (virtual_mouse - self.jatayu_pos).normalize_or_zero();
        } else {
            mv = mv.normalize_or_zero();
        }
        self.jatayu_pos.x = (self.jatayu_pos.x + mv.x * speed * dt).clamp(40.0, 560.0);
        self.jatayu_pos.y = (self.jatayu_pos.y + mv.y * speed * dt).clamp(64.0, 290.0);

        // Melee Talon Dive Hitbox against Ravana's Pushpaka Vimana
        if self.jatayu_dash_timer > 0.0 && (self.jatayu_pos - self.vimana_pos).length() < 46.0 {
            self.jatayu_dash_timer = 0.0;
            self.vimana_hp = (self.vimana_hp - 36.0).max(0.0);
            self.vimana_flash = 0.18;
            self.shake_timer = 0.35;
            sounds.play(&sounds.smash, 0.9);
            self.floating_texts.push(L8FloatingText {
                text: "TALON REND! -36".to_string(),
                pos: self.vimana_pos + vec2(-32.0, -40.0),
                color: PAL_GOLD_0,
                timer: 0.9,
                max_timer: 0.9,
            });
        }

        // Vimana motion & Ravana's Dark Spear Barrage
        self.vimana_attack_timer += dt;
        self.vimana_pos.y = 148.0 + (self.vimana_attack_timer * 1.8).sin() * 48.0;
        self.vimana_pos.x = 490.0 + (self.vimana_attack_timer * 1.1).cos() * 45.0;

        if ((self.vimana_attack_timer * 10.0) as i32) % 16 == 0 && self.projectiles.iter().filter(|p| !p.is_player).count() < 5 {
            let dir = (self.jatayu_pos - self.vimana_pos).normalize_or_zero();
            self.projectiles.push(L8Projectile {
                pos: self.vimana_pos + vec2(-22.0, -10.0),
                vel: dir * 215.0,
                is_player: false,
                active: true,
            });
            sounds.play(&sounds.shoot, 0.5);
        }

        // Projectiles vs Vimana & Jatayu
        for p in self.projectiles.iter_mut().filter(|p| p.active) {
            p.pos += p.vel * dt;
            if p.is_player {
                if (p.pos - self.vimana_pos).length() < 36.0 {
                    p.active = false;
                    self.vimana_hp = (self.vimana_hp - 24.0).max(0.0);
                    self.vimana_flash = 0.14;
                    sounds.play(&sounds.hit, 0.8);
                    self.floating_texts.push(L8FloatingText {
                        text: "-24".to_string(),
                        pos: p.pos + vec2(0.0, -12.0),
                        color: PAL_GOLD_0,
                        timer: 0.8,
                        max_timer: 0.8,
                    });
                }
            } else if self.jatayu_invuln <= 0.0 && (p.pos - self.jatayu_pos).length() < 20.0 {
                p.active = false;
                self.jatayu_hp = (self.jatayu_hp - 14).max(0);
                stats.prana = self.jatayu_hp;
                self.jatayu_invuln = 0.6;
                self.shake_timer = 0.3;
                sounds.play(&sounds.hurt, 0.8);
                if self.jatayu_hp <= 0 {
                    return Some(FightResultL8::Defeat);
                }
            }

            if p.pos.x < -20.0 || p.pos.x > SCREEN_W + 20.0 || p.pos.y < -20.0 || p.pos.y > SCREEN_H + 20.0 {
                p.active = false;
            }
        }
        self.projectiles.retain(|p| p.active);

        if self.vimana_hp <= 0.0 {
            self.trigger_jatayu_finale(sounds);
        }
        None
    }

    fn trigger_jatayu_finale(&mut self, sounds: &SoundEffects) {
        self.phase = Level8Phase::JatayuFinaleDialogue;
        self.projectiles.clear();
        self.shake_timer = 1.2;
        sounds.play(&sounds.thunder, 1.0);
        self.dialogue = Some((
            "Jatayu's Immortal Sacrifice".to_string(),
            vec![
                "With thunderous wingbeats, noble Jatayu shatters Pushpaka's golden yoke and snaps Ravana's celestial bow!".to_string(),
                "Enraged, Ten-Headed Ravana unsheathes Chandrahasa and severs Jatayu's wings before fleeing southward across the clouds.".to_string(),
                "Grieving yet unbroken, Jatayu preserves one final breath for Rama: 'Ravana took Sita SOUTHWARD toward Lanka!'".to_string(),
                "With Rama's blessings upon the fallen hero, the brothers turn south toward sacred Rishyamuka & KISHKINDHA!".to_string(),
            ],
            0,
        ));
    }

    // ========================================================================
    // RENDERING (720p CRISP 16-BIT VEDIC ART)
    // ========================================================================
    pub fn draw(&self, font: &Font, time: f32, stats: &PlayerStats, virtual_mouse: Vec2) {
        let shake = if self.shake_timer > 0.0 {
            vec2((time * 40.0).sin() * 3.0, (time * 50.0).cos() * 3.0)
        } else {
            Vec2::ZERO
        };

        match self.phase {
            Level8Phase::IntroDialogue
            | Level8Phase::GoldenDeerChase
            | Level8Phase::MarichaRevealDialogue => {
                self.draw_glade_arena(time, shake);
                if matches!(self.phase, Level8Phase::MarichaRevealDialogue) {
                    draw_maricha(self.maricha_pos, 0.0, time);
                } else {
                    for deer in &self.deer_targets {
                        if deer.active {
                            self.draw_golden_deer(deer, time);
                        }
                    }
                }
                draw_hero_exile(self.rama_pos, true, self.aim_dir, self.bow_tension, self.is_drawing, time);
                if self.phase == Level8Phase::IntroDialogue {
                    draw_hero_exile(vec2(74.0, 240.0), false, vec2(1.0, 0.0), 0.0, false, time);
                    draw_sita_hermit(vec2(48.0, 240.0), time);
                }
            }
            Level8Phase::LakshmanaRekha | Level8Phase::RavanaAbductionDialogue => {
                self.draw_glade_arena(time, shake);
                draw_hermitage_kutir(self.sita_pos.x - 42.0, 242.0, time);
                draw_sita_hermit(self.sita_pos, time);
                draw_hero_exile(self.lakshmana_pos, false, vec2(-1.0, 0.0), 0.0, false, time);

                // Blazing Lakshmana Rekha fiery threshold arcs on earth!
                if self.rekha_progress > 0.0 {
                    let radius = 44.0 * (self.rekha_progress / 100.0);
                    let glow = ((time * 8.0).sin() * 0.25 + 0.75).clamp(0.0, 1.0);
                    draw_ellipse(
                        self.sita_pos.x,
                        242.0,
                        radius,
                        9.0,
                        0.0,
                        Color::new(1.0, 0.75, 0.15, glow),
                    );
                    draw_line(
                        self.sita_pos.x + radius,
                        236.0,
                        self.sita_pos.x + radius,
                        248.0,
                        2.5,
                        PAL_FIRE_0,
                    );
                }

                if self.phase == Level8Phase::LakshmanaRekha {
                    draw_pixel_text(font, "[SPACE] DRAW LAKSHMANA REKHA", 104.0, 198.0, 8, PAL_GOLD_0);
                } else {
                    // Ravana entering in saffron disguise / revealing Dashagriva
                    self.draw_ravana_vimana(vec2(480.0, 145.0), 0.0, time);
                }
            }
            Level8Phase::JatayuSkyDuel | Level8Phase::JatayuFinaleDialogue | Level8Phase::Complete => {
                self.draw_sky_duel_arena(time, shake);
                self.draw_ravana_vimana(self.vimana_pos + shake, self.vimana_flash, time);
                if !(self.jatayu_invuln > 0.0 && ((self.jatayu_invuln * 18.0) as i32 % 2 == 0)) {
                    self.draw_jatayu(self.jatayu_pos, self.jatayu_dash_timer > 0.0, time);
                }
            }
        }

        // Projectiles & Particles
        for p in &self.projectiles {
            if p.active {
                if p.is_player {
                    draw_astra_projectile(p.pos, p.vel, AstraProjectileType::Kodanda, time);
                } else {
                    let tail = p.pos - p.vel.normalize_or_zero() * 14.0;
                    draw_line(tail.x, tail.y, p.pos.x, p.pos.y, 2.5, PAL_RED_0);
                    draw_circle(p.pos.x, p.pos.y, 2.5, PAL_FIRE_1);
                }
            }
        }
        for pt in &self.particles {
            let a = (pt.timer / pt.max_timer).clamp(0.0, 1.0);
            let mut c = pt.color;
            c.a *= a;
            draw_rectangle(pt.pos.x - pt.size * 0.5, pt.pos.y - pt.size * 0.5, pt.size, pt.size, c);
        }
        for ft in &self.floating_texts {
            let a = (ft.timer / ft.max_timer).clamp(0.0, 1.0);
            let mut c = ft.color;
            c.a *= a;
            draw_pixel_text(font, &ft.text, ft.pos.x, ft.pos.y, 8, c);
        }

        // Top HUD (cleanly spaced, crisp 8px)
        self.draw_level8_hud(font, stats);

        if let Some((msg, _)) = &self.action_message {
            let tw = measure_text(msg, Some(font), 8, 1.0).width + 24.0;
            let bw = tw.clamp(220.0, SCREEN_W - 24.0);
            let bx = (SCREEN_W - bw) * 0.5;
            draw_rectangle(bx, 42.0, bw, 22.0, Color::new(0.08, 0.07, 0.12, 0.92));
            draw_rectangle_lines(bx, 42.0, bw, 22.0, 1.2, PAL_GOLD_1);
            draw_pixel_text(font, msg, bx + 12.0, 57.0, 8, PAL_GOLD_0);
        }

        if let Some((speaker, lines, idx)) = &self.dialogue {
            if *idx < lines.len() {
                draw_dialogue_box(font, speaker, &lines[*idx], time, *idx, lines.len(), virtual_mouse);
            }
        }

        draw_reticle(virtual_mouse, false);
    }

    fn draw_glade_arena(&self, time: f32, shake: Vec2) {
        // Golden Sunlit Morning Sky Gradient
        draw_rectangle(0.0, 0.0, SCREEN_W, 85.0, Color::new(0.24, 0.42, 0.68, 1.0));
        draw_rectangle(0.0, 85.0, SCREEN_W, 85.0, Color::new(0.38, 0.58, 0.78, 1.0));
        draw_rectangle(0.0, 170.0, SCREEN_W, 72.0, Color::new(0.62, 0.74, 0.78, 1.0));

        // Warm Sunlight Beams
        draw_circle(520.0, 64.0, 22.0, PAL_GOLD_1);
        draw_circle(520.0, 64.0, 15.0, PAL_GOLD_0);

        for i in 0..6 {
            let hx = i as f32 * 130.0 - 20.0;
            draw_triangle(
                vec2(hx, 240.0),
                vec2(hx + 65.0, 134.0 + (i % 2) as f32 * 16.0),
                vec2(hx + 140.0, 240.0),
                Color::new(0.22, 0.34, 0.40, 0.85),
            );
        }

        for (tx, th, tw, var) in [(55.0, 185.0, 18.0, 0), (210.0, 175.0, 18.0, 1), (380.0, 190.0, 20.0, 2), (565.0, 180.0, 18.0, 3)] {
            draw_ancient_forest_tree(tx + shake.x * 0.25, 240.0, th, tw, var, time);
        }

        // Lush Meadow Floor & Wildflowers
        draw_rectangle(0.0, 240.0, SCREEN_W, 24.0, PAL_GRASS_2);
        draw_rectangle(0.0, 264.0, SCREEN_W, SCREEN_H - 264.0, PAL_GRASS_3);
        for i in 0..14 {
            let fx = i as f32 * 48.0 + 18.0;
            draw_circle(fx, 250.0 + (i % 3) as f32 * 8.0, 2.0, if i % 2 == 0 { PAL_GOLD_0 } else { Color::new(1.0, 0.7, 0.85, 1.0) });
        }
    }

    fn draw_sky_duel_arena(&self, time: f32, shake: Vec2) {
        // Dramatic Storming Dusk Sky high above Dandakaranya
        draw_rectangle(0.0, 0.0, SCREEN_W, 110.0, Color::new(0.14, 0.10, 0.24, 1.0));
        draw_rectangle(0.0, 110.0, SCREEN_W, 120.0, Color::new(0.30, 0.16, 0.28, 1.0));
        draw_rectangle(0.0, 230.0, SCREEN_W, 130.0, Color::new(0.46, 0.24, 0.24, 1.0));

        // Scrolling Mountain Cloud Banks below
        for i in 0..8 {
            let cx = ((i as f32 * 100.0) - time * 55.0).rem_euclid(SCREEN_W + 120.0) - 60.0 + shake.x;
            let cy = 305.0 + (i % 3) as f32 * 14.0;
            draw_circle(cx, cy, 34.0, Color::new(0.75, 0.65, 0.72, 0.45));
            draw_circle(cx + 28.0, cy + 6.0, 26.0, Color::new(0.85, 0.75, 0.78, 0.4));
        }

        // Far Dandakaranya forest canopy silhouettes at bottom edge
        for i in 0..9 {
            let px = i as f32 * 84.0 - 15.0;
            draw_triangle(vec2(px, SCREEN_H), vec2(px + 48.0, 316.0), vec2(px + 96.0, SCREEN_H), Color::new(0.09, 0.14, 0.12, 0.9));
        }
    }

    fn draw_golden_deer(&self, deer: &DeerTarget, time: f32) {
        let bounce = deer.jump_phase.sin().abs() * 14.0;
        let x = deer.pos.x;
        let y = deer.pos.y - bounce;
        let dir = if deer.vel.x >= 0.0 { 1.0 } else { -1.0 };

        draw_ellipse(x, deer.pos.y + 1.0, 12.0, 4.0, 0.0, Color::new(0.0, 0.0, 0.0, 0.35));

        if deer.hit_flash > 0.0 {
            draw_rectangle(x - 10.0, y - 20.0, 20.0, 20.0, WHITE);
            return;
        }

        let body_c = if deer.is_real { PAL_GOLD_1 } else { Color::new(0.85, 0.70, 0.35, 0.78) };
        let hi_c = if deer.is_real { PAL_GOLD_0 } else { PAL_TURMERIC_1 };

        // Graceful legs
        let stride = (deer.jump_phase * 2.0).sin() * 4.0;
        draw_line(x - 6.0, y - 6.0, x - 8.0 + stride, y, 2.0, PAL_GOLD_3);
        draw_line(x + 6.0, y - 6.0, x + 8.0 - stride, y, 2.0, PAL_GOLD_3);

        // Burnished gold body + jeweled silver spots on real Maricha
        draw_rectangle(x - 9.0, y - 13.0, 18.0, 8.0, body_c);
        draw_circle(x - 3.0, y - 10.0, 1.4, WHITE);
        draw_circle(x + 2.0, y - 8.0, 1.4, WHITE);

        // Neck, head & golden antlers
        let hx = x + 9.0 * dir;
        let hy = y - 19.0;
        draw_line(x + 6.0 * dir, y - 11.0, hx, hy, 4.0, body_c);
        draw_circle(hx, hy, 4.0, hi_c);
        draw_line(hx, hy - 3.0, hx - 3.0 * dir, hy - 10.0, 1.5, PAL_SAND_0);
        draw_line(hx - 1.0 * dir, hy - 6.0, hx + 3.0 * dir, hy - 10.0, 1.2, PAL_SAND_0);

        // Shimmer aura on real deer
        if deer.is_real {
            let p = ((time * 6.0).sin() * 0.25 + 0.45).clamp(0.0, 1.0);
            draw_circle_lines(x, y - 10.0, 18.0, 1.0, Color::new(1.0, 0.95, 0.5, p));
        }
    }

    fn draw_jatayu(&self, pos: Vec2, dashing: bool, time: f32) {
        let x = pos.x;
        let y = pos.y;
        let flap = (time * if dashing { 24.0 } else { 11.0 }).sin() * 14.0;

        if dashing {
            draw_circle_lines(x, y, 24.0, 1.8, PAL_GOLD_0);
        }

        // Mighty Wingspan (Multi-tone Eagle Brown & Gold Wingtips)
        draw_triangle(vec2(x, y - 2.0), vec2(x - 28.0, y - 14.0 + flap), vec2(x - 10.0, y + 8.0), PAL_SAND_3);
        draw_triangle(vec2(x - 4.0, y), vec2(x - 24.0, y - 10.0 + flap), vec2(x - 8.0, y + 6.0), PAL_GOLD_2);
        draw_triangle(vec2(x, y - 2.0), vec2(x + 20.0, y - 14.0 - flap), vec2(x + 8.0, y + 6.0), PAL_SAND_3);

        // Regal Eagle Torso & Golden Talons
        draw_ellipse(x, y + 2.0, 12.0, 7.0, 0.0, Color::new(0.45, 0.26, 0.14, 1.0));
        draw_line(x + 4.0, y + 6.0, x + 12.0, y + 12.0, 2.0, PAL_GOLD_1);
        draw_line(x + 2.0, y + 7.0, x + 9.0, y + 14.0, 2.0, PAL_GOLD_1);

        // Crowned Eagle Head & Hooked Beak
        draw_circle(x + 11.0, y - 4.0, 5.5, WHITE);
        draw_triangle(vec2(x + 15.0, y - 5.0), vec2(x + 15.0, y - 1.0), vec2(x + 23.0, y - 1.0), PAL_GOLD_0);
        draw_circle(x + 13.0, y - 5.0, 1.2, PAL_FIRE_1);
    }

    fn draw_ravana_vimana(&self, pos: Vec2, flash: f32, time: f32) {
        let x = pos.x;
        let y = pos.y + (time * 3.0).sin() * 3.0;
        if flash > 0.0 {
            draw_rectangle(x - 40.0, y - 44.0, 80.0, 62.0, WHITE);
            return;
        }

        // Golden Swan-Winged Pushpaka Vimana Chariot
        draw_rectangle(x - 38.0, y, 76.0, 16.0, PAL_GOLD_2);
        draw_rectangle(x - 34.0, y + 2.0, 68.0, 12.0, PAL_RED_3);
        draw_rectangle_lines(x - 38.0, y, 76.0, 16.0, 1.5, PAL_GOLD_0);
        // Swan Prow on left + Celestial Gems
        draw_triangle(vec2(x - 38.0, y + 14.0), vec2(x - 38.0, y), vec2(x - 52.0, y - 8.0), PAL_GOLD_0);
        draw_circle(x - 48.0, y - 6.0, 2.0, PAL_RED_0);

        // Princess Sita seated in back right of Pushpaka
        draw_sita_hermit(vec2(x + 22.0, y + 2.0), time);

        // TEN-HEADED RAVANA (DASHAGRIVA!)
        draw_rectangle(x - 16.0, y - 28.0, 28.0, 28.0, PAL_RED_2);
        draw_rectangle_lines(x - 16.0, y - 28.0, 28.0, 28.0, 1.2, PAL_GOLD_1);
        // 5 Front Visible Demon Heads (representing Ten-Headed majesty!) + Golden Crowns
        for (idx, hdx) in [-16.0f32, -9.0, -2.0, 5.0, 12.0].iter().enumerate() {
            let hx = x + hdx;
            let hy = y - 34.0 - if idx == 2 { 3.0 } else { 0.0 };
            draw_circle(hx, hy, 4.5, PAL_RED_1);
            draw_triangle(vec2(hx - 3.0, hy - 3.0), vec2(hx + 3.0, hy - 3.0), vec2(hx, hy - 9.0), PAL_GOLD_0);
            draw_circle(hx - 1.5, hy, 0.9, PAL_FIRE_0);
        }
        // Dark Chandrahasa sword / bow held forward
        draw_line(x - 16.0, y - 16.0, x - 34.0, y - 26.0, 2.5, Color::new(0.85, 0.90, 1.0, 1.0));
    }

    fn draw_level8_hud(&self, font: &Font, stats: &PlayerStats) {
        let is_sky = self.phase == Level8Phase::JatayuSkyDuel;
        let hp_label = if is_sky { "JATAYU HP" } else { "PRANA" };
        let hp_val = if is_sky { self.jatayu_hp } else { stats.prana };
        let hp_max = if is_sky { self.jatayu_max_hp } else { stats.max_prana };

        draw_pixel_text(font, hp_label, 12.0, 12.0, 8, PAL_GOLD_1);
        draw_rectangle(12.0, 15.0, 114.0, 8.0, Color::new(0.12, 0.08, 0.10, 0.9));
        let pw = (hp_val as f32 / hp_max as f32).clamp(0.0, 1.0) * 112.0;
        draw_rectangle(13.0, 16.0, pw, 6.0, Color::new(0.85, 0.25, 0.25, 1.0));
        draw_rectangle_lines(12.0, 15.0, 114.0, 8.0, 1.0, PAL_GOLD_2);

        let bx = 180.0;
        let bw = 280.0;
        match self.phase {
            Level8Phase::GoldenDeerChase => {
                draw_pixel_text(font, &format!("GOLDEN DEER TRACKED: {} / 3 HITS", self.deer_hits), bx + 16.0, 12.0, 8, PAL_GOLD_0);
                draw_rectangle(bx, 15.0, bw, 10.0, Color::new(0.12, 0.10, 0.14, 0.92));
                draw_rectangle(bx + 1.0, 16.0, (bw - 2.0) * (self.deer_hits as f32 / 3.0), 8.0, PAL_GOLD_1);
                draw_rectangle_lines(bx, 15.0, bw, 10.0, 1.0, PAL_GOLD_1);
            }
            Level8Phase::LakshmanaRekha => {
                draw_pixel_text(font, &format!("LAKSHMANA REKHA WARD: {:.0}%", self.rekha_progress), bx + 28.0, 12.0, 8, PAL_FIRE_1);
                draw_rectangle(bx, 15.0, bw, 10.0, Color::new(0.12, 0.10, 0.14, 0.92));
                draw_rectangle(bx + 1.0, 16.0, (bw - 2.0) * (self.rekha_progress / 100.0), 8.0, PAL_SAFFRON_1);
                draw_rectangle_lines(bx, 15.0, bw, 10.0, 1.0, PAL_GOLD_1);
            }
            Level8Phase::JatayuSkyDuel => {
                draw_pixel_text(font, "RAVANA'S PUSHPAKA VIMANA", bx + 40.0, 12.0, 8, PAL_GOLD_0);
                draw_rectangle(bx, 15.0, bw, 10.0, Color::new(0.15, 0.08, 0.10, 0.92));
                draw_rectangle(bx + 1.0, 16.0, (bw - 2.0) * (self.vimana_hp / self.vimana_max_hp).clamp(0.0, 1.0), 8.0, PAL_FIRE_1);
                draw_rectangle_lines(bx, 15.0, bw, 10.0, 1.2, PAL_GOLD_1);
            }
            _ => {}
        }
    }
}

// ============================================================================
// LEVEL 8 VICTORY SCREEN
// ============================================================================
pub fn draw_victory_l8(font: &Font, time: f32) {
    draw_rectangle(0.0, 0.0, SCREEN_W, SCREEN_H, Color::new(0.06, 0.05, 0.10, 0.96));

    let sway = (time * 5.0).sin() * 2.5;
    draw_pixel_text(font, "ARANYA KANDA: JATAYU'S SACRIFICE!", 64.0, 40.0 + sway, 16, PAL_GOLD_0);

    let sub = "GOLDEN DEER MARICHA, LAKSHMANA REKHA & RAVANA SKY DUEL";
    draw_pixel_text(font, sub, 98.0, 60.0, 8, PAL_TURMERIC_1);

    let rw_x = 34.0;
    let rw_y = 76.0;
    let rw_w = 572.0;
    let rw_h = 212.0;
    draw_rectangle(rw_x, rw_y, rw_w, rw_h, Color::new(0.10, 0.09, 0.16, 0.95));
    draw_rectangle_lines(rw_x, rw_y, rw_w, rw_h, 1.5, PAL_GOLD_1);

    draw_pixel_text(font, "CHAPTER 8 MILESTONES COMPLETED (ARANYA KANDA FINALE):", rw_x + 16.0, rw_y + 22.0, 8, PAL_GOLD_0);
    draw_pixel_text(font, "[+] Golden Deer Slain : Pierced Maricha's Maya disguise!", rw_x + 16.0, rw_y + 44.0, 8, WHITE);
    draw_pixel_text(font, "[+] Lakshmana Rekha   : Traced blazing 7-fold ward circle!", rw_x + 16.0, rw_y + 64.0, 8, PAL_SAFFRON_0);
    draw_pixel_text(font, "[+] Pushpaka Shatter  : Battled 10-headed Ravana as Jatayu!", rw_x + 16.0, rw_y + 84.0, 8, PAL_FIRE_1);
    draw_pixel_text(font, "[+] Southward Clue    : Jatayu revealed Lanka's direction!", rw_x + 16.0, rw_y + 104.0, 8, Color::new(0.45, 0.92, 0.55, 1.0));

    draw_line(rw_x + 16.0, rw_y + 122.0, rw_x + rw_w - 16.0, rw_y + 122.0, 1.0, PAL_GOLD_3);
    draw_pixel_text(font, "★ NEXT BOOK: KISHKINDHA KANDA (HANUMAN, SUGRIVA & VALI) ★", rw_x + 16.0, rw_y + 142.0, 8, PAL_FIRE_1);
    draw_pixel_text(font, "Armed with Jatayu's dying words, Rama & Lakshmana journey", rw_x + 16.0, rw_y + 162.0, 8, Color::new(0.92, 0.85, 0.85, 1.0));
    draw_pixel_text(font, "south to Rishyamuka Hill to meet Sri Hanuman & Sugriva!", rw_x + 16.0, rw_y + 180.0, 8, Color::new(0.92, 0.85, 0.85, 1.0));

    let blink = (time * 4.0).sin() > 0.0;
    if blink {
        draw_pixel_text(font, "PRESS [SPACE] OR CLICK TO RETURN TO CHAPTER SELECT ▶", 92.0, 312.0, 8, PAL_GOLD_0);
    }
    draw_pixel_text(font, "Press [R] to Replay Chapter 8 (Maricha & Jatayu)", 128.0, 336.0, 8, Color::new(0.75, 0.75, 0.80, 1.0));
}
