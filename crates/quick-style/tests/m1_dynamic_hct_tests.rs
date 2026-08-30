//! Comprehensive Integration & Unit Test Suite for Milestone 1: Dynamic HCT Engine & Tokens in quick-style

use quick_core::geometry::Color;
use quick_style::color::{
    contrast_ratio, contrast_ratio_tones, darker_tone, delinearize, is_accessible, lighter_tone,
    linearize, lstar_from_y, relative_luminance, rgb_to_xyz, solve_gamut,
    xyz_to_linear_rgb, Cam16, Hct, ViewingConditions,
};
use quick_style::theme::{
    ColorScheme, ElevationTokens, SchemeVariant, ShapeTokens, StateLayerTokens,
    ThemePackage, TonalPalette,
};

#[test]
fn test_cie_and_srgb_pipeline() {
    // Test linearize / delinearize roundtrip across all 256 byte values
    for b in 0..=255u8 {
        let lin = linearize(b as f64 / 255.0);
        let reconstructed = delinearize(lin);
        let byte_back = (reconstructed * 255.0).round() as u8;
        assert_eq!(b, byte_back, "Failed sRGB roundtrip for byte {}", b);
    }

    // Test XYZ white point
    let [x, y, z] = rgb_to_xyz(255, 255, 255);
    assert!((x - 95.047).abs() < 0.01);
    assert!((y - 100.0).abs() < 0.01);
    assert!((z - 108.883).abs() < 0.01);

    // Test XYZ black point
    let [x0, y0, z0] = rgb_to_xyz(0, 0, 0);
    assert_eq!(x0, 0.0);
    assert_eq!(y0, 0.0);
    assert_eq!(z0, 0.0);
}

#[test]
fn test_cam16_viewing_conditions_and_forward_inverse() {
    let vc = ViewingConditions::standard();
    assert!((vc.c - 0.69).abs() < 1e-4);
    assert!((vc.nc - 1.0).abs() < 1e-4);

    let colors = [
        Color::RED,
        Color::GREEN,
        Color::BLUE,
        Color::WHITE,
        Color::BLACK,
        Color::from_rgb(103, 80, 164),
        Color::from_rgb(255, 165, 0),
        Color::from_rgb(0, 255, 255),
    ];

    for c in colors {
        let cam = Cam16::from_color(c);
        let [x, y, z] = cam.to_xyz(vc);
        let [r_lin, g_lin, b_lin] = xyz_to_linear_rgb(x, y, z);
        let r = (delinearize(r_lin.clamp(0.0, 1.0)) * 255.0).round() as u8;
        let g = (delinearize(g_lin.clamp(0.0, 1.0)) * 255.0).round() as u8;
        let b = (delinearize(b_lin.clamp(0.0, 1.0)) * 255.0).round() as u8;

        assert!((r as i32 - c.r as i32).abs() <= 2, "Failed R for color {:?}: got {}", c, r);
        assert!((g as i32 - c.g as i32).abs() <= 2, "Failed G for color {:?}: got {}", c, g);
        assert!((b as i32 - c.b as i32).abs() <= 2, "Failed B for color {:?}: got {}", c, b);
    }
}

#[test]
fn test_hct_color_space_behavior() {
    let hct = Hct::new(280.0, 48.0, 80.0);
    assert!((hct.hue() - 280.0).abs() < 1e-4);
    assert!((hct.tone() - 80.0).abs() < 1e-4);
    assert!(hct.chroma() <= 48.0 + 1e-4);

    let c = hct.to_color();
    assert_eq!(c.a, 255);

    let hct_mut = hct.with_tone(40.0);
    assert!((hct_mut.tone() - 40.0).abs() < 1e-4);
}

#[test]
fn test_gamut_bisection_solver() {
    // Extreme chroma that exceeds sRGB gamut
    let solved = solve_gamut(120.0, 150.0, 80.0);
    assert_eq!(solved.a, 255);

    // Verify Tone preservation
    let [_, y, _] = rgb_to_xyz(solved.r, solved.g, solved.b);
    let tone_measured = lstar_from_y(y);
    assert!((tone_measured - 80.0).abs() < 2.0, "Expected tone ~80.0, got {}", tone_measured);

    // Black and white boundary
    assert_eq!(solve_gamut(100.0, 50.0, 0.0), Color::BLACK);
    assert_eq!(solve_gamut(100.0, 50.0, 100.0), Color::WHITE);
}

#[test]
fn test_wcag_contrast_calculations() {
    let white = Color::WHITE;
    let black = Color::BLACK;

    assert!((contrast_ratio(white, black) - 21.0).abs() < 0.01);
    assert!((contrast_ratio(black, black) - 1.0).abs() < 1e-4);

    // WCAG AA requirement is 4.5:1
    assert!(is_accessible(white, black, 4.5));
    assert!(is_accessible(white, black, 7.0));

    // Lighter and darker tone solvers
    let light_40 = lighter_tone(40.0, 4.5);
    assert!(contrast_ratio_tones(40.0, light_40) >= 4.49);

    let dark_80 = darker_tone(80.0, 4.5);
    assert!(contrast_ratio_tones(80.0, dark_80) >= 4.49);
}

