use crate::game::entities::alien::AlienFormation;
use crate::game::entities::mystery_ship::MysteryShip;
use crate::game::entities::player::Player;
use crate::game::entities::projectile::Projectile;
use crate::game::entities::shield::Shield;
use crate::input::key_states::KeyStates;

// Represents the complete state of the game at any point in time
///
/// This struct contains score, player status, entities, and game progression information.
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

/// Represents the different screens in the game
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum GameScreen {
    StartScreen,
    Playing,
    GameOver,
}

/// Returns the default screen (StartScreen)
impl Default for GameScreen {
    fn default() -> Self {
        GameScreen::StartScreen
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Test that a new GameState is created with default values
    #[test]
    fn test_game_state_default() {
        let state = GameState::default();
        assert_eq!(state.score, 0);
        assert_eq!(state.high_score, 0);
        assert!(!state.game_over);
        assert_eq!(state.level, 0);
        assert_eq!(state.lives, 0);
        assert_eq!(state.player_projectiles.len(), 0);
        assert_eq!(state.alien_projectiles.len(), 0);
        assert_eq!(state.player_shoot_cooldown, 0.0);
        assert_eq!(state.alien_shoot_cooldown, 0.0);
        assert_eq!(state.shields.len(), 0);
        assert_eq!(state.invincibility_timer, 0.0);
        assert!(state.mystery_ship.is_none());
        assert_eq!(state.screen, GameScreen::StartScreen);
    }

    // Test that GameScreen default is StartScreen
    #[test]
    fn test_game_screen_default() {
        let screen = GameScreen::default();
        assert_eq!(screen, GameScreen::StartScreen);
    }

    // Test GameScreen equality
    #[test]
    fn test_game_screen_equality() {
        assert_eq!(GameScreen::StartScreen, GameScreen::StartScreen);
        assert_ne!(GameScreen::StartScreen, GameScreen::Playing);
        assert_ne!(GameScreen::StartScreen, GameScreen::GameOver);
        assert_ne!(GameScreen::Playing, GameScreen::GameOver);
    }
}
