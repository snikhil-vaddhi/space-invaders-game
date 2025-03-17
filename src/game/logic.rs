use crate::game::entities::alien::AlienFormation;
use crate::game::entities::mystery_ship::MysteryShip;
use crate::game::entities::projectile::Projectile;
use crate::game::state::{GameScreen, GameState};
use rand::Rng;

const GAME_WIDTH: f32 = 1024.0;
const GAME_HEIGHT: f32 = 700.0;

/// Updates the player's position and state based on input and game conditions
/// # Arguments
/// * `game_state` - Mutable reference to the current game state
/// * `delta_time` - Time elapsed since last update in seconds

pub fn update_player(game_state: &mut GameState, delta_time: f64) {
    let player = &mut game_state.player;
    let dt = delta_time as f32;

    if game_state.key_states.left {
        player.move_left(dt, 0.0);
    }
    if game_state.key_states.right {
        player.move_right(dt, GAME_WIDTH);
    }

    if game_state.invincibility_timer > 0.0 {
        game_state.invincibility_timer -= delta_time;

        let elapsed_time = 3.0 - game_state.invincibility_timer;

        let interval = 0.1;
        let phase = (elapsed_time / interval) as i32;
        player.is_hit = phase % 2 == 0;
    } else {
        player.is_hit = false;
    }
}

/// Updates the alien formation's position and state
/// # Arguments
/// * `game_state` - Mutable reference to the current game state
/// * `delta_time` - Time elapsed since last update in seconds
pub fn update_aliens(game_state: &mut GameState, delta_time: f64) {
    let formation = &mut game_state.alien_formation;

    formation.move_timer += delta_time;

    if formation.move_timer >= formation.move_interval {
        formation.move_timer = 0.0;

        if formation.check_edges() {
            formation.direction *= -1.0;
            formation.should_descend = true;
        }

        for alien in &mut formation.aliens {
            if !alien.is_alive {
                continue;
            }

            alien.position.x += formation.direction * formation.speed;

            if formation.should_descend {
                alien.position.y += 20.0;
            }

            alien.animation_frame += 1;
        }

        formation.should_descend = false;

        let lowest_y = formation.get_lowest_y();
        if lowest_y >= 550.0 {
            game_state.game_over = true;
        }
    }
}

/// Marks an alien as destroyed and updates game state accordingly
/// # Arguments
/// * `game_state` - Mutable reference to the current game state
/// * `alien_index` - Index of the alien to destroy
/// # Returns -> `true` if the alien was successfully destroyed, `false` otherwise

pub fn destroy_alien(game_state: &mut GameState, alien_index: usize) -> bool {
    let formation = &mut game_state.alien_formation;

    if alien_index >= formation.aliens.len() || !formation.aliens[alien_index].is_alive {
        return false;
    }

    let alien = &mut formation.aliens[alien_index];
    alien.is_alive = false;

    let points = alien.alien_type.points();
    game_state.score += points;
    if game_state.score > game_state.high_score {
        game_state.high_score = game_state.score;
    }

    formation.aliens_killed += 1;

    let living_count = formation.count_living();
    if living_count > 0 {
        let total_aliens = formation.aliens.len();
        let percent_remaining = living_count as f64 / total_aliens as f64;
        formation.move_interval = 0.1 + percent_remaining * 0.4;
    }

    true
}

/// Handles player shooting logic based on input and cooldown
/// # Arguments
/// * `game_state` - Mutable reference to the current game state
/// * `delta_time` - Time elapsed since last update in seconds
pub fn handle_player_shooting(game_state: &mut GameState, delta_time: f64) {
    if game_state.player_shoot_cooldown > 0.0 {
        game_state.player_shoot_cooldown -= delta_time;
    }

    if game_state.key_states.shift && game_state.player_shoot_cooldown <= 0.0 {
        let projectile = Projectile::player(&game_state.player);
        game_state.player_projectiles.push(projectile);

        game_state.player_shoot_cooldown = 0.5;
    }
}

