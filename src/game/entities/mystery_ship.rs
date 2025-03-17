use crate::utils::position::Position;

/// Represents the special mystery ship that occasionally appears at the top of the screen
/// The mystery ship moves horizontally across the screen and awards bonus points
/// when shot by the player. The points awarded are variable.
#[derive(Clone, PartialEq)]
pub struct MysteryShip {
    pub position: Position,
    pub width: f32,
    pub height: f32,
    pub speed: f32,
    pub active: bool,
    pub direction: f32,
    pub points: i32,
}

impl MysteryShip {
    /// Creates a new mystery ship in an inactive state
    /// The ship starts positioned off-screen and will not be visible
    /// until its active property is set to true.
    /// # Returns -> A new MysteryShip instance
    pub fn new() -> Self {
        Self {
            position: Position::new(-50.0, 10.0),
            width: 60.0,
            height: 20.0,
            speed: 4.0,
            active: false,
            direction: 1.0,
            points: 100,
        }
    }

    /// Updates the mystery ship's position based on elapsed time
    /// If the ship moves off-screen, it becomes inactive.
    /// # Arguments
    /// * `dt` - Delta time in seconds since the last update
    /// * `game_width` - The width of the game screen in pixels

    pub fn update(&mut self, dt: f32, game_width: f32) {
        if !self.active {
            return;
        }

        self.position.x += self.speed * self.direction * dt * 60.0;

        if (self.direction > 0.0 && self.position.x > game_width + 50.0)
            || (self.direction < 0.0 && self.position.x < -50.0)
        {
            self.active = false;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Test that a new mystery ship is created with the correct initial properties
    #[test]
    fn test_mystery_ship_creation() {
        let ship = MysteryShip::new();
        assert_eq!(ship.position.x, -50.0);
        assert_eq!(ship.position.y, 10.0);
        assert_eq!(ship.width, 60.0);
        assert_eq!(ship.height, 20.0);
        assert_eq!(ship.speed, 4.0);
        assert!(!ship.active);
        assert_eq!(ship.direction, 1.0);
        assert_eq!(ship.points, 100);
    }

    // Test that an inactive ship doesn't move when updated
    #[test]
    fn test_inactive_ship_doesnt_move() {
        let mut ship = MysteryShip::new();
        let initial_x = ship.position.x;

        ship.update(0.1, 800.0);

        assert_eq!(ship.position.x, initial_x);
    }

    // Test that an active ship moves in the correct direction
    #[test]
    fn test_active_ship_movement() {
        let mut ship = MysteryShip::new();
        ship.active = true;
        let initial_x = ship.position.x;

        ship.direction = 1.0;
        ship.update(0.1, 800.0);
        assert!(ship.position.x > initial_x);

        let current_x = ship.position.x;
        ship.direction = -1.0;
        ship.update(0.1, 800.0);
        assert!(ship.position.x < current_x);
    }

    // Test that a ship becomes inactive when it moves off-screen
    #[test]
    fn test_ship_becomes_inactive_offscreen() {
        let mut ship = MysteryShip::new();
        ship.active = true;

        ship.direction = 1.0;
        ship.position.x = 830.0;
        ship.update(0.1, 800.0);
        assert!(!ship.active);

        ship.active = true;
        ship.direction = -1.0;
        ship.position.x = -45.0;
        ship.update(0.1, 800.0);
        assert!(!ship.active);
    }
}
