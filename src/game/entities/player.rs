use crate::utils::position::Position;

/// Represents the player's ship in the game
///
/// The player can move horizontally and fire projectiles upward.
/// The ship has properties for position, size, movement speed, and projectile
#[derive(Clone, PartialEq)]
pub struct Player {
    pub position: Position,
    pub width: f32,
    pub height: f32,
    pub projectile_spawn_x: f32,
    pub projectile_spawn_y: f32,
    pub speed: f32,
    pub is_hit: bool,
    pub hit_animation_time: f64,
}

impl Default for Player {
    /// Creates a new player with default values
    /// # Returns -> A new Player instance positioned at the bottom center of the screen
    fn default() -> Self {
        Self {
            position: Position::new(512.0, 690.0),
            width: 50.0,
            height: 30.0,
            speed: 200.0,
            is_hit: false,
            hit_animation_time: 0.0,
            projectile_spawn_x: 0.0,
            projectile_spawn_y: -15.0,
        }
    }
}

/// Moves the player left, right while respecting the boundary
/// # Arguments
/// * `dt` - Delta time in seconds since the last update
/// * `left_boundary` - The left boundary of the game area in pixels
impl Player {
    pub fn move_left(&mut self, dt: f32, left_boundary: f32) {
        let new_x = self.position.x - self.speed * dt;
        self.position.x = new_x.max(left_boundary + self.width / 2.0);
    }

    pub fn move_right(&mut self, dt: f32, right_boundary: f32) {
        let new_x = self.position.x + self.speed * dt;
        self.position.x = new_x.min(right_boundary - self.width / 2.0);
    }

    /// Calculates the position where projectiles should spawn
    /// # Returns -> A tuple of (x, y) coordinates for the projectile spawn position

    pub fn get_projectile_spawn_position(&self) -> (f32, f32) {
        (
            self.position.x + self.projectile_spawn_x,
            self.position.y + self.projectile_spawn_y,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Test that a player is created with default values
    #[test]
    fn test_player_default() {
        let player = Player::default();
        assert_eq!(player.position.x, 512.0);
        assert_eq!(player.position.y, 690.0);
        assert_eq!(player.width, 50.0);
        assert_eq!(player.height, 30.0);
        assert_eq!(player.speed, 200.0);
        assert!(!player.is_hit);
        assert_eq!(player.hit_animation_time, 0.0);
        assert_eq!(player.projectile_spawn_x, 0.0);
        assert_eq!(player.projectile_spawn_y, -15.0);
    }

    // Test that move_left correctly updates player position
    #[test]
    fn test_move_left() {
        let mut player = Player::default();
        let initial_x = player.position.x;
        player.move_left(0.1, 0.0);
        assert!(player.position.x < initial_x);
        assert_eq!(player.position.x, initial_x - player.speed * 0.1);
    }

    // Test that move_right correctly updates player position
    #[test]
    fn test_move_right() {
        let mut player = Player::default();
        let initial_x = player.position.x;
        player.move_right(0.1, 1024.0);
        assert!(player.position.x > initial_x);
        assert_eq!(player.position.x, initial_x + player.speed * 0.1);
    }

    // Test that player respects left boundary
    #[test]
    fn test_left_boundary() {
        let mut player = Player::default();
        let boundary = 100.0;
        player.position.x = boundary + player.width;
        player.move_left(1.0, boundary);
        assert_eq!(player.position.x, boundary + player.width / 2.0);
    }

    // Test that player respects right boundary
    #[test]
    fn test_right_boundary() {
        let mut player = Player::default();
        let boundary = 800.0;
        player.position.x = boundary - player.width;
        player.move_right(1.0, boundary);
        assert_eq!(player.position.x, boundary - player.width / 2.0);
    }

    // Test that get_projectile_spawn_position returns the correct position
    #[test]
    fn test_projectile_spawn_position() {
        let player = Player::default();
        let (spawn_x, spawn_y) = player.get_projectile_spawn_position();
        assert_eq!(spawn_x, player.position.x + player.projectile_spawn_x);
        assert_eq!(spawn_y, player.position.y + player.projectile_spawn_y);
    }
}
