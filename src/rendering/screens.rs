use dioxus::prelude::*;

#[component]
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

#[derive(Props, Clone, PartialEq)]
pub struct GameOverScreenProps {
    pub score: i32,
    pub high_score: i32,
}

#[component]
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
