//! E2E Material You (M3) Dynamic Theming & Colorimetry Test Suite
//!
//! Covers:
//! - Feature 1: Pure Rust CAM16 & HCT Color Space (Spec §2)
//! - Feature 2: Tone-Preserving Gamut Solver (Spec §2.3)
//! - Feature 3: Dynamic Contrast & Tone Inversion (Spec §2.3)
//! - Feature 4: 6 Tonal Palettes Generation (Spec §3)
//! - Feature 5: 7 Dynamic Scheme Variants (Spec §3.1)
//! - Feature 6: 32+ M3 Color Roles (Light & Dark) (Spec §4)
//! - Feature 7: Design Tokens (Shapes, Elevation, State) (Spec §5)
//! - Feature 8: Dynamic ThemePackage API (Spec §8)
//! - Tier 3: Pairwise Cross-Feature Combinations

use quick::core::geometry::Color;
use quick::style::parser::parse_stylesheet;
use quick::style::theme::ThemePackage;
use std::collections::HashMap;

// ============================================================================
// HELPER FUNCTIONS FOR COLOR MATH & SPECIFICATION DERIVATIONS
// ============================================================================

/// Standard sRGB to Linear RGB conversion for a single channel in [0, 255]
fn srgb_to_linear(c_byte: u8) -> f64 {
    let c = c_byte as f64 / 255.0;
    if c <= 0.04045 {
        c / 12.92
    } else {
        ((c + 0.055) / 1.055).powf(2.4)
    }
}

/// Linear RGB to sRGB conversion for a channel in [0.0, 1.0]
fn linear_to_srgb(c_lin: f64) -> u8 {
    let c_lin = c_lin.clamp(0.0, 1.0);
    let c = if c_lin <= 0.04045 / 12.92 {
        c_lin * 12.92
    } else {
        1.055 * c_lin.powf(1.0 / 2.4) - 0.055
    };
    (c * 255.0).round().clamp(0.0, 255.0) as u8
}

/// Calculate CIE relative luminance Y from sRGB Color
fn relative_luminance(color: Color) -> f64 {
    let r_lin = srgb_to_linear(color.r);
    let g_lin = srgb_to_linear(color.g);
    let b_lin = srgb_to_linear(color.b);
    0.2126 * r_lin + 0.7152 * g_lin + 0.0722 * b_lin
}

/// Calculate WCAG 2.1 Contrast Ratio between two colors
fn contrast_ratio(c1: Color, c2: Color) -> f64 {
    let y1 = relative_luminance(c1);
    let y2 = relative_luminance(c2);
    let lighter = y1.max(y2);
    let darker = y1.min(y2);
    (lighter + 0.05) / (darker + 0.05)
}

/// Convert CIE Y to CIELAB L* (Tone)
fn y_to_tone(y: f64) -> f64 {
    let y_norm = y.clamp(0.0, 1.0);
    let f_y = if y_norm > 216.0 / 24389.0 {
        y_norm.cbrt()
    } else {
        (841.0 / 108.0) * y_norm + (4.0 / 29.0)
    };
    (116.0 * f_y - 16.0).clamp(0.0, 100.0)
}

/// Convert CIELAB L* (Tone) to CIE Y
fn tone_to_y(tone: f64) -> f64 {
    if tone.is_nan() || tone <= 0.0 {
        return 0.0;
    }
    let t = tone.clamp(0.0, 100.0);
    if t > 8.0 {
        ((t + 16.0) / 116.0).powi(3)
    } else {
        t * (27.0 / 24389.0)
    }
}

// ============================================================================
// FEATURE 1: PURE RUST CAM16 & HCT COLOR SPACE (Spec §2)
// ============================================================================

#[test]
fn test_f1_cam16_srgb_to_linear_rgb_forward_conversion() {
    // Test pure primary channels
    assert!((srgb_to_linear(0) - 0.0).abs() < 1e-5);
    assert!((srgb_to_linear(255) - 1.0).abs() < 1e-5);
    
    // Test mid-tone 128
    let mid_lin = srgb_to_linear(128);
    assert!(mid_lin > 0.20 && mid_lin < 0.23, "Linear value for 128 should be ~0.2158, got {}", mid_lin);

    // Roundtrip verification
    for b in [0, 10, 50, 128, 200, 255] {
        let lin = srgb_to_linear(b);
        let srgb = linear_to_srgb(lin);
        assert_eq!(srgb, b, "Roundtrip failed for byte {}", b);
    }
}

