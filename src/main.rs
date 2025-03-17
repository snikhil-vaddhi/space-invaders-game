use dioxus::prelude::*;
use dioxus_desktop::tao::dpi::LogicalSize;
use dioxus_desktop::tao::window::WindowBuilder;

use rendering::game::Game;

mod game;
mod input;
mod rendering;
mod utils;

const MAIN_CSS: Asset = asset!("/assets/styling/main.css");

fn main() {
    let window = WindowBuilder::new()
        .with_title("Space Invaders")
        .with_inner_size(LogicalSize::new(1024.0, 768.0))
        .with_resizable(false);

    dioxus::LaunchBuilder::new()
        .with_cfg(dioxus_desktop::Config::new().with_window(window))
        .launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: MAIN_CSS }

        Game {}
    }
}
