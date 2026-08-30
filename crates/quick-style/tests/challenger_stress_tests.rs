//! Adversarial Challenger Stress Test Suite for Milestone 1
//! Dynamic HCT Engine, Scheme Variants, Color Roles, Tokens, and CSS Generator.

use quick_core::geometry::Color;
use quick_style::color::{contrast_ratio, relative_luminance};
use quick_style::parser::parse_stylesheet;
use quick_style::theme::{
    ColorScheme, ElevationTokens, MotionTokens, SchemeVariant, ShapeTokens,
    StateLayerTokens, ThemePackage,
};
use std::str::FromStr;

// ============================================================================
// ADVERSARIAL SEED COLOR DATASET
// ============================================================================

const ADVERSARIAL_SEEDS: &[(&str, &str)] = &[
    // Vibrant Reds
    ("vibrant_red_pure", "#FF0000"),
    ("vibrant_red_material", "#E53935"),
    ("dark_red", "#B71C1C"),
    // Muted Pastels
    ("pastel_blue", "#B0C4DE"),
    ("pastel_pink", "#F4C2C2"),
    ("pastel_warm", "#E8D5C4"),
    ("pastel_lavender", "#D8BFD8"),
    // Monochrome Grays
    ("mid_gray", "#808080"),
    ("pure_black", "#000000"),
    ("pure_white", "#FFFFFF"),
    ("dark_gray", "#1E1E1E"),
    ("light_gray", "#E0E0E0"),
    // Cyans
    ("pure_cyan", "#00FFFF"),
    ("teal_cyan", "#00BCD4"),
    ("deep_cyan", "#006064"),
    ("pale_cyan", "#80DEEA"),
    // Golds & Yellows
    ("pure_gold", "#FFD700"),
    ("amber_gold", "#FFC107"),
    ("deep_orange_gold", "#FF6F00"),
    ("light_yellow", "#FFF9C4"),
    // Edge Cases & Primaries
    ("pure_green", "#00FF00"),
    ("pure_blue", "#0000FF"),
    ("m3_purple_default", "#6750A4"),
    ("pure_magenta", "#FF00FF"),
    ("deep_orange", "#FF5722"),
    ("near_black", "#010101"),
    ("near_white", "#FEFEFE"),
];

const ALL_VARIANTS: &[SchemeVariant] = &[
    SchemeVariant::TonalSpot,
    SchemeVariant::Vibrant,
    SchemeVariant::Expressive,
    SchemeVariant::Fidelity,
    SchemeVariant::Content,
    SchemeVariant::Monochrome,
    SchemeVariant::Neutral,
];

// ============================================================================
// 1. SCHEME VARIANTS & TONAL PALETTES STRESS TESTS
// ============================================================================

#[test]
fn test_adversarial_all_variants_across_all_seeds() {
    for &(name, hex) in ADVERSARIAL_SEEDS {
        let seed = Color::from_hex(hex).unwrap_or_else(|e| panic!("Failed to parse hex {}: {}", hex, e));

        for &variant in ALL_VARIANTS {
            let core_palette = variant.generate_palette(seed);

            // Error palette must be invariant: Hue 25.0, Chroma 84.0
            assert_eq!(core_palette.error.hue(), 25.0, "Variant {:?} seed {} error hue mismatch", variant, name);
            assert_eq!(core_palette.error.chroma(), 84.0, "Variant {:?} seed {} error chroma mismatch", variant, name);

            // Monochrome rule: all non-error palettes must have Chroma 0.0
            if variant == SchemeVariant::Monochrome {
                assert_eq!(core_palette.primary.chroma(), 0.0, "Monochrome primary chroma != 0 for {}", name);
                assert_eq!(core_palette.secondary.chroma(), 0.0, "Monochrome secondary chroma != 0 for {}", name);
                assert_eq!(core_palette.tertiary.chroma(), 0.0, "Monochrome tertiary chroma != 0 for {}", name);
                assert_eq!(core_palette.neutral.chroma(), 0.0, "Monochrome neutral chroma != 0 for {}", name);
                assert_eq!(core_palette.neutral_variant.chroma(), 0.0, "Monochrome neutral_variant chroma != 0 for {}", name);
            }

            // Neutral rule: specific chroma values
            if variant == SchemeVariant::Neutral {
                assert_eq!(core_palette.primary.chroma(), 12.0);
                assert_eq!(core_palette.secondary.chroma(), 8.0);
                assert_eq!(core_palette.tertiary.chroma(), 16.0);
                assert_eq!(core_palette.neutral.chroma(), 2.0);
                assert_eq!(core_palette.neutral_variant.chroma(), 2.0);
            }

            // Test monotonicity of tone sampling on all 6 palettes
            let palettes = [
                ("primary", &core_palette.primary),
                ("secondary", &core_palette.secondary),
                ("tertiary", &core_palette.tertiary),
                ("neutral", &core_palette.neutral),
                ("neutral_variant", &core_palette.neutral_variant),
                ("error", &core_palette.error),
            ];

            let sample_tones = [0.0, 10.0, 20.0, 30.0, 40.0, 50.0, 60.0, 70.0, 80.0, 90.0, 95.0, 100.0];
            for (p_name, palette) in palettes {
                let mut prev_lum = -0.001;
                for &t in &sample_tones {
                    let color = palette.get(t);
                    assert_eq!(color.a, 255, "Alpha not 255 for {} palette in {:?} for seed {}", p_name, variant, name);

                    let lum = relative_luminance(color);
                    assert!(
                        lum >= prev_lum - 1e-4,
                        "Luminance non-monotonic in {} palette ({:?}, seed {}): tone {} lum {} < prev {}",
                        p_name, variant, name, t, lum, prev_lum
                    );
                    prev_lum = lum;
                }

                // Tone 0 must be pure black
                assert_eq!(palette.get(0.0), Color::BLACK, "Tone 0 not black in {} ({:?}, seed {})", p_name, variant, name);
                // Tone 100 must be pure white
                assert_eq!(palette.get(100.0), Color::WHITE, "Tone 100 not white in {} ({:?}, seed {})", p_name, variant, name);
            }
        }
    }
}

