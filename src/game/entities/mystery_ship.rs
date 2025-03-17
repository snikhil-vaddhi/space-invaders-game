use crate::utils::position::Position;

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
