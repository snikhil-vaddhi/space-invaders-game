use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct ScoreBoardProps {
    pub score: i32,
    pub high_score: i32,
    pub lives: i32,
    pub level: i32,
}

#[component]
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
