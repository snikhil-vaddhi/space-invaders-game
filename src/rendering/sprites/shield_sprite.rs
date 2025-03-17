use crate::game::entities::shield::Shield;
use dioxus::prelude::*;

#[derive(Props, PartialEq, Clone)]
pub struct ShieldSpriteProps {
    pub shield: Shield,
}

#[component]
pub fn ShieldSprite(props: ShieldSpriteProps) -> Element {
    let shield = &props.shield;

    let shield_segments = shield.segments.iter().enumerate().map(|(y, row)| {
        let row_segments = row.iter().enumerate().map(|(x, &active)| {
            rsx! {
                div {
                    class: if active { "shield-segment active" } else { "shield-segment" },
                    key: "{x}-{y}",
                }
            }
        });

        rsx! {
            div { class: "shield-row", {row_segments} }
        }
    });

    rsx! {
        div {
            class: "shield",
            style: "left: {shield.position.x}px; top: {shield.position.y}px; width: {shield.width}px; height: {shield.height}px;",
            {shield_segments}
        }
    }
}
