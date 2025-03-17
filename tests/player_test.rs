// Test that player movement works correctly with game boundaries
#[test]
fn test_player_movement_with_boundaries() {
    use space_invaders::game::entities::player::Player;

    let mut player = Player::default();
    let left_boundary = 0.0;
    let right_boundary = 1024.0;

    // Move player to left boundary
    player.position.x = left_boundary + 30.0;
    player.move_left(0.5, left_boundary);

    // Player should not move beyond left boundary
    assert!(player.position.x >= left_boundary + player.width / 2.0);

    // Move player to right boundary
    player.position.x = right_boundary - 30.0;
    player.move_right(0.5, right_boundary);

    // Player should not move beyond right boundary
    assert!(player.position.x <= right_boundary - player.width / 2.0);
}

// Test that player's projectile spawn position is correctly calculated
#[test]
fn test_player_projectile_position() {
    use space_invaders::game::entities::player::Player;

    let mut player = Player::default();
    player.position.x = 500.0;
    player.position.y = 700.0;
    player.projectile_spawn_x = 5.0;
    player.projectile_spawn_y = -20.0;

    let (spawn_x, spawn_y) = player.get_projectile_spawn_position();
    assert_eq!(spawn_x, 505.0);
    assert_eq!(spawn_y, 680.0);
}
