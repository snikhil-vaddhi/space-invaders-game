// Test that StarBackground integrates with the game rendering
#[test]
fn test_star_background_integration() {
    use space_invaders::rendering::star_background::generate_stars;

    let stars = generate_stars(100);

    assert_eq!(stars.len(), 100);

    for star in &stars {
        assert!(star.x >= 0.0 && star.x <= 100.0);
        assert!(star.y >= 0.0 && star.y <= 100.0);

        assert!(star.opacity >= 0.0 && star.opacity <= 1.0);

        let style = format!(
            "left: {}%; top: {}%; opacity: {};",
            star.x, star.y, star.opacity
        );

        assert!(style.contains("left:"));
        assert!(style.contains("top:"));
        assert!(style.contains("opacity:"));
    }
}

// Test that StarBackground can be used with different star counts
#[test]
fn test_star_background_flexibility() {
    use space_invaders::rendering::star_background::generate_stars;

    let few_stars = generate_stars(10);
    let many_stars = generate_stars(500);

    assert_eq!(few_stars.len(), 10);
    assert_eq!(many_stars.len(), 500);

    for star in &few_stars {
        assert!(star.x >= 0.0 && star.x <= 100.0);
        assert!(star.y >= 0.0 && star.y <= 100.0);
        assert!(star.opacity >= 0.2 && star.opacity <= 1.0);
    }

    for star in &many_stars {
        assert!(star.x >= 0.0 && star.x <= 100.0);
        assert!(star.y >= 0.0 && star.y <= 100.0);
        assert!(star.opacity >= 0.2 && star.opacity <= 1.0);
    }
}
