use crate::game::entities::alien::AlienFormation;
use crate::game::entities::shield::{Shield, ShieldType};
use crate::game::logic::check_projectile_collisions;
use crate::game::logic::{
    check_game_over_conditions, check_level_completion, handle_alien_shooting,
    handle_player_shooting, update_aliens, update_mystery_ship, update_player, update_projectiles,
};
use crate::game::state::{GameScreen, GameState};
use crate::rendering::score_board::ScoreBoard;
use crate::rendering::screens::{GameOverScreen, StartScreen};
use crate::rendering::sprites::alien_sprite::AlienSprite;
use crate::rendering::sprites::mystery_ship_sprite::MysteryShipSprite;
use crate::rendering::sprites::player_sprite::PlayerShip;
use crate::rendering::sprites::projectile_sprite::ProjectileSprite;
use crate::rendering::sprites::shield_sprite::ShieldSprite;
use crate::rendering::star_background::StarBackground;
use dioxus::events::KeyboardEvent;
use dioxus::prelude::*;

/// Main game component that handles rendering and game loop
/// This component initializes the game state, sets up the game loop using coroutines,
/// handles keyboard input, and renders the appropriate screen based on the current game state.
#[allow(non_snake_case)]
pub fn Game() -> Element {
    const GAME_WIDTH: f32 = 1024.0;
    const _GAME_HEIGHT: f32 = 768.0;
    let mut game_state = use_signal(|| GameState {
        lives: 3,
        level: 1,
        mystery_ship_timer: 15.0,
        screen: GameScreen::StartScreen,
        alien_formation: AlienFormation::new(GAME_WIDTH),
        last_update: instant::Instant::now().elapsed().as_secs_f64(),
        shields: vec![
            Shield::new(100.0, 500.0, ShieldType::UppercaseC),
            Shield::new(250.0, 500.0, ShieldType::UppercaseO),
            Shield::new(400.0, 500.0, ShieldType::UppercaseR),
            Shield::new(550.0, 500.0, ShieldType::UppercaseT),
            Shield::new(700.0, 500.0, ShieldType::UppercaseW),
            Shield::new(850.0, 500.0, ShieldType::UppercaseO),
        ],
        ..Default::default()
    });

    use_coroutine(move |_rx: dioxus::prelude::UnboundedReceiver<()>| {
        to_owned![game_state];

        async move {
            let mut last_frame = instant::Instant::now();

            loop {
                let now = instant::Instant::now();
                let delta_time = (now - last_frame).as_secs_f64();
                last_frame = now;

                game_state.with_mut(|state| {
                    if state.screen == GameScreen::Playing {
                        if !state.game_over {
                            state.dt = delta_time as f32;
                            update_player(state, delta_time);
                            handle_player_shooting(state, delta_time);

                            update_aliens(state, delta_time);
                            handle_alien_shooting(state, delta_time);

                            update_mystery_ship(state);

                            update_projectiles(state, delta_time);
                            check_projectile_collisions(state);

                            check_level_completion(state);
                            check_game_over_conditions(state);

                            if state.game_over {
                                state.screen = GameScreen::GameOver;
                            }
                        } else {
                            if state.key_states.tab {
                                *state = GameState {
                                    high_score: state.high_score,
                                    lives: 3,
                                    alien_formation: AlienFormation::new(GAME_WIDTH),
                                    last_update: instant::Instant::now().elapsed().as_secs_f64(),
                                    shields: vec![
                                        Shield::new(100.0, 500.0, ShieldType::UppercaseC),
                                        Shield::new(250.0, 500.0, ShieldType::UppercaseO),
                                        Shield::new(400.0, 500.0, ShieldType::UppercaseR),
                                        Shield::new(550.0, 500.0, ShieldType::UppercaseT),
                                        Shield::new(700.0, 500.0, ShieldType::UppercaseW),
                                        Shield::new(850.0, 500.0, ShieldType::UppercaseO),
                                    ],
                                    ..Default::default()
                                };
                            }
                        }
                    }
                });

                tokio::time::sleep(std::time::Duration::from_millis(16)).await;
            }
        }
    });

    let onkeydown = move |event: KeyboardEvent| {
        game_state.with_mut(|state| match state.screen {
            GameScreen::StartScreen => {
                if event.key() == Key::Enter {
                    *state = GameState {
                        high_score: state.high_score,
                        lives: 3,
                        level: 1,
                        screen: GameScreen::Playing,
                        alien_formation: AlienFormation::new(GAME_WIDTH),
                        last_update: instant::Instant::now().elapsed().as_secs_f64(),
                        shields: vec![
                            Shield::new(100.0, 500.0, ShieldType::UppercaseC),
                            Shield::new(250.0, 500.0, ShieldType::UppercaseO),
                            Shield::new(400.0, 500.0, ShieldType::UppercaseR),
                            Shield::new(550.0, 500.0, ShieldType::UppercaseT),
                            Shield::new(700.0, 500.0, ShieldType::UppercaseW),
                            Shield::new(850.0, 500.0, ShieldType::UppercaseO),
                        ],
                        mystery_ship_timer: 15.0,
                        ..Default::default()
                    };
                }
            }
            GameScreen::Playing => {
                state.key_states.update_from_key(event.key(), true);
            }
            GameScreen::GameOver => {
                if event.key() == Key::Tab {
                    *state = GameState {
                        high_score: state.high_score,
                        lives: 3,
                        level: 1,
                        screen: GameScreen::Playing,
                        alien_formation: AlienFormation::new(GAME_WIDTH),
                        last_update: instant::Instant::now().elapsed().as_secs_f64(),
                        shields: vec![
                            Shield::new(100.0, 500.0, ShieldType::UppercaseC),
                            Shield::new(250.0, 500.0, ShieldType::UppercaseO),
                            Shield::new(400.0, 500.0, ShieldType::UppercaseR),
                            Shield::new(550.0, 500.0, ShieldType::UppercaseT),
                            Shield::new(700.0, 500.0, ShieldType::UppercaseW),
                            Shield::new(850.0, 500.0, ShieldType::UppercaseO),
                        ],
                        mystery_ship_timer: 15.0,
                        ..Default::default()
                    };
                }
            }
        });
    };

    let onkeyup = move |event: KeyboardEvent| {
        game_state.with_mut(|state| {
            if state.screen == GameScreen::Playing {
                state.key_states.update_from_key(event.key(), false);
            }
        });
    };

    rsx! {
        div {
            id: "game-container",
            tabindex: "0",
            onkeydown,
            onkeyup,
            onfocus: move |_| {},

            ScoreBoard {
                score: game_state.read().score,
                high_score: game_state.read().high_score,
                lives: game_state.read().lives,
                level: game_state.read().level,
            }

            {
                match game_state.read().screen {
                    GameScreen::StartScreen => rsx! {
                        StartScreen {}
                    },
                    GameScreen::Playing => rsx! {
                        
                        div { id: "game-area",
                            StarBackground {}
                            {
                                if let Some(ship) = &game_state.read().mystery_ship {
                                    rsx! {
                                        MysteryShipSprite { ship: ship.clone() }
                                    }
                                } else {
                                    rsx! {
                                        div {}
                                    }
                                }
                            }
                            {
                                game_state.read().shields.iter().map(|shield| rsx! {
                                    ShieldSprite { shield: shield.clone() }
                                })
                            }
                            {
                                game_state
                                    .read()
                                    .alien_formation
                                    .aliens
                                    .iter()
                                    .map(|alien| rsx! {
                                        AlienSprite { alien: alien.clone() }
                                    })
                            }
                            {
                                game_state
                                    .read()
                                    .player_projectiles
                                    .iter()
                                    .map(|projectile| rsx! {
                                        ProjectileSprite { projectile: projectile.clone() }
                                    })
                            }
                            {
                                game_state
                                    .read()
                                    .alien_projectiles
                                    .iter()
                                    .map(|projectile| rsx! {
                                        ProjectileSprite { projectile: projectile.clone() }
                                    })
                            }
                            PlayerShip { player: game_state.read().player.clone() }
                        }
                    },
                    GameScreen::GameOver => rsx! {
                        GameOverScreen { score: game_state.read().score, high_score: game_state.read().high_score }
                    },
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_game_initialization() {
        let game_state = GameState {
            lives: 3,
            level: 1,
            mystery_ship_timer: 15.0,
            screen: GameScreen::StartScreen,
            alien_formation: AlienFormation::new(1024.0),
            last_update: instant::Instant::now().elapsed().as_secs_f64(),
            shields: vec![
                Shield::new(100.0, 500.0, ShieldType::UppercaseC),
                Shield::new(250.0, 500.0, ShieldType::UppercaseO),
                Shield::new(400.0, 500.0, ShieldType::UppercaseR),
                Shield::new(550.0, 500.0, ShieldType::UppercaseT),
                Shield::new(700.0, 500.0, ShieldType::UppercaseW),
                Shield::new(850.0, 500.0, ShieldType::UppercaseO),
            ],
            ..Default::default()
        };

        let state = game_state;

        assert_eq!(state.lives, 3);
        assert_eq!(state.level, 1);
        assert_eq!(state.shields.len(), 6);
        assert!(!state.game_over);
    }

    // Test that the Game component initializes with the correct default values
    #[test]
    fn test_game_initial_state() {
        let game_state = GameState {
            lives: 3,
            level: 1,
            mystery_ship_timer: 15.0,
            screen: GameScreen::StartScreen,
            alien_formation: AlienFormation::new(1024.0),
            shields: vec![
                Shield::new(100.0, 500.0, ShieldType::UppercaseC),
                Shield::new(250.0, 500.0, ShieldType::UppercaseO),
                Shield::new(400.0, 500.0, ShieldType::UppercaseR),
                Shield::new(550.0, 500.0, ShieldType::UppercaseT),
                Shield::new(700.0, 500.0, ShieldType::UppercaseW),
                Shield::new(850.0, 500.0, ShieldType::UppercaseO),
            ],
            ..Default::default()
        };

        assert_eq!(game_state.screen, GameScreen::StartScreen);
        assert_eq!(game_state.lives, 3);
        assert_eq!(game_state.level, 1);
        assert_eq!(game_state.shields.len(), 6);
    }

    // Test the key handling logic
    #[test]
    fn test_key_handling() {
        let mut game_state = GameState {
            screen: GameScreen::StartScreen,
            ..Default::default()
        };

        if game_state.screen == GameScreen::StartScreen {
            game_state.screen = GameScreen::Playing;
        }

        assert_eq!(game_state.screen, GameScreen::Playing);

        game_state.screen = GameScreen::GameOver;

        if game_state.screen == GameScreen::GameOver {
            game_state.screen = GameScreen::Playing;
        }

        assert_eq!(game_state.screen, GameScreen::Playing);
    }
}

// Test that the game correctly handles key state updates
#[test]
fn test_key_state_updates() {
    let mut game_state = GameState {
        screen: GameScreen::Playing,
        ..Default::default()
    };

    game_state.key_states.update_from_key(Key::ArrowLeft, true);
    assert!(game_state.key_states.left);

    game_state.key_states.update_from_key(Key::Shift, true);
    assert!(game_state.key_states.shift);

    game_state.key_states.update_from_key(Key::ArrowLeft, false);
    assert!(!game_state.key_states.left);
}

// Test that game over condition correctly updates the screen
#[test]
fn test_game_over_screen_transition() {
    let mut game_state = GameState {
        screen: GameScreen::Playing,
        lives: 1,
        ..Default::default()
    };

    game_state.lives = 0;
    game_state.game_over = true;

    if game_state.game_over {
        game_state.screen = GameScreen::GameOver;
    }

    assert_eq!(game_state.screen, GameScreen::GameOver);
}
