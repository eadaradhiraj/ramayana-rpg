use macroquad::prelude::*;

mod audio;
mod level1;
mod level2;
mod level3;
mod render;
mod types;

use audio::SoundEffects;
use level1::{FightResult, Level1Fight};
use level2::{FightResultL2, Level2Fight};
use level3::{FightResultL3, Level3Fight};
use render::*;
use types::*;

fn window_conf() -> Conf {
    Conf {
        window_title: "RAMAYANA: THE RETRO EPIC (2D Rust + WASM)".to_string(),
        window_width: 960,  // 16:9 Widescreen
        window_height: 540, // 16:9 Widescreen
        window_resizable: true,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    // Hide hardware mouse cursor so custom retro targeting reticle is used
    show_mouse(false);

    // Setup 640x360 (16:9) retro render target with crisp nearest-neighbor pixel scaling
    let render_target = render_target(SCREEN_W as u32, SCREEN_H as u32);
    render_target.texture.set_filter(FilterMode::Nearest);

    let mut render_cam = Camera2D::from_display_rect(Rect::new(0.0, 0.0, SCREEN_W, SCREEN_H));
    render_cam.render_target = Some(render_target.clone());

    // Load authentic retro pixel font (Press Start 2P) directly from embedded bytes
    let mut font = load_ttf_font_from_bytes(include_bytes!("../assets/fonts/PressStart2P-Regular.ttf"))
        .expect("Failed to load PressStart2P pixel font");
    font.set_filter(FilterMode::Nearest);

    // Initialize retro synthesized sound effects
    let sounds = SoundEffects::init().await;

    let mut stats = PlayerStats::default();
    let mut game_state = GameState::TitleScreen;
    let mut level1_fight = Level1Fight::new();
    let mut level2_fight = Level2Fight::new();
    let mut level3_fight = Level3Fight::new();
    let mut global_time: f32 = 0.0;

    loop {
        let dt = get_frame_time().min(0.05); // Cap delta time to prevent physics blowup
        global_time += dt;

        // Calculate aspect-ratio letterboxed scaling (16:9)
        let scale = (screen_width() / SCREEN_W).min(screen_height() / SCREEN_H).max(0.1);
        let ox = (screen_width() - (SCREEN_W * scale)) * 0.5;
        let oy = (screen_height() - (SCREEN_H * scale)) * 0.5;

        // Map mouse position to 640x360 virtual coordinates
        let (mx, my) = mouse_position();
        let virtual_mouse = vec2((mx - ox) / scale, (my - oy) / scale);

        // ------------------------------------------------------------------------
        // 1. Render game world into the 640x360 render target
        // ------------------------------------------------------------------------
        set_camera(&render_cam);
        clear_background(BLACK);

        // Main State Machine
        match &mut game_state {
            GameState::TitleScreen => {
                draw_title_screen(&font, global_time, virtual_mouse);
                draw_reticle(virtual_mouse, false);

                // Open Interactive Level Select
                if is_key_pressed(KeyCode::L) || is_key_pressed(KeyCode::Tab) {
                    sounds.play(&sounds.shoot, 0.6);
                    game_state = GameState::LevelSelect { selected: 0 };
                } else if is_key_pressed(KeyCode::Key1) {
                    sounds.play(&sounds.shoot, 0.7);
                    game_state = GameState::Dialogue {
                        speaker: "Sage Vishwamitra".to_string(),
                        lines: vec![
                            "Prince Rama, we enter the desolate wasteland of Tataka.".to_string(),
                            "Once the thriving realms of Malada and Karusha, she laid all to ruin.".to_string(),
                            "Her dark sorcery veils her from sight. Use Shabdavedhi to strike her ripples!".to_string(),
                            "Hurry! If dusk falls (90s), her frenzy doubles speed & malice!".to_string(),
                        ],
                        idx: 0,
                        target: DialogueTarget::StartFight(LevelId::L1Tataka),
                    };
                } else if is_key_pressed(KeyCode::Key2) {
                    sounds.play(&sounds.shoot, 0.7);
                    game_state = GameState::Dialogue {
                        speaker: "Sage Vishwamitra".to_string(),
                        lines: vec![
                            "Welcome to Siddhashrama, the Hermitage of Accomplishment!".to_string(),
                            "For six days and six nights, we conduct the holy Yajna sacrifice.".to_string(),
                            "Rakshasas Maricha and Subahu will attempt to defile the sacrificial fire with blood!".to_string(),
                            "Protect the central Yajna Altar at all costs. Do not let its Purity fall to zero!".to_string(),
                            "Use [1] Kodanda for free arrows, [2] Vayavyastra to scatter swarms, [3] Manavastra to fling Maricha, and [4] Agneyastra to incinerate Subahu!".to_string(),
                        ],
                        idx: 0,
                        target: DialogueTarget::StartFight(LevelId::L2Siddhashrama),
                    };
                } else if is_key_pressed(KeyCode::Key3) {
                    sounds.play(&sounds.shoot, 0.7);
                    game_state = GameState::Dialogue {
                        speaker: "King Janaka".to_string(),
                        lines: vec![
                            "Prince Rama, welcome to the ancient court of Mithila!".to_string(),
                            "I have proclaimed the holy Swayamvara for Princess Sita.".to_string(),
                            "Before you lies the cosmic bow of Lord Shiva—the great Pinaka, resting on an eight-wheeled iron cart.".to_string(),
                            "Hundreds of mighty kings could not even move it. If you can lift and string it, Sita is yours!".to_string(),
                            "Approach the carriage with [WASD], lift Pinaka with [SPACE], and draw the celestial string!".to_string(),
                        ],
                        idx: 0,
                        target: DialogueTarget::StartFight(LevelId::L3Mithila),
                    };
                } else if is_mouse_button_pressed(MouseButton::Left) {
                    sounds.play(&sounds.shoot, 0.7);
                    match get_title_screen_action_at(virtual_mouse) {
                        Some(0) => {
                            game_state = GameState::Dialogue {
                                speaker: "Sage Vishwamitra".to_string(),
                                lines: vec![
                                    "Prince Rama, we enter the desolate wasteland of Tataka.".to_string(),
                                    "Once the thriving realms of Malada and Karusha, she laid all to ruin.".to_string(),
                                    "Her dark sorcery veils her from sight. Use Shabdavedhi to strike her ripples!".to_string(),
                                    "Hurry! If dusk falls (90s), her frenzy doubles speed & malice!".to_string(),
                                ],
                                idx: 0,
                                target: DialogueTarget::StartFight(LevelId::L1Tataka),
                            };
                        }
                        Some(1) => {
                            game_state = GameState::Dialogue {
                                speaker: "Sage Vishwamitra".to_string(),
                                lines: vec![
                                    "Welcome to Siddhashrama, the Hermitage of Accomplishment!".to_string(),
                                    "For six days and six nights, we conduct the holy Yajna sacrifice.".to_string(),
                                    "Rakshasas Maricha and Subahu will attempt to defile the sacrificial fire with blood!".to_string(),
                                    "Protect the central Yajna Altar at all costs. Do not let its Purity fall to zero!".to_string(),
                                    "Use [1] Kodanda for free arrows, [2] Vayavyastra to scatter swarms, [3] Manavastra to fling Maricha, and [4] Agneyastra to incinerate Subahu!".to_string(),
                                ],
                                idx: 0,
                                target: DialogueTarget::StartFight(LevelId::L2Siddhashrama),
                            };
                        }
                        Some(2) => {
                            game_state = GameState::Dialogue {
                                speaker: "King Janaka".to_string(),
                                lines: vec![
                                    "Prince Rama, welcome to the ancient court of Mithila!".to_string(),
                                    "I have proclaimed the holy Swayamvara for Princess Sita.".to_string(),
                                    "Before you lies the cosmic bow of Lord Shiva—the great Pinaka, resting on an eight-wheeled iron cart.".to_string(),
                                    "Hundreds of mighty kings could not even move it. If you can lift and string it, Sita is yours!".to_string(),
                                    "Approach the carriage with [WASD], lift Pinaka with [SPACE], and draw the celestial string!".to_string(),
                                ],
                                idx: 0,
                                target: DialogueTarget::StartFight(LevelId::L3Mithila),
                            };
                        }
                        _ => {
                            game_state = GameState::LevelSelect { selected: 0 };
                        }
                    }
                } else if is_key_pressed(KeyCode::Space) || is_key_pressed(KeyCode::Enter) {
                    sounds.play(&sounds.shoot, 0.7);
                    game_state = GameState::LevelSelect { selected: 0 };
                }
            }

            GameState::LevelSelect { selected } => {
                draw_level_select(&font, *selected, global_time, virtual_mouse);
                draw_reticle(virtual_mouse, false);

                // Mouse hover updates card selection
                if let Some(card_idx) = get_level_select_card_at(virtual_mouse) {
                    *selected = card_idx;
                }

                // Keyboard card cycling
                if is_key_pressed(KeyCode::Left) || is_key_pressed(KeyCode::A) {
                    *selected = (*selected + 2) % 3;
                    sounds.play(&sounds.shoot, 0.3);
                } else if is_key_pressed(KeyCode::Right) || is_key_pressed(KeyCode::D) {
                    *selected = (*selected + 1) % 3;
                    sounds.play(&sounds.shoot, 0.3);
                }

                // Direct hotkey jumps
                if is_key_pressed(KeyCode::Key1) {
                    *selected = 0;
                } else if is_key_pressed(KeyCode::Key2) {
                    *selected = 1;
                } else if is_key_pressed(KeyCode::Key3) {
                    *selected = 2;
                }

                // Launch selected level
                let should_launch = is_key_pressed(KeyCode::Space)
                    || is_key_pressed(KeyCode::Enter)
                    || is_mouse_button_pressed(MouseButton::Left);

                if should_launch {
                    sounds.play(&sounds.shoot, 0.7);
                    match *selected {
                        0 => {
                            game_state = GameState::Dialogue {
                                speaker: "Sage Vishwamitra".to_string(),
                                lines: vec![
                                    "Prince Rama, we enter the desolate wasteland of Tataka.".to_string(),
                                    "Once the thriving realms of Malada and Karusha, she laid all to ruin.".to_string(),
                                    "Her dark sorcery veils her from sight. Use Shabdavedhi to strike her ripples!".to_string(),
                                    "Hurry! If dusk falls (90s), her frenzy doubles speed & malice!".to_string(),
                                ],
                                idx: 0,
                                target: DialogueTarget::StartFight(LevelId::L1Tataka),
                            };
                        }
                        1 => {
                            game_state = GameState::Dialogue {
                                speaker: "Sage Vishwamitra".to_string(),
                                lines: vec![
                                    "Welcome to Siddhashrama, the Hermitage of Accomplishment!".to_string(),
                                    "For six days and six nights, we conduct the holy Yajna sacrifice.".to_string(),
                                    "Rakshasas Maricha and Subahu will attempt to defile the sacrificial fire with blood!".to_string(),
                                    "Protect the central Yajna Altar at all costs. Do not let its Purity fall to zero!".to_string(),
                                    "Use [1] Kodanda for free arrows, [2] Vayavyastra to scatter swarms, [3] Manavastra to fling Maricha, and [4] Agneyastra to incinerate Subahu!".to_string(),
                                ],
                                idx: 0,
                                target: DialogueTarget::StartFight(LevelId::L2Siddhashrama),
                            };
                        }
                        _ => {
                            game_state = GameState::Dialogue {
                                speaker: "King Janaka".to_string(),
                                lines: vec![
                                    "Prince Rama, welcome to the ancient court of Mithila!".to_string(),
                                    "I have proclaimed the holy Swayamvara for Princess Sita.".to_string(),
                                    "Before you lies the cosmic bow of Lord Shiva—the great Pinaka, resting on an eight-wheeled iron cart.".to_string(),
                                    "Hundreds of mighty kings could not even move it. If you can lift and string it, Sita is yours!".to_string(),
                                    "Approach the carriage with [WASD], lift Pinaka with [SPACE], and draw the celestial string!".to_string(),
                                ],
                                idx: 0,
                                target: DialogueTarget::StartFight(LevelId::L3Mithila),
                            };
                        }
                    }
                }

                if is_key_pressed(KeyCode::Escape) {
                    game_state = GameState::TitleScreen;
                }
            }

            GameState::Dialogue {
                speaker,
                lines,
                idx,
                target,
            } => {
                // Background matches level context
                match target {
                    DialogueTarget::StartFight(LevelId::L3Mithila)
                    | DialogueTarget::Victory(LevelId::L3Mithila) => {
                        level3_fight.draw(&font, virtual_mouse, global_time);
                    }
                    DialogueTarget::StartFight(LevelId::L2Siddhashrama)
                    | DialogueTarget::Victory(LevelId::L2Siddhashrama) => {
                        draw_siddhashrama_arena(global_time, 100.0);
                        draw_rama(vec2(240.0, 180.0), vec2(1.0, 0.0), 0.0, 0.0, global_time);
                    }
                    _ => {
                        draw_battlefield(global_time, 90.0);
                        draw_rama(vec2(120.0, 180.0), vec2(1.0, 0.0), 0.0, 0.0, global_time);
                    }
                }

                if *idx < lines.len() {
                    draw_dialogue_box(&font, speaker, &lines[*idx], global_time);
                }
                draw_reticle(virtual_mouse, false);

                if is_key_pressed(KeyCode::Space)
                    || is_key_pressed(KeyCode::Enter)
                    || is_mouse_button_pressed(MouseButton::Left)
                {
                    sounds.play(&sounds.shoot, 0.5);
                    *idx += 1;
                    if *idx >= lines.len() {
                        match target {
                            DialogueTarget::StartFight(level_id) => {
                                match level_id {
                                    LevelId::L1Tataka => {
                                        stats = PlayerStats::default();
                                        level1_fight = Level1Fight::new();
                                        game_state = GameState::Playing(LevelId::L1Tataka);
                                    }
                                    LevelId::L2Siddhashrama => {
                                        stats = PlayerStats {
                                            prana: 100,
                                            max_prana: 100,
                                            tejas: 100,
                                            max_tejas: 100,
                                            unlocked_astras: vec![
                                                Astra::KodandaBow,
                                                Astra::Vayavyastra,
                                                Astra::Manavastra,
                                                Astra::Agneyastra,
                                            ],
                                            selected_astra: Astra::KodandaBow,
                                        };
                                        level2_fight = Level2Fight::new();
                                        game_state = GameState::Playing(LevelId::L2Siddhashrama);
                                    }
                                    LevelId::L3Mithila => {
                                        level3_fight = Level3Fight::new();
                                        game_state = GameState::Playing(LevelId::L3Mithila);
                                    }
                                    _ => {}
                                }
                            }
                            DialogueTarget::Victory(level_id) => {
                                game_state = GameState::LevelComplete(*level_id);
                            }
                            DialogueTarget::StartCutscene(cutscene) => {
                                game_state = GameState::Cutscene {
                                    cutscene_type: *cutscene,
                                    slide_idx: 0,
                                };
                            }
                        }
                    }
                }
            }

            GameState::Playing(level_id) => {
                // Pause Menu Toggle
                if is_key_pressed(KeyCode::Escape) {
                    sounds.play(&sounds.shoot, 0.4);
                    game_state = GameState::Paused {
                        level_id: *level_id,
                        selected: 0,
                    };
                } else {
                    match level_id {
                        LevelId::L1Tataka => {
                            let fight_result = level1_fight.update(dt, virtual_mouse, &mut stats, &sounds);
                            level1_fight.draw(&stats, virtual_mouse, &font);

                            match fight_result {
                                Some(FightResult::Victory) => {
                                    game_state = GameState::Dialogue {
                                        speaker: "Sage Vishwamitra".to_string(),
                                        lines: vec![
                                            "Well struck, Prince of Ayodhya! Tataka is vanquished!".to_string(),
                                            "Your arrow pierced the unseen ripple with Shabdavedhi mastery.".to_string(),
                                            "I bestow upon you the divine Astras: Vayavya, Manava, and Agneya.".to_string(),
                                            "Now onward to Siddhashrama to guard the sacred Yajna!".to_string(),
                                        ],
                                        idx: 0,
                                        target: DialogueTarget::Victory(LevelId::L1Tataka),
                                    };
                                }
                                Some(FightResult::Defeat) => {
                                    game_state = GameState::GameOver(LevelId::L1Tataka);
                                }
                                None => {}
                            }
                        }
                        LevelId::L2Siddhashrama => {
                            let fight_result = level2_fight.update(dt, virtual_mouse, &mut stats, &sounds);
                            level2_fight.draw(&stats, virtual_mouse, &font);

                            match fight_result {
                                Some(FightResultL2::Victory) => {
                                    game_state = GameState::Dialogue {
                                        speaker: "Sage Vishwamitra".to_string(),
                                        lines: vec![
                                            "Sadhu! Sadhu! The sacred Yajna of Siddhashrama is preserved!".to_string(),
                                            "With Manavastra, you flung Maricha 100 yojanas into the distant sea!".to_string(),
                                            "With Agneyastra, Subahu and his gore cauldrons are burned to ashes!".to_string(),
                                            "The Devas rain flowers! Now accompany me to King Janaka's court in Mithila!".to_string(),
                                        ],
                                        idx: 0,
                                        target: DialogueTarget::Victory(LevelId::L2Siddhashrama),
                                    };
                                }
                                Some(FightResultL2::Defeat) => {
                                    game_state = GameState::GameOver(LevelId::L2Siddhashrama);
                                }
                                None => {}
                            }
                        }
                        LevelId::L3Mithila => {
                            let fight_result = level3_fight.update(dt, virtual_mouse, &mut stats, &sounds);
                            level3_fight.draw(&font, virtual_mouse, global_time);

                            match fight_result {
                                Some(FightResultL3::Victory) => {
                                    game_state = GameState::Dialogue {
                                        speaker: "King Janaka".to_string(),
                                        lines: vec![
                                            "Sadhu! Sadhu! The cosmic Pinaka of Lord Shiva has broken in two!".to_string(),
                                            "None in heaven or on earth could even budge it. Prince Rama, your divine strength has fulfilled my vow!".to_string(),
                                            "Princess Sita is yours! Let both royal families assemble in the sacred pavilion to recite their lineages!".to_string(),
                                        ],
                                        idx: 0,
                                        target: DialogueTarget::StartCutscene(CutsceneType::RamaBirth),
                                    };
                                }
                                None => {}
                            }
                        }
                        _ => {
                            draw_pixel_text(&font, "COMING SOON IN LEVEL 4", 180.0, 180.0, 14, WHITE);
                        }
                    }
                }
            }

            GameState::Paused { level_id, selected } => {
                // Keep the battle arena frozen behind the pause menu
                match level_id {
                    LevelId::L1Tataka => {
                        level1_fight.draw(&stats, virtual_mouse, &font);
                    }
                    LevelId::L2Siddhashrama => {
                        level2_fight.draw(&stats, virtual_mouse, &font);
                    }
                    LevelId::L3Mithila => {
                        level3_fight.draw(&font, virtual_mouse, global_time);
                    }
                    _ => {}
                }

                draw_pause_menu(&font, *selected, global_time, virtual_mouse);
                draw_reticle(virtual_mouse, false);

                // Mouse hover updates selection
                if let Some(action) = get_pause_menu_action_at(virtual_mouse) {
                    *selected = action;
                }

                // Keyboard navigation
                if is_key_pressed(KeyCode::Up) || is_key_pressed(KeyCode::W) {
                    *selected = (*selected + 3) % 4;
                    sounds.play(&sounds.shoot, 0.3);
                } else if is_key_pressed(KeyCode::Down) || is_key_pressed(KeyCode::S) {
                    *selected = (*selected + 1) % 4;
                    sounds.play(&sounds.shoot, 0.3);
                }

                // Hotkey shortcuts
                let mut chosen_action: Option<usize> = None;
                if is_key_pressed(KeyCode::Escape) {
                    chosen_action = Some(0); // Resume
                } else if is_key_pressed(KeyCode::R) {
                    chosen_action = Some(1); // Restart
                } else if is_key_pressed(KeyCode::L) {
                    chosen_action = Some(2); // Select Level
                } else if is_key_pressed(KeyCode::T) {
                    chosen_action = Some(3); // Title
                } else if is_key_pressed(KeyCode::Space)
                    || is_key_pressed(KeyCode::Enter)
                    || is_mouse_button_pressed(MouseButton::Left)
                {
                    chosen_action = Some(*selected);
                }

                if let Some(act) = chosen_action {
                    sounds.play(&sounds.shoot, 0.5);
                    match act {
                        0 => {
                            // Resume
                            game_state = GameState::Playing(*level_id);
                        }
                        1 => {
                            // Restart
                            match level_id {
                                LevelId::L1Tataka => {
                                    stats = PlayerStats::default();
                                    level1_fight = Level1Fight::new();
                                    game_state = GameState::Playing(LevelId::L1Tataka);
                                }
                                LevelId::L2Siddhashrama => {
                                    stats = PlayerStats {
                                        prana: 100,
                                        max_prana: 100,
                                        tejas: 100,
                                        max_tejas: 100,
                                        unlocked_astras: vec![
                                            Astra::KodandaBow,
                                            Astra::Vayavyastra,
                                            Astra::Manavastra,
                                            Astra::Agneyastra,
                                        ],
                                        selected_astra: Astra::KodandaBow,
                                    };
                                    level2_fight = Level2Fight::new();
                                    game_state = GameState::Playing(LevelId::L2Siddhashrama);
                                }
                                LevelId::L3Mithila => {
                                    level3_fight = Level3Fight::new();
                                    game_state = GameState::Playing(LevelId::L3Mithila);
                                }
                                _ => {}
                            }
                        }
                        2 => {
                            // Level Select
                            let card_sel = match level_id {
                                LevelId::L1Tataka => 0,
                                LevelId::L2Siddhashrama => 1,
                                _ => 2,
                            };
                            game_state = GameState::LevelSelect { selected: card_sel };
                        }
                        _ => {
                            // Title
                            game_state = GameState::TitleScreen;
                        }
                    }
                }
            }

            GameState::Cutscene { cutscene_type, slide_idx } => {
                draw_cutscene_screen(&font, *cutscene_type, *slide_idx, global_time, virtual_mouse);

                if is_key_pressed(KeyCode::Space)
                    || is_key_pressed(KeyCode::Enter)
                    || is_mouse_button_pressed(MouseButton::Left)
                {
                    sounds.play(&sounds.bell, 0.6);
                    let total = get_cutscene_total_slides(*cutscene_type);
                    if *slide_idx + 1 < total {
                        *slide_idx += 1;
                    } else {
                        match cutscene_type {
                            CutsceneType::RamaBirth => {
                                game_state = GameState::Cutscene {
                                    cutscene_type: CutsceneType::SitaBirth,
                                    slide_idx: 0,
                                };
                            }
                            CutsceneType::SitaBirth => {
                                game_state = GameState::Cutscene {
                                    cutscene_type: CutsceneType::SitaKalyanam,
                                    slide_idx: 0,
                                };
                            }
                            CutsceneType::SitaKalyanam => {
                                game_state = GameState::Cutscene {
                                    cutscene_type: CutsceneType::ParashuramaEncounter,
                                    slide_idx: 0,
                                };
                            }
                            CutsceneType::ParashuramaEncounter => {
                                sounds.play(&sounds.victory, 0.8);
                                game_state = GameState::LevelComplete(LevelId::L3Mithila);
                            }
                        }
                    }
                }

                if is_key_pressed(KeyCode::Escape) {
                    game_state = GameState::TitleScreen;
                }
            }

            GameState::LevelComplete(level_id) => {
                match level_id {
                    LevelId::L1Tataka => {
                        draw_victory_screen(&font, global_time);
                        draw_reticle(virtual_mouse, false);

                        if is_key_pressed(KeyCode::Space)
                            || is_key_pressed(KeyCode::Enter)
                            || is_mouse_button_pressed(MouseButton::Left)
                        {
                            sounds.play(&sounds.shoot, 0.6);
                            game_state = GameState::Dialogue {
                                speaker: "Sage Vishwamitra".to_string(),
                                lines: vec![
                                    "Welcome to Siddhashrama, the Hermitage of Accomplishment.".to_string(),
                                    "For six days and six nights, we conduct the holy Yajna sacrifice.".to_string(),
                                    "Maricha and Subahu seek to desecrate the holy altar with impure gore!".to_string(),
                                    "Protect the sacred central Yajna fire! Do not let its Purity fall to zero!".to_string(),
                                    "Use [1] Kodanda for free arrows, [2] Vayavyastra for wind gales, [3] Manavastra for homing strikes, and [4] Agneyastra for sacred fire!".to_string(),
                                ],
                                idx: 0,
                                target: DialogueTarget::StartFight(LevelId::L2Siddhashrama),
                            };
                        }
                        if is_key_pressed(KeyCode::R) {
                            stats = PlayerStats::default();
                            level1_fight = Level1Fight::new();
                            game_state = GameState::Playing(LevelId::L1Tataka);
                        }
                    }
                    LevelId::L2Siddhashrama => {
                        draw_victory_l2(&font, global_time);
                        draw_reticle(virtual_mouse, false);

                        if is_key_pressed(KeyCode::Space)
                            || is_key_pressed(KeyCode::Enter)
                            || is_mouse_button_pressed(MouseButton::Left)
                        {
                            sounds.play(&sounds.shoot, 0.6);
                            game_state = GameState::Dialogue {
                                speaker: "King Janaka".to_string(),
                                lines: vec![
                                    "Prince Rama, welcome to the ancient court of Mithila!".to_string(),
                                    "I have proclaimed the holy Swayamvara for Princess Sita.".to_string(),
                                    "Before you lies the cosmic bow of Lord Shiva—the great Pinaka, resting on an eight-wheeled iron cart.".to_string(),
                                    "Hundreds of mighty kings could not even move it. If you can lift and string it, Sita is yours!".to_string(),
                                    "Approach the carriage with [WASD], lift Pinaka with [SPACE], and draw the celestial string!".to_string(),
                                ],
                                idx: 0,
                                target: DialogueTarget::StartFight(LevelId::L3Mithila),
                            };
                        }
                        if is_key_pressed(KeyCode::R) {
                            stats = PlayerStats {
                                prana: 100,
                                max_prana: 100,
                                tejas: 100,
                                max_tejas: 100,
                                unlocked_astras: vec![
                                    Astra::KodandaBow,
                                    Astra::Vayavyastra,
                                    Astra::Manavastra,
                                    Astra::Agneyastra,
                                ],
                                selected_astra: Astra::KodandaBow,
                            };
                            level2_fight = Level2Fight::new();
                            game_state = GameState::Playing(LevelId::L2Siddhashrama);
                        }
                    }
                    LevelId::L3Mithila => {
                        draw_victory_l3(&font, global_time);
                        draw_reticle(virtual_mouse, false);

                        if is_key_pressed(KeyCode::Space)
                            || is_key_pressed(KeyCode::Enter)
                            || is_mouse_button_pressed(MouseButton::Left)
                        {
                            sounds.play(&sounds.shoot, 0.6);
                            game_state = GameState::LevelSelect { selected: 0 };
                        }
                        if is_key_pressed(KeyCode::R) {
                            level3_fight = Level3Fight::new();
                            game_state = GameState::Playing(LevelId::L3Mithila);
                        }
                    }
                    _ => {}
                }
            }

            GameState::GameOver(level_id) => {
                match level_id {
                    LevelId::L1Tataka => {
                        draw_game_over(&font, global_time);
                        draw_reticle(virtual_mouse, false);

                        if is_key_pressed(KeyCode::R) || is_mouse_button_pressed(MouseButton::Left) {
                            stats = PlayerStats::default();
                            level1_fight = Level1Fight::new();
                            game_state = GameState::Playing(LevelId::L1Tataka);
                        }
                    }
                    LevelId::L2Siddhashrama => {
                        draw_game_over_l2(&font, global_time);
                        draw_reticle(virtual_mouse, false);

                        if is_key_pressed(KeyCode::R) || is_mouse_button_pressed(MouseButton::Left) {
                            stats = PlayerStats {
                                prana: 100,
                                max_prana: 100,
                                tejas: 100,
                                max_tejas: 100,
                                unlocked_astras: vec![
                                    Astra::KodandaBow,
                                    Astra::Vayavyastra,
                                    Astra::Manavastra,
                                    Astra::Agneyastra,
                                ],
                                selected_astra: Astra::KodandaBow,
                            };
                            level2_fight = Level2Fight::new();
                            game_state = GameState::Playing(LevelId::L2Siddhashrama);
                        }
                    }
                    _ => {}
                }

                if is_key_pressed(KeyCode::Escape) {
                    game_state = GameState::TitleScreen;
                }
            }
        }

        // ------------------------------------------------------------------------
        // 2. Draw scaled & letterboxed texture to display
        // ------------------------------------------------------------------------
        set_default_camera();
        clear_background(BLACK);

        draw_texture_ex(
            &render_target.texture,
            ox,
            oy,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(SCREEN_W * scale, SCREEN_H * scale)),
                flip_y: true, // Flip y so OpenGL texture is right-side up
                ..Default::default()
            },
        );

        next_frame().await;
    }
}