// ============================================================================
// 2. 47+ M3 COLOR ROLES & CONTRAST GUARANTEES IN LIGHT & DARK MODES
// ============================================================================

#[test]
fn test_adversarial_47_color_roles_and_contrast_across_all_seeds_and_schemes() {
    for &(name, hex) in ADVERSARIAL_SEEDS {
        let seed = Color::from_hex(hex).unwrap();

        for &variant in ALL_VARIANTS {
            for &is_dark in &[false, true] {
                let scheme = if is_dark {
                    ColorScheme::dark(seed, variant)
                } else {
                    ColorScheme::light(seed, variant)
                };

                // 1. Verify all 47+ roles are present in iteration
                let role_count = scheme.iter().count();
                assert!(role_count >= 47, "Expected at least 47 roles, got {} for seed {} ({:?}, is_dark={})", role_count, name, variant, is_dark);

                // 2. Verify Map generation has both snake_case and kebab-case keys
                let map = scheme.to_map();
                assert!(map.len() >= 47, "Map contains fewer than 47 entries");
                assert!(map.contains_key("primary"));
                assert!(map.contains_key("on_primary"));
                assert!(map.contains_key("on-primary"));
                assert!(map.contains_key("primary_container"));
                assert!(map.contains_key("primary-container"));
                assert!(map.contains_key("surface_container_highest"));
                assert!(map.contains_key("surface-container-highest"));
                assert!(map.contains_key("outline_variant"));
                assert!(map.contains_key("outline-variant"));
                assert!(map.contains_key("shadow"));
                assert!(map.contains_key("scrim"));

                // 3. Verify get_by_name lookup works for both snake_case and kebab-case
                assert_eq!(scheme.get_by_name("primary"), Some(scheme.primary));
                assert_eq!(scheme.get_by_name("on_primary"), Some(scheme.on_primary));
                assert_eq!(scheme.get_by_name("on-primary"), Some(scheme.on_primary));
                assert_eq!(scheme.get_by_name("surface_container_high"), Some(scheme.surface_container_high));
                assert_eq!(scheme.get_by_name("surface-container-high"), Some(scheme.surface_container_high));
                assert_eq!(scheme.get_by_name("non_existent"), None);

                // 4. Test Key Contrast Guarantees
                // A. Primary vs On-Primary
                let cr_primary = contrast_ratio(scheme.primary, scheme.on_primary);
                assert!(
                    cr_primary >= 4.4,
                    "Primary vs On-Primary failed contrast ({:.2}) for seed {} ({:?}, is_dark={})",
                    cr_primary, name, variant, is_dark
                );

                // B. Primary Container vs On-Primary Container
                let cr_primary_container = contrast_ratio(scheme.primary_container, scheme.on_primary_container);
                assert!(
                    cr_primary_container >= 4.4,
                    "Primary Container vs On-Primary Container failed contrast ({:.2}) for seed {} ({:?}, is_dark={})",
                    cr_primary_container, name, variant, is_dark
                );

                // C. Secondary vs On-Secondary
                let cr_secondary = contrast_ratio(scheme.secondary, scheme.on_secondary);
                assert!(
                    cr_secondary >= 4.4,
                    "Secondary vs On-Secondary failed contrast ({:.2}) for seed {} ({:?}, is_dark={})",
                    cr_secondary, name, variant, is_dark
                );

                // D. Secondary Container vs On-Secondary Container
                let cr_secondary_container = contrast_ratio(scheme.secondary_container, scheme.on_secondary_container);
                assert!(
                    cr_secondary_container >= 4.4,
                    "Secondary Container vs On-Secondary Container failed contrast ({:.2}) for seed {} ({:?}, is_dark={})",
                    cr_secondary_container, name, variant, is_dark
                );

                // E. Tertiary vs On-Tertiary
                let cr_tertiary = contrast_ratio(scheme.tertiary, scheme.on_tertiary);
                assert!(
                    cr_tertiary >= 4.4,
                    "Tertiary vs On-Tertiary failed contrast ({:.2}) for seed {} ({:?}, is_dark={})",
                    cr_tertiary, name, variant, is_dark
                );

                // F. Tertiary Container vs On-Tertiary Container
                let cr_tertiary_container = contrast_ratio(scheme.tertiary_container, scheme.on_tertiary_container);
                assert!(
                    cr_tertiary_container >= 4.4,
                    "Tertiary Container vs On-Tertiary Container failed contrast ({:.2}) for seed {} ({:?}, is_dark={})",
                    cr_tertiary_container, name, variant, is_dark
                );

                // G. Error vs On-Error
                let cr_error = contrast_ratio(scheme.error, scheme.on_error);
                assert!(
                    cr_error >= 4.4,
                    "Error vs On-Error failed contrast ({:.2}) for seed {} ({:?}, is_dark={})",
                    cr_error, name, variant, is_dark
                );

                // H. Error Container vs On-Error Container
                let cr_error_container = contrast_ratio(scheme.error_container, scheme.on_error_container);
                assert!(
                    cr_error_container >= 4.4,
                    "Error Container vs On-Error Container failed contrast ({:.2}) for seed {} ({:?}, is_dark={})",
                    cr_error_container, name, variant, is_dark
                );

                // I. Surface vs On-Surface
                let cr_surface = contrast_ratio(scheme.surface, scheme.on_surface);
                assert!(
                    cr_surface >= 4.4,
                    "Surface vs On-Surface failed contrast ({:.2}) for seed {} ({:?}, is_dark={})",
                    cr_surface, name, variant, is_dark
                );

                // J. Surface Variant vs On-Surface Variant (>= 4.0:1)
                let cr_surf_var = contrast_ratio(scheme.surface_variant, scheme.on_surface_variant);
                assert!(
                    cr_surf_var >= 4.0,
                    "Surface Variant vs On-Surface Variant failed contrast ({:.2}) for seed {} ({:?}, is_dark={})",
                    cr_surf_var, name, variant, is_dark
                );

                // K. Inverse Surface vs Inverse On-Surface
                let cr_inv_surf = contrast_ratio(scheme.inverse_surface, scheme.inverse_on_surface);
                assert!(
                    cr_inv_surf >= 4.4,
                    "Inverse Surface vs Inverse On-Surface failed contrast ({:.2}) for seed {} ({:?}, is_dark={})",
                    cr_inv_surf, name, variant, is_dark
                );

                // 5. Surface Container Hierarchy Luminance Ordering
                let lum_lowest = relative_luminance(scheme.surface_container_lowest);
                let lum_low = relative_luminance(scheme.surface_container_low);
                let lum_mid = relative_luminance(scheme.surface_container);
                let lum_high = relative_luminance(scheme.surface_container_high);
                let lum_highest = relative_luminance(scheme.surface_container_highest);

                if is_dark {
                    // Dark mode: lowest < low < mid < high < highest
                    assert!(lum_lowest <= lum_low + 1e-4, "Dark mode lum_lowest > lum_low for {}", name);
                    assert!(lum_low <= lum_mid + 1e-4, "Dark mode lum_low > lum_mid for {}", name);
                    assert!(lum_mid <= lum_high + 1e-4, "Dark mode lum_mid > lum_high for {}", name);
                    assert!(lum_high <= lum_highest + 1e-4, "Dark mode lum_high > lum_highest for {}", name);
                } else {
                    // Light mode: lowest >= low >= mid >= high >= highest
                    assert!(lum_lowest >= lum_low - 1e-4, "Light mode lum_lowest < lum_low for {}", name);
                    assert!(lum_low >= lum_mid - 1e-4, "Light mode lum_low < lum_mid for {}", name);
                    assert!(lum_mid >= lum_high - 1e-4, "Light mode lum_mid < lum_high for {}", name);
                    assert!(lum_high >= lum_highest - 1e-4, "Light mode lum_high < lum_highest for {}", name);
                }
            }
        }
    }
}