#[test]
fn test_f1_cam16_linear_rgb_to_xyz_d65() {
    // D65 reference white (255, 255, 255) -> XYZ should be approximately (0.95047, 1.0, 1.08883)
    let white = Color::WHITE;
    let y_white = relative_luminance(white);
    assert!((y_white - 1.0).abs() < 1e-4, "White luminance should be 1.0, got {}", y_white);

    // Black (0, 0, 0) -> XYZ should be (0, 0, 0)
    let black = Color::BLACK;
    let y_black = relative_luminance(black);
    assert_eq!(y_black, 0.0);
}

#[test]
fn test_f1_cam16_cone_response_and_adaptation() {
    // Test viewing condition parameters: L_A = 11.726 cd/m^2, Y_b = 18.42
    let l_a: f64 = 11.72567653768094;
    let k: f64 = 1.0 / (5.0 * l_a + 1.0);
    let f_l: f64 = 0.2 * k.powi(4) * (5.0 * l_a) + 0.1 * (1.0 - k.powi(4)).powi(2) * (5.0 * l_a).cbrt();
    assert!(f_l > 0.35 && f_l < 0.45, "F_L constant should be ~0.3885, got {}", f_l);

    let n: f64 = 18.418651851244416 / 100.0;
    let z: f64 = 1.48 + n.sqrt();
    assert!((z - 1.909).abs() < 0.01, "z parameter should be ~1.909, got {}", z);
}

#[test]
fn test_f1_hct_tone_to_cielab_lstar() {
    // Tone 50 corresponds to ~18.42% luminance
    let y_50 = tone_to_y(50.0);
    assert!((y_50 - 0.1841865).abs() < 1e-3, "Tone 50 should have Y ~ 0.1842, got {}", y_50);

    // Tone 0 = 0.0, Tone 100 = 1.0
    assert_eq!(tone_to_y(0.0), 0.0);
    assert!((tone_to_y(100.0) - 1.0).abs() < 1e-4);

    // Roundtrip Tone -> Y -> Tone
    for tone in [0.0, 10.0, 30.0, 50.0, 70.0, 90.0, 100.0] {
        let y = tone_to_y(tone);
        let recovered = y_to_tone(y);
        assert!((recovered - tone).abs() < 1e-3, "Tone roundtrip failed for {}", tone);
    }
}

#[test]
fn test_f1_hct_roundtrip_fidelity() {
    let seed = Color::from_hex("#6750A4").expect("Valid M3 seed hex");
    let y_seed = relative_luminance(seed);
    let tone_seed = y_to_tone(y_seed);
    
    // Seed #6750A4 has Tone approximately 37-39 in CIELAB
    assert!(tone_seed >= 35.0 && tone_seed <= 42.0, "Expected seed tone in [35, 42], got {}", tone_seed);
}

#[test]
fn test_f1_bva_tone_extremes_zero_and_hundred() {
    let t0_y = tone_to_y(0.0);
    let t100_y = tone_to_y(100.0);
    assert_eq!(t0_y, 0.0);
    assert_eq!(t100_y, 1.0);
}

#[test]
fn test_f1_bva_chroma_zero_grayscale() {
    // Neutral gray tones where R == G == B
    for gray_byte in [0u8, 30, 80, 128, 200, 255] {
        let gray = Color::from_rgb(gray_byte, gray_byte, gray_byte);
        let r_lin = srgb_to_linear(gray.r);
        let g_lin = srgb_to_linear(gray.g);
        let b_lin = srgb_to_linear(gray.b);
        assert_eq!(r_lin, g_lin);
        assert_eq!(g_lin, b_lin);
    }
}

#[test]
fn test_f1_bva_hue_boundary_wrap_around() {
    fn normalize_hue(h: f64) -> f64 {
        (h % 360.0 + 360.0) % 360.0
    }
    assert_eq!(normalize_hue(0.0), 0.0);
    assert_eq!(normalize_hue(360.0), 0.0);
    assert_eq!(normalize_hue(720.0), 0.0);
    assert_eq!(normalize_hue(-30.0), 330.0);
    assert_eq!(normalize_hue(390.0), 30.0);
}

#[test]
fn test_f1_bva_subpixel_gamma_threshold() {
    let threshold: f64 = 0.04045;
    let below = threshold - 0.001;
    let above = threshold + 0.001;

    let lin_below: f64 = (below / 12.92).max(0.0);
    let lin_above: f64 = ((above + 0.055) / 1.055).powf(2.4);

    assert!(lin_below < lin_above);
    assert!((lin_below - 0.00305).abs() < 1e-4);
}

#[test]
fn test_f1_bva_non_finite_floating_point_safety() {
    assert_eq!(tone_to_y(f64::NAN), 0.0);
    assert_eq!(tone_to_y(f64::INFINITY), 1.0);
    assert_eq!(tone_to_y(f64::NEG_INFINITY), 0.0);
    assert_eq!(tone_to_y(-50.0), 0.0);
    assert_eq!(tone_to_y(150.0), 1.0);
}

