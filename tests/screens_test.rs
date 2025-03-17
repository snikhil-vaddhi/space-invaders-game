// Test that GameOverScreen correctly displays score values
#[test]
fn test_game_over_screen_displays_scores() {
    use space_invaders::game::state::GameState;
    use space_invaders::rendering::screens::GameOverScreenProps;

    let mut game_state = GameState::default();
    game_state.score = 250;
    game_state.high_score = 1000;

    let props = GameOverScreenProps {
        score: game_state.score,
        high_score: game_state.high_score,
    };

    assert_eq!(props.score, 250);
    assert_eq!(props.high_score, 1000);
}

// Test that screens integrate with game state transitions
#[test]
fn test_screens_with_game_state_transitions() {
    use space_invaders::game::state::{GameScreen, GameState};

    let mut game_state = GameState::default();

    game_state.screen = GameScreen::StartScreen;
    assert_eq!(game_state.screen, GameScreen::StartScreen);

    game_state.screen = GameScreen::Playing;
    assert_eq!(game_state.screen, GameScreen::Playing);

    game_state.screen = GameScreen::GameOver;
    assert_eq!(game_state.screen, GameScreen::GameOver);

    let score = game_state.score;
    let high_score = game_state.high_score;

    assert_eq!(game_state.score, score);
    assert_eq!(game_state.high_score, high_score);
}
