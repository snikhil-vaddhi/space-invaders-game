use crate::game::entities::projectile::Projectile;
use dioxus::prelude::*;

#[derive(Props, PartialEq, Clone)]
pub struct ProjectileSpriteProps {
    pub projectile: Projectile,
}

#[component]
pub fn ProjectileSprite(props: ProjectileSpriteProps) -> Element {
    let projectile = &props.projectile;

    let class = if projectile.is_player_projectile {
        "projectile player-projectile"
    } else {
        "projectile alien-projectile"
    };

    rsx! {
        div {
            class: "{class}",
            style: "left: {projectile.position.x}px; top: {projectile.position.y}px; width: {projectile.width}px; height: {projectile.height}px;",
        }
    }
}