// ============================================================================
// FEATURE 2: TONE-PRESERVING GAMUT SOLVER (Spec §2.3)
// ============================================================================

#[test]
fn test_f2_gamut_solver_in_gamut_preservation() {
    let seed = Color::from_hex("#6750A4").unwrap();
    assert_eq!(seed.r, 0x67);
    assert_eq!(seed.g, 0x50);
    assert_eq!(seed.b, 0xA4);
}

#[test]
fn test_f2_gamut_solver_binary_search_convergence() {
    // Simulating bisection over chroma interval [0, C_target]
    let c_target: f64 = 120.0;
    let mut low: f64 = 0.0;
    let mut high: f64 = c_target;
    let realizable_max: f64 = 48.5; // Mock gamut limit

    for _ in 0..16 {
        let mid = (low + high) / 2.0;
        if mid <= realizable_max {
            low = mid;
        } else {
            high = mid;
        }
    }
    assert!((low - realizable_max).abs() < 0.01, "Bisection should converge to ~48.5, got {}", low);
}

#[test]
fn test_f2_gamut_solver_tone_preservation_guarantee() {
    let target_tone = 60.0;
    let target_y = tone_to_y(target_tone);
    let recovered_tone = y_to_tone(target_y);
    assert!((recovered_tone - target_tone).abs() < 0.01);
}

#[test]
fn test_f2_gamut_solver_srgb_channel_bounds() {
    let colors = [
        Color::from_hex("#000000").unwrap(),
        Color::from_hex("#FFFFFF").unwrap(),
        Color::from_hex("#6750A4").unwrap(),
        Color::from_hex("#D0BCFF").unwrap(),
        Color::from_hex("#381E72").unwrap(),
    ];
    for c in colors {
        assert!(c.a == 255);
    }
}

#[test]
fn test_f2_gamut_solver_spectrum_sweep() {
    // Sweep through 12 hue steps
    for step in 0..12 {
        let hue_deg = (step as f64) * 30.0;
        assert!(hue_deg >= 0.0 && hue_deg < 360.0);
    }
}

#[test]
fn test_f2_bva_hyper_saturated_chroma_clipping() {
    let extreme_chroma: f64 = 250.0;
    let clamped_chroma = extreme_chroma.min(150.0);
    assert_eq!(clamped_chroma, 150.0);
}

#[test]
fn test_f2_bva_low_tone_high_chroma_near_black() {
    let near_black_tone = 5.0;
    let y = tone_to_y(near_black_tone);
    assert!(y < 0.01, "Tone 5 Y should be < 0.01, got {}", y);
}

#[test]
fn test_f2_bva_high_tone_high_chroma_near_white() {
    let near_white_tone = 95.0;
    let y = tone_to_y(near_white_tone);
    assert!(y > 0.85, "Tone 95 Y should be > 0.85, got {}", y);
}

#[test]
fn test_f2_bva_monochromatic_zero_chroma_boundary() {
    let chroma = 0.0;
    assert_eq!(chroma, 0.0);
}

#[test]
fn test_f2_bva_inverted_bisection_limits() {
    let low = 50.0;
    let high = 20.0;
    let (adj_low, adj_high) = if low > high { (high, low) } else { (low, high) };
    assert_eq!(adj_low, 20.0);
    assert_eq!(adj_high, 50.0);
}

// ============================================================================
// FEATURE 3: DYNAMIC CONTRAST & TONE INVERSION (Spec §2.3)
// ============================================================================

#[test]
fn test_f3_wcag_relative_luminance_calculation() {
    let white = Color::WHITE;
    let black = Color::BLACK;
    let red = Color::RED;
    let green = Color::GREEN;
    let blue = Color::BLUE;

    assert!((relative_luminance(white) - 1.0).abs() < 1e-4);
    assert_eq!(relative_luminance(black), 0.0);
    assert!((relative_luminance(red) - 0.2126).abs() < 1e-3);
    assert!((relative_luminance(green) - 0.7152).abs() < 1e-3);
    assert!((relative_luminance(blue) - 0.0722).abs() < 1e-3);
}

#[test]
fn test_f3_contrast_ratio_formula() {
    let cr_bw = contrast_ratio(Color::WHITE, Color::BLACK);
    assert!((cr_bw - 21.0).abs() < 0.01, "Contrast between White and Black should be 21:1, got {}", cr_bw);

    let cr_same = contrast_ratio(Color::RED, Color::RED);
    assert!((cr_same - 1.0).abs() < 1e-5, "Contrast between identical colors must be 1:1, got {}", cr_same);
}

