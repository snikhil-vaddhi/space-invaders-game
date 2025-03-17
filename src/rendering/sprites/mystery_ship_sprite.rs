use crate::game::entities::mystery_ship::MysteryShip;
use dioxus::prelude::*;

#[derive(Props, PartialEq, Clone)]
pub struct MysteryShipSpriteProps {
    pub ship: MysteryShip,
}

#[component]
pub fn MysteryShipSprite(props: MysteryShipSpriteProps) -> Element {
    let ship = &props.ship;

    if !ship.active {
        return rsx! {
            div {}
        };
    }

    rsx! {
        div {
            class: "mystery-ship",
            style: "left: {ship.position.x}px; top: {ship.position.y}px; width: {ship.width}px; height: {ship.height}px;",
        }
    }
}