// ============================================================================
// 3. CONTRAST LEVEL VARIATION TESTS
// ============================================================================

#[test]
fn test_adversarial_contrast_level_adjustments() {
    let seed = Color::from_hex("#6750A4").unwrap();
    let variant = SchemeVariant::TonalSpot;
    let core_palette = variant.generate_palette(seed);

    let contrast_steps = [-1.0, -0.5, 0.0, 0.5, 1.0];

    for &is_dark in &[false, true] {
        let mut prev_cr = 0.0;
        for &c in &contrast_steps {
            let scheme = ColorScheme::from_core_palette_with_contrast(&core_palette, is_dark, c);
            let cr = contrast_ratio(scheme.primary, scheme.on_primary);

            // Contrast ratio must increase monotonically with contrast level
            assert!(cr > prev_cr, "Contrast ratio not strictly increasing at c={}: {} <= {}", c, cr, prev_cr);
            prev_cr = cr;

            // Standard and high contrast (c >= 0.0) must meet WCAG AA (>= 4.5:1), reduced contrast must meet >= 3.0:1
            if c >= 0.0 {
                assert!(cr >= 4.5, "Contrast ratio too low at c={}: {}", c, cr);
            } else {
                assert!(cr >= 3.0, "Contrast ratio too low at c={}: {}", c, cr);
            }
            assert!(cr <= 21.0);
        }
    }
}