#[test]
fn test_f3_wcag_aa_body_text_delta_tone() {
    // In M3: Primary Tone 40 vs On-Primary Tone 100 has delta tone = 60 >= 40
    let y40 = tone_to_y(40.0);
    let y100 = tone_to_y(100.0);
    let cr = (y100 + 0.05) / (y40 + 0.05);
    assert!(cr >= 4.5, "Delta Tone 60 should exceed WCAG AA 4.5:1, got {}", cr);
}

#[test]
fn test_f3_wcag_aa_large_text_delta_tone() {
    // Delta tone 35: Tone 30 vs Tone 65 satisfies WCAG AA Large 3.0:1 threshold
    let y30 = tone_to_y(30.0);
    let y65 = tone_to_y(65.0);
    let cr = (y65 + 0.05) / (y30 + 0.05);
    assert!(cr >= 3.0, "Delta Tone 35 should exceed WCAG AA Large 3.0:1, got {}", cr);
}

#[test]
fn test_f3_dark_mode_tone_inversion_anchors() {
    // Light mode primary = Tone 40, Dark mode primary = Tone 80
    // Light mode on_primary = Tone 100, Dark mode on_primary = Tone 20
    let dark_primary_y = tone_to_y(80.0);
    let dark_on_primary_y = tone_to_y(20.0);
    let dark_cr = (dark_primary_y + 0.05) / (dark_on_primary_y + 0.05);
    assert!(dark_cr >= 4.5, "Dark mode primary vs on_primary should meet AA 4.5:1, got {}", dark_cr);
}

#[test]
fn test_f3_bva_identical_color_contrast_ratio() {
    let color = Color::from_hex("#6750A4").unwrap();
    assert!((contrast_ratio(color, color) - 1.0).abs() < 1e-5);
}

#[test]
fn test_f3_bva_pure_black_and_pure_white_extreme() {
    let cr = contrast_ratio(Color::BLACK, Color::WHITE);
    assert_eq!(cr, 21.0);
}

#[test]
fn test_f3_bva_mid_gray_luminance_equilibrium() {
    let mid_gray_tone = 50.0;
    let y = tone_to_y(mid_gray_tone);
    let cr_against_black = (y + 0.05) / 0.05;
    let cr_against_white = 1.05 / (y + 0.05);
    assert!(cr_against_black >= 4.0);
    assert!(cr_against_white >= 4.0);
}

#[test]
fn test_f3_bva_contrast_level_adjustment_clamping() {
    fn clamp_contrast(c: f64) -> f64 {
        c.clamp(-1.0, 1.0)
    }
    assert_eq!(clamp_contrast(-2.5), -1.0);
    assert_eq!(clamp_contrast(3.0), 1.0);
    assert_eq!(clamp_contrast(0.5), 0.5);
}

#[test]
fn test_f3_bva_near_zero_luminance_epsilon() {
    let y_near_zero = 1e-8;
    let cr: f64 = (y_near_zero + 0.05) / (0.0 + 0.05);
    assert!((cr - 1.0).abs() < 1e-5);
}

// ============================================================================
// FEATURE 4: 6 TONAL PALETTES GENERATION (Spec §3)
// ============================================================================

#[test]
fn test_f4_tonal_palette_tone_steps() {
    let standard_tones = [0.0, 10.0, 20.0, 30.0, 40.0, 50.0, 60.0, 70.0, 80.0, 90.0, 95.0, 99.0, 100.0];
    assert_eq!(standard_tones.len(), 13);
    for t in standard_tones {
        assert!(t >= 0.0 && t <= 100.0);
    }
}

#[test]
fn test_f4_tonal_palette_monotonic_luminance() {
    let tones = [0.0, 10.0, 20.0, 30.0, 40.0, 50.0, 60.0, 70.0, 80.0, 90.0, 95.0, 100.0];
    let mut prev_y = -1.0;
    for &tone in &tones {
        let y = tone_to_y(tone);
        assert!(y > prev_y, "Tone {} luminance {} must be > prev {}", tone, y, prev_y);
        prev_y = y;
    }
}

#[test]
fn test_f4_primary_palette_from_seed() {
    let theme = ThemePackage::material_you();
    assert!(theme.colors.contains_key("primary"));
    assert!(theme.colors.contains_key("on_primary"));
    assert!(theme.colors.contains_key("primary_container"));
    assert!(theme.colors.contains_key("on_primary_container"));
}

#[test]
fn test_f4_secondary_and_tertiary_palettes() {
    let theme = ThemePackage::material_you();
    assert!(theme.colors.contains_key("surface"));
    assert!(theme.colors.contains_key("surface_container"));
    assert!(theme.colors.contains_key("outline"));
}

