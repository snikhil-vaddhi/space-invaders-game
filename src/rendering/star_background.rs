use dioxus::prelude::*;
use rand::Rng;

#[derive(Clone)]
pub struct Star {
    pub x: f32,
    pub y: f32,
    pub opacity: f32,
}

pub fn generate_stars(count: usize) -> Vec<Star> {
    let mut rng = rand::thread_rng();
    let mut stars = Vec::with_capacity(count);
    for _ in 0..count {
        stars.push(Star {
            x: rng.gen::<f32>() * 100.0,
            y: rng.gen::<f32>() * 100.0,
            opacity: rng.gen::<f32>() * 0.8 + 0.2,
        });
    }
    stars
}

#[allow(non_snake_case)]
pub fn StarBackground() -> Element {
    let stars = use_signal(|| generate_stars(100));

    rsx! {
        div { id: "starfield",
            for star in stars().iter() {
                div {
                    class: "star",
                    style: "left: {star.x}%; top: {star.y}%; opacity: {star.opacity};",
                }
            }
        }
    }
}
