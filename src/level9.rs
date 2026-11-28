use macroquad::prelude::*;
use crate::audio::SoundEffects;
use crate::render::{
    draw_ancient_forest_tree, draw_astra_projectile, draw_dialogue_box, draw_hero_exile,
    draw_pixel_text, draw_reticle, is_dialogue_prev_button_hovered, PAL_FIRE_1,
    PAL_FIRE_3, PAL_GOLD_0, PAL_GOLD_1, PAL_GOLD_2, PAL_GOLD_3, PAL_GRASS_1, PAL_GRASS_2,
    PAL_GRASS_3, PAL_RED_1, PAL_RED_2, PAL_SAFFRON_0, PAL_SAFFRON_1, PAL_SAFFRON_2, PAL_SAND_0,
    PAL_SAND_1, PAL_SAND_3, PAL_SAND_4, PAL_TURMERIC_1, SCREEN_H, SCREEN_W,
};
use crate::types::{AstraProjectileType, PlayerStats};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Level9Phase {
    IntroDialogue,
    SevenSalTreesTrial,
    ValiFirstDuelDialogue,
    ValiFirstDuel,
    GarlandDialogue,
    ValiSecondDuel,
    ValiDharmaDialogue,
    SugrivaCoronationDialogue,
    Complete,
}

#[derive(Clone, Debug)]
pub struct SalTree {
    pub pos: Vec2,
    pub pierced: bool,
    pub shake: f32,
}

#[derive(Clone, Debug)]
pub struct L9Projectile {
    pub pos: Vec2,
    pub vel: Vec2,
    pub is_player: bool,
    pub pierced_count: usize,
    pub active: bool,
}

#[derive(Clone, Debug)]
pub struct L9Particle {
    pub pos: Vec2,
    pub vel: Vec2,
    pub color: Color,
    pub size: f32,
    pub timer: f32,
    pub max_timer: f32,
}

#[derive(Clone, Debug)]
pub struct L9FloatingText {
    pub text: String,
    pub pos: Vec2,
    pub color: Color,
    pub timer: f32,
    pub max_timer: f32,
}

#[allow(dead_code)]
pub enum FightResultL9 {
    Continue,
    Victory,
    Defeat,
}

pub struct Level9Fight {
    pub phase: Level9Phase,

    // Act I: Rishyamuka — Dundubhi Skeleton & 7 Sal Trees Trial
    pub rama_pos: Vec2,
    pub aim_dir: Vec2,
    pub bow_tension: f32,
    pub is_drawing: bool,
    pub dundubhi_pos: Vec2,
    pub dundubhi_vel: Vec2,
    pub dundubhi_kicked: bool,
    pub dundubhi_gone: bool,
    pub sal_trees: Vec<SalTree>,
    pub trees_pierced_once: bool,
    pub trial_flash_timer: f32,

    // Act II & III: Kishkindha Gate — Sugriva & Vali
    pub sugriva_pos: Vec2,
    pub sugriva_hp: i32,
    pub sugriva_max_hp: i32,
    pub sugriva_invuln: f32,
    pub sugriva_strike_cd: f32,
    pub sugriva_dash_cd: f32,
    pub sugriva_dash_timer: f32,
    pub sugriva_hits_landed: i32,

    pub vali_pos: Vec2,
    pub vali_hp: f32,
    pub vali_max_hp: f32,
    pub vali_attack_timer: f32,
    pub vali_flash: f32,
    pub clash_phase: f32,

    pub rama_duel_invuln: f32,
    pub shake_timer: f32,
    pub projectiles: Vec<L9Projectile>,
    pub particles: Vec<L9Particle>,
    pub floating_texts: Vec<L9FloatingText>,
    pub dialogue: Option<(String, Vec<String>, usize)>,
    pub action_message: Option<(String, f32)>,
}

