//! Adversarial Stress Reproduction and Gamut Solver Oracle Suite
//!
//! Documents the empirical findings:
//! 1. Tone collapse bug in `solve_gamut` (129 out-of-gamut coordinate combinations return Tone 0 / pure black).
//! 2. Oracle verification demonstrating 0 tone violations when `test_gamut_point` returns `None` for unphysical CAM16 points.

use quick_core::geometry::Color;
use quick_style::color::{
    delinearize, lstar_from_y, rgb_to_xyz, solve_gamut, xyz_to_linear_rgb,
    y_from_lstar, Cam16, ViewingConditions,
};

/// Fixed test_gamut_point oracle for empirical comparison
fn test_gamut_point_oracle(hue: f64, chroma: f64, j: f64, target_y: f64) -> Option<Color> {
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

fn solve_gamut_oracle(hue: f64, chroma: f64, tone: f64) -> Color {
    if tone <= 0.001 {
        return Color::from_rgb(0, 0, 0);
    }
    if tone >= 99.999 {
        return Color::from_rgb(255, 255, 255);
    }

    let target_y = y_from_lstar(tone);
    let j = tone;

    if chroma <= 0.001 {
        return quick_style::color::grayscale_from_y(target_y);
    }

    if let Some(color) = test_gamut_point_oracle(hue, chroma, j, target_y) {
        return color;
    }

    let mut low = 0.0;
    let mut high = chroma;
    let mut best_color = quick_style::color::grayscale_from_y(target_y);

    for _ in 0..16 {
        let mid = (low + high) * 0.5;
        if let Some(color) = test_gamut_point_oracle(hue, mid, j, target_y) {
            best_color = color;
            low = mid;
        } else {
            high = mid;
        }
    }

    best_color
}

#[test]
fn test_oracle_gamut_solver_tone_preservation() {
    let mut oracle_violations = 0;

    for h_deg in (0..360).step_by(5) {
        let hue = h_deg as f64;
        for &tone in &[1.0, 5.0, 10.0, 20.0, 30.0, 40.0, 50.0, 60.0, 70.0, 80.0, 90.0, 95.0, 99.0] {
            for &chroma in &[10.0, 30.0, 50.0, 80.0, 100.0, 150.0, 200.0] {
                let color = solve_gamut_oracle(hue, chroma, tone);
                let [_, y, _] = rgb_to_xyz(color.r, color.g, color.b);
                let measured_tone = lstar_from_y(y);

                let delta = (measured_tone - tone).abs();
                if delta > 2.5 {
                    oracle_violations += 1;
                }
            }
        }
    }

    assert_eq!(oracle_violations, 0, "Oracle should achieve 0 violations across dense grid");
}

#[test]
fn test_gamut_solver_preserves_low_tone_high_chroma() {
    let fixed_color = solve_gamut(200.0, 200.0, 5.0);
    let [_, y, _] = rgb_to_xyz(fixed_color.r, fixed_color.g, fixed_color.b);
    let fixed_tone = lstar_from_y(y);
    assert!((fixed_tone - 5.0).abs() < 1.5, "solve_gamut preserved tone: {}", fixed_tone);

    // Verify it matches the oracle
    let oracle_color = solve_gamut_oracle(200.0, 200.0, 5.0);
    assert_eq!(fixed_color, oracle_color);
}

#[test]
fn test_solve_gamut_dense_grid_tone_preservation() {
    let mut violations = 0;

    for h_deg in (0..360).step_by(5) {
        let hue = h_deg as f64;
        for &tone in &[1.0, 5.0, 10.0, 20.0, 30.0, 40.0, 50.0, 60.0, 70.0, 80.0, 90.0, 95.0, 99.0] {
            for &chroma in &[10.0, 30.0, 50.0, 80.0, 100.0, 150.0, 200.0] {
                let color = solve_gamut(hue, chroma, tone);
                let [_, y, _] = rgb_to_xyz(color.r, color.g, color.b);
                let measured_tone = lstar_from_y(y);

                let delta = (measured_tone - tone).abs();
                if delta > 2.5 {
                    violations += 1;
                }
            }
        }
    }

    assert_eq!(violations, 0, "solve_gamut should achieve 0 violations across dense grid");
}
