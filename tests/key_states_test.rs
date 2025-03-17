// Test that KeyStates properly integrates with game state
#[test]
fn test_key_states_with_game_state() {
    use dioxus::events::Key;
    use space_invaders::game::state::GameState;

    let mut game_state = GameState::default();

    assert!(!game_state.key_states.left);
    assert!(!game_state.key_states.right);

    game_state.key_states.update_from_key(Key::ArrowLeft, true);
    assert!(game_state.key_states.left);

    game_state.key_states.update_from_key(Key::ArrowRight, true);
    assert!(game_state.key_states.right);

    game_state.key_states.update_from_key(Key::ArrowLeft, false);
    assert!(!game_state.key_states.left);
    assert!(game_state.key_states.right);
}

// Test that KeyStates can be cloned and compared
#[test]
fn test_key_states_clone_and_equality() {
    use dioxus::events::Key;
    use space_invaders::input::key_states::KeyStates;

    let mut key_states1 = KeyStates::default();
    key_states1.update_from_key(Key::ArrowLeft, true);
    key_states1.update_from_key(Key::Shift, true);

    let key_states2 = key_states1.clone();

    assert_eq!(key_states1, key_states2);

    key_states1.update_from_key(Key::ArrowLeft, false);

    assert_ne!(key_states1, key_states2);
}
