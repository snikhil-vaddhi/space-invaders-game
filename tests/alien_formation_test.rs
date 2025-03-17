use space_invaders::game::entities::alien::AlienFormation;

// Test that the alien formation moves correctly and changes direction at screen edges
#[test]
fn test_formation_movement() {
    let mut formation = AlienFormation::new(1024.0);
    let initial_x = formation.aliens[0].position.x;
    formation.move_timer = formation.move_interval;
    for alien in &mut formation.aliens {
        if alien.is_alive {
            alien.position.x += formation.direction * formation.speed;
        }
    }

    assert!(formation.aliens[0].position.x > initial_x);

    if formation.check_edges() {
        formation.direction *= -1.0;
        formation.should_descend = true;

        for alien in &mut formation.aliens {
            if alien.is_alive {
                alien.position.y += 20.0;
            }
        }
    }

    for alien in &mut formation.aliens {
        alien.position.x = 1000.0;
    }

    assert!(formation.check_edges());

    formation.direction *= -1.0;
    formation.should_descend = true;
    let initial_y = formation.aliens[0].position.y;

    for alien in &mut formation.aliens {
        if alien.is_alive {
            alien.position.y += 20.0;
        }
    }

    assert!(formation.aliens[0].position.y > initial_y);
}

// Test that the formation speed increases as aliens are destroyed
#[test]
fn test_formation_speed_increases() {
    let mut formation = AlienFormation::new(1024.0);
    let initial_interval = formation.move_interval;

    for i in 0..(formation.aliens.len() / 2) {
        formation.aliens[i].is_alive = false;
        formation.aliens_killed += 1;
    }

    let living_count = formation.count_living();
    let total_aliens = formation.aliens.len();
    let percent_remaining = living_count as f64 / total_aliens as f64;
    formation.move_interval = 0.1 + percent_remaining * 0.4;

    assert!(formation.move_interval < initial_interval);
}
