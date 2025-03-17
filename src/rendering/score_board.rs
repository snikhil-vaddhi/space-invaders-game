use dioxus::prelude::*;

/// A component that displays the game's score information
/// This component renders the player's current score, lives remaining,
/// high score, and current level in a formatted display at the top of the
#[derive(Props, Clone, PartialEq, Debug)]
pub struct ScoreBoardProps {
    pub score: i32,
    pub high_score: i32,
    pub lives: i32,
    pub level: i32,
}

/// Renders the game's score information
#[component]
#[allow(non_snake_case)]
pub fn ScoreBoard(props: ScoreBoardProps) -> Element {
    rsx! {
        div { id: "score-board",
            div { class: "score", "Score: {props.score}" }
            div { class: "lives", "Lives: {props.lives}" }
            div { class: "high-score", "High Score: {props.high_score}" }
            div { class: "level", "Level: {props.level}" }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Test that ScoreBoardProps correctly implements PartialEq
    #[test]
    fn test_score_board_props_equality() {
        let props1 = ScoreBoardProps {
            score: 100,
            high_score: 500,
            lives: 3,
            level: 1,
        };

        let props2 = ScoreBoardProps {
            score: 100,
            high_score: 500,
            lives: 3,
            level: 1,
        };

        let props3 = ScoreBoardProps {
            score: 200,
            high_score: 500,
            lives: 3,
            level: 1,
        };

        assert_eq!(props1, props2);
        assert_ne!(props1, props3);
    }

    // Test that ScoreBoardProps can be cloned
    #[test]
    fn test_score_board_props_clone() {
        let props = ScoreBoardProps {
            score: 100,
            high_score: 500,
            lives: 3,
            level: 1,
        };

        let cloned_props = props.clone();
        assert_eq!(props, cloned_props);
    }
}