/// Handles alien shooting logic based on random selection and cooldown
/// # Arguments
/// * `game_state` - Mutable reference to the current game state
/// * `delta_time` - Time elapsed since last update in seconds

pub fn handle_alien_shooting(game_state: &mut GameState, delta_time: f64) {
    if game_state.alien_shoot_cooldown > 0.0 {
        game_state.alien_shoot_cooldown -= delta_time;
    }

    if game_state.alien_shoot_cooldown <= 0.0 {
        let living_aliens: Vec<&_> = game_state
            .alien_formation
            .aliens
            .iter()
            .filter(|a| a.is_alive)
            .collect();

        if !living_aliens.is_empty() {
            let mut rng = rand::thread_rng();
            let random_index = rng.gen_range(0..living_aliens.len());
            let shooting_alien = living_aliens[random_index];

            let projectile = Projectile::alien(shooting_alien);
            game_state.alien_projectiles.push(projectile);

            game_state.alien_shoot_cooldown = rng.gen_range(0.5..2.0);
        }
    }
}

/// Updates all projectiles' positions and removes those that are off-screen
/// # Arguments
/// * `game_state` - Mutable reference to the current game state
/// * `delta_time` - Time elapsed since last update in seconds

pub fn update_projectiles(game_state: &mut GameState, delta_time: f64) {
    let dt = delta_time as f32;

    for projectile in &mut game_state.player_projectiles {
        projectile.update(dt);
    }

    for projectile in &mut game_state.alien_projectiles {
        projectile.update(dt);
    }

    game_state
        .player_projectiles
        .retain(|p| !p.is_off_screen(0.0));
    game_state
        .alien_projectiles
        .retain(|p| !p.is_off_screen(GAME_HEIGHT));
}

/// Checks if the current level is complete and prepares the next level if needed
/// # Arguments
/// * `game_state` - Mutable reference to the current game state

pub fn check_level_completion(game_state: &mut GameState) {
    if game_state.alien_formation.count_living() == 0 {
        game_state.level += 1;

        let mut new_formation = AlienFormation::new(GAME_WIDTH);
        new_formation.speed *= 1.0 + (game_state.level as f32 * 0.1);
        new_formation.move_interval *= 0.9;
        game_state.alien_formation = new_formation;
    }
}

/// Checks for game over conditions and updates game state accordingly
/// # Arguments
/// * `game_state` - Mutable reference to the current game state

pub fn check_game_over_conditions(game_state: &mut GameState) {
    if game_state.lives <= 0 {
        game_state.game_over = true;
        game_state.screen = GameScreen::GameOver;
    }

    let player_y = game_state.player.position.y;
    let lowest_alien_y = game_state.alien_formation.get_lowest_y();

    if lowest_alien_y >= player_y - 50.0 {
        game_state.game_over = true;
        game_state.screen = GameScreen::GameOver;
    }
}

/// Updates the mystery ship's state or spawns a new one
/// # Arguments
/// * `game_state` - Mutable reference to the current game state

pub fn update_mystery_ship(game_state: &mut GameState) {
    if let Some(ship) = &mut game_state.mystery_ship {
        ship.update(game_state.dt, GAME_WIDTH);

        if !ship.active {
            game_state.mystery_ship = None;
        }
    } else {
        game_state.mystery_ship_timer -= game_state.dt;

        if game_state.mystery_ship_timer <= 0.0 {
            let mut rng = rand::thread_rng();
            game_state.mystery_ship_timer = rng.gen_range(15.0..30.0);

            if rng.gen_bool(0.7) {
                let mut ship = MysteryShip::new();

                if rng.gen_bool(0.5) {
                    ship.direction = 1.0;
                    ship.position.x = -50.0;
                } else {
                    ship.direction = -1.0;
                    ship.position.x = 850.0;
                }

                ship.active = true;
                ship.points = rng.gen_range(1..7) * 50;

                game_state.mystery_ship = Some(ship);
            }
        }
    }
}

