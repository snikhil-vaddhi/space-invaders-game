use crate::game::entities::alien::Alien;
use crate::game::entities::player::Player;
use crate::utils::position::Position;

#[derive(Clone, PartialEq)]
pub struct Projectile {
    pub position: Position,
    pub velocity: f32,
    pub width: f32,
    pub height: f32,
    pub is_player_projectile: bool,
}

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

    pub fn player(player: &Player) -> Self {
        let (x, y) = player.get_projectile_spawn_position();
        Self::new(x, y, true)
    }

    pub fn alien(alien: &Alien) -> Self {
        let x = alien.position.x + alien.width / 2.0 - 1.5;
        let y = alien.position.y + alien.height;
        Self::new(x, y, false)
    }

    pub fn update(&mut self, dt: f32) {
        self.position.y += self.velocity * dt;
    }

    pub fn is_off_screen(&self, screen_height: f32) -> bool {
        (self.is_player_projectile && self.position.y < -self.height)
            || (!self.is_player_projectile && self.position.y > screen_height)
    }
}
