//! Comprehensive Adversarial & Empirical Test Suite for Milestone 1
//! Dynamic HCT Engine & Tokens (`quick-style`)

use quick_core::geometry::Color;
use quick_style::color::{
    contrast_ratio, contrast_ratio_tones, delinearize,
    linearize, lstar_from_y, rgb_to_xyz,
    y_from_lstar, Hct,
};
use quick_style::theme::{
    ColorScheme, ElevationTokens, SchemeVariant, ShapeTokens, StateLayerTokens,
    ThemePackage,
};
use std::str::FromStr;

#[test]
fn test_adversarial_cie_pipeline_extremes() {
    // 1. Black, White, and Grayscale points
    assert_eq!(rgb_to_xyz(0, 0, 0), [0.0, 0.0, 0.0]);
    let [xw, yw, zw] = rgb_to_xyz(255, 255, 255);
    assert!((xw - 95.047).abs() < 1e-2);
    assert!((yw - 100.0).abs() < 1e-2);
    assert!((zw - 108.883).abs() < 1e-2);

    // 2. Linearization bounds and monotonicity
    let mut prev_lin = -1.0;
    for b in 0..=255u8 {
        let norm = b as f64 / 255.0;
        let lin = linearize(norm);
        assert!(lin >= prev_lin, "Linearization not monotonic at {}", b);
        assert!(lin >= 0.0 && lin <= 1.0);
        let delin = delinearize(lin);
        assert!((delin - norm).abs() < 1e-5, "Delinearize roundtrip failed at {}", b);
        prev_lin = lin;
    }

    // 3. L* and Y roundtrip bounds
    assert_eq!(y_from_lstar(0.0), 0.0);
    assert!((y_from_lstar(100.0) - 100.0).abs() < 1e-4);
    assert_eq!(lstar_from_y(0.0), 0.0);
    assert!((lstar_from_y(100.0) - 100.0).abs() < 1e-4);

    let mut prev_y = -1.0;
    for t in 0..=1000 {
        let tone = t as f64 * 0.1;
        let y = y_from_lstar(tone);
        assert!(y >= prev_y, "y_from_lstar not monotonic at {}", tone);
        let tone_back = lstar_from_y(y);
        assert!((tone_back - tone).abs() < 1e-4, "L*/Y roundtrip failed at {}", tone);
        prev_y = y;
    }
}

#[test]
fn test_adversarial_hct_extremes_and_boundary_handling() {
    // 1. Negative Chroma
    let hct_neg_c = Hct::new(180.0, -100.0, 50.0);
    assert_eq!(hct_neg_c.chroma(), 0.0);
    assert_eq!(hct_neg_c.tone(), 50.0);
    assert_eq!(hct_neg_c.hue(), 180.0);

    // 2. Out of bounds tone clamping
    let hct_neg_t = Hct::new(180.0, 40.0, -50.0);
    assert_eq!(hct_neg_t.tone(), 0.0);
    assert_eq!(hct_neg_t.to_color(), Color::from_rgb(0, 0, 0));

    let hct_huge_t = Hct::new(180.0, 40.0, 150.0);
    assert_eq!(hct_huge_t.tone(), 100.0);
    assert_eq!(hct_huge_t.to_color(), Color::from_rgb(255, 255, 255));

    // 3. Hue normalization across positive & negative wraps
    for (raw_h, expected_h) in [
        (-720.0, 0.0),
        (-360.0, 0.0),
        (-270.0, 90.0),
        (-180.0, 180.0),
        (-90.0, 270.0),
        (0.0, 0.0),
        (360.0, 0.0),
        (450.0, 90.0),
        (720.0, 0.0),
        (1080.0, 0.0),
    ] {
        let hct = Hct::new(raw_h, 30.0, 50.0);
        assert!(
            (hct.hue() - expected_h).abs() < 1e-4,
            "Hue {} expected {}, got {}",
            raw_h,
            expected_h,
            hct.hue()
        );
    }

    // 4. Mutator methods
    let mut hct = Hct::new(100.0, 30.0, 40.0);
    hct.set_hue(200.0);
    assert_eq!(hct.hue(), 200.0);
    hct.set_chroma(60.0);
    assert_eq!(hct.chroma(), 60.0);
    hct.set_tone(80.0);
    assert_eq!(hct.tone(), 80.0);

    let hct2 = hct.with_hue(300.0).with_chroma(10.0).with_tone(20.0);
    assert_eq!(hct2.hue(), 300.0);
    assert_eq!(hct2.chroma(), 10.0);
    assert_eq!(hct2.tone(), 20.0);
}

