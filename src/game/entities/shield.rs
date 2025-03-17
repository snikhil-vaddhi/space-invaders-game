use crate::utils::position::Position;

/// Shield module provides defensive structures shaped like letters that can be damaged by projectiles
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum ShieldType {
    UppercaseC,
    UppercaseO,
    UppercaseR,
    UppercaseT,
    UppercaseW,
}

/// A defensive structure composed of destructible segments arranged in a letter pattern
#[derive(Clone, PartialEq)]
pub struct Shield {
    pub position: Position,
    pub width: f32,
    pub height: f32,
    pub shield_type: ShieldType,
    pub segments: Vec<Vec<bool>>,
}

/// Creates a new shield with the specified position and design
/// # Arguments
/// * `x` - X coordinate of the shield's top-left corner
/// * `y` - Y coordinate of the shield's top-left corner
/// * `shield_type` - The letter design to use for this shield
///
impl Shield {
    pub fn new(x: f32, y: f32, shield_type: ShieldType) -> Self {
        let width = 100.0;
        let height = 50.0;
        let segments = create_shield_segments(shield_type);

        Self {
            position: Position::new(x, y),
            width,
            height,
            shield_type,
            segments,
        }
    }

    /// Checks if a point hits an active shield segment and destroys it if hit
    /// # Arguments
    /// * `x` - X coordinate of the hit point
    /// * `y` - Y coordinate of the hit point
    /// # Returns
    /// true if an active segment was hit, false otherwise
    pub fn is_hit(&mut self, x: f32, y: f32) -> bool {
        let relative_x = (x - self.position.x) / self.width;
        let relative_y = (y - self.position.y) / self.height;

        if relative_x < 0.0 || relative_x >= 1.0 || relative_y < 0.0 || relative_y >= 1.0 {
            return false;
        }

        let segment_x = (relative_x * self.segments[0].len() as f32) as usize;
        let segment_y = (relative_y * self.segments.len() as f32) as usize;

        if self.segments[segment_y][segment_x] {
            self.segments[segment_y][segment_x] = false;
            true
        } else {
            false
        }
    }

    /// Checks collision between a projectile and the shield using its four corners
    /// # Arguments
    /// * `proj_x` - X coordinate of the projectile's top-left corner
    /// * `proj_y` - Y coordinate of the projectile's top-left corner
    /// * `proj_width` - Width of the projectile
    /// * `proj_height` - Height of the projectile
    /// # Returns -> true if any corner hits an active shield segment

    pub fn check_projectile_collision(
        &mut self,
        proj_x: f32,
        proj_y: f32,
        proj_width: f32,
        proj_height: f32,
    ) -> bool {
        let points = [
            (proj_x, proj_y),
            (proj_x + proj_width, proj_y),
            (proj_x, proj_y + proj_height),
            (proj_x + proj_width, proj_y + proj_height),
        ];

        for (x, y) in points.iter() {
            if self.is_hit(*x, *y) {
                return true;
            }
        }

        false
    }
}

pub fn create_shield_segments(shield_type: ShieldType) -> Vec<Vec<bool>> {
    match shield_type {
        ShieldType::UppercaseC => create_letter_c(),
        ShieldType::UppercaseR => create_letter_r(),
        ShieldType::UppercaseT => create_letter_t(),
        ShieldType::UppercaseW => create_letter_w(),
        ShieldType::UppercaseO => create_letter_o(),
    }
}

fn create_letter_c() -> Vec<Vec<bool>> {
    vec![
        vec![
            false, true, true, true, true, true, true, true, true, true, true, true, false,
        ],
        vec![
            true, true, true, true, true, true, true, true, true, true, true, true, true,
        ],
        vec![
            true, true, true, false, false, false, false, false, false, false, false, false, false,
        ],
        vec![
            true, true, true, false, false, false, false, false, false, false, false, false, false,
        ],
        vec![
            true, true, true, false, false, false, false, false, false, false, false, false, false,
        ],
        vec![
            true, true, true, true, true, true, true, true, true, true, true, true, true,
        ],
        vec![
            false, true, true, true, true, true, true, true, true, true, true, true, false,
        ],
    ]
}

fn create_letter_o() -> Vec<Vec<bool>> {
    vec![
        vec![
            false, true, true, true, true, true, true, true, true, true, true, true, false,
        ],
        vec![
            true, true, true, true, true, true, true, true, true, true, true, true, true,
        ],
        vec![
            true, true, true, false, false, false, false, false, false, false, true, true, true,
        ],
        vec![
            true, true, true, false, false, false, false, false, false, false, true, true, true,
        ],
        vec![
            true, true, true, false, false, false, false, false, false, false, true, true, true,
        ],
        vec![
            true, true, true, true, true, true, true, true, true, true, true, true, true,
        ],
        vec![
            false, true, true, true, true, true, true, true, true, true, true, true, false,
        ],
    ]
}

