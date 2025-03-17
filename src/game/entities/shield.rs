use crate::utils::position::Position;

#[derive(Clone, Copy, PartialEq)]
pub enum ShieldType {
    UppercaseC,
    UppercaseO,
    UppercaseR,
    UppercaseT,
    UppercaseW,
}

#[derive(Clone, PartialEq)]
pub struct Shield {
    pub position: Position,
    pub width: f32,
    pub height: f32,
    pub shield_type: ShieldType,
    pub segments: Vec<Vec<bool>>,
}

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
