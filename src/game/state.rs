use crate::game::entities::alien::AlienFormation;
use crate::game::entities::mystery_ship::MysteryShip;
use crate::game::entities::player::Player;
use crate::game::entities::projectile::Projectile;
use crate::game::entities::shield::Shield;
use crate::input::key_states::KeyStates;

#[derive(Default)]
pub struct GameState {
    pub score: i32,
    pub high_score: i32,
    pub game_over: bool,
    pub level: i32,
    pub lives: i32,
    pub player: Player,
    pub key_states: KeyStates,
    pub player_projectiles: Vec<Projectile>,
    pub alien_projectiles: Vec<Projectile>,
    pub player_shoot_cooldown: f64,
    pub alien_shoot_cooldown: f64,
    pub alien_formation: AlienFormation,
    pub shields: Vec<Shield>,
    #[allow(dead_code)]
    pub last_update: f64,
    pub dt: f32,
    pub invincibility_timer: f64,
    pub mystery_ship: Option<MysteryShip>,
    pub mystery_ship_timer: f32,
    pub screen: GameScreen,
}

#[derive(Clone, Copy, PartialEq)]
pub enum GameScreen {
    StartScreen,
    Playing,
    GameOver,
}

impl Default for GameScreen {
    fn default() -> Self {
        GameScreen::StartScreen
    }
}