#[test]
fn test_adversarial_special_colors_fidelity() {
    // Pure Black
    let black = Color::BLACK;
    let hct_b = Hct::from_color(black);
    assert_eq!(hct_b.tone(), 0.0);
    assert_eq!(hct_b.to_color(), black);
    assert_eq!(hct_b.to_argb_u32(), 0xFF000000);

    // Pure White
    let white = Color::WHITE;
    let hct_w = Hct::from_color(white);
    assert!((hct_w.tone() - 100.0).abs() < 1e-3);
    assert_eq!(hct_w.to_color(), white);
    assert_eq!(hct_w.to_argb_u32(), 0xFFFFFFFF);

    // Mid Gray
    let gray = Color::from_rgb(128, 128, 128);
    let hct_g = Hct::from_color(gray);
    assert!((hct_g.tone() - 53.58).abs() < 1.0);
    let g_rec = hct_g.to_color();
    assert_eq!(g_rec.r, 128);
    assert_eq!(g_rec.g, 128);
    assert_eq!(g_rec.b, 128);
}

#[test]
fn test_adversarial_scheme_variants_parsing_and_contract() {
    // 1. FromStr and Display roundtrips
    let variants = [
        (SchemeVariant::TonalSpot, "tonal_spot"),
        (SchemeVariant::Vibrant, "vibrant"),
        (SchemeVariant::Expressive, "expressive"),
        (SchemeVariant::Fidelity, "fidelity"),
        (SchemeVariant::Content, "content"),
        (SchemeVariant::Monochrome, "monochrome"),
        (SchemeVariant::Neutral, "neutral"),
    ];

    for (v, s) in variants {
        assert_eq!(v.to_string(), s);
        assert_eq!(SchemeVariant::from_str(s).unwrap(), v);
        assert_eq!(SchemeVariant::from_str(&s.replace('_', "-")).unwrap(), v);
        assert_eq!(SchemeVariant::from_str(&s.to_uppercase()).unwrap(), v);
    }

    assert!(SchemeVariant::from_str("invalid_variant_name").is_err());

    // 2. Default variant is TonalSpot
    assert_eq!(SchemeVariant::default(), SchemeVariant::TonalSpot);
}

#[test]
fn test_adversarial_color_scheme_contrast_and_accessibility_matrix() {
    let seeds = [
        Color::from_hex("#6750A4").unwrap(),
        Color::from_hex("#386A20").unwrap(),
        Color::from_hex("#006874").unwrap(),
        Color::from_hex("#9C4146").unwrap(),
        Color::from_hex("#7D5260").unwrap(),
        Color::RED,
        Color::GREEN,
        Color::BLUE,
        Color::from_rgb(255, 255, 0),
        Color::from_rgb(0, 255, 255),
        Color::from_rgb(255, 0, 255),
        Color::BLACK,
        Color::WHITE,
        Color::from_rgb(128, 128, 128),
    ];

    let variants = [
        SchemeVariant::TonalSpot,
        SchemeVariant::Vibrant,
        SchemeVariant::Expressive,
        SchemeVariant::Fidelity,
        SchemeVariant::Content,
        SchemeVariant::Monochrome,
        SchemeVariant::Neutral,
    ];

    for &seed in &seeds {
        for &variant in &variants {
            let light = ColorScheme::light(seed, variant);
            let dark = ColorScheme::dark(seed, variant);

            // Check all 47 roles exist and have alpha 255
            assert_eq!(light.iter().count(), 49);
            assert_eq!(dark.iter().count(), 49);

            for (role_name, col) in light.iter() {
                assert_eq!(col.a, 255, "Alpha not 255 for light role {}", role_name);
                assert!(light.get_by_name(role_name).is_some());
                assert!(light.get_by_name(&role_name.replace('_', "-")).is_some());
            }

            // Light Mode WCAG AA Invariants (CR >= 4.5:1)
            assert!(
                contrast_ratio(light.primary, light.on_primary) >= 4.5,
                "Light primary contrast failed for seed {:?} variant {:?}",
                seed,
                variant
            );
            assert!(
                contrast_ratio(light.secondary, light.on_secondary) >= 4.5,
                "Light secondary contrast failed for seed {:?} variant {:?}",
                seed,
                variant
            );
            assert!(
                contrast_ratio(light.tertiary, light.on_tertiary) >= 4.5,
                "Light tertiary contrast failed for seed {:?} variant {:?}",
                seed,
                variant
            );
            assert!(
                contrast_ratio(light.error, light.on_error) >= 4.5,
                "Light error contrast failed for seed {:?} variant {:?}",
                seed,
                variant
            );
            assert!(
                contrast_ratio(light.surface, light.on_surface) >= 4.5,
                "Light surface contrast failed for seed {:?} variant {:?}",
                seed,
                variant
            );

            // Dark Mode WCAG AA Invariants (CR >= 4.5:1)
            assert!(
                contrast_ratio(dark.primary, dark.on_primary) >= 4.5,
                "Dark primary contrast failed for seed {:?} variant {:?}",
                seed,
                variant
            );
            assert!(
                contrast_ratio(dark.secondary, dark.on_secondary) >= 4.5,
                "Dark secondary contrast failed for seed {:?} variant {:?}",
                seed,
                variant
            );
            assert!(
                contrast_ratio(dark.tertiary, dark.on_tertiary) >= 4.5,
                "Dark tertiary contrast failed for seed {:?} variant {:?}",
                seed,
                variant
            );
            assert!(
                contrast_ratio(dark.error, dark.on_error) >= 4.5,
                "Dark error contrast failed for seed {:?} variant {:?}",
                seed,
                variant
            );
            assert!(
                contrast_ratio(dark.surface, dark.on_surface) >= 4.5,
                "Dark surface contrast failed for seed {:?} variant {:?}",
                seed,
                variant
            );
        }
    }
}

