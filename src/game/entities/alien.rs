use crate::utils::position::Position;

pub const GAME_WIDTH: f32 = 1024.0;

#[derive(Clone, Copy, PartialEq)]
pub enum AlienType {
    Small,
    Medium,
    Large,
}

impl AlienType {
    pub fn points(&self) -> i32 {
        match self {
            AlienType::Small => 30,
            AlienType::Medium => 20,
            AlienType::Large => 10,
        }
    }

    pub fn size(&self) -> (f32, f32) {
        match self {
            AlienType::Small => (30.0, 30.0),
            AlienType::Medium => (35.0, 30.0),
            AlienType::Large => (40.0, 30.0),
        }
    }
}

#[derive(Clone, PartialEq)]
pub struct Alien {
    pub position: Position,
    pub alien_type: AlienType,
    pub width: f32,
    pub height: f32,
    pub is_alive: bool,
    pub animation_frame: usize,
}

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

    pub fn count_living(&self) -> usize {
        self.aliens.iter().filter(|a| a.is_alive).count()
    }

    pub fn get_lowest_y(&self) -> f32 {
        self.aliens
            .iter()
            .filter(|a| a.is_alive)
            .map(|a| a.position.y + a.height)
            .max_by(|a, b| a.partial_cmp(b).unwrap())
            .unwrap_or(0.0)
    }
}
