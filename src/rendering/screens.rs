use dioxus::prelude::*;

/// Displays the initial game screen with title and instructions
/// This component renders the start screen that appears when the game first loads,
/// showing the game title, instructions, and a prompt to start the game.

#[component]
#[allow(non_snake_case)]
pub fn StartScreen() -> Element {
    rsx! {
        div { id: "start-screen",
            div { class: "title", "SPACE INVADERS" }
            div { class: "subtitle", "CORTWO EDITION" }
            div { class: "instructions",
                p { "Use LEFT and RIGHT arrow keys to move" }
                p { "Press SHIFT to fire" }
                p { "Destroy all aliens to advance to the next level" }
                p { "Protect your ship and don't let aliens reach the bottom" }
            }
            div { class: "start-prompt", "Press ENTER to start" }
        }
    }
}

/// This component renders the start screen that appears when the game first loads,
/// showing the game title, instructions, and a prompt to start the game.

#[derive(Props, Clone, PartialEq, Debug)]
pub struct GameOverScreenProps {
    pub score: i32,
    pub high_score: i32,
}

/// Displays the game over screen with final score and restart prompt
#[component]
#[allow(non_snake_case)]
pub fn GameOverScreen(props: GameOverScreenProps) -> Element {
    rsx! {
        div { id: "game-over-screen",
            div { class: "game-over-title", "GAME OVER" }
            div { class: "final-score", "Your Score: {props.score}" }
            div { class: "high-score", "High Score: {props.high_score}" }
            div { class: "restart-prompt", "Press TAB to play again" }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Test that GameOverScreenProps correctly implements PartialEq
    #[test]
    fn test_game_over_screen_props_equality() {
        let props1 = GameOverScreenProps {
            score: 100,
            high_score: 500,
        };

        let props2 = GameOverScreenProps {
            score: 100,
            high_score: 500,
        };

        let props3 = GameOverScreenProps {
            score: 200,
            high_score: 500,
        };

        assert_eq!(props1, props2);
        assert_ne!(props1, props3);
    }

    // Test that GameOverScreenProps can be cloned
    #[test]
    fn test_game_over_screen_props_clone() {
        let props = GameOverScreenProps {
            score: 100,
            high_score: 500,
        };

        let cloned_props = props.clone();
        assert_eq!(props, cloned_props);
    }

    // Test that StartScreen component can be created
    #[test]
    fn test_start_screen_creation() {
        // This test simply verifies that the StartScreen component can be created
        // without panicking
        let _ = StartScreen {};

        // If we get here, the test passes
        assert!(true);
    }

    // Test that GameOverScreen component can be created with props
    #[test]
    fn test_game_over_screen_creation() {
        let props = GameOverScreenProps {
            score: 100,
            high_score: 500,
        };

        // This test verifies that the GameOverScreen component can be created
        // with props without panicking
        let _ = GameOverScreen(props);

        // If we get here, the test passes
        assert!(true);
    }
}