/// Checks if a projectile has hit the mystery ship
/// # Arguments
/// * `mystery_ship` - Mutable reference to the optional mystery ship
/// * `projectile` - Reference to the projectile to check
/// # Returns ->`Some(points)` if the ship was hit, `None` otherwise

pub fn check_mystery_ship_hit(
    mystery_ship: &mut Option<MysteryShip>,
    projectile: &Projectile,
) -> Option<i32> {
    if let Some(ship) = mystery_ship {
        if !ship.active || !projectile.is_player_projectile {
            return None;
        }

        if projectile.position.x < ship.position.x + ship.width
            && projectile.position.x + projectile.width > ship.position.x
            && projectile.position.y < ship.position.y + ship.height
            && projectile.position.y + projectile.height > ship.position.y
        {
            let points = ship.points;
            ship.active = false;
            return Some(points);
        }
    }
    None
}

/// Checks for collisions between projectiles and game entities
/// # Arguments
/// * `game_state` - Mutable reference to the current game state

pub fn check_projectile_collisions(game_state: &mut GameState) {
    let mut projectiles_to_remove = Vec::new();
    let mut aliens_to_destroy = Vec::new();
    let mut mystery_ship_hits = Vec::new();
    let mut shield_hits = Vec::new();
    let mut points_to_add = 0;

    for (proj_idx, projectile) in game_state.player_projectiles.iter().enumerate() {
        if let Some(points) = check_mystery_ship_hit(&mut game_state.mystery_ship, projectile) {
            mystery_ship_hits.push(proj_idx);
            points_to_add += points;
            game_state.mystery_ship_timer = 5.0;
        }
    }

    if points_to_add > 0 {
        game_state.score += points_to_add;
        if game_state.score > game_state.high_score {
            game_state.high_score = game_state.score;
        }
    }

    for idx in mystery_ship_hits.iter().rev() {
        if *idx < game_state.player_projectiles.len() {
            game_state.player_projectiles.remove(*idx);
        }
    }

    check_shield_projectile_collisions(game_state, &mut shield_hits);

    game_state.player_projectiles.retain(|p| {
        !shield_hits.iter().any(|&(shield_idx, is_player)| {
            is_player && p.position.y > game_state.shields[shield_idx].position.y
        })
    });

    game_state.alien_projectiles.retain(|p| {
        !shield_hits.iter().any(|&(shield_idx, is_player)| {
            !is_player
                && p.position.y
                    < game_state.shields[shield_idx].position.y
                        + game_state.shields[shield_idx].height
        })
    });

    for (proj_idx, projectile) in game_state.player_projectiles.iter().enumerate() {
        for (alien_idx, alien) in game_state.alien_formation.aliens.iter().enumerate() {
            if !alien.is_alive {
                continue;
            }

            if projectile.position.x < alien.position.x + alien.width
                && projectile.position.x + projectile.width > alien.position.x
                && projectile.position.y < alien.position.y + alien.height
                && projectile.position.y + projectile.height > alien.position.y
            {
                projectiles_to_remove.push(proj_idx);
                aliens_to_destroy.push(alien_idx);
                break;
            }
        }
    }

    for alien_idx in aliens_to_destroy {
        destroy_alien(game_state, alien_idx);
    }

    for idx in projectiles_to_remove.iter().rev() {
        if *idx < game_state.player_projectiles.len() {
            game_state.player_projectiles.remove(*idx);
        }
    }

    check_player_collisions(game_state);
}

/// Helper function to check for collisions between projectiles and shields
/// # Arguments
/// * `game_state` - Mutable reference to the current game state
/// * `shield_hits` - Mutable reference to a vector that will store shield hit information

