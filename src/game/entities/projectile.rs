use crate::game::entities::alien::Alien;
use crate::game::entities::player::Player;
use crate::utils::position::Position;

/// Represents a projectile fired by either the player or an alien
/// Player projectiles move upward, while alien projectiles move downward.
#[derive(Clone, PartialEq)]
pub struct Projectile {
    pub position: Position,
    pub velocity: f32,
    pub width: f32,
    pub height: f32,
    pub is_player_projectile: bool,
}

/// Creates a new projectile at the specified position
/// # Arguments
/// * `x` - The x-coordinate of the projectile
/// * `y` - The y-coordinate of the projectile
/// * `is_player_projectile` - Whether this is a player projectile (true) or alien projectile (false)
/// # Returns -> A new Projectile instance with appropriate velocity and dimensions
impl Projectile {
    pub fn new(x: f32, y: f32, is_player_projectile: bool) -> Self {
        let width = 3.0;
        let height = 15.0;
        let velocity = if is_player_projectile { -400.0 } else { 200.0 };

        Self {
            position: Position::new(x, y),
            velocity,
            width,
            height,
            is_player_projectile,
        }
    }

    /// Creates a player projectile at the player's firing position
    /// # Arguments
    /// * `player` - Reference to the player that is firing
    /// # Returns -> A new player projectile positioned at the player's cannon

    pub fn player(player: &Player) -> Self {
        let (x, y) = player.get_projectile_spawn_position();
        Self::new(x, y, true)
    }

    /// Creates an alien projectile at the alien's firing position
    /// # Arguments
    /// * `alien` - Reference to the alien that is firing
    /// # Returns -> A new alien projectile positioned at the bottom of the alien
    pub fn alien(alien: &Alien) -> Self {
        let x = alien.position.x + alien.width / 2.0 - 1.5;
        let y = alien.position.y + alien.height;
        Self::new(x, y, false)
    }

    /// Updates the projectile's position based on its velocity
    /// # Arguments
    /// * `dt` - Delta time in seconds since the last update
    pub fn update(&mut self, dt: f32) {
        self.position.y += self.velocity * dt;
    }

    /// Checks if the projectile has moved off the screen
    /// # Arguments
    /// * `screen_height` - The height of the game screen in pixels
    /// # Returns -> `true` if the projectile is off-screen, `false` otherwise
    pub fn is_off_screen(&self, screen_height: f32) -> bool {
        (self.is_player_projectile && self.position.y < -self.height)
            || (!self.is_player_projectile && self.position.y > screen_height)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Test that a new projectile is created with the correct properties
    #[test]
    fn test_projectile_creation() {
        let player_proj = Projectile::new(100.0, 200.0, true);
        assert_eq!(player_proj.position.x, 100.0);
        assert_eq!(player_proj.position.y, 200.0);
        assert_eq!(player_proj.width, 3.0);
        assert_eq!(player_proj.height, 15.0);
        assert_eq!(player_proj.velocity, -400.0);
        assert!(player_proj.is_player_projectile);

        let alien_proj = Projectile::new(300.0, 400.0, false);
        assert_eq!(alien_proj.position.x, 300.0);
        assert_eq!(alien_proj.position.y, 400.0);
        assert_eq!(alien_proj.velocity, 200.0);
        assert!(!alien_proj.is_player_projectile);
    }

    // Test that a player projectile is created at the correct position
    #[test]
    fn test_player_projectile() {
        let player = Player::default();
        let (spawn_x, spawn_y) = player.get_projectile_spawn_position();

        let proj = Projectile::player(&player);
        assert_eq!(proj.position.x, spawn_x);
        assert_eq!(proj.position.y, spawn_y);
        assert!(proj.is_player_projectile);
    }

    // Test that an alien projectile is created at the correct position
    #[test]
    fn test_alien_projectile() {
        let alien = Alien::new(100.0, 50.0, crate::game::entities::alien::AlienType::Medium);
        let expected_x = alien.position.x + alien.width / 2.0 - 1.5;
        let expected_y = alien.position.y + alien.height;

        let proj = Projectile::alien(&alien);
        assert_eq!(proj.position.x, expected_x);
        assert_eq!(proj.position.y, expected_y);
        assert!(!proj.is_player_projectile);
    }

    // Test that update correctly changes the projectile position
    #[test]
    fn test_update() {
        let mut player_proj = Projectile::new(100.0, 200.0, true);
        let initial_y = player_proj.position.y;
        player_proj.update(0.1);
        assert_eq!(
            player_proj.position.y,
            initial_y + player_proj.velocity * 0.1
        );

        let mut alien_proj = Projectile::new(300.0, 400.0, false);
        let initial_y = alien_proj.position.y;
        alien_proj.update(0.1);
        assert_eq!(alien_proj.position.y, initial_y + alien_proj.velocity * 0.1);
    }

    // Test that is_off_screen correctly detects when projectiles leave the screen
    #[test]
    fn test_is_off_screen() {
        let screen_height = 768.0;

        // Player projectile off top of screen
        let mut player_proj = Projectile::new(100.0, -20.0, true);
        assert!(player_proj.is_off_screen(screen_height));

        // Player projectile on screen
        player_proj.position.y = 100.0;
        assert!(!player_proj.is_off_screen(screen_height));

        // Alien projectile off bottom of screen
        let mut alien_proj = Projectile::new(300.0, 800.0, false);
        assert!(alien_proj.is_off_screen(screen_height));

        // Alien projectile on screen
        alien_proj.position.y = 500.0;
        assert!(!alien_proj.is_off_screen(screen_height));
    }
}