fn create_letter_r() -> Vec<Vec<bool>> {
    vec![
        vec![
            true, true, true, true, true, true, true, true, true, true, false, false, false,
        ],
        vec![
            true, true, true, false, false, false, false, false, true, true, true, false, false,
        ],
        vec![
            true, true, true, false, false, false, false, false, true, true, true, false, false,
        ],
        vec![
            true, true, true, true, true, true, true, true, true, true, false, false, false,
        ],
        vec![
            true, true, true, false, true, true, true, false, false, false, false, false, false,
        ],
        vec![
            true, true, true, false, false, true, true, true, false, false, false, false, false,
        ],
        vec![
            true, true, true, false, false, false, true, true, true, false, false, false, false,
        ],
    ]
}

fn create_letter_t() -> Vec<Vec<bool>> {
    vec![
        vec![
            true, true, true, true, true, true, true, true, true, true, true, true, true,
        ],
        vec![
            true, true, true, true, true, true, true, true, true, true, true, true, true,
        ],
        vec![
            false, false, false, false, true, true, true, true, true, false, false, false, false,
        ],
        vec![
            false, false, false, false, true, true, true, true, true, false, false, false, false,
        ],
        vec![
            false, false, false, false, true, true, true, true, true, false, false, false, false,
        ],
        vec![
            false, false, false, false, true, true, true, true, true, false, false, false, false,
        ],
        vec![
            false, false, false, false, true, true, true, true, true, false, false, false, false,
        ],
    ]
}

fn create_letter_w() -> Vec<Vec<bool>> {
    vec![
        vec![
            true, true, true, false, false, false, false, false, false, false, true, true, true,
        ],
        vec![
            true, true, true, false, false, false, false, false, false, false, true, true, true,
        ],
        vec![
            true, true, true, false, false, false, false, false, false, false, true, true, true,
        ],
        vec![
            true, true, true, false, false, false, true, false, false, false, true, true, true,
        ],
        vec![
            true, true, true, false, false, true, true, true, false, false, true, true, true,
        ],
        vec![
            true, true, true, false, true, true, false, true, true, false, true, true, true,
        ],
        vec![
            false, true, true, true, true, false, false, false, true, true, true, true, false,
        ],
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    // Test that a shield is created with the correct properties
    #[test]
    fn test_shield_creation() {
        let shield = Shield::new(100.0, 200.0, ShieldType::UppercaseC);
        assert_eq!(shield.position.x, 100.0);
        assert_eq!(shield.position.y, 200.0);
        assert_eq!(shield.width, 100.0);
        assert_eq!(shield.height, 50.0);
        assert_eq!(shield.shield_type, ShieldType::UppercaseC);
        assert_eq!(shield.segments.len(), 7); // 7 rows
        assert_eq!(shield.segments[0].len(), 13); // 13 columns
    }

    // Test that is_hit correctly detects hits on active segments
    #[test]
    fn test_is_hit() {
        let mut shield = Shield::new(0.0, 0.0, ShieldType::UppercaseC);

        let hit_x = 15.0;
        let hit_y = 15.0;
        assert!(shield.is_hit(hit_x, hit_y));

        let segment_x = (hit_x / shield.width * shield.segments[0].len() as f32) as usize;
        let segment_y = (hit_y / shield.height * shield.segments.len() as f32) as usize;

        assert!(!shield.segments[segment_y][segment_x]);

        assert!(!shield.is_hit(150.0, 25.0));

        assert!(!shield.is_hit(hit_x, hit_y));
    }

    // Test that check_projectile_collision correctly detects collisions
    #[test]
    fn test_projectile_collision() {
        let mut shield = Shield::new(0.0, 0.0, ShieldType::UppercaseO);

        assert!(shield.check_projectile_collision(10.0, 10.0, 10.0, 10.0));

        assert!(shield.check_projectile_collision(90.0, 40.0, 20.0, 20.0));

        assert!(!shield.check_projectile_collision(110.0, 60.0, 10.0, 10.0));
    }

    // Test that different shield types create different segment patterns
    #[test]
    fn test_shield_types() {
        let c_segments = create_shield_segments(ShieldType::UppercaseC);
        let o_segments = create_shield_segments(ShieldType::UppercaseO);

        assert_ne!(c_segments, o_segments);

        assert!(!c_segments[2][12]);
        assert!(o_segments[2][12]);
    }
}
