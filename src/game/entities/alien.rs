use crate::utils::position::Position;

pub const GAME_WIDTH: f32 = 1024.0;

/// Represents the different types of aliens in the game
/// Each alien type has different point values and sizes.
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum AlienType {
    Small,
    Medium,
    Large,
}

/// Returns -> the point value for destroying this type of alien
impl AlienType {
    pub fn points(&self) -> i32 {
        match self {
            AlienType::Small => 30,
            AlienType::Medium => 20,
            AlienType::Large => 10,
        }
    }
    /// Returns -> the width and height of this alien type in
    /// form of tuple (width, height) in pixels
    pub fn size(&self) -> (f32, f32) {
        match self {
            AlienType::Small => (30.0, 30.0),
            AlienType::Medium => (35.0, 30.0),
            AlienType::Large => (40.0, 30.0),
        }
    }
}

/// Aliens have a position, type, size, and animation state.
#[derive(Clone, PartialEq)]
pub struct Alien {
    pub position: Position,
    pub alien_type: AlienType,
    pub width: f32,
    pub height: f32,
    pub is_alive: bool,
    pub animation_frame: usize,
}

/// Creates a new alien at the specified position and type
/// # Arguments
/// * `x` - The x-coordinate of the alien
/// * `y` - The y-coordinate of the alien
/// * `alien_type` - The type of alien to create
/// # Returns -> A new Alien instance
impl Alien {
    pub fn new(x: f32, y: f32, alien_type: AlienType) -> Self {
        let (width, height) = alien_type.size();
        Self {
            position: Position::new(x, y),
            alien_type,
            width,
            height,
            is_alive: true,
            animation_frame: 0,
        }
    }
}

// Represents a formation of aliens that move together
/// The formation manages the collective movement and behavior of all aliens.
/// It handles direction changes, descent movements, and speed adjustments
/// as aliens are destroyed.

#[derive(Default)]
pub struct AlienFormation {
    pub aliens: Vec<Alien>,
    pub direction: f32,
    pub speed: f32,
    pub move_timer: f64,
    pub move_interval: f64,
    pub should_descend: bool,
    pub aliens_killed: usize,
}

/// Creates a new alien formation with a standard grid layout
/// # Arguments
/// * `_screen_width` - The width of the game screen (currently unused)
/// # Returns -> A new `AlienFormation` with 5 rows and 11 columns of aliens

impl AlienFormation {
    pub fn new(_screen_width: f32) -> Self {
        let mut aliens = Vec::new();
        let rows = 5;
        let cols = 11;
        let spacing_x = 50.0;
        let spacing_y = 50.0;

        let formation_width = cols as f32 * spacing_x;
        let start_x = (GAME_WIDTH - formation_width) / 2.0;
        let start_y = 50.0;

        for row in 0..rows {
            let alien_type = match row {
                0 => AlienType::Small,
                1 | 2 => AlienType::Medium,
                _ => AlienType::Large,
            };

            for col in 0..cols {
                let x = start_x + col as f32 * spacing_x;
                let y = start_y + row as f32 * spacing_y;
                aliens.push(Alien::new(x, y, alien_type));
            }
        }

        Self {
            aliens,
            direction: 1.0,
            speed: 20.0,
            move_timer: 0.0,
            move_interval: 0.5,
            should_descend: false,
            aliens_killed: 0,
        }
    }

    /// Checks if any living alien has reached the edge of the screen
    /// This is used to determine when the formation should change direction
    /// and move downward
    /// # Returns -> `true` if any alien has reached the edge, `false` otherwise

    pub fn check_edges(&self) -> bool {
        for alien in &self.aliens {
            if !alien.is_alive {
                continue;
            }

            if (self.direction > 0.0 && alien.position.x + alien.width >= GAME_WIDTH - 20.0)
                || (self.direction < 0.0 && alien.position.x <= 20.0)
            {
                return true;
            }
        }
        false
    }

    /// Counts the number of aliens that are still alive
    /// # Returns -> The count of living aliens in the formation

    pub fn count_living(&self) -> usize {
        self.aliens.iter().filter(|a| a.is_alive).count()
    }