impl Level9Fight {
    pub fn new(stats: &mut PlayerStats) -> Self {
        stats.prana = stats.max_prana;
        stats.tejas = stats.max_tejas;

        // 7 Collinear Sal Trees sloping gently upward on Rishyamuka ridge
        // Start (255, 256) to End (531, 226) -> exact straight line so 1 clean shot pierces all 7!
        let mut sal_trees = Vec::with_capacity(7);
        for i in 0..7 {
            let t = i as f32 / 6.0;
            sal_trees.push(SalTree {
                pos: vec2(255.0 + t * 276.0, 256.0 - t * 30.0),
                pierced: false,
                shake: 0.0,
            });
        }

        let mut fight = Self {
            phase: Level9Phase::IntroDialogue,
            rama_pos: vec2(105.0, 260.0),
            aim_dir: vec2(1.0, -0.1),
            bow_tension: 0.0,
            is_drawing: false,
            dundubhi_pos: vec2(195.0, 268.0),
            dundubhi_vel: Vec2::ZERO,
            dundubhi_kicked: false,
            dundubhi_gone: false,
            sal_trees,
            trees_pierced_once: false,
            trial_flash_timer: 0.0,

            sugriva_pos: vec2(240.0, 258.0),
            sugriva_hp: 100,
            sugriva_max_hp: 100,
            sugriva_invuln: 0.0,
            sugriva_strike_cd: 0.0,
            sugriva_dash_cd: 0.0,
            sugriva_dash_timer: 0.0,
            sugriva_hits_landed: 0,

            vali_pos: vec2(470.0, 258.0),
            vali_hp: 180.0,
            vali_max_hp: 180.0,
            vali_attack_timer: 0.6,
            vali_flash: 0.0,
            clash_phase: 0.0,

            rama_duel_invuln: 0.0,
            shake_timer: 0.0,
            projectiles: Vec::new(),
            particles: Vec::new(),
            floating_texts: Vec::new(),
            dialogue: None,
            action_message: Some((
                "LAKE PAMPA & RISHYAMUKA — HANUMAN GUIDES THE PRINCES TO SUGRIVA!".to_string(),
                5.0,
            )),
        };

        fight.dialogue = Some((
            "Rishyamuka Mountain Sanctuary".to_string(),
            vec![
                "Sri Hanuman: 'O Princes of Raghu! I am Hanuman, son of Vayu. Mount my shoulders—King Sugriva awaits atop Rishyamuka!'".to_string(),
                "King Sugriva: 'Behold these silk wrappings and golden anklets—Mother Sita cast them down as Ravana carried her south!'".to_string(),
                "King Sugriva: 'With Sacred Agni as witness, I pledge my Vanaras! Yet my brother Vali stole my throne—and wears Indra's invincible boon!'".to_string(),
                "King Sugriva: 'If even half my strength passes to Vali in battle, can your arrows match him? Behold Dundubhi's bones and the Seven Sal Trees!'".to_string(),
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
    ) -> FightResultL9 {
        if self.shake_timer > 0.0 {
            self.shake_timer = (self.shake_timer - dt).max(0.0);
        }
        if self.sugriva_invuln > 0.0 {
            self.sugriva_invuln = (self.sugriva_invuln - dt).max(0.0);
        }
        if self.sugriva_strike_cd > 0.0 {
            self.sugriva_strike_cd = (self.sugriva_strike_cd - dt).max(0.0);
        }
        if self.sugriva_dash_cd > 0.0 {
            self.sugriva_dash_cd = (self.sugriva_dash_cd - dt).max(0.0);
        }
        if self.sugriva_dash_timer > 0.0 {
            self.sugriva_dash_timer = (self.sugriva_dash_timer - dt).max(0.0);
        }
        if self.vali_flash > 0.0 {
            self.vali_flash = (self.vali_flash - dt).max(0.0);
        }
        if self.rama_duel_invuln > 0.0 {
            self.rama_duel_invuln = (self.rama_duel_invuln - dt).max(0.0);
        }
        if self.trial_flash_timer > 0.0 {
            self.trial_flash_timer = (self.trial_flash_timer - dt).max(0.0);
        }
        if let Some((_, ref mut timer)) = self.action_message {
            *timer -= dt;
            if *timer <= 0.0 {
                self.action_message = None;
            }
        }

        // Active Dialogue Handling (with Back/Re-read support)
        if let Some((_, ref lines, ref mut idx)) = self.dialogue {
            let prev_clicked =
                is_mouse_button_pressed(MouseButton::Left) && is_dialogue_prev_button_hovered(virtual_mouse);
            let prev_key = is_key_pressed(KeyCode::Left) || is_key_pressed(KeyCode::Backspace);

            if (prev_clicked || prev_key) && *idx > 0 {
                *idx -= 1;
                sounds.play(&sounds.shoot, 0.4);
                return FightResultL9::Continue;
            }
            if interact {
                *idx += 1;
                sounds.play(&sounds.shoot, 0.4);
                if *idx >= lines.len() {
                    self.dialogue = None;
                    return self.on_dialogue_complete(sounds);
                }
            }
            return FightResultL9::Continue;
        }

        // Developer Quick-Advance Key [K]
        if is_key_pressed(KeyCode::K) {
            match self.phase {
                Level9Phase::SevenSalTreesTrial => {
                    self.dundubhi_kicked = true;
                    self.dundubhi_gone = true;
                    for t in &mut self.sal_trees {
                        t.pierced = true;
                    }
                    self.trigger_vali_duel1_dialogue(sounds);
                    return FightResultL9::Continue;
                }
                Level9Phase::ValiFirstDuel => {
                    self.sugriva_hits_landed = 5;
                    self.trigger_garland_dialogue(sounds);
                    return FightResultL9::Continue;
                }
                Level9Phase::ValiSecondDuel => {
                    self.vali_hp = 0.0;
                    self.trigger_vali_dharma_dialogue(sounds);
                    return FightResultL9::Continue;
                }
                _ => {}
            }
        }

        // Phase Logic
        match self.phase {
            Level9Phase::SevenSalTreesTrial => {
                self.update_sal_trees_trial(dt, sounds, virtual_mouse, interact);
            }
            Level9Phase::ValiFirstDuel => {
                if let Some(res) = self.update_vali_first_duel(dt, sounds, interact) {
                    return res;
                }
            }
            Level9Phase::ValiSecondDuel => {
                if let Some(res) = self.update_vali_second_duel(dt, sounds, stats, virtual_mouse, interact) {
                    return res;
                }
            }
            Level9Phase::Complete => return FightResultL9::Victory,
            _ => {}
        }

        // Shared particle & floating text tick
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

        FightResultL9::Continue
    }

    fn on_dialogue_complete(&mut self, sounds: &SoundEffects) -> FightResultL9 {
        match self.phase {
            Level9Phase::IntroDialogue => {
                self.phase = Level9Phase::SevenSalTreesTrial;
                self.action_message = Some((
                    "ACT I: WALK TO DUNDUBHI'S SKELETON & PRESS [SPACE] TO FLICK IT!".to_string(),
                    6.0,
                ));
            }
            Level9Phase::ValiFirstDuelDialogue => {
                self.phase = Level9Phase::ValiFirstDuel;
                self.projectiles.clear();
                self.sugriva_pos = vec2(210.0, 262.0);
                self.sugriva_hp = 100;
                self.sugriva_hits_landed = 0;
                self.vali_pos = vec2(470.0, 262.0);
                self.vali_hp = 180.0;
                sounds.play(&sounds.roar, 0.9);
                self.action_message = Some((
                    "ACT II: FIGHT AS SUGRIVA! [WASD] MOVE, [SPACE/CLICK] STRIKE, [SHIFT] DODGE!".to_string(),
                    6.5,
                ));
            }
            Level9Phase::GarlandDialogue => {
                self.phase = Level9Phase::ValiSecondDuel;
                self.projectiles.clear();
                self.rama_pos = vec2(78.0, 262.0);
                self.sugriva_pos = vec2(275.0, 262.0);
                self.vali_pos = vec2(465.0, 262.0);
                self.vali_hp = 180.0;
                self.vali_attack_timer = 1.4;
                sounds.play(&sounds.conch, 0.9);
                self.action_message = Some((
                    "ACT III: AIM KODANDA AS RAMA & STRIKE VALI (AVOID GARLANDED SUGRIVA)!".to_string(),
                    6.5,
                ));
            }
            Level9Phase::ValiDharmaDialogue => {
                self.phase = Level9Phase::SugrivaCoronationDialogue;
                sounds.play(&sounds.bell, 0.8);
                self.dialogue = Some((
                    "Kishkindha Citadel — Sugriva's Coronation".to_string(),
                    vec![
                        "King Sugriva: 'By Rama's grace I hold Kishkindha! Four great Vanara armies shall scour North, East, West, and South for Sita!'".to_string(),
                        "Lord Rama: 'Take my golden Mudrika Signet Ring, Hanuman. Only you carry the devotion to reach across the southern sea!'".to_string(),
                        "Sri Hanuman: 'Rama-Nama dwells in my heart! At the southern shore of Mahendragiri, I shall leap the hundred-yojana Ocean!'".to_string(),
                    ],
                    0,
                ));
            }
            Level9Phase::SugrivaCoronationDialogue => {
                self.phase = Level9Phase::Complete;
                return FightResultL9::Victory;
            }
            _ => {}
        }
        FightResultL9::Continue
    }

    fn update_sal_trees_trial(
        &mut self,
        dt: f32,
        sounds: &SoundEffects,
        virtual_mouse: Vec2,
        interact: bool,
    ) {
        let mut mv = Vec2::ZERO;
        if is_key_down(KeyCode::A) || is_key_down(KeyCode::Left) {
            mv.x -= 1.0;
        }
        if is_key_down(KeyCode::D) || is_key_down(KeyCode::Right) {
            mv.x += 1.0;
        }
        if is_key_down(KeyCode::W) || is_key_down(KeyCode::Up) {
            mv.y -= 1.0;
        }
        if is_key_down(KeyCode::S) || is_key_down(KeyCode::Down) {
            mv.y += 1.0;
        }
        if mv.length_squared() > 0.0 {
            self.rama_pos += mv.normalize() * 145.0 * dt;
        }
        self.rama_pos.x = self.rama_pos.x.clamp(45.0, 215.0);
        self.rama_pos.y = self.rama_pos.y.clamp(215.0, 295.0);

        // Animate kicked Dundubhi skeleton flying off 10 yojanas!
        if self.dundubhi_kicked && !self.dundubhi_gone {
            self.dundubhi_pos += self.dundubhi_vel * dt;
            self.dundubhi_vel.y += 280.0 * dt;
            if self.dundubhi_pos.x > SCREEN_W + 60.0 || self.dundubhi_pos.y < -60.0 {
                self.dundubhi_gone = true;
            }
        }

        // Step 1: Flick Dundubhi's Skeleton with Rama's Big Toe
        if !self.dundubhi_kicked {
            let near_bones = (self.rama_pos - self.dundubhi_pos).length() < 54.0;
            if near_bones && interact {
                self.dundubhi_kicked = true;
                self.dundubhi_vel = vec2(440.0, -290.0);
                self.shake_timer = 0.25;
                sounds.play(&sounds.smash, 0.9);
                self.spawn_burst(self.dundubhi_pos, PAL_SAND_0, 18);
                self.floating_texts.push(L9FloatingText {
                    text: "FLICKED 10 YOJANAS WITH HIS BIG TOE!".to_string(),
                    pos: self.dundubhi_pos + vec2(-90.0, -24.0),
                    color: PAL_GOLD_0,
                    timer: 2.0,
                    max_timer: 2.0,
                });
                self.action_message = Some((
                    "NOW LINE UP ALL 7 SAL TREES WITH KODANDA & PIERCE THEM IN ONE ARROW!".to_string(),
                    6.5,
                ));
            }
            return;
        }

        // Step 2: 7 Sal Trees Single-Arrow Alignment
        for t in &mut self.sal_trees {
            if t.shake > 0.0 {
                t.shake = (t.shake - dt).max(0.0);
            }
        }

        let origin = self.rama_pos + vec2(0.0, -24.0);
        let dir = (virtual_mouse - origin).normalize_or_zero();
        if dir.length_squared() > 0.01 {
            self.aim_dir = dir;
        }

        let down = is_mouse_button_down(MouseButton::Left);
        let quick_space = is_key_pressed(KeyCode::Space);

        if down && !self.is_drawing {
            self.is_drawing = true;
            self.bow_tension = 0.0;
        } else if down && self.is_drawing {
            self.bow_tension = (self.bow_tension + dt * 2.8).min(1.0);
        } else if (!down && self.is_drawing) || quick_space {
            let power = if quick_space { 0.85 } else { self.bow_tension };
            self.is_drawing = false;
            self.bow_tension = 0.0;
            if power > 0.15 {
                sounds.play(&sounds.shoot, 0.8);
                for t in &mut self.sal_trees {
                    t.pierced = false;
                }
                self.projectiles.push(L9Projectile {
                    pos: origin + self.aim_dir * 12.0,
                    vel: self.aim_dir * 620.0,
                    is_player: true,
                    pierced_count: 0,
                    active: true,
                });
            }
        }

        // Tick arrows through Sal Trees
        let mut all_seven_hit = false;
        let mut partial_miss: Option<(Vec2, usize)> = None;
        let mut burst_events: Vec<Vec2> = Vec::new();

        for p in &mut self.projectiles {
            if !p.active {
                continue;
            }
            // Sub-step 4x per frame so fast arrow never skips a slender trunk
            let steps = 4;
            let sub_dt = dt / steps as f32;
            for _ in 0..steps {
                p.pos += p.vel * sub_dt;
                for tree in &mut self.sal_trees {
                    if tree.pierced {
                        continue;
                    }
                    // Trunk piercing zone: centered near y - 34 .. y - 10
                    let dx = (p.pos.x - tree.pos.x).abs();
                    let dy = (p.pos.y - (tree.pos.y - 22.0)).abs();
                    if dx < 12.0 && dy < 20.0 {
                        tree.pierced = true;
                        tree.shake = 0.45;
                        p.pierced_count += 1;
                        burst_events.push(vec2(tree.pos.x, p.pos.y));
                    }
                }
            }

            if p.pos.x > SCREEN_W + 20.0 || p.pos.y < -20.0 || p.pos.y > SCREEN_H + 20.0 {
                p.active = false;
                if p.pierced_count == 7 {
                    all_seven_hit = true;
                } else if p.pierced_count > 0 {
                    partial_miss = Some((vec2(300.0, 175.0), p.pierced_count));
                }
            }
        }
        self.projectiles.retain(|p| p.active);

        for bpos in burst_events {
            sounds.play(&sounds.hit, 0.5);
            self.spawn_burst(bpos, PAL_GOLD_0, 8);
        }

        if all_seven_hit && !self.trees_pierced_once {
            self.trees_pierced_once = true;
            self.trial_flash_timer = 0.8;
            self.shake_timer = 0.45;
            sounds.play(&sounds.thunder, 0.9);
            sounds.play(&sounds.victory, 0.7);
            self.floating_texts.push(L9FloatingText {
                text: "SAPTA-SALA-BHEDANA! ALL 7 TREES PIERCED!".to_string(),
                pos: vec2(215.0, 155.0),
                color: PAL_GOLD_0,
                timer: 2.2,
                max_timer: 2.2,
            });
            self.trigger_vali_duel1_dialogue(sounds);
        } else if let Some((fpos, cnt)) = partial_miss {
            sounds.play(&sounds.wind, 0.5);
            self.floating_texts.push(L9FloatingText {
                text: format!("PIERCED {}/7 — LINE ALL 7 TREES IN GREEN & SHOOT!", cnt),
                pos: fpos,
                color: PAL_SAFFRON_0,
                timer: 1.6,
                max_timer: 1.6,
            });
        }
    }

    fn count_aligned_trees(&self) -> usize {
        let origin = self.rama_pos + vec2(0.0, -24.0);
        if self.aim_dir.x <= 0.1 {
            return 0;
        }
        let mut aligned = 0;
        for t in &self.sal_trees {
            let t_ray = (t.pos.x - origin.x) / self.aim_dir.x;
            if t_ray > 0.0 {
                let y_at_tree = origin.y + self.aim_dir.y * t_ray;
                if (y_at_tree - (t.pos.y - 22.0)).abs() < 18.0 {
                    aligned += 1;
                }
            }
        }
        aligned
    }

    fn trigger_vali_duel1_dialogue(&mut self, sounds: &SoundEffects) {
        self.phase = Level9Phase::ValiFirstDuelDialogue;
        self.projectiles.clear();
        sounds.play(&sounds.conch, 0.8);
        self.dialogue = Some((
            "Gates of Kishkindha Citadel".to_string(),
            vec![
                "King Sugriva: 'Unbelievable! One Kodanda shaft clove all seven Sal trees, entered Patala, and returned to your quiver!'".to_string(),
                "Lord Rama: 'March to Kishkindha's gate, Sugriva! Sound your thunderous challenge. I shall watch from behind these Sal trees!'".to_string(),
                "King Vali: 'WHO DARES ROAR AT MY CITADEL?! Foolish Sugriva—have you forgotten my golden Indra-Mala?!'".to_string(),
            ],
            0,
        ));
    }

    fn update_vali_first_duel(
        &mut self,
        dt: f32,
        sounds: &SoundEffects,
        interact: bool,
    ) -> Option<FightResultL9> {
        let mut mv = Vec2::ZERO;
        if is_key_down(KeyCode::A) || is_key_down(KeyCode::Left) {
            mv.x -= 1.0;
        }
        if is_key_down(KeyCode::D) || is_key_down(KeyCode::Right) {
            mv.x += 1.0;
        }
        if is_key_down(KeyCode::W) || is_key_down(KeyCode::Up) {
            mv.y -= 1.0;
        }
        if is_key_down(KeyCode::S) || is_key_down(KeyCode::Down) {
            mv.y += 1.0;
        }

        // Dash with Shift / Right Click
        if (is_key_pressed(KeyCode::LeftShift) || is_mouse_button_pressed(MouseButton::Right))
            && self.sugriva_dash_cd <= 0.0
        {
            self.sugriva_dash_timer = 0.22;
            self.sugriva_dash_cd = 0.85;
            self.sugriva_invuln = 0.28;
            sounds.play(&sounds.wind, 0.6);
        }

        let speed = if self.sugriva_dash_timer > 0.0 { 310.0 } else { 170.0 };
        if mv.length_squared() > 0.0 {
            self.sugriva_pos += mv.normalize() * speed * dt;
        }
        self.sugriva_pos.x = self.sugriva_pos.x.clamp(125.0, 585.0);
        self.sugriva_pos.y = self.sugriva_pos.y.clamp(210.0, 305.0);

        // Sugriva Melee Strike (SPACE or Left Click)
        if interact && self.sugriva_strike_cd <= 0.0 {
            self.sugriva_strike_cd = 0.35;
            let dist = (self.sugriva_pos - self.vali_pos).length();
            if dist < 62.0 {
                self.vali_hp -= 18.0;
                self.vali_flash = 0.14;
                self.sugriva_hits_landed += 1;
                self.shake_timer = 0.16;
                sounds.play(&sounds.smash, 0.85);
                self.spawn_burst(self.vali_pos + vec2(0.0, -22.0), PAL_GOLD_0, 12);
                self.floating_texts.push(L9FloatingText {
                    text: format!("VANARA CLASH! ({}/5)", self.sugriva_hits_landed),
                    pos: self.vali_pos + vec2(-36.0, -52.0),
                    color: PAL_GOLD_0,
                    timer: 1.0,
                    max_timer: 1.0,
                });

                if self.sugriva_hits_landed >= 5 {
                    // Indra-Mala power surges! Trigger Garland Dialogue!
                    self.trigger_garland_dialogue(sounds);
                    return None;
                }
            } else {
                sounds.play(&sounds.shoot, 0.4);
                self.floating_texts.push(L9FloatingText {
                    text: "STEP CLOSER TO STRIKE VALI!".to_string(),
                    pos: self.sugriva_pos + vec2(-58.0, -45.0),
                    color: PAL_SAFFRON_0,
                    timer: 0.7,
                    max_timer: 0.7,
                });
            }
        }

        // Vali chases Sugriva and hurls shockwave boulders
        let to_sug = (self.sugriva_pos - self.vali_pos).normalize_or_zero();
        self.vali_pos += to_sug * 76.0 * dt;
        self.vali_pos.x = self.vali_pos.x.clamp(140.0, 585.0);
        self.vali_pos.y = self.vali_pos.y.clamp(210.0, 305.0);

        self.vali_attack_timer -= dt;
        if self.vali_attack_timer <= 0.0 {
            self.vali_attack_timer = 1.6;
            sounds.play(&sounds.boulder, 0.7);
            self.projectiles.push(L9Projectile {
                pos: self.vali_pos + vec2(0.0, -22.0),
                vel: to_sug * 205.0,
                is_player: false,
                pierced_count: 0,
                active: true,
            });
        }

        // Direct contact tremor if too close while Vali lunges
        if (self.sugriva_pos - self.vali_pos).length() < 24.0 && self.sugriva_invuln <= 0.0 {
            self.sugriva_hp -= 12;
            self.sugriva_invuln = 0.9;
            self.shake_timer = 0.2;
            sounds.play(&sounds.hurt, 0.75);
            if self.sugriva_hp <= 0 {
                return Some(FightResultL9::Defeat);
            }
        }

        // Tick enemy projectiles
        let mut sug_hit = false;
        for p in &mut self.projectiles {
            if !p.active {
                continue;
            }
            p.pos += p.vel * dt;
            if (p.pos - (self.sugriva_pos + vec2(0.0, -18.0))).length() < 16.0 && self.sugriva_invuln <= 0.0 {
                p.active = false;
                sug_hit = true;
            }
            if p.pos.x < -20.0 || p.pos.x > SCREEN_W + 20.0 || p.pos.y < -20.0 || p.pos.y > SCREEN_H + 20.0 {
                p.active = false;
            }
        }
        self.projectiles.retain(|p| p.active);

        if sug_hit {
            self.sugriva_hp -= 15;
            self.sugriva_invuln = 0.75;
            self.shake_timer = 0.18;
            sounds.play(&sounds.hurt, 0.75);
            if self.sugriva_hp <= 0 {
                return Some(FightResultL9::Defeat);
            }
        }

        None
    }

    fn trigger_garland_dialogue(&mut self, sounds: &SoundEffects) {
        self.phase = Level9Phase::GarlandDialogue;
        self.projectiles.clear();
        sounds.play(&sounds.bell, 0.8);
        self.dialogue = Some((
            "Rishyamuka Grove — The Gajapushpi Garland".to_string(),
            vec![
                "King Sugriva: 'Brother Rama! Vali's Indra-Mala drained half my strength and battered me back—why did your Kodanda stay silent?!'".to_string(),
                "Lord Rama: 'Forgive me, friend! You and Vali shone like twin moons in form and stance—I dared not risk striking you!'".to_string(),
                "Lord Rama: 'Wear this radiant Gajapushpi creeper-garland! Return to Kishkindha now—by this bloom, my Dharma-Bana shall know you!'".to_string(),
            ],
            0,
        ));
    }

    fn update_vali_second_duel(
        &mut self,
        dt: f32,
        sounds: &SoundEffects,
        stats: &mut PlayerStats,
        virtual_mouse: Vec2,
        interact: bool,
    ) -> Option<FightResultL9> {
        // Rama moves behind the Sal trees on the left & dodges Vali's flying boulders
        let mut mv = Vec2::ZERO;
        if is_key_down(KeyCode::A) || is_key_down(KeyCode::Left) {
            mv.x -= 1.0;
        }
        if is_key_down(KeyCode::D) || is_key_down(KeyCode::Right) {
            mv.x += 1.0;
        }
        if is_key_down(KeyCode::W) || is_key_down(KeyCode::Up) {
            mv.y -= 1.0;
        }
        if is_key_down(KeyCode::S) || is_key_down(KeyCode::Down) {
            mv.y += 1.0;
        }
        if mv.length_squared() > 0.0 {
            self.rama_pos += mv.normalize() * 155.0 * dt;
        }
        self.rama_pos.x = self.rama_pos.x.clamp(45.0, 175.0);
        self.rama_pos.y = self.rama_pos.y.clamp(205.0, 305.0);

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
        } else if (!down && self.is_drawing) || interact {
            let fire_pow = if interact { 0.85 } else { self.bow_tension };
            self.is_drawing = false;
            self.bow_tension = 0.0;
            if fire_pow > 0.12 {
                sounds.play(&sounds.shoot, 0.8);
                self.projectiles.push(L9Projectile {
                    pos: origin + self.aim_dir * 12.0,
                    vel: self.aim_dir * (430.0 + fire_pow * 180.0),
                    is_player: true,
                    pierced_count: 0,
                    active: true,
                });
            }
        }

        // Dynamic Foreground Dance: Garlanded Sugriva vs Vali weave & leap past each other!
        self.clash_phase += dt * 1.9;
        let center_x = 390.0;
        let weave = (self.clash_phase * 1.15).sin() * 95.0;
        let jump_s = (self.clash_phase * 3.1).sin().abs() * 24.0;
        let jump_v = (self.clash_phase * 2.7 + 1.4).sin().abs() * 32.0;

        self.sugriva_pos = vec2(center_x - weave * 0.7, 260.0 - jump_s);
        self.vali_pos = vec2(center_x + weave, 260.0 - jump_v);

        // Vali periodically hurls shockwave boulders toward Rama's grove
        self.vali_attack_timer -= dt;
        if self.vali_attack_timer <= 0.0 {
            self.vali_attack_timer = 1.65;
            let to_rama = (self.rama_pos + vec2(0.0, -18.0) - (self.vali_pos + vec2(0.0, -24.0))).normalize_or_zero();
            sounds.play(&sounds.boulder, 0.65);
            self.projectiles.push(L9Projectile {
                pos: self.vali_pos + vec2(0.0, -24.0),
                vel: to_rama * 215.0,
                is_player: false,
                pierced_count: 0,
                active: true,
            });
        }

        // Check arrow & boulder collisions
        let mut vali_struck = false;
        let mut sug_warn = false;
        let mut rama_struck = false;

        for p in &mut self.projectiles {
            if !p.active {
                continue;
            }
            p.pos += p.vel * dt;

            if p.is_player {
                let d_vali = (p.pos - (self.vali_pos + vec2(0.0, -20.0))).length();
                let d_sug = (p.pos - (self.sugriva_pos + vec2(0.0, -20.0))).length();
                if d_vali < 22.0 {
                    p.active = false;
                    vali_struck = true;
                } else if d_sug < 19.0 {
                    p.active = false;
                    sug_warn = true;
                }
            } else if (p.pos - (self.rama_pos + vec2(0.0, -20.0))).length() < 16.0 && self.rama_duel_invuln <= 0.0 {
                p.active = false;
                rama_struck = true;
            }

            if p.pos.x < -20.0 || p.pos.x > SCREEN_W + 20.0 || p.pos.y < -20.0 || p.pos.y > SCREEN_H + 20.0 {
                p.active = false;
            }
        }
        self.projectiles.retain(|p| p.active);

        if sug_warn {
            sounds.play(&sounds.wind, 0.5);
            self.floating_texts.push(L9FloatingText {
                text: "THAT'S GARLANDED SUGRIVA! AIM FOR VALI!".to_string(),
                pos: self.sugriva_pos + vec2(-95.0, -48.0),
                color: Color::new(1.0, 0.72, 0.92, 1.0),
                timer: 1.2,
                max_timer: 1.2,
            });
        }

        if rama_struck {
            stats.prana -= 14;
            self.rama_duel_invuln = 0.75;
            self.shake_timer = 0.2;
            sounds.play(&sounds.hurt, 0.75);
            if stats.prana <= 0 {
                stats.prana = 0;
                return Some(FightResultL9::Defeat);
            }
        }

        if vali_struck {
            self.vali_hp -= 60.0;
            self.vali_flash = 0.22;
            self.shake_timer = 0.3;
            sounds.play(&sounds.hit, 0.9);
            sounds.play(&sounds.thunder, 0.7);
            self.spawn_burst(self.vali_pos + vec2(0.0, -22.0), PAL_GOLD_0, 16);
            self.floating_texts.push(L9FloatingText {
                text: "DHARMA-BANA HIT! (-60)".to_string(),
                pos: self.vali_pos + vec2(-42.0, -52.0),
                color: PAL_GOLD_0,
                timer: 1.1,
                max_timer: 1.1,
            });

            if self.vali_hp <= 0.0 {
                self.vali_hp = 0.0;
                self.trigger_vali_dharma_dialogue(sounds);
            }
        }

        None
    }

    fn trigger_vali_dharma_dialogue(&mut self, sounds: &SoundEffects) {
        self.phase = Level9Phase::ValiDharmaDialogue;
        self.projectiles.clear();
        sounds.play(&sounds.victory, 0.7);
        self.dialogue = Some((
            "Fall of King Vali — Counsel of Dharma".to_string(),
            vec![
                "King Vali: 'Rama! Why did you strike me from behind the trees while I fought Sugriva—is that Kshatriya Dharma?!'".to_string(),
                "Lord Rama: 'By Indra's boon, no foe facing you kept full strength—and you transgressed Dharma by seizing Sugriva's queen Ruma!'".to_string(),
                "King Vali: 'Truth is spoken, Raghava! My pride blinded me. Sugriva—take this Golden Indra-Mala & cherish my young son Angada!'".to_string(),
            ],
            0,
        ));
    }

    fn spawn_burst(&mut self, pos: Vec2, color: Color, count: usize) {
        for i in 0..count {
            let a = (i as f32 / count as f32) * std::f32::consts::TAU;
            self.particles.push(L9Particle {
                pos,
                vel: vec2(a.cos(), a.sin()) * 90.0,
                color,
                size: 3.0,
                timer: 0.45,
                max_timer: 0.45,
            });
        }
    }

    pub fn draw(&self, font: &Font, time: f32, stats: &PlayerStats, virtual_mouse: Vec2) {
        let shake = if self.shake_timer > 0.0 {
            vec2((time * 72.0).sin() * 4.0, (time * 88.0).cos() * 3.0)
        } else {
            Vec2::ZERO
        };

        match self.phase {
            Level9Phase::IntroDialogue | Level9Phase::SevenSalTreesTrial => {
                self.draw_rishyamuka_arena(time, shake);

                // Dundubhi Buffalo Skeleton (unless flung off-screen)
                if !self.dundubhi_gone {
                    self.draw_dundubhi_skeleton(self.dundubhi_pos + shake);
                    if !self.dundubhi_kicked && (self.rama_pos - self.dundubhi_pos).length() < 60.0 {
                        draw_pixel_text(
                            font,
                            "[SPACE / CLICK] FLICK BONES",
                            self.dundubhi_pos.x - 62.0,
                            self.dundubhi_pos.y - 36.0,
                            8,
                            PAL_GOLD_0,
                        );
                    }
                }

                // 7 Collinear Giant Sal Trees
                for (i, tree) in self.sal_trees.iter().enumerate() {
                    let s_off = if tree.shake > 0.0 {
                        (time * 55.0 + i as f32).sin() * 3.0
                    } else {
                        0.0
                    };
                    self.draw_sal_tree(tree.pos + vec2(s_off, 0.0) + shake, tree.pierced, time, i);
                }

                // Rama's Dotted Alignment Sightline after Dundubhi is kicked
                if self.dundubhi_kicked && self.phase == Level9Phase::SevenSalTreesTrial {
                    let origin = self.rama_pos + vec2(0.0, -24.0);
                    let aligned = self.count_aligned_trees();
                    let line_col = if aligned == 7 {
                        Color::new(0.42, 0.96, 0.55, 0.85)
                    } else {
                        Color::new(1.0, 0.85, 0.38, 0.45)
                    };
                    let end_p = origin + self.aim_dir * 480.0;
                    draw_line(origin.x, origin.y, end_p.x, end_p.y, if aligned == 7 { 2.0 } else { 1.0 }, line_col);
                    if aligned == 7 {
                        draw_pixel_text(
                            font,
                            "7/7 ALIGNED! SHOOT!",
                            origin.x + 18.0,
                            origin.y - 26.0,
                            8,
                            Color::new(0.42, 0.96, 0.55, 1.0),
                        );
                    }
                }

                // Rama, Hanuman & Sugriva watching the trial
                draw_hero_exile(
                    self.rama_pos + shake,
                    true,
                    self.aim_dir,
                    self.bow_tension,
                    self.is_drawing,
                    time,
                );
                self.draw_vanara(vec2(64.0, 274.0) + shake, VanaraKind::Hanuman, false, time);
                self.draw_vanara(vec2(92.0, 290.0) + shake, VanaraKind::Sugriva, false, time);
            }
            _ => {
                // Act II & III: Kishkindha Gate Arena
                self.draw_kishkindha_arena(time, shake);

                let has_garland = !matches!(
                    self.phase,
                    Level9Phase::ValiFirstDuelDialogue | Level9Phase::ValiFirstDuel
                );

                // Rama behind cover Sal Trees (during ValiFirstDuel he watches; in SecondDuel you control him)
                draw_hero_exile(
                    self.rama_pos + shake,
                    true,
                    self.aim_dir,
                    self.bow_tension,
                    self.is_drawing,
                    time,
                );
                // Foreground Sal cover trunks partially framing Rama
                self.draw_cover_trunk(vec2(142.0, 298.0) + shake);

                // Hanuman nearby observing
                self.draw_vanara(vec2(118.0, 295.0) + shake, VanaraKind::Hanuman, false, time);

                // Sugriva & Vali in center-right arena
                if self.sugriva_invuln <= 0.0 || ((time * 24.0) as i32 % 2 == 0) {
                    self.draw_vanara(
                        self.sugriva_pos + shake,
                        VanaraKind::Sugriva,
                        has_garland,
                        time,
                    );
                }

                if self.vali_flash > 0.0 {
                    draw_rectangle(
                        self.vali_pos.x - 16.0,
                        self.vali_pos.y - 42.0,
                        32.0,
                        42.0,
                        WHITE,
                    );
                } else if self.phase != Level9Phase::SugrivaCoronationDialogue
                    && self.phase != Level9Phase::Complete
                {
                    self.draw_vanara(self.vali_pos + shake, VanaraKind::Vali, true, time);
                }
            }
        }

        // Draw Projectiles
        for p in &self.projectiles {
            if p.is_player {
                draw_astra_projectile(p.pos, p.vel, AstraProjectileType::Kodanda, time);
            } else {
                draw_circle(p.pos.x, p.pos.y, 7.0, PAL_SAND_4);
                draw_circle(p.pos.x - 2.0, p.pos.y - 2.0, 3.5, PAL_SAND_1);
                draw_circle_lines(p.pos.x, p.pos.y, 8.5, 1.0, PAL_FIRE_3);
            }
        }

        // Particles & Floating Texts
        for pt in &self.particles {
            let alpha = (pt.timer / pt.max_timer).clamp(0.0, 1.0);
            let mut c = pt.color;
            c.a = alpha;
            draw_rectangle(pt.pos.x, pt.pos.y, pt.size, pt.size, c);
        }
        for ft in &self.floating_texts {
            let alpha = (ft.timer / ft.max_timer).clamp(0.0, 1.0);
            let mut c = ft.color;
            c.a = alpha;
            draw_pixel_text(font, &ft.text, ft.pos.x, ft.pos.y, 8, c);
        }

        // Top HUD
        self.draw_level9_hud(font, stats);

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

    fn draw_rishyamuka_arena(&self, time: f32, shake: Vec2) {
        // Golden Morning Sky over Rishyamuka Peaks & Shimmering Lake Pampa
        draw_rectangle(0.0, 0.0, SCREEN_W, 85.0, Color::new(0.24, 0.46, 0.72, 1.0));
        draw_rectangle(0.0, 85.0, SCREEN_W, 75.0, Color::new(0.42, 0.64, 0.80, 1.0));
        draw_rectangle(0.0, 160.0, SCREEN_W, 75.0, Color::new(0.68, 0.78, 0.74, 1.0));

        // Rishyamuka Mountain Cliffs
        for (px, ph, pw, col) in [
            (85.0, 100.0, 180.0, Color::new(0.32, 0.28, 0.36, 1.0)),
            (280.0, 128.0, 210.0, Color::new(0.38, 0.30, 0.34, 1.0)),
            (490.0, 110.0, 200.0, Color::new(0.30, 0.26, 0.32, 1.0)),
        ] {
            draw_triangle(
                vec2(px - pw * 0.5, 220.0),
                vec2(px + pw * 0.5, 220.0),
                vec2(px, 220.0 - ph),
                col,
            );
        }

        // Lake Pampa Shimmering Strip on left-back
        draw_rectangle(0.0, 208.0, SCREEN_W, 24.0, Color::new(0.16, 0.46, 0.68, 1.0));
        for i in 0..7 {
            let wx = ((i as f32 * 96.0) + time * 18.0).rem_euclid(SCREEN_W);
            draw_line(wx, 218.0 + (i % 2) as f32 * 4.0, wx + 28.0, 218.0 + (i % 2) as f32 * 4.0, 1.2, WHITE);
        }
        // Pink Lotus blossoms on Lake Pampa
        for lx in [48.0, 118.0, 172.0] {
            draw_circle(lx, 222.0, 3.0, Color::new(1.0, 0.65, 0.80, 1.0));
        }

        // Foreground Lush Plateau
        draw_rectangle(0.0, 232.0, SCREEN_W, SCREEN_H - 232.0, PAL_GRASS_2);
        draw_rectangle(0.0, 278.0, SCREEN_W, SCREEN_H - 278.0, PAL_GRASS_3);
        draw_ancient_forest_tree(26.0 + shake.x * 0.3, 244.0, 145.0, 15.0, 0, time);
    }

    fn draw_kishkindha_arena(&self, time: f32, shake: Vec2) {
        // Dramatic Sunset Crimson-Gold Sky over Kishkindha Valley
        draw_rectangle(0.0, 0.0, SCREEN_W, 90.0, Color::new(0.20, 0.14, 0.32, 1.0));
        draw_rectangle(0.0, 90.0, SCREEN_W, 90.0, Color::new(0.42, 0.22, 0.30, 1.0));
        draw_rectangle(0.0, 180.0, SCREEN_W, 60.0, Color::new(0.68, 0.38, 0.24, 1.0));

        // Kishkindha Mountain Citadel Cliffs & Carved Golden Gate (Right Back)
        draw_triangle(vec2(380.0, 232.0), vec2(640.0, 232.0), vec2(545.0, 76.0), Color::new(0.32, 0.22, 0.20, 1.0));
        draw_triangle(vec2(260.0, 232.0), vec2(490.0, 232.0), vec2(375.0, 118.0), Color::new(0.26, 0.18, 0.18, 1.0));

        // Carved Golden Fortress Gateway Archway
        let gx = 540.0 + shake.x * 0.3;
        draw_rectangle(gx - 38.0, 168.0, 76.0, 64.0, PAL_SAND_4);
        draw_rectangle_lines(gx - 38.0, 168.0, 76.0, 64.0, 2.0, PAL_GOLD_1);
        draw_rectangle(gx - 18.0, 188.0, 36.0, 44.0, Color::new(0.08, 0.05, 0.06, 1.0));
        draw_triangle(vec2(gx - 42.0, 168.0), vec2(gx + 42.0, 168.0), vec2(gx, 140.0), PAL_GOLD_2);

        // Battleground Terrace
        draw_rectangle(0.0, 232.0, SCREEN_W, 38.0, Color::new(0.46, 0.34, 0.24, 1.0));
        draw_rectangle(0.0, 270.0, SCREEN_W, SCREEN_H - 270.0, PAL_GRASS_3);

        // Cover trees on left where Rama stands watch
        draw_ancient_forest_tree(36.0 + shake.x * 0.2, 248.0, 172.0, 16.0, 1, time);
        draw_ancient_forest_tree(94.0 + shake.x * 0.2, 246.0, 168.0, 15.0, 2, time);
    }

    fn draw_cover_trunk(&self, pos: Vec2) {
        draw_rectangle(pos.x - 7.0, pos.y - 125.0, 14.0, 125.0, Color::new(0.20, 0.13, 0.10, 0.9));
        draw_rectangle(pos.x - 4.0, pos.y - 125.0, 5.0, 125.0, Color::new(0.32, 0.22, 0.16, 0.9));
        draw_circle(pos.x, pos.y - 128.0, 20.0, Color::new(0.14, 0.34, 0.16, 0.92));
    }

    fn draw_dundubhi_skeleton(&self, pos: Vec2) {
        let x = pos.x;
        let y = pos.y;
        draw_ellipse(x, y + 2.0, 22.0, 6.0, 0.0, Color::new(0.0, 0.0, 0.0, 0.35));
        // Massive bleached ribs
        for dx in [-14.0f32, -6.0, 2.0] {
            draw_line(x + dx, y - 18.0, x + dx - 3.0, y, 2.5, PAL_SAND_0);
        }
        // Colossal horned buffalo skull
        draw_rectangle(x + 4.0, y - 18.0, 18.0, 14.0, Color::new(0.92, 0.88, 0.80, 1.0));
        draw_circle(x + 16.0, y - 11.0, 2.5, BLACK);
        draw_line(x + 12.0, y - 18.0, x + 26.0, y - 30.0, 3.0, PAL_SAND_1);
        draw_line(x + 6.0, y - 18.0, x - 4.0, y - 28.0, 3.0, PAL_SAND_1);
    }

    fn draw_sal_tree(&self, pos: Vec2, pierced: bool, time: f32, idx: usize) {
        let x = pos.x;
        let y = pos.y;
        let sway = (time * 1.4 + idx as f32 * 0.7).sin() * 1.2;

        // Thick ancient Sal trunk
        draw_rectangle(x - 6.0, y - 76.0, 12.0, 76.0, Color::new(0.24, 0.15, 0.10, 1.0));
        draw_rectangle(x - 3.0, y - 76.0, 5.0, 76.0, Color::new(0.36, 0.24, 0.16, 1.0));

        // Piercing Target Ring at chest/arrow height (`y - 22.0`)
        let target_y = y - 22.0;
        if pierced {
            draw_circle(x, target_y, 5.0, PAL_GOLD_0);
            draw_circle(x, target_y, 2.2, BLACK);
        } else {
            draw_circle_lines(x, target_y, 5.5, 1.0, Color::new(1.0, 0.86, 0.35, 0.65));
        }

        // Lush Crown Canopy
        draw_circle(x - 10.0 + sway, y - 82.0, 14.0, PAL_GRASS_3);
        draw_circle(x + 10.0 + sway, y - 80.0, 13.0, PAL_GRASS_3);
        draw_circle(x + sway, y - 92.0, 16.0, PAL_GRASS_2);
        draw_circle(x - 3.0 + sway, y - 98.0, 10.0, PAL_GRASS_1);
    }

    fn draw_vanara(&self, pos: Vec2, kind: VanaraKind, special_garland: bool, time: f32) {
        let x = pos.x;
        let y = pos.y;
        let bob = (time * 4.5).sin() * 1.2;
        draw_ellipse(x, y + 1.0, 12.0, 4.0, 0.0, Color::new(0.0, 0.0, 0.0, 0.4));

        // Prehensile Vanara Tail curling gracefully upward
        let tail_c = match kind {
            VanaraKind::Hanuman => PAL_GOLD_1,
            VanaraKind::Sugriva => PAL_SAFFRON_2,
            VanaraKind::Vali => PAL_RED_1,
        };
        let t_sway = (time * 5.0).sin() * 4.0;
        draw_line(x - 5.0, y - 10.0, x - 16.0, y - 22.0 + t_sway, 2.5, tail_c);
        draw_line(x - 16.0, y - 22.0 + t_sway, x - 13.0, y - 34.0 + t_sway, 2.0, tail_c);

        // Legs & Dhoti
        let dhoti_c = match kind {
            VanaraKind::Hanuman => PAL_SAFFRON_1,
            VanaraKind::Sugriva => Color::new(0.88, 0.56, 0.16, 1.0),
            VanaraKind::Vali => PAL_RED_2,
        };
        draw_rectangle(x - 6.0, y - 13.0, 12.0, 9.0, dhoti_c);
        draw_rectangle(x - 5.0, y - 4.0, 4.0, 4.0, PAL_SAND_3);
        draw_rectangle(x + 1.0, y - 4.0, 4.0, 4.0, PAL_SAND_3);

        // Torso
        let (fur_mid, fur_hi) = match kind {
            VanaraKind::Hanuman => (PAL_SAND_0, PAL_GOLD_0),
            VanaraKind::Sugriva => (PAL_SAND_3, PAL_SAND_1),
            VanaraKind::Vali => (Color::new(0.75, 0.34, 0.24, 1.0), PAL_GOLD_1),
        };
        let w_scale = if kind == VanaraKind::Vali { 8.5 } else { 7.0 };
        let h_off = if kind == VanaraKind::Vali { 2.0 } else { 0.0 };
        draw_rectangle(x - w_scale, y - 26.0 - h_off + bob, w_scale * 2.0, 13.0 + h_off, fur_mid);
        draw_rectangle(x - w_scale + 2.0, y - 25.0 - h_off + bob, w_scale * 2.0 - 4.0, 9.0, fur_hi);

        // Special Garlands!
        if kind == VanaraKind::Vali && special_garland {
            // Blazing Golden Indra-Mala
            draw_circle_lines(x, y - 21.0 + bob, 9.0, 2.0, PAL_GOLD_0);
            draw_circle(x, y - 12.0 + bob, 2.5, PAL_FIRE_1);
        } else if kind == VanaraKind::Sugriva && special_garland {
            // Gajapushpi White & Magenta Flower Garland
            draw_circle_lines(x, y - 20.0 + bob, 8.5, 2.2, WHITE);
            draw_circle(x - 5.0, y - 16.0 + bob, 2.0, Color::new(1.0, 0.45, 0.85, 1.0));
            draw_circle(x + 5.0, y - 16.0 + bob, 2.0, Color::new(1.0, 0.45, 0.85, 1.0));
            draw_circle(x, y - 13.0 + bob, 2.2, PAL_GOLD_0);
        }

        // Head & Royal Vanara Crown / Golden Aura
        draw_circle(x, y - 32.0 - h_off + bob, 6.0, fur_mid);
        draw_circle(x + 2.0, y - 31.0 - h_off + bob, 3.0, PAL_SAND_0);
        // Crown
        draw_triangle(
            vec2(x - 4.5, y - 37.0 - h_off + bob),
            vec2(x + 4.5, y - 37.0 - h_off + bob),
            vec2(x, y - 44.0 - h_off + bob),
            PAL_GOLD_1,
        );
        draw_circle(x + 2.5, y - 33.0 - h_off + bob, 1.0, BLACK);

        // Hanuman's Golden Gada Mace
        if kind == VanaraKind::Hanuman {
            draw_line(x + 7.0, y - 18.0, x + 13.0, y - 32.0, 2.0, PAL_GOLD_2);
            draw_circle(x + 14.0, y - 34.0, 4.5, PAL_GOLD_0);
        }
    }

    fn draw_level9_hud(&self, font: &Font, stats: &PlayerStats) {
        let (hp_label, hp_val, hp_max) = if self.phase == Level9Phase::ValiFirstDuel {
            ("SUGRIVA HP", self.sugriva_hp, self.sugriva_max_hp)
        } else {
            ("PRANA", stats.prana, stats.max_prana)
        };

        draw_pixel_text(font, hp_label, 12.0, 12.0, 8, PAL_GOLD_1);
        draw_rectangle(12.0, 15.0, 114.0, 8.0, Color::new(0.12, 0.08, 0.10, 0.9));
        let pw = (hp_val as f32 / hp_max as f32).clamp(0.0, 1.0) * 112.0;
        draw_rectangle(13.0, 16.0, pw, 6.0, Color::new(0.85, 0.25, 0.25, 1.0));
        draw_rectangle_lines(12.0, 15.0, 114.0, 8.0, 1.0, PAL_GOLD_2);

        let bx = 180.0;
        let bw = 280.0;
        match self.phase {
            Level9Phase::SevenSalTreesTrial => {
                let title = if !self.dundubhi_kicked {
                    "TRIAL 1: FLICK DUNDUBHI BONES"
                } else {
                    "TRIAL 2: PIERCE ALL 7 SAL TREES IN 1 SHOT"
                };
                let prog = if !self.dundubhi_kicked {
                    0.25
                } else if self.trees_pierced_once {
                    1.0
                } else {
                    0.55
                };
                draw_pixel_text(font, title, bx + 6.0, 12.0, 8, PAL_GOLD_0);
                draw_rectangle(bx, 15.0, bw, 10.0, Color::new(0.12, 0.10, 0.14, 0.92));
                draw_rectangle(bx + 1.0, 16.0, (bw - 2.0) * prog, 8.0, PAL_GRASS_1);
                draw_rectangle_lines(bx, 15.0, bw, 10.0, 1.0, PAL_GOLD_1);
            }
            Level9Phase::ValiFirstDuel => {
                draw_pixel_text(
                    font,
                    &format!("DUEL AT KISHKINDHA: {} / 5 CLASHES", self.sugriva_hits_landed),
                    bx + 12.0,
                    12.0,
                    8,
                    PAL_GOLD_0,
                );
                draw_rectangle(bx, 15.0, bw, 10.0, Color::new(0.15, 0.08, 0.10, 0.92));
                draw_rectangle(
                    bx + 1.0,
                    16.0,
                    (bw - 2.0) * (self.sugriva_hits_landed as f32 / 5.0),
                    8.0,
                    PAL_SAFFRON_1,
                );
                draw_rectangle_lines(bx, 15.0, bw, 10.0, 1.2, PAL_GOLD_1);
            }
            Level9Phase::ValiSecondDuel => {
                draw_pixel_text(font, "KING VALI (INDRA-MALA)", bx + 44.0, 12.0, 8, PAL_GOLD_0);
                draw_rectangle(bx, 15.0, bw, 10.0, Color::new(0.15, 0.08, 0.10, 0.92));
                draw_rectangle(
                    bx + 1.0,
                    16.0,
                    (bw - 2.0) * (self.vali_hp / self.vali_max_hp).clamp(0.0, 1.0),
                    8.0,
                    PAL_FIRE_3,
                );
                draw_rectangle_lines(bx, 15.0, bw, 10.0, 1.2, PAL_GOLD_1);
            }
            _ => {}
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum VanaraKind {
    Hanuman,
    Sugriva,
    Vali,
}

// ============================================================================
// LEVEL 9 VICTORY SCREEN
// ============================================================================
pub fn draw_victory_l9(font: &Font, time: f32) {
    draw_rectangle(0.0, 0.0, SCREEN_W, SCREEN_H, Color::new(0.06, 0.05, 0.10, 0.96));

    let sway = (time * 5.0).sin() * 2.5;
    draw_pixel_text(
        font,
        "KISHKINDHA: ALLIANCE OF DHARMA!",
        72.0,
        40.0 + sway,
        16,
        PAL_GOLD_0,
    );

    let sub = "HANUMAN'S DEVOTION, 7 SAL TREES & KING SUGRIVA CROWNED";
    draw_pixel_text(font, sub, 92.0, 60.0, 8, PAL_TURMERIC_1);

    let rw_x = 34.0;
    let rw_y = 76.0;
    let rw_w = 572.0;
    let rw_h = 212.0;
    draw_rectangle(rw_x, rw_y, rw_w, rw_h, Color::new(0.10, 0.09, 0.16, 0.95));
    draw_rectangle_lines(rw_x, rw_y, rw_w, rw_h, 1.5, PAL_GOLD_1);

    draw_pixel_text(
        font,
        "CHAPTER 9 MILESTONES COMPLETED (KISHKINDHA KANDA):",
        rw_x + 16.0,
        rw_y + 22.0,
        8,
        PAL_GOLD_0,
    );
    draw_pixel_text(
        font,
        "[+] Rishyamuka Pact  : United with Sri Hanuman & King Sugriva!",
        rw_x + 16.0,
        rw_y + 44.0,
        8,
        WHITE,
    );
    draw_pixel_text(
        font,
        "[+] Sapta-Sala       : Tossed Dundubhi & pierced 7 Sal trees!",
        rw_x + 16.0,
        rw_y + 64.0,
        8,
        PAL_SAFFRON_0,
    );
    draw_pixel_text(
        font,
        "[+] Garland Duel     : Slayed Vali & crowned Sugriva in Kishkindha!",
        rw_x + 16.0,
        rw_y + 84.0,
        8,
        PAL_FIRE_1,
    );
    draw_pixel_text(
        font,
        "[+] Rama's Mudrika   : Entrusted signet ring to Hanuman for Sita!",
        rw_x + 16.0,
        rw_y + 104.0,
        8,
        Color::new(0.45, 0.92, 0.55, 1.0),
    );

    draw_line(
        rw_x + 16.0,
        rw_y + 122.0,
        rw_x + rw_w - 16.0,
        rw_y + 122.0,
        1.0,
        PAL_GOLD_3,
    );
    draw_pixel_text(
        font,
        "★ NEXT BOOK: SUNDARA KANDA (HANUMAN'S OCEAN LEAP TO LANKA) ★",
        rw_x + 16.0,
        rw_y + 142.0,
        8,
        PAL_FIRE_1,
    );
    draw_pixel_text(
        font,
        "From Mount Mahendragiri, Sri Hanuman swells to mountain size",
        rw_x + 16.0,
        rw_y + 162.0,
        8,
        Color::new(0.92, 0.85, 0.85, 1.0),
    );
    draw_pixel_text(
        font,
        "and vaults across 100 yojanas of ocean toward golden Lanka!",
        rw_x + 16.0,
        rw_y + 180.0,
        8,
        Color::new(0.92, 0.85, 0.85, 1.0),
    );

    let blink = (time * 4.0).sin() > 0.0;
    if blink {
        draw_pixel_text(
            font,
            "PRESS [SPACE] OR CLICK TO RETURN TO CHAPTER SELECT ▶",
            92.0,
            312.0,
            8,
            PAL_GOLD_0,
        );
    }
    draw_pixel_text(
        font,
        "Press [R] to Replay Chapter 9 (Kishkindha Kanda)",
        124.0,
        336.0,
        8,
        Color::new(0.75, 0.75, 0.80, 1.0),
    );
}