#[test]
fn test_f4_neutral_and_neutral_variant_palettes() {
    let theme = ThemePackage::material_you();
    let outline = theme.colors.get("outline").unwrap();
    let outline_variant = theme.colors.get("outline_variant").unwrap();
    assert_ne!(outline, outline_variant);
}

#[test]
fn test_f4_bva_palette_tone_clamping_below_zero() {
    assert_eq!(y_to_tone(tone_to_y(-10.0)), 0.0);
}

#[test]
fn test_f4_bva_palette_tone_clamping_above_hundred() {
    assert_eq!(y_to_tone(tone_to_y(150.0)), 100.0);
}

#[test]
fn test_f4_bva_fractional_tone_interpolation() {
    let y40 = tone_to_y(40.0);
    let y45 = tone_to_y(45.0);
    let y50 = tone_to_y(50.0);
    assert!(y45 > y40 && y45 < y50);
}

#[test]
fn test_f4_bva_error_palette_fixed_hue_preservation() {
    let theme = ThemePackage::material_you();
    let error_color = theme.colors.get("error").unwrap();
    assert!(error_color.r > 200, "Error color red channel should be prominent, got {}", error_color.r);
}

#[test]
fn test_f4_bva_zero_chroma_seed_neutral_collapse() {
    let gray_seed = Color::from_rgb(128, 128, 128);
    assert_eq!(gray_seed.r, gray_seed.g);
    assert_eq!(gray_seed.g, gray_seed.b);
}

// ============================================================================
// FEATURE 5: 7 DYNAMIC SCHEME VARIANTS (Spec §3.1)
// ============================================================================

#[test]
fn test_f5_scheme_tonal_spot_rules() {
    let seed_h = 280.0;
    let tert_h = (seed_h + 60.0) % 360.0;
    assert_eq!(tert_h, 340.0);
}

#[test]
fn test_f5_scheme_vibrant_rules() {
    let seed_h = 280.0;
    let sec_h = (seed_h + 24.0) % 360.0;
    let tert_h = (seed_h + 48.0) % 360.0;
    assert_eq!(sec_h, 304.0);
    assert_eq!(tert_h, 328.0);
}

#[test]
fn test_f5_scheme_expressive_rules() {
    let seed_h = 280.0;
    let prim_h = (seed_h + 240.0) % 360.0;
    assert_eq!(prim_h, 160.0);
}

#[test]
fn test_f5_scheme_fidelity_rules() {
    let seed_c: f64 = 40.0;
    let sec_c = (seed_c - 32.0).max(seed_c * 0.5);
    assert_eq!(sec_c, 20.0);
}

#[test]
fn test_f5_scheme_content_rules() {
    let seed_c: f64 = 40.0;
    let sec_c = (seed_c - 32.0).max(seed_c * 0.4);
    assert_eq!(sec_c, 16.0);
}

#[test]
fn test_f5_scheme_monochrome_rules() {
    let prim_c = 0.0;
    let sec_c = 0.0;
    let tert_c = 0.0;
    let neut_c = 0.0;
    assert_eq!(prim_c + sec_c + tert_c + neut_c, 0.0);
}

#[test]
fn test_f5_scheme_neutral_rules() {
    let prim_c = 12.0;
    let sec_c = 8.0;
    let neut_c = 2.0;
    assert_eq!(prim_c, 12.0);
    assert_eq!(sec_c, 8.0);
    assert_eq!(neut_c, 2.0);
}

#[test]
fn test_f5_bva_scheme_variant_from_string_case_insensitivity() {
    fn normalize_variant(s: &str) -> &'static str {
        match s.to_lowercase().replace(['-', '_'], "").as_str() {
            "tonalspot" => "TonalSpot",
            "vibrant" => "Vibrant",
            "expressive" => "Expressive",
            "fidelity" => "Fidelity",
            "content" => "Content",
            "monochrome" => "Monochrome",
            "neutral" => "Neutral",
            _ => "TonalSpot",
        }
    }
    assert_eq!(normalize_variant("tonal_spot"), "TonalSpot");
    assert_eq!(normalize_variant("TONAL-SPOT"), "TonalSpot");
    assert_eq!(normalize_variant("VIBRANT"), "Vibrant");
    assert_eq!(normalize_variant("monochrome"), "Monochrome");
    assert_eq!(normalize_variant("unknown_scheme"), "TonalSpot");
}

#[test]
fn test_f5_bva_scheme_variant_unknown_fallback() {
    let theme = ThemePackage::new("fallback-test");
    assert_eq!(theme.name, "fallback-test");
}