// ============================================================================
// 4. SHAPE, ELEVATION, STATE LAYERS, AND MOTION TOKENS
// ============================================================================

#[test]
fn test_adversarial_shape_tokens_api_and_customization() {
    let mut shapes = ShapeTokens::default();

    // Default values
    assert_eq!(shapes.corner_none, 0.0);
    assert_eq!(shapes.corner_extra_small, 4.0);
    assert_eq!(shapes.corner_small, 8.0);
    assert_eq!(shapes.corner_medium, 12.0);
    assert_eq!(shapes.corner_large, 16.0);
    assert_eq!(shapes.corner_extra_large, 28.0);
    assert_eq!(shapes.corner_full, 9999.0);

    // Aliases
    assert_eq!(shapes.get("none"), Some(&0.0));
    assert_eq!(shapes.get("xs"), Some(&4.0));
    assert_eq!(shapes.get("extra_small"), Some(&4.0));
    assert_eq!(shapes.get("sm"), Some(&8.0));
    assert_eq!(shapes.get("small"), Some(&8.0));
    assert_eq!(shapes.get("md"), Some(&12.0));
    assert_eq!(shapes.get("medium"), Some(&12.0));
    assert_eq!(shapes.get("lg"), Some(&16.0));
    assert_eq!(shapes.get("large"), Some(&16.0));
    assert_eq!(shapes.get("xl"), Some(&28.0));
    assert_eq!(shapes.get("extra_large"), Some(&28.0));
    assert_eq!(shapes.get("full"), Some(&9999.0));
    assert_eq!(shapes.get("pill"), Some(&9999.0));

    // Custom insertions
    shapes.insert("custom_card", 20.0);
    assert_eq!(shapes.get("custom_card"), Some(&20.0));
    assert!(shapes.contains_key("custom_card"));
    assert_eq!(shapes.len(), 8);

    // Map conversion
    let map = shapes.to_map();
    assert_eq!(map.get("custom_card"), Some(&20.0));
    assert_eq!(map.get("corner_large"), Some(&16.0));
}

