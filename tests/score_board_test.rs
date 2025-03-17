// Test that ScoreBoard integrates with GameState
#[test]
fn test_score_board_with_game_state() {
    use space_invaders::game::state::GameState;
    use space_invaders::rendering::score_board::ScoreBoardProps;

    let mut game_state = GameState::default();
    game_state.score = 250;
    game_state.high_score = 1000;
    game_state.lives = 2;
    game_state.level = 3;

    let props = ScoreBoardProps {
        score: game_state.score,
        high_score: game_state.high_score,
        lives: game_state.lives,
        level: game_state.level,
    };

    assert_eq!(props.score, 250);
    assert_eq!(props.high_score, 1000);
    assert_eq!(props.lives, 2);
    assert_eq!(props.level, 3);
}

// tests/score_board_test.rs

#[test]
fn test_score_board_props() {
    use space_invaders::rendering::score_board::ScoreBoardProps;

    let props = ScoreBoardProps {
        score: 100,
        high_score: 500,
        lives: 3,
        level: 1,
    };

    assert_eq!(props.score, 100);
    assert_eq!(props.high_score, 500);
    assert_eq!(props.lives, 3);
    assert_eq!(props.level, 1);
}
