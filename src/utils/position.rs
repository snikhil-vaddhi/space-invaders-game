/// Represents a 2D position in the game world
/// This struct is used throughout the game to track the positions of
/// various entities such as the player, aliens, projectiles, and shields.

#[derive(Clone, PartialEq, Debug)]
pub struct Position {
    pub x: f32,
    pub y: f32,
}

/// Creates a new Position with the specified coordinates
/// # Arguments
/// * `x` - The x-coordinate
/// * `y` - The y-coordinate
/// # Returns -> A new Position instance with the given coordinates

impl Position {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Test that a new position is created with the correct coordinates
    #[test]
    fn test_position_new() {
        let pos = Position::new(10.0, 20.0);
        assert_eq!(pos.x, 10.0);
        assert_eq!(pos.y, 20.0);
    }

    // Test that Position implements PartialEq correctly
    #[test]
    fn test_position_equality() {
        let pos1 = Position::new(10.0, 20.0);
        let pos2 = Position::new(10.0, 20.0);
        let pos3 = Position::new(30.0, 40.0);

        assert_eq!(pos1, pos2);
        assert_ne!(pos1, pos3);
    }

    // Test that Position can be cloned
    #[test]
    fn test_position_clone() {
        let pos = Position::new(10.0, 20.0);
        let cloned_pos = pos.clone();

        assert_eq!(pos, cloned_pos);
    }

    // Test that Position implements Debug
    #[test]
    fn test_position_debug() {
        let pos = Position::new(10.0, 20.0);
        let debug_str = format!("{:?}", pos);

        assert!(debug_str.contains("10"));
        assert!(debug_str.contains("20"));
    }
}
