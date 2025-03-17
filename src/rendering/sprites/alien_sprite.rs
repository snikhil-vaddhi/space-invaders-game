use crate::game::entities::alien::Alien;
use dioxus::prelude::*;

#[derive(Props, PartialEq, Clone)]
pub struct AlienSpriteProps {
    pub alien: Alien,
}

#[component]
pub fn AlienSprite(props: AlienSpriteProps) -> Element {
    let alien = &props.alien;

    if !alien.is_alive {
        return rsx! {
            div {}
        };
    }

    let alien_class = match alien.alien_type {
        crate::game::entities::alien::AlienType::Small => "alien-small",
        crate::game::entities::alien::AlienType::Medium => "alien-medium",
        crate::game::entities::alien::AlienType::Large => "alien-large",
    };

    let frame_class = if alien.animation_frame % 2 == 0 {
        "frame-1"
    } else {
        "frame-2"
    };

    rsx! {
        div {
            class: "alien {alien_class} {frame_class}",
            style: "left: {alien.position.x}px; top: {alien.position.y}px; width: {alien.width}px; height: {alien.height}px;",
        }
    }
}