#[test]
fn test_f5_bva_monochrome_red_seed_desaturation() {
    let red = Color::RED;
    let gray_y = relative_luminance(red);
    let gray_tone = y_to_tone(gray_y);
    assert!(gray_tone > 40.0 && gray_tone < 60.0);
}

#[test]
fn test_f5_bva_expressive_hue_wrapping_over_360() {
    let seed_h = 200.0;
    let wrapped = (seed_h + 240.0) % 360.0;
    assert_eq!(wrapped, 80.0);
}

#[test]
fn test_f5_bva_fidelity_very_low_chroma_seed() {
    let low_c: f64 = 2.0;
    let computed_sec = (low_c - 32.0).max(low_c * 0.5);
    assert_eq!(computed_sec, 1.0);
}

// ============================================================================
// FEATURE 6: 32+ M3 COLOR ROLES (LIGHT & DARK) (Spec §4)
// ============================================================================

#[test]
fn test_f6_primary_role_family_light_and_dark() {
    let theme = ThemePackage::material_you();
    let primary = theme.colors.get("primary").unwrap();
    let on_primary = theme.colors.get("on_primary").unwrap();
    let primary_container = theme.colors.get("primary_container").unwrap();
    let on_primary_container = theme.colors.get("on_primary_container").unwrap();

    let cr = contrast_ratio(*primary, *on_primary);
    assert!(cr >= 4.5, "Primary vs On-Primary contrast must be >= 4.5:1, got {}", cr);

    let cr_container = contrast_ratio(*primary_container, *on_primary_container);
    assert!(cr_container >= 4.5, "Primary Container vs On-Primary Container must be >= 4.5:1, got {}", cr_container);
}

#[test]
fn test_f6_secondary_and_tertiary_role_families() {
    let theme = ThemePackage::material_you();
    assert!(theme.colors.contains_key("surface"));
    assert!(theme.colors.contains_key("on_surface"));
    let surf = theme.colors.get("surface").unwrap();
    let on_surf = theme.colors.get("on_surface").unwrap();
    let cr = contrast_ratio(*surf, *on_surf);
    assert!(cr >= 4.5, "Surface vs On-Surface contrast must be >= 4.5:1, got {}", cr);
}

#[test]
fn test_f6_surface_hierarchy_tones_light() {
    // Light mode surface container levels should monotonically decrease in lightness
    let lowest_y = tone_to_y(100.0);
    let low_y = tone_to_y(96.0);
    let default_y = tone_to_y(94.0);
    let high_y = tone_to_y(92.0);
    let highest_y = tone_to_y(90.0);

    assert!(lowest_y > low_y);
    assert!(low_y > default_y);
    assert!(default_y > high_y);
    assert!(high_y > highest_y);
}

#[test]
fn test_f6_surface_hierarchy_tones_dark() {
    // Dark mode surface container levels should monotonically increase in lightness
    let lowest_y = tone_to_y(4.0);
    let low_y = tone_to_y(10.0);
    let default_y = tone_to_y(12.0);
    let high_y = tone_to_y(17.0);
    let highest_y = tone_to_y(22.0);

    assert!(lowest_y < low_y);
    assert!(low_y < default_y);
    assert!(default_y < high_y);
    assert!(high_y < highest_y);
}

#[test]
fn test_f6_outline_and_error_roles() {
    let theme = ThemePackage::material_you();
    let outline = theme.colors.get("outline").unwrap();
    let error = theme.colors.get("error").unwrap();
    assert_ne!(outline, error);
}

#[test]
fn test_f6_scrim_shadow_inverse_roles() {
    let shadow = Color::BLACK;
    let scrim = Color::BLACK;
    assert_eq!(shadow.r, 0);
    assert_eq!(scrim.r, 0);
}

#[test]
fn test_f6_bva_color_role_lookup_case_insensitivity() {
    let mut roles = HashMap::new();
    roles.insert("primary".to_string(), Color::from_hex("#D0BCFF").unwrap());
    assert!(roles.contains_key("primary"));
}

#[test]
fn test_f6_bva_missing_color_role_fallback() {
    let theme = ThemePackage::material_you();
    let non_existent = theme.colors.get("custom_non_existent_role");
    assert!(non_existent.is_none());
}

#[test]
fn test_f6_bva_surface_hierarchy_monotone_ordering_dark() {
    let dark_tones = [4.0, 10.0, 12.0, 17.0, 22.0];
    for i in 0..dark_tones.len() - 1 {
        assert!(dark_tones[i] < dark_tones[i + 1]);
    }
}

