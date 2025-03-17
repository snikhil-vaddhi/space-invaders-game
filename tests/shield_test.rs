// Test that shields are created and positioned correctly
#[test]
fn test_shield_creation_and_positioning() {
    use space_invaders::game::entities::shield::{Shield, ShieldType};
    let shield = Shield::new(100.0, 200.0, ShieldType::UppercaseC);
    assert_eq!(shield.position.x, 100.0);
    assert_eq!(shield.position.y, 200.0);
    assert_eq!(shield.shield_type, ShieldType::UppercaseC);
}

// Test that projectile collisions are detected correctly
#[test]
fn test_shield_projectile_collision() {
    use space_invaders::game::entities::shield::{Shield, ShieldType};
    let mut shield = Shield::new(100.0, 200.0, ShieldType::UppercaseC);

    assert!(shield.check_projectile_collision(150.0, 225.0, 3.0, 15.0));

    assert!(!shield.check_projectile_collision(50.0, 50.0, 3.0, 15.0));
}

// Test that shields degrade properly when hit multiple times
#[test]
fn test_shield_degradation() {
    use space_invaders::game::entities::shield::{Shield, ShieldType};
    let mut shield = Shield::new(100.0, 200.0, ShieldType::UppercaseC);
    let initial_active_segments = shield
        .segments
        .iter()
        .flat_map(|row| row.iter())
        .filter(|&&active| active)
        .count();

    shield.check_projectile_collision(150.0, 225.0, 3.0, 15.0);
    shield.check_projectile_collision(160.0, 220.0, 3.0, 15.0);
    shield.check_projectile_collision(140.0, 230.0, 3.0, 15.0);

    let remaining_active_segments = shield
        .segments
        .iter()
        .flat_map(|row| row.iter())
        .filter(|&&active| active)
        .count();

    assert!(remaining_active_segments < initial_active_segments);
}
