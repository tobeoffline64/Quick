//! Tone-Preserving Gamut Mapping Solver via Binary Search Bisection.

use crate::color::cam16::{Cam16, ViewingConditions};
use crate::color::cie::{delinearize, xyz_to_linear_rgb, y_from_lstar};
use quick_core::geometry::Color;

/// Generates a pure grayscale color matching the given relative luminance Y.
#[inline]
pub fn grayscale_from_y(y: f64) -> Color {
    let srgb = delinearize(y / 100.0);
    let val = (srgb * 255.0).round().clamp(0.0, 255.0) as u8;
    Color::from_rgb(val, val, val)
}

/// Tests whether an HCT point (J, Chroma, Hue) can be realized within the sRGB gamut at target Y.
pub fn test_gamut_point(hue: f64, chroma: f64, j: f64, target_y: f64) -> Option<Color> {
    let cam = Cam16::from_jch(j, chroma, hue);
    let [x, y, z] = cam.to_xyz(ViewingConditions::standard());

    if y <= 1e-9 {
        if target_y <= 1e-9 {
            return Some(Color::from_rgb(0, 0, 0));
        } else {
            return None;
        }
    }

    // Force strict tone preservation by scaling XYZ to target Y
    let scale = target_y / y;
    let x_scaled = x * scale;
    let y_scaled = target_y;
    let z_scaled = z * scale;

    let [r_lin, g_lin, b_lin] = xyz_to_linear_rgb(x_scaled, y_scaled, z_scaled);

    const TOLERANCE: f64 = 0.001;
    if r_lin < -TOLERANCE
        || r_lin > 1.0 + TOLERANCE
        || g_lin < -TOLERANCE
        || g_lin > 1.0 + TOLERANCE
        || b_lin < -TOLERANCE
        || b_lin > 1.0 + TOLERANCE
    {
        return None;
    }

    let r = delinearize(r_lin.clamp(0.0, 1.0));
    let g = delinearize(g_lin.clamp(0.0, 1.0));
    let b = delinearize(b_lin.clamp(0.0, 1.0));

    if r < -TOLERANCE
        || r > 1.0 + TOLERANCE
        || g < -TOLERANCE
        || g > 1.0 + TOLERANCE
        || b < -TOLERANCE
        || b > 1.0 + TOLERANCE
    {
        return None;
    }

    let r_u8 = (r * 255.0).round().clamp(0.0, 255.0) as u8;
    let g_u8 = (g * 255.0).round().clamp(0.0, 255.0) as u8;
    let b_u8 = (b * 255.0).round().clamp(0.0, 255.0) as u8;

    Some(Color::from_rgb(r_u8, g_u8, b_u8))
}

/// Tone-preserving gamut solver finding maximum in-gamut sRGB color preserving Hue and Tone.
pub fn solve_gamut(hue: f64, chroma: f64, tone: f64) -> Color {
    if tone <= 0.001 {
        return Color::from_rgb(0, 0, 0);
    }
    if tone >= 99.999 {
        return Color::from_rgb(255, 255, 255);
    }

    let target_y = y_from_lstar(tone);
    let j = tone;

    if chroma <= 0.001 {
        return grayscale_from_y(target_y);
    }

    // Direct test: is requested chroma already inside sRGB gamut?
    if let Some(color) = test_gamut_point(hue, chroma, j, target_y) {
        return color;
    }

    // 16-iteration binary search bisection over Chroma
    let mut low = 0.0;
    let mut high = chroma;
    let mut best_color = grayscale_from_y(target_y);

    for _ in 0..16 {
        let mid = (low + high) * 0.5;
        if let Some(color) = test_gamut_point(hue, mid, j, target_y) {
            best_color = color;
            low = mid;
        } else {
            high = mid;
        }
    }

    best_color
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gamut_boundaries_and_extreme_chroma() {
        assert_eq!(solve_gamut(280.0, 50.0, 0.0), Color::from_rgb(0, 0, 0));
        assert_eq!(solve_gamut(280.0, 50.0, 100.0), Color::from_rgb(255, 255, 255));

        // High chroma out-of-gamut bisection
        let color = solve_gamut(280.0, 150.0, 90.0);
        assert!(color.r > 200);
        assert!(color.g > 150);
        assert!(color.b > 200);
    }

    #[test]
    fn test_gamut_point_unphysical_y_rejection() {
        use crate::color::cie::{lstar_from_y, rgb_to_xyz, y_from_lstar};

        let target_y = y_from_lstar(5.0);
        // Tone 5.0 with extreme chroma 200.0 at Hue 200.0 yields y <= 0 in CAM16
        let point = test_gamut_point(200.0, 200.0, 5.0, target_y);
        assert!(point.is_none());

        // solve_gamut must preserve Tone 5.0 within tolerance rather than returning black
        let solved = solve_gamut(200.0, 200.0, 5.0);
        let [_, y, _] = rgb_to_xyz(solved.r, solved.g, solved.b);
        let measured_tone = lstar_from_y(y);
        assert!((measured_tone - 5.0).abs() < 2.0, "Expected tone ~5.0, got {}", measured_tone);
    }
}
