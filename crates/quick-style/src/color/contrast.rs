//! WCAG 2.1 Contrast Ratio and Tone Calculation Engine.

use crate::color::cie::{linearize, lstar_from_y, y_from_lstar};
use quick_core::geometry::Color;

/// Computes WCAG 2.1 relative luminance Y in range [0.0, 1.0].
pub fn relative_luminance(color: Color) -> f64 {
    let r = linearize(color.r as f64 / 255.0);
    let g = linearize(color.g as f64 / 255.0);
    let b = linearize(color.b as f64 / 255.0);
    0.2126 * r + 0.7152 * g + 0.0722 * b
}

/// Computes WCAG 2.1 contrast ratio between two sRGB colors in range [1.0, 21.0].
pub fn contrast_ratio(color_a: Color, color_b: Color) -> f64 {
    let y1 = relative_luminance(color_a);
    let y2 = relative_luminance(color_b);
    let lighter = y1.max(y2);
    let darker = y1.min(y2);
    (lighter + 0.05) / (darker + 0.05)
}

/// Computes contrast ratio directly between two CIELAB tones in range [0.0, 100.0].
pub fn contrast_ratio_tones(tone_a: f64, tone_b: f64) -> f64 {
    let y1 = y_from_lstar(tone_a) / 100.0;
    let y2 = y_from_lstar(tone_b) / 100.0;
    let lighter = y1.max(y2);
    let darker = y1.min(y2);
    (lighter + 0.05) / (darker + 0.05)
}

/// Finds the tone that achieves the specified contrast ratio above `tone`.
pub fn lighter_tone(tone: f64, ratio: f64) -> f64 {
    let dark_y = y_from_lstar(tone) / 100.0;
    let light_y = (dark_y + 0.05) * ratio - 0.05;
    if light_y > 1.0 {
        100.0
    } else {
        lstar_from_y(light_y * 100.0).clamp(0.0, 100.0)
    }
}

/// Finds the tone that achieves the specified contrast ratio below `tone`.
pub fn darker_tone(tone: f64, ratio: f64) -> f64 {
    let light_y = y_from_lstar(tone) / 100.0;
    let dark_y = (light_y + 0.05) / ratio - 0.05;
    if dark_y < 0.0 {
        0.0
    } else {
        lstar_from_y(dark_y * 100.0).clamp(0.0, 100.0)
    }
}

/// Returns true if the contrast ratio between color_a and color_b meets or exceeds min_ratio.
pub fn is_accessible(color_a: Color, color_b: Color, min_ratio: f64) -> bool {
    contrast_ratio(color_a, color_b) >= min_ratio
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_contrast_ratios_and_accessibility() {
        let black = Color::BLACK;
        let white = Color::WHITE;

        let cr = contrast_ratio(black, white);
        assert!((cr - 21.0).abs() < 0.01, "Contrast ratio black/white: {}", cr);
        assert!(is_accessible(black, white, 4.5));
        assert!(is_accessible(black, white, 7.0));

        let cr_same = contrast_ratio(black, black);
        assert!((cr_same - 1.0).abs() < 0.01);

        // Tone contrast invariant: Tone 40 vs Tone 100 should be >= 4.5:1
        let cr_40_100 = contrast_ratio_tones(40.0, 100.0);
        assert!(cr_40_100 >= 4.5, "Tone 40 vs 100 CR: {}", cr_40_100);

        // Tone 80 vs Tone 20 should be >= 7.0:1
        let cr_80_20 = contrast_ratio_tones(80.0, 20.0);
        assert!(cr_80_20 >= 7.0, "Tone 80 vs 20 CR: {}", cr_80_20);

        // Dynamic tone solvers
        let light = lighter_tone(40.0, 4.5);
        assert!(light >= 85.0, "lighter_tone(40, 4.5) = {}", light);

        let dark = darker_tone(80.0, 4.5);
        assert!((dark - 35.41).abs() < 1.0, "darker_tone(80, 4.5) = {}", dark);
    }
}
