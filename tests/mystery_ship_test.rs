// Test that the mystery ship correctly interacts with the game boundaries
#[test]
fn test_mystery_ship_boundaries() {
    use space_invaders::game::entities::mystery_ship::MysteryShip;

    let game_width = 800.0;
    let mut ship = MysteryShip::new();
    ship.active = true;

    ship.position.x = game_width - 10.0;
    ship.direction = 1.0;

    ship.update(0.5, game_width);
    assert!(!ship.active);

    ship.active = true;
    ship.position.x = -40.0;
    ship.direction = -1.0;

    ship.update(0.5, game_width);
    assert!(!ship.active);
}

// Test that the mystery ship moves at the expected speed
#[test]
fn test_mystery_ship_speed() {
    use space_invaders::game::entities::mystery_ship::MysteryShip;

    let mut ship = MysteryShip::new();
    ship.active = true;
    ship.position.x = 100.0;
    ship.speed = 5.0;
    ship.direction = 1.0;

    let dt = 0.1;
    ship.update(dt, 800.0);

    let expected_x = 100.0 + (5.0 * 1.0 * dt * 60.0);
    assert_eq!(ship.position.x, expected_x);
}
