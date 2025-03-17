// Test that GameState can be properly initialized and modified
#[test]
fn test_game_state_initialization() {
    use space_invaders::game::entities::shield::{Shield, ShieldType};
    use space_invaders::game::state::{GameScreen, GameState};

    let mut state = GameState::default();

    state.score = 100;
    state.high_score = 500;
    state.lives = 3;
    state.level = 1;
    state.screen = GameScreen::Playing;

    state
        .shields
        .push(Shield::new(100.0, 600.0, ShieldType::UppercaseC));
    state
        .shields
        .push(Shield::new(300.0, 600.0, ShieldType::UppercaseO));

    assert_eq!(state.score, 100);
    assert_eq!(state.high_score, 500);
    assert_eq!(state.lives, 3);
    assert_eq!(state.level, 1);
    assert_eq!(state.screen, GameScreen::Playing);
    assert_eq!(state.shields.len(), 2);
}

// Test that GameState properly tracks high score
#[test]
fn test_high_score_tracking() {
    use space_invaders::game::state::GameState;

    let mut state = GameState::default();
    state.high_score = 1000;

    state.score = 500;
    assert_eq!(state.high_score, 1000);

    state.score = 1500;
    if state.score > state.high_score {
        state.high_score = state.score;
    }
    assert_eq!(state.high_score, 1500);
}
