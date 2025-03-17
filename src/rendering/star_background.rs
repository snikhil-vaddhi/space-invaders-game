use dioxus::prelude::*;
use rand::Rng;

/// Represents a single star in the background starfield

#[derive(Clone)]
pub struct Star {
    pub x: f32,
    pub y: f32,
    pub opacity: f32,
}

/// Generates a collection of randomly positioned stars
/// # Arguments
/// * `count` - The number of stars to generate
/// # Returns -> A vector containing the specified number of randomly positioned stars

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

/// Renders a starry background for the game

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

#[cfg(test)]
mod tests {
    use super::*;

    // Test that stars are generated with the correct count
    #[test]
    fn test_generate_stars_count() {
        let stars = generate_stars(50);
        assert_eq!(stars.len(), 50);

        let stars = generate_stars(100);
        assert_eq!(stars.len(), 100);
    }

    // Test that generated stars have valid properties
    #[test]
    fn test_star_properties() {
        let stars = generate_stars(20);

        for star in stars {
            assert!(star.x >= 0.0 && star.x <= 100.0);
            assert!(star.y >= 0.0 && star.y <= 100.0);

            assert!(star.opacity >= 0.2 && star.opacity <= 1.0);
        }
    }

    // Test that Star can be cloned
    #[test]
    fn test_star_clone() {
        let star = Star {
            x: 50.0,
            y: 75.0,
            opacity: 0.5,
        };

        let cloned_star = star.clone();

        assert_eq!(star.x, cloned_star.x);
        assert_eq!(star.y, cloned_star.y);
        assert_eq!(star.opacity, cloned_star.opacity);
    }

    // Test that stars are randomly distributed
    #[test]
    fn test_stars_randomness() {
        let stars1 = generate_stars(100);
        let stars2 = generate_stars(100);

        let mut all_identical = true;

        for i in 0..stars1.len() {
            if stars1[i].x != stars2[i].x
                || stars1[i].y != stars2[i].y
                || stars1[i].opacity != stars2[i].opacity
            {
                all_identical = false;
                break;
            }
        }

        assert!(!all_identical);
    }
}
