use crate::utils::position::Position;

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

impl Player {
    pub fn move_left(&mut self, dt: f32, left_boundary: f32) {
        let new_x = self.position.x - self.speed * dt;
        self.position.x = new_x.max(left_boundary + self.width / 2.0);
    }

    pub fn move_right(&mut self, dt: f32, right_boundary: f32) {
        let new_x = self.position.x + self.speed * dt;
        self.position.x = new_x.min(right_boundary - self.width / 2.0);
    }

    pub fn get_projectile_spawn_position(&self) -> (f32, f32) {
        (
            self.position.x + self.projectile_spawn_x,
            self.position.y + self.projectile_spawn_y,
        )
    }
}
