use crate::game::entities::player::Player;
use dioxus::prelude::*;

#[derive(Props, PartialEq, Clone)]
pub struct PlayerShipProps {
    pub player: Player,
}

#[component]
pub fn PlayerShip(props: PlayerShipProps) -> Element {
    let player = &props.player;
    let hit_class = if player.is_hit { "player-hit" } else { "" };

    rsx! {
        div {
            class: "player {hit_class}",
            style: "left: {player.position.x - player.width/2.0}px;
                   top: {player.position.y}px; 
                   width: {player.width}px; 
                   height: {player.height}px;",
            div { class: "player-cannon" }
            div { class: "player-body" }
        }
    }
}
