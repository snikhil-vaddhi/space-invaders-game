// Test that Position works correctly with game entities
#[test]
fn test_position_with_game_entities() {
    use space_invaders::game::entities::player::Player;
    use space_invaders::utils::position::Position;

    let pos = Position::new(100.0, 200.0);

    let mut player = Player::default();
    player.position = pos.clone();

    assert_eq!(player.position.x, 100.0);
    assert_eq!(player.position.y, 200.0);

    player.position.x += 50.0;

    assert_eq!(player.position.x, 150.0);
    assert_eq!(player.position.y, 200.0);
}

// Test that Position can be used for collision detection
#[test]
fn test_position_for_collision_detection() {
    use space_invaders::utils::position::Position;

    let pos1 = Position::new(100.0, 100.0);
    let pos2 = Position::new(110.0, 110.0);

    let dx = pos1.x - pos2.x;
    let dy = pos1.y - pos2.y;
    let distance = (dx * dx + dy * dy).sqrt();

    assert_eq!(distance, 14.142136);

    let collision_threshold = 20.0;
    let collision = distance < collision_threshold;

    assert!(collision);
}
