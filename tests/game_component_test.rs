// Test game state transitions
#[test]
fn test_game_state_transitions() {
    use space_invaders::game::state::{GameScreen, GameState};

    let mut game_state = GameState::default();
    game_state.screen = GameScreen::StartScreen;

    // Transition from StartScreen to Playing
    game_state.screen = GameScreen::Playing;
    assert_eq!(game_state.screen, GameScreen::Playing);

    // Transition from Playing to GameOver
    game_state.game_over = true;
    game_state.screen = GameScreen::GameOver;
    assert_eq!(game_state.screen, GameScreen::GameOver);
}

// Test game reset functionality
#[test]
fn test_game_reset() {
    use space_invaders::game::entities::alien::AlienFormation;
    use space_invaders::game::entities::shield::{Shield, ShieldType};
    use space_invaders::game::state::{GameScreen, GameState};

    // Create a game over state
    let mut game_state = GameState {
        high_score: 1000,
        score: 500,
        lives: 0,
        game_over: true,
        screen: GameScreen::GameOver,
        ..Default::default()
    };

    // Reset the game (simulating the reset logic in the Game component)
    game_state = GameState {
        high_score: game_state.high_score,
        lives: 3,
        level: 1,
        screen: GameScreen::Playing,
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

    // Verify reset state
    assert_eq!(game_state.screen, GameScreen::Playing);
    assert_eq!(game_state.lives, 3);
    assert_eq!(game_state.score, 0);
    assert_eq!(game_state.high_score, 1000); // High score should be preserved
    assert!(!game_state.game_over);
}

// Test that level completion increases difficulty
#[test]
fn test_level_completion_increases_difficulty() {
    use space_invaders::game::logic::check_level_completion;
    use space_invaders::game::state::GameState;

    let mut game_state = GameState::default();
    game_state.level = 1;

    for alien in &mut game_state.alien_formation.aliens {
        alien.is_alive = false;
    }

    check_level_completion(&mut game_state);

    assert_eq!(game_state.level, 2);

    assert!(game_state.alien_formation.aliens.iter().all(|a| a.is_alive));
}

// Test that player shooting creates projectiles at the correct position
#[test]
fn test_player_shooting_projectile_position() {
    use space_invaders::game::entities::player::Player;
    use space_invaders::game::logic::handle_player_shooting;
    use space_invaders::game::state::GameState;

    let mut game_state = GameState::default();
    game_state.player = Player::default();
    game_state.key_states.shift = true;
    game_state.player_shoot_cooldown = 0.0;

    // Handle shooting
    handle_player_shooting(&mut game_state, 0.1);

    assert_eq!(game_state.player_projectiles.len(), 1);

    let projectile = &game_state.player_projectiles[0];
    let (spawn_x, spawn_y) = game_state.player.get_projectile_spawn_position();
    assert_eq!(projectile.position.x, spawn_x);
    assert_eq!(projectile.position.y, spawn_y);
}