fn check_shield_projectile_collisions(
    game_state: &mut GameState,
    shield_hits: &mut Vec<(usize, bool)>,
) {
    for (shield_idx, shield) in game_state.shields.iter_mut().enumerate() {
        for projectile in game_state
            .player_projectiles
            .iter()
            .chain(game_state.alien_projectiles.iter())
        {
            if shield.check_projectile_collision(
                projectile.position.x,
                projectile.position.y,
                projectile.width,
                projectile.height,
            ) {
                shield_hits.push((shield_idx, projectile.is_player_projectile));
            }
        }
    }
}

/// Helper function to check for collisions between alien projectiles and the player
/// # Arguments
/// * `game_state` - Mutable reference to the current game state

fn check_player_collisions(game_state: &mut GameState) {
    let mut alien_projectiles_to_remove = Vec::new();

    for (proj_idx, projectile) in game_state.alien_projectiles.iter().enumerate() {
        if game_state.invincibility_timer > 0.0 {
            continue;
        }

        let player_x_pixels = game_state.player.position.x - 25.0;
        if projectile.position.x < player_x_pixels + game_state.player.width
            && projectile.position.x + projectile.width > player_x_pixels
            && projectile.position.y < game_state.player.position.y + game_state.player.height
            && projectile.position.y + projectile.height > game_state.player.position.y
        {
            alien_projectiles_to_remove.push(proj_idx);

            game_state.player.is_hit = true;
            game_state.invincibility_timer = 3.0;

            game_state.lives -= 1;

            if game_state.lives <= 0 {
                game_state.game_over = true;
            }

            break;
        }
    }

    for idx in alien_projectiles_to_remove.iter().rev() {
        if *idx < game_state.alien_projectiles.len() {
            game_state.alien_projectiles.remove(*idx);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::game::entities::player::Player;
    // use crate::game::entities::shield::{Shield, ShieldType};
    // use crate::input::key_states::KeyStates;

    // Test that player moves left when left key is pressed
    #[test]
    fn test_player_moves_left() {
        let mut game_state = GameState::default();
        game_state.player = Player::default();
        game_state.key_states.left = true;

        let initial_x = game_state.player.position.x;
        update_player(&mut game_state, 0.1);

        assert!(game_state.player.position.x < initial_x);
    }

    // Test that player moves right when right key is pressed
    #[test]
    fn test_player_moves_right() {
        let mut game_state = GameState::default();
        game_state.player = Player::default();
        game_state.key_states.right = true;

        let initial_x = game_state.player.position.x;
        update_player(&mut game_state, 0.1);

        assert!(game_state.player.position.x > initial_x);
    }

    // Test that player shooting creates a projectile
    #[test]
    fn test_player_shooting() {
        let mut game_state = GameState::default();
        game_state.player = Player::default();
        game_state.key_states.shift = true;
        game_state.player_shoot_cooldown = 0.0;

        handle_player_shooting(&mut game_state, 0.1);

        assert_eq!(game_state.player_projectiles.len(), 1);
        assert!(game_state.player_shoot_cooldown > 0.0);
    }

    // Test that alien destruction increases score
    #[test]
    fn test_destroy_alien() {
        let mut game_state = GameState::default();
        game_state.alien_formation = AlienFormation::new(GAME_WIDTH);
        let initial_score = game_state.score;

        destroy_alien(&mut game_state, 0);

        assert!(!game_state.alien_formation.aliens[0].is_alive);
        assert!(game_state.score > initial_score);
    }

    // Test that level completion creates a new alien formation
    #[test]
    fn test_level_completion() {
        let mut game_state = GameState::default();
        game_state.alien_formation = AlienFormation::new(GAME_WIDTH);
        let initial_level = game_state.level;

        // Kill all aliens
        for alien in &mut game_state.alien_formation.aliens {
            alien.is_alive = false;
        }

        check_level_completion(&mut game_state);

        assert_eq!(game_state.level, initial_level + 1);
        assert!(game_state.alien_formation.count_living() > 0);
    }
}