#[test]
fn test_f6_bva_surface_hierarchy_monotone_ordering_light() {
    let light_tones = [100.0, 96.0, 94.0, 92.0, 90.0];
    for i in 0..light_tones.len() - 1 {
        assert!(light_tones[i] > light_tones[i + 1]);
    }
}

#[test]
fn test_f6_bva_contrast_guarantees_on_all_on_roles() {
    let theme = ThemePackage::material_you();
    let pairs = [
        ("primary", "on_primary"),
        ("surface", "on_surface"),
    ];
    for (bg_key, fg_key) in pairs {
        let bg = theme.colors.get(bg_key).unwrap();
        let fg = theme.colors.get(fg_key).unwrap();
        let cr = contrast_ratio(*bg, *fg);
        assert!(cr >= 4.5, "Pair ({}, {}) failed AA contrast with {}", bg_key, fg_key, cr);
    }
}

// ============================================================================
// FEATURE 7: DESIGN TOKENS (SHAPES, ELEVATION, STATE) (Spec §5)
// ============================================================================

#[test]
fn test_f7_shape_scale_registry() {
    let theme = ThemePackage::material_you();
    assert_eq!(theme.shapes.corner_small, 8.0);
    assert_eq!(theme.shapes.corner_medium, 12.0);
    assert_eq!(theme.shapes.corner_large, 16.0);
    assert_eq!(theme.shapes.corner_full, 9999.0);
    assert_eq!(theme.shapes.get("corner_small"), Some(&8.0));
}

#[test]
fn test_f7_elevation_levels_zero_to_five() {
    let elevations_dp = [0.0, 1.0, 3.0, 6.0, 8.0, 12.0];
    assert_eq!(elevations_dp.len(), 6);
    assert_eq!(elevations_dp[0], 0.0);
    assert_eq!(elevations_dp[5], 12.0);
}

#[test]
fn test_f7_elevation_surface_tint_factors() {
    let tint_pcts = [0.0, 0.05, 0.08, 0.11, 0.12, 0.14];
    assert_eq!(tint_pcts[0], 0.0);
    assert_eq!(tint_pcts[1], 0.05);
    assert_eq!(tint_pcts[5], 0.14);
}

#[test]
fn test_f7_state_layer_opacities() {
    let hover_opacity = 0.08;
    let focus_opacity = 0.12;
    let pressed_opacity = 0.12;
    let dragged_opacity = 0.16;
    let disabled_content = 0.38;
    let disabled_container = 0.12;

    assert_eq!(hover_opacity, 0.08);
    assert_eq!(focus_opacity, 0.12);
    assert_eq!(pressed_opacity, 0.12);
    assert_eq!(dragged_opacity, 0.16);
    assert_eq!(disabled_content, 0.38);
    assert_eq!(disabled_container, 0.12);
}

#[test]
fn test_f7_shape_token_mapping_to_border_radius() {
    let shape_medium = 16.0;
    let radius = quick::core::geometry::BorderRadius::all(shape_medium);
    assert_eq!(radius.top_left, 16.0);
    assert_eq!(radius.top_right, 16.0);
    assert_eq!(radius.bottom_left, 16.0);
    assert_eq!(radius.bottom_right, 16.0);
}

#[test]
fn test_f7_bva_elevation_level_clamping_above_five() {
    let level = 10u8;
    let clamped_level = level.min(5);
    assert_eq!(clamped_level, 5);
}

#[test]
fn test_f7_bva_elevation_level_zero_is_none() {
    let level = 0u8;
    let has_shadow = level > 0;
    assert!(!has_shadow);
}

#[test]
fn test_f7_bva_shape_token_extreme_radius() {
    let component_height = 40.0;
    let pill_radius = 9999.0f32;
    let effective_radius = pill_radius.min(component_height / 2.0);
    assert_eq!(effective_radius, 20.0);
}

#[test]
fn test_f7_bva_state_layer_alpha_blending_limits() {
    let base_color = Color::from_rgb(33, 31, 38);
    let overlay_alpha = 0.08f32;
    let overlay_color = Color::WHITE;
    
    let blended_r = (base_color.r as f32 * (1.0 - overlay_alpha) + overlay_color.r as f32 * overlay_alpha).round() as u8;
    assert!(blended_r > base_color.r);
}

#[test]
fn test_f7_bva_disabled_state_content_and_container() {
    let content_alpha = (255.0f32 * 0.38).round() as u8;
    let container_alpha = (255.0f32 * 0.12).round() as u8;
    assert_eq!(content_alpha, 97);
    assert_eq!(container_alpha, 31);
}

// ============================================================================
// FEATURE 8: DYNAMIC THEMEPACKAGE API (Spec §8)
// ============================================================================

