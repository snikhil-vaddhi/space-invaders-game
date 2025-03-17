// Test that projectiles move at the correct speed
#[test]
fn test_projectile_movement_speed() {
    use space_invaders::game::entities::projectile::Projectile;

    let mut player_proj = Projectile::new(100.0, 200.0, true);
    let mut alien_proj = Projectile::new(300.0, 400.0, false);

    let dt = 0.1;
    let initial_player_y = player_proj.position.y;
    let initial_alien_y = alien_proj.position.y;

    // Update projectiles
    player_proj.update(dt);
    alien_proj.update(dt);

    // Check player projectile moved up at correct speed
    assert_eq!(player_proj.position.y, initial_player_y - 400.0 * dt);

    // Check alien projectile moved down at correct speed
    assert_eq!(alien_proj.position.y, initial_alien_y + 200.0 * dt);
}

// Test that projectiles correctly detect when they're off screen
#[test]
fn test_projectile_off_screen_detection() {
    use space_invaders::game::entities::projectile::Projectile;

    let screen_height = 768.0;

    // Test player projectile going off top
    let mut player_proj = Projectile::new(100.0, 10.0, true);
    assert!(!player_proj.is_off_screen(screen_height));

    // Move it up until it's off screen
    while !player_proj.is_off_screen(screen_height) {
        player_proj.update(0.1);
    }

    assert!(player_proj.position.y < 0.0);

    // Test alien projectile going off bottom
    let mut alien_proj = Projectile::new(300.0, 700.0, false);
    assert!(!alien_proj.is_off_screen(screen_height));

    // Move it down until it's off screen
    while !alien_proj.is_off_screen(screen_height) {
        alien_proj.update(0.1);
    }

    assert!(alien_proj.position.y > screen_height);
}