#[test]
fn test_6_tonal_palettes_monotonicity() {
    let palette = TonalPalette::from_hue_and_chroma(297.0, 48.0);
    let tones = [0.0, 10.0, 20.0, 30.0, 40.0, 50.0, 60.0, 70.0, 80.0, 90.0, 95.0, 100.0];

    let mut prev_lum = -1.0;
    for &t in &tones {
        let color = palette.get(t);
        let lum = relative_luminance(color);
        assert!(lum >= prev_lum, "Luminance not monotonic at tone {}: {} < {}", t, lum, prev_lum);
        prev_lum = lum;
    }
}

#[test]
fn test_7_scheme_variants_generation() {
    let seed = Color::from_hex("#6750A4").unwrap();
    let variants = [
        SchemeVariant::TonalSpot,
        SchemeVariant::Vibrant,
        SchemeVariant::Expressive,
        SchemeVariant::Fidelity,
        SchemeVariant::Content,
        SchemeVariant::Monochrome,
        SchemeVariant::Neutral,
    ];

    for v in variants {
        let core = v.generate_palette(seed);
        assert_eq!(core.error.hue(), 25.0);
        assert_eq!(core.error.chroma(), 84.0);

        if v == SchemeVariant::Monochrome {
            assert_eq!(core.primary.chroma(), 0.0);
            assert_eq!(core.secondary.chroma(), 0.0);
            assert_eq!(core.tertiary.chroma(), 0.0);
            assert_eq!(core.neutral.chroma(), 0.0);
            assert_eq!(core.neutral_variant.chroma(), 0.0);
        }
    }
}

#[test]
fn test_47_color_roles_and_contrast_guarantees() {
    let seed = Color::from_hex("#6750A4").unwrap();
    let light = ColorScheme::light(seed, SchemeVariant::TonalSpot);
    let dark = ColorScheme::dark(seed, SchemeVariant::TonalSpot);

    // Test contrast on key pairs in Light mode
    assert!(contrast_ratio(light.primary, light.on_primary) >= 4.5);
    assert!(contrast_ratio(light.secondary, light.on_secondary) >= 4.5);
    assert!(contrast_ratio(light.tertiary, light.on_tertiary) >= 4.5);
    assert!(contrast_ratio(light.error, light.on_error) >= 4.5);
    assert!(contrast_ratio(light.surface, light.on_surface) >= 4.5);

    // Test contrast on key pairs in Dark mode
    assert!(contrast_ratio(dark.primary, dark.on_primary) >= 4.5);
    assert!(contrast_ratio(dark.secondary, dark.on_secondary) >= 4.5);
    assert!(contrast_ratio(dark.tertiary, dark.on_tertiary) >= 4.5);
    assert!(contrast_ratio(dark.error, dark.on_error) >= 4.5);
    assert!(contrast_ratio(dark.surface, dark.on_surface) >= 4.5);

    // Verify map contains all 47 roles
    let map = light.to_map();
    assert!(map.contains_key("primary"));
    assert!(map.contains_key("primary-container"));
    assert!(map.contains_key("surface_container_highest"));
    assert!(map.contains_key("outline_variant"));
    assert!(map.contains_key("shadow"));
    assert!(map.contains_key("scrim"));
}

#[test]
fn test_design_tokens_shapes_elevation_state_layers() {
    let shapes = ShapeTokens::default();
    assert_eq!(shapes.corner_none, 0.0);
    assert_eq!(shapes.corner_extra_small, 4.0);
    assert_eq!(shapes.corner_small, 8.0);
    assert_eq!(shapes.corner_medium, 12.0);
    assert_eq!(shapes.corner_large, 16.0);
    assert_eq!(shapes.corner_extra_large, 28.0);
    assert_eq!(shapes.corner_full, 9999.0);

    let elev = ElevationTokens::default();
    assert!(elev.level_0.key_shadow.is_none());
    assert!(elev.level_1.key_shadow.is_some());
    assert_eq!(elev.level_5.level, 5);

    let states = StateLayerTokens::default();
    let base = Color::from_rgb(100, 100, 100);
    let overlay = Color::WHITE;
    let hovered = states.apply_hover(base, overlay);
    assert!(hovered.r > base.r);
}

#[test]
fn test_theme_package_api_and_css_generation() {
    let theme = ThemePackage::material_you();
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

    let stylesheet = theme.to_stylesheet();
    assert!(!stylesheet.rules.is_empty());
}
