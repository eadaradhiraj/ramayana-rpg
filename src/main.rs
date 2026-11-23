use macroquad::prelude::*;

mod audio;
mod level1;
mod render;
mod types;

use audio::SoundEffects;
use level1::{FightResult, Level1Fight};
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
                draw_title_screen(&font, global_time);
                draw_reticle(virtual_mouse, false);

                if is_key_pressed(KeyCode::Space)
                    || is_key_pressed(KeyCode::Enter)
                    || is_mouse_button_pressed(MouseButton::Left)
                {
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
                }
            }

            GameState::Dialogue {
                speaker,
                lines,
                idx,
                target,
            } => {
                draw_battlefield(global_time, 90.0);
                draw_rama(vec2(120.0, 180.0), vec2(1.0, 0.0), 0.0, 0.0, global_time);

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
                                stats = PlayerStats::default();
                                level1_fight = Level1Fight::new();
                                game_state = GameState::Playing(*level_id);
                            }
                            DialogueTarget::Victory(level_id) => {
                                game_state = GameState::LevelComplete(*level_id);
                            }
                        }
                    }
                }
            }

            GameState::Playing(level_id) => {
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
                    _ => {
                        draw_pixel_text(&font, "COMING SOON IN LEVEL 2", 180.0, 180.0, 14, WHITE);
                    }
                }
            }

            GameState::LevelComplete(_level_id) => {
                draw_victory_screen(&font, global_time);
                draw_reticle(virtual_mouse, false);

                if is_key_pressed(KeyCode::Space)
                    || is_key_pressed(KeyCode::Enter)
                    || is_mouse_button_pressed(MouseButton::Left)
                {
                    sounds.play(&sounds.shoot, 0.6);
                    game_state = GameState::TitleScreen;
                }
                if is_key_pressed(KeyCode::R) {
                    stats = PlayerStats::default();
                    level1_fight = Level1Fight::new();
                    game_state = GameState::Playing(LevelId::L1Tataka);
                }
            }

            GameState::GameOver(level_id) => {
                draw_game_over(&font, global_time);
                draw_reticle(virtual_mouse, false);

                if is_key_pressed(KeyCode::R) || is_mouse_button_pressed(MouseButton::Left) {
                    stats = PlayerStats::default();
                    if *level_id == LevelId::L1Tataka {
                        level1_fight = Level1Fight::new();
                        game_state = GameState::Playing(LevelId::L1Tataka);
                    }
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