#[test]
fn test_adversarial_elevation_tokens_dual_shadows_and_tints() {
    let elev = ElevationTokens::default();

    // Level 0: 0 dp, None shadows, 0% tint
    let l0 = elev.get(0);
    assert_eq!(l0.elevation_dp, 0.0);
    assert!(l0.key_shadow.is_none());
    assert!(l0.ambient_shadow.is_none());
    assert_eq!(l0.surface_tint_opacity, 0.0);
    assert_eq!(elev.to_css_box_shadow(0), "none");

    // Level 1: 1 dp, 5% tint
    let l1 = elev.get(1);
    assert_eq!(l1.elevation_dp, 1.0);
    assert!(l1.key_shadow.is_some());
    assert!(l1.ambient_shadow.is_some());
    assert_eq!(l1.surface_tint_opacity, 0.05);
    let shadow_css1 = elev.to_css_box_shadow(1);
    assert!(shadow_css1.contains("rgba(0, 0, 0, 0.30)"));
    assert!(shadow_css1.contains("rgba(0, 0, 0, 0.15)"));

    // Level 2: 3 dp, 8% tint
    let l2 = elev.get(2);
    assert_eq!(l2.elevation_dp, 3.0);
    assert_eq!(l2.surface_tint_opacity, 0.08);

    // Level 3: 6 dp, 11% tint
    let l3 = elev.get(3);
    assert_eq!(l3.elevation_dp, 6.0);
    assert_eq!(l3.surface_tint_opacity, 0.11);

    // Level 4: 8 dp, 12% tint
    let l4 = elev.get(4);
    assert_eq!(l4.elevation_dp, 8.0);
    assert_eq!(l4.surface_tint_opacity, 0.12);

    // Level 5: 12 dp, 14% tint
    let l5 = elev.get(5);
    assert_eq!(l5.elevation_dp, 12.0);
    assert_eq!(l5.surface_tint_opacity, 0.14);

    // Boundary clamping: level 99 returns level 5
    let l99 = elev.get(99);
    assert_eq!(l99.level, 5);

    // Surface tint blending calculation
    let base_surf = Color::from_rgb(20, 18, 24);
    let tint_col = Color::from_rgb(208, 188, 255);
    let tinted_l1 = elev.calculate_surface_tint(1, base_surf, tint_col);
    assert!(tinted_l1.r >= base_surf.r);
    assert!(tinted_l1.b >= base_surf.b);

    let tinted_l0 = elev.calculate_surface_tint(0, base_surf, tint_col);
    assert_eq!(tinted_l0, base_surf);
}

#[test]
fn test_adversarial_state_layer_tokens_and_blending() {
    let states = StateLayerTokens::default();
    assert_eq!(states.hover, 0.08);
    assert_eq!(states.focus, 0.12);
    assert_eq!(states.pressed, 0.12);
    assert_eq!(states.dragged, 0.16);
    assert_eq!(states.disabled_container, 0.12);
    assert_eq!(states.disabled_content, 0.38);

    let base = Color::from_rgb(40, 40, 40);
    let overlay = Color::from_rgb(200, 200, 200);

    // Hover blending (8%)
    let hovered = states.apply_hover(base, overlay);
    assert_eq!(hovered.r, (40.0f32 * 0.92 + 200.0f32 * 0.08).round() as u8);

    // Pressed blending (12%)
    let pressed = states.apply_pressed(base, overlay);
    assert_eq!(pressed.r, (40.0f32 * 0.88 + 200.0f32 * 0.12).round() as u8);

    // Focus blending (12%)
    let focused = states.apply_focus(base, overlay);
    assert_eq!(focused, pressed);

    // Dragged blending (16%)
    let dragged = states.apply_dragged(base, overlay);
    assert_eq!(dragged.r, (40.0f32 * 0.84 + 200.0f32 * 0.16).round() as u8);

    // Disabled container (12% alpha)
    let dis_container = states.apply_disabled_container(base);
    assert_eq!(dis_container.a, (255.0f32 * 0.12).round() as u8);
    assert_eq!(dis_container.r, base.r);

    // Disabled content (38% alpha)
    let dis_content = states.apply_disabled_content(overlay);
    assert_eq!(dis_content.a, (255.0f32 * 0.38).round() as u8);
    assert_eq!(dis_content.r, overlay.r);

    // Robustness against NaN and Infinite alpha values
    let nan_blend = states.blend(base, overlay, f32::NAN);
    assert_eq!(nan_blend, base);

    let inf_blend = states.blend(base, overlay, f32::INFINITY);
    assert_eq!(inf_blend, overlay);

    let neg_inf_blend = states.blend(base, overlay, f32::NEG_INFINITY);
    assert_eq!(neg_inf_blend, base);
}