#[test]
fn test_adversarial_contrast_ratio_monotonicity() {
    // Monotonicity of contrast ratio with respect to tone distance
    for base_tone in [0.0, 20.0, 40.0, 50.0, 60.0, 80.0, 100.0] {
        let mut prev_cr = 1.0;
        for delta in 1..=50 {
            let t_above = (base_tone + delta as f64).min(100.0);
            let cr_above = contrast_ratio_tones(base_tone, t_above);
            assert!(cr_above >= prev_cr);
            prev_cr = cr_above;
        }
    }

    // Inverse and symmetry
    for t1 in [0.0, 10.0, 30.0, 50.0, 70.0, 90.0, 100.0] {
        for t2 in [0.0, 10.0, 30.0, 50.0, 70.0, 90.0, 100.0] {
            assert_eq!(contrast_ratio_tones(t1, t2), contrast_ratio_tones(t2, t1));
        }
    }
}

#[test]
fn test_adversarial_design_tokens_and_theme_package() {
    // 1. Shapes
    let shapes = ShapeTokens::default();
    assert_eq!(shapes.corner_none, 0.0);
    assert_eq!(shapes.corner_extra_small, 4.0);
    assert_eq!(shapes.corner_small, 8.0);
    assert_eq!(shapes.corner_medium, 12.0);
    assert_eq!(shapes.corner_large, 16.0);
    assert_eq!(shapes.corner_extra_large, 28.0);
    assert_eq!(shapes.corner_full, 9999.0);

    // 2. Elevation Levels 0..5
    let elev = ElevationTokens::default();
    assert_eq!(elev.level_0.level, 0);
    assert!(elev.level_0.key_shadow.is_none());
    assert_eq!(elev.level_0.surface_tint_opacity, 0.0);

    assert_eq!(elev.level_1.level, 1);
    assert!(elev.level_1.key_shadow.is_some());
    assert!((elev.level_1.surface_tint_opacity - 0.05).abs() < 1e-4);

    assert_eq!(elev.level_2.level, 2);
    assert!((elev.level_2.surface_tint_opacity - 0.08).abs() < 1e-4);

    assert_eq!(elev.level_3.level, 3);
    assert!((elev.level_3.surface_tint_opacity - 0.11).abs() < 1e-4);

    assert_eq!(elev.level_4.level, 4);
    assert!((elev.level_4.surface_tint_opacity - 0.12).abs() < 1e-4);

    assert_eq!(elev.level_5.level, 5);
    assert!((elev.level_5.surface_tint_opacity - 0.14).abs() < 1e-4);

    // 3. State Layer Opacities
    let state = StateLayerTokens::default();
    assert_eq!(state.hover, 0.08);
    assert_eq!(state.focus, 0.12);
    assert_eq!(state.pressed, 0.12);
    assert_eq!(state.dragged, 0.16);
    assert_eq!(state.disabled_content, 0.38);
    assert_eq!(state.disabled_container, 0.12);

    let base = Color::from_rgb(50, 50, 50);
    let on_surf = Color::WHITE;
    let hovered = state.apply_hover(base, on_surf);
    assert!(hovered.r > base.r);

    let pressed = state.apply_pressed(base, on_surf);
    assert!(pressed.r > hovered.r);

    // 4. Dynamic ThemePackage and CSS generation
    let theme = ThemePackage::from_seed_color(Color::from_hex("#6750A4").unwrap(), SchemeVariant::Vibrant, true);
    assert_eq!(theme.name, "material-you");
    assert!(theme.is_dark);

    let css = theme.generate_css();
    assert!(css.contains("Button"));
    assert!(css.contains("Card"));
    assert!(css.contains("Switch"));
    assert!(css.contains("Checkbox"));
    assert!(css.contains("Slider"));
    assert!(css.contains("Chip"));
    assert!(css.contains("ProgressBar"));
    assert!(css.contains("TextInput"));

    // Verify stylesheet parser parses the generated CSS successfully
    let sheet = theme.to_stylesheet();
    assert!(!sheet.rules.is_empty());
}