    /// Gets the y-coordinate of the lowest living alien in the formation
    /// This is used to check if aliens have reached the bottom of the screen.
    /// # Returns -> The y-coordinate of the bottom edge of the lowest alien, or 0.0 if no
    /// aliens are alive

    pub fn get_lowest_y(&self) -> f32 {
        self.aliens
            .iter()
            .filter(|a| a.is_alive)
            .map(|a| a.position.y + a.height)
            .max_by(|a, b| a.partial_cmp(b).unwrap())
            .unwrap_or(0.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Test that each alien type returns the correct point value
    #[test]
    fn test_alien_type_points() {
        assert_eq!(AlienType::Small.points(), 30);
        assert_eq!(AlienType::Medium.points(), 20);
        assert_eq!(AlienType::Large.points(), 10);
    }

    // Test that each alien type returns the correct size dimensions
    #[test]
    fn test_alien_type_size() {
        assert_eq!(AlienType::Small.size(), (30.0, 30.0));
        assert_eq!(AlienType::Medium.size(), (35.0, 30.0));
        assert_eq!(AlienType::Large.size(), (40.0, 30.0));
    }

    // Test that a new alien is created with the correct properties
    #[test]
    fn test_alien_creation() {
        let alien = Alien::new(100.0, 200.0, AlienType::Medium);
        assert_eq!(alien.position.x, 100.0);
        assert_eq!(alien.position.y, 200.0);
        assert_eq!(alien.alien_type, AlienType::Medium);
        assert_eq!(alien.width, 35.0);
        assert_eq!(alien.height, 30.0);
        assert!(alien.is_alive);
        assert_eq!(alien.animation_frame, 0);
    }

    // Test that a new formation is created with the correct initial state
    #[test]
    fn test_alien_formation_creation() {
        let formation = AlienFormation::new(1024.0);
        assert_eq!(formation.aliens.len(), 55);
        assert_eq!(formation.direction, 1.0);
        assert_eq!(formation.speed, 20.0);
        assert_eq!(formation.move_interval, 0.5);
        assert!(!formation.should_descend);
        assert_eq!(formation.aliens_killed, 0);

        assert_eq!(formation.aliens[0].alien_type, AlienType::Small);
        assert_eq!(formation.aliens[11].alien_type, AlienType::Medium);
        assert_eq!(formation.aliens[22].alien_type, AlienType::Medium);
        assert_eq!(formation.aliens[33].alien_type, AlienType::Large);
        assert_eq!(formation.aliens[44].alien_type, AlienType::Large);
    }

    // Test that the formation correctly detects when aliens reach screen edges
    #[test]
    fn test_check_edges() {
        let mut formation = AlienFormation::new(1024.0);
        assert!(!formation.check_edges());

        for alien in &mut formation.aliens {
            alien.position.x = GAME_WIDTH - alien.width - 15.0;
        }
        assert!(formation.check_edges());

        for alien in &mut formation.aliens {
            alien.position.x = 500.0;
        }
        assert!(!formation.check_edges());

        for alien in &mut formation.aliens {
            alien.position.x = 15.0;
        }
        formation.direction = -1.0;
        assert!(formation.check_edges());
    }

    // Test that the formation correctly counts living aliens
    #[test]
    fn test_count_living() {
        let mut formation = AlienFormation::new(1024.0);
        assert_eq!(formation.count_living(), 55);

        for i in 0..5 {
            formation.aliens[i].is_alive = false;
        }
        assert_eq!(formation.count_living(), 50);

        for alien in &mut formation.aliens {
            alien.is_alive = false;
        }
        assert_eq!(formation.count_living(), 0);
    }

    // Test that the formation correctly finds the lowest living alien
    #[test]
    fn test_get_lowest_y() {
        let mut formation = AlienFormation::new(1024.0);

        let initial_lowest = formation.get_lowest_y();

        let last_alien_index = formation.aliens.len() - 1;
        formation.aliens[last_alien_index].position.y += 100.0;

        assert!(formation.get_lowest_y() > initial_lowest);

        formation.aliens[last_alien_index].is_alive = false;

        assert_eq!(formation.get_lowest_y(), initial_lowest);

        for alien in &mut formation.aliens {
            alien.is_alive = false;
        }

        assert_eq!(formation.get_lowest_y(), 0.0);
    }
}