#[test]
fn test_adversarial_motion_tokens() {
    let motion = MotionTokens::default();
    assert_eq!(motion.duration_short_1, 50);
    assert_eq!(motion.duration_short_2, 100);
    assert_eq!(motion.duration_short_3, 150);
    assert_eq!(motion.duration_short_4, 200);
    assert_eq!(motion.duration_medium_1, 250);
    assert_eq!(motion.duration_medium_2, 300);
    assert_eq!(motion.duration_medium_3, 350);
    assert_eq!(motion.duration_medium_4, 400);
    assert_eq!(motion.duration_long_1, 450);
    assert_eq!(motion.duration_long_2, 500);
}

// ============================================================================
// 5. DYNAMIC THEMEPACKAGE API & CSS GENERATOR STRESS TESTS
// ============================================================================

#[test]
fn test_adversarial_theme_package_from_seed_color_and_hex() {
    for &(name, hex) in ADVERSARIAL_SEEDS {
        for &variant in ALL_VARIANTS {
            for &is_dark in &[false, true] {
                let theme_from_hex = ThemePackage::from_seed_hex(hex, variant, is_dark).unwrap();
                let seed_col = Color::from_hex(hex).unwrap();
                let theme_from_col = ThemePackage::from_seed_color(seed_col, variant, is_dark);

                assert_eq!(theme_from_hex.name, "material-you");
                assert_eq!(theme_from_hex.is_dark, is_dark);
                assert_eq!(theme_from_hex.color_scheme, theme_from_col.color_scheme, "Mismatch for seed {}", name);

                // CSS Generation must produce parseable stylesheet
                let css = theme_from_hex.generate_css();
                assert!(!css.is_empty(), "Generated CSS is empty for {} ({:?}, is_dark={})", name, variant, is_dark);

                let stylesheet = parse_stylesheet(&css);
                assert!(stylesheet.rules.len() >= 10, "Stylesheet has too few rules ({}) for {}", stylesheet.rules.len(), name);

                // Verify specific core component rules exist
                assert!(css.contains("Button, Button[variant=\"filled\"]"));
                assert!(css.contains("Button[variant=\"tonal\"]"));
                assert!(css.contains("Button[variant=\"elevated\"]"));
                assert!(css.contains("Button[variant=\"outlined\"]"));
                assert!(css.contains("Button[variant=\"text\"]"));
                assert!(css.contains("Card, Card[variant=\"elevated\"]"));
                assert!(css.contains("Card[variant=\"filled\"]"));
                assert!(css.contains("Card[variant=\"outlined\"]"));
                assert!(css.contains("Switch"));
                assert!(css.contains("Checkbox"));
                assert!(css.contains("Slider"));
                assert!(css.contains("Chip"));
                assert!(css.contains("ProgressBar"));
                assert!(css.contains("TextInput"));
                assert!(css.contains("VStack#app-root"));
            }
        }
    }
}

#[test]
fn test_adversarial_theme_package_invalid_seeds_and_fallbacks() {
    assert!(ThemePackage::from_seed_hex("not-a-hex", SchemeVariant::TonalSpot, true).is_err());
    assert!(ThemePackage::from_seed_hex("#12", SchemeVariant::TonalSpot, true).is_err());
    assert!(ThemePackage::from_seed_hex("#GGGGGG", SchemeVariant::TonalSpot, true).is_err());
    assert!(ThemePackage::from_seed_hex("", SchemeVariant::TonalSpot, true).is_err());

    // SchemeVariant FromStr
    assert_eq!(SchemeVariant::from_str("tonal_spot").unwrap(), SchemeVariant::TonalSpot);
    assert_eq!(SchemeVariant::from_str("TONAL-SPOT").unwrap(), SchemeVariant::TonalSpot);
    assert_eq!(SchemeVariant::from_str("vibrant").unwrap(), SchemeVariant::Vibrant);
    assert_eq!(SchemeVariant::from_str("expressive").unwrap(), SchemeVariant::Expressive);
    assert_eq!(SchemeVariant::from_str("fidelity").unwrap(), SchemeVariant::Fidelity);
    assert_eq!(SchemeVariant::from_str("content").unwrap(), SchemeVariant::Content);
    assert_eq!(SchemeVariant::from_str("monochrome").unwrap(), SchemeVariant::Monochrome);
    assert_eq!(SchemeVariant::from_str("neutral").unwrap(), SchemeVariant::Neutral);
    assert!(SchemeVariant::from_str("invalid_variant").is_err());
}