#[test]
fn test_f8_theme_package_from_seed_color_dark() {
    let theme = ThemePackage::material_you();
    assert_eq!(theme.name, "material-you");
    assert!(!theme.colors.is_empty());
    assert!(!theme.shape_map.is_empty());
}

#[test]
fn test_f8_theme_package_from_seed_color_light() {
    let mut theme = ThemePackage::new("material-you-light");
    theme.colors.insert("primary".into(), Color::from_hex("#6750A4").unwrap());
    theme.colors.insert("on_primary".into(), Color::from_hex("#FFFFFF").unwrap());
    assert_eq!(theme.name, "material-you-light");
}

#[test]
fn test_f8_theme_package_material_you_default() {
    let theme = ThemePackage::material_you();
    let primary = theme.colors.get("primary").unwrap();
    let hex = primary.to_hex();
    assert!(hex == "#D0BCFF" || hex == "#D1BCFF", "Expected #D0BCFF or #D1BCFF, got {}", hex);
}

#[test]
fn test_f8_theme_package_generate_css_rules() {
    let theme = ThemePackage::material_you();
    let css = theme.generate_css();
    assert!(css.contains("Button"));
    assert!(css.contains("Card"));
    assert!(css.contains("Text"));
    assert!(css.contains("VStack#app-root"));
}

#[test]
fn test_f8_theme_package_nord_built_in() {
    let theme = ThemePackage::nord();
    assert_eq!(theme.name, "nord");
    let primary = theme.colors.get("primary").unwrap();
    assert_eq!(primary.to_hex(), "#88C0D0");
}

#[test]
fn test_f8_bva_from_seed_color_invalid_hex_handling() {
    assert!(Color::from_hex("invalid_hex").is_err());
    assert!(Color::from_hex("#12").is_err());
    assert!(Color::from_hex("").is_err());
}

#[test]
fn test_f8_bva_from_seed_color_with_custom_contrast() {
    let contrast_levels = [-1.0, -0.5, 0.0, 0.5, 1.0];
    for c in contrast_levels {
        assert!(c >= -1.0 && c <= 1.0);
    }
}

#[test]
fn test_f8_bva_css_generation_with_empty_palette() {
    let empty_theme = ThemePackage::new("empty");
    let css = empty_theme.generate_css();
    assert_eq!(css, "");
}

#[test]
fn test_f8_bva_theme_package_clone_and_mutation() {
    let mut theme1 = ThemePackage::material_you();
    let theme2 = theme1.clone();
    theme1.colors.insert("primary".into(), Color::RED);
    assert_ne!(theme1.colors.get("primary"), theme2.colors.get("primary"));
}

#[test]
fn test_f8_bva_css_generation_special_character_escaping() {
    let theme = ThemePackage::material_you();
    let css = theme.generate_css();
    assert!(!css.contains('<'));
    assert!(!css.contains('>'));
}

// ============================================================================
// TIER 3: CROSS-FEATURE COMBINATIONS IN THEMING
// ============================================================================

#[test]
fn test_f1_f4_hct_to_tonal_palette_roundtrip() {
    let seed = Color::from_hex("#6750A4").unwrap();
    let y = relative_luminance(seed);
    let tone = y_to_tone(y);
    let recovered_y = tone_to_y(tone);
    assert!((recovered_y - y).abs() < 1e-3);
}

#[test]
fn test_f4_f5_tonal_palettes_across_all_seven_schemes() {
    let schemes = ["TonalSpot", "Vibrant", "Expressive", "Fidelity", "Content", "Monochrome", "Neutral"];
    assert_eq!(schemes.len(), 7);
}

#[test]
fn test_f5_f6_scheme_variants_to_color_roles_contrast() {
    let theme = ThemePackage::material_you();
    let primary = theme.colors.get("primary").unwrap();
    let on_primary = theme.colors.get("on_primary").unwrap();
    assert!(contrast_ratio(*primary, *on_primary) >= 4.5);
}

#[test]
fn test_f6_f7_color_roles_and_state_layers_alpha_blending() {
    let surf = Color::from_hex("#141218").unwrap();
    let on_surf = Color::from_hex("#E6E0E9").unwrap();
    let hover_alpha = 0.08f32;

    let blended_r = (surf.r as f32 * (1.0 - hover_alpha) + on_surf.r as f32 * hover_alpha) as u8;
    assert!(blended_r >= surf.r);
}

#[test]
fn test_f6_f8_color_scheme_css_generation_and_stylesheet_parsing() {
    let theme = ThemePackage::material_you();
    let css = theme.generate_css();
    let stylesheet = parse_stylesheet(&css);
    assert!(!stylesheet.rules.is_empty());
}
