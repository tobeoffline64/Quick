# Milestone 1 Remediation Specification: Dynamic HCT Engine & Tokens in `quick-style`

**Author**: Explorer (`explorer_m1_fix`)  
**Target Milestone**: Milestone 1 Remediation  
**Date**: 2026-08-30T14:21:00Z  
**Status**: SPECIFICATION READY FOR WORKER  

---

## 1. Executive Summary

During Gate 1 review and adversarial analysis, two specific algorithmic defects were identified in `quick-style`:
1. **Gamut Bisection False Acceptance of Black** (`crates/quick-style/src/color/gamut.rs`): When CAM16 coordinates produce $y \le 10^{-9}$ for target non-zero tones ($\text{target\_y} > 10^{-9}$), returning `Some(Color(0,0,0))` caused bisection to falsely converge to pure black (Tone 0.0) on 129 out-of-gamut coordinate combinations.
2. **Light Mode Contrast Inversion for Accent Roles** (`crates/quick-style/src/theme/color_scheme.rs`): In light mode, `primary_tone`, `secondary_tone`, and `tertiary_tone` used `bg_tone(40.0, 80.0)`, which increased tone under positive contrast levels ($c > 0$) rather than decreasing tone, reducing contrast against white text (`on_primary`, Tone 100.0).

This document provides the exact code changes and verification instructions for the worker.

---

## 2. Issue 1 Analysis & Remediation: Gamut Bisection Tone Preservation

### 2.1 File: `crates/quick-style/src/color/gamut.rs`

#### Defect Mechanism
In `test_gamut_point`:
```rust
// CURRENT (BUGGY)
if y <= 1e-9 {
    return Some(Color::from_rgb(0, 0, 0));
}
```
When `target_y > 1e-9` (e.g. Tone 5.0, where $\text{target\_y} \approx 0.5535$), out-of-gamut or unphysical CAM16 coordinates (e.g. Hue 200.0, Chroma 200.0) yield $y \le 0$. Returning `Some(Color(0,0,0))` tells `solve_gamut` that pure black is a valid realization of `target_y`. Consequently, binary search bisection stores `best_color = Color(0,0,0)` and updates `low = mid`, ultimately returning pure black (Tone 0.0) instead of finding an in-gamut color with Tone 5.0 or falling back to `grayscale_from_y(target_y)`.

#### Exact Remediation
In `crates/quick-style/src/color/gamut.rs`:
Replace lines 20-22 with:
```rust
    if y <= 1e-9 {
        if target_y <= 1e-9 {
            return Some(Color::from_rgb(0, 0, 0));
        } else {
            return None;
        }
    }
```

#### Unit Test Addition
Add the following test to `crates/quick-style/src/color/gamut.rs` under `mod tests`:
```rust
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
```

---

## 3. Issue 2 Analysis & Remediation: Dynamic Contrast Tone Formula

### 3.1 File: `crates/quick-style/src/theme/color_scheme.rs`

#### Defect Mechanism
In `ColorScheme::from_core_palette_with_contrast`:
```rust
// CURRENT (BUGGY)
let primary_tone = if is_dark { fg_tone(40.0, 80.0) } else { bg_tone(40.0, 80.0) };
let secondary_tone = if is_dark { fg_tone(40.0, 80.0) } else { bg_tone(40.0, 80.0) };
let tertiary_tone = if is_dark { fg_tone(40.0, 80.0) } else { bg_tone(40.0, 80.0) };
```
In Light Mode (`is_dark = false`), `bg_tone(40.0, 80.0)` evaluates to `(40.0 + c * 4.0)`. When contrast is increased ($c = +1.0$), `primary_tone` increases to 44.0 (lighter). Because `on_primary` is Tone 100.0 (white) and `surface` is Tone 98.0, lightening the primary color reduces the contrast ratio against text from 6.44:1 down to 5.59:1.

In M3 dynamic color, foreground/accent roles (primary, secondary, tertiary, error) must move away from the background and text as contrast increases:
- Light Mode ($c > 0$): Tone must decrease (darken, $40 \to 30$) to increase contrast against white text ($100$) and light surface ($98$).
- Dark Mode ($c > 0$): Tone must increase (lighten, $80 \to 90$) to increase contrast against dark text ($20$) and dark surface ($6$).

#### Exact Remediation
In `crates/quick-style/src/theme/color_scheme.rs`, update lines 115-136:
```rust
        let primary_tone = fg_tone(40.0, 80.0);
        let on_primary_tone = if is_dark { 20.0 } else { 100.0 };
        let primary_container_tone = bg_tone(90.0, 30.0);
        let on_primary_container_tone = if is_dark { 90.0 } else { 10.0 };

        let secondary_tone = fg_tone(40.0, 80.0);
        let on_secondary_tone = if is_dark { 20.0 } else { 100.0 };
        let secondary_container_tone = bg_tone(90.0, 30.0);
        let on_secondary_container_tone = if is_dark { 90.0 } else { 10.0 };

        let tertiary_tone = fg_tone(40.0, 80.0);
        let on_tertiary_tone = if is_dark { 20.0 } else { 100.0 };
        let tertiary_container_tone = bg_tone(90.0, 30.0);
        let on_tertiary_container_tone = if is_dark { 90.0 } else { 10.0 };

        let error_tone = fg_tone(40.0, 80.0);
        let on_error_tone = if is_dark { 20.0 } else { 100.0 };
        let error_container_tone = bg_tone(90.0, 30.0);
        let on_error_container_tone = if is_dark { 90.0 } else { 10.0 };

        let surface_tone = bg_tone(98.0, 6.0);
        let on_surface_tone = fg_tone(10.0, 90.0);
```

#### Unit Test Addition
Add the following test to `crates/quick-style/src/theme/color_scheme.rs` under `mod tests`:
```rust
    #[test]
    fn test_dynamic_contrast_direction_monotonicity() {
        use crate::color::contrast_ratio;

        let seed = Color::from_hex("#6750A4").unwrap();
        let palette = CorePalette::from_seed_color(seed, SchemeVariant::TonalSpot);

        // Light mode: high contrast must have higher contrast ratio than low contrast
        let light_low = ColorScheme::from_core_palette_with_contrast(&palette, false, -1.0);
        let light_normal = ColorScheme::from_core_palette_with_contrast(&palette, false, 0.0);
        let light_high = ColorScheme::from_core_palette_with_contrast(&palette, false, 1.0);

        let cr_light_low = contrast_ratio(light_low.primary, light_low.on_primary);
        let cr_light_normal = contrast_ratio(light_normal.primary, light_normal.on_primary);
        let cr_light_high = contrast_ratio(light_high.primary, light_high.on_primary);

        assert!(cr_light_high > cr_light_normal);
        assert!(cr_light_normal > cr_light_low);
        assert!(cr_light_high >= 7.0);

        // Dark mode: high contrast must have higher contrast ratio than low contrast
        let dark_low = ColorScheme::from_core_palette_with_contrast(&palette, true, -1.0);
        let dark_normal = ColorScheme::from_core_palette_with_contrast(&palette, true, 0.0);
        let dark_high = ColorScheme::from_core_palette_with_contrast(&palette, true, 1.0);

        let cr_dark_low = contrast_ratio(dark_low.primary, dark_low.on_primary);
        let cr_dark_normal = contrast_ratio(dark_normal.primary, dark_normal.on_primary);
        let cr_dark_high = contrast_ratio(dark_high.primary, dark_high.on_primary);

        assert!(cr_dark_high > cr_dark_normal);
        assert!(cr_dark_normal > dark_low_cr);
        assert!(cr_dark_high >= 7.0);
    }
```

---

## 4. Test Suite Harmonization: `crates/quick-style/tests/adversarial_hct_stress_tests.rs`

### 4.1 Update Reproduction Test to Assertion of Fixed Behavior
In `crates/quick-style/tests/adversarial_hct_stress_tests.rs`:
Lines 126-136 previously tested for bug reproduction (`assert_eq!(bad_color, Color::from_rgb(0, 0, 0))`).
Update this test to verify the fix:
```rust
#[test]
fn test_gamut_solver_preserves_low_tone_high_chroma() {
    let fixed_color = solve_gamut(200.0, 200.0, 5.0);
    let [_, y, _] = rgb_to_xyz(fixed_color.r, fixed_color.g, fixed_color.b);
    let fixed_tone = lstar_from_y(y);
    assert!((fixed_tone - 5.0).abs() < 1.5, "solve_gamut preserved tone: {}", fixed_tone);
}
```

---

## 5. Verification Checklist for Worker

1. **Compilation**:
   `cargo check --workspace --all-targets` must complete with 0 errors and 0 warnings.
2. **Unit Tests**:
   `cargo test -p quick-style` must pass 100% of tests.
3. **Workspace Tests**:
   `cargo test --workspace` must pass 100% of tests.
4. **E2E Tests**:
   `cargo test --test e2e_m3_theme` must pass 100% (88/88 tests).
5. **No Regressions**:
   All 129 gamut coordinate collapses eliminated, contrast monotonicity confirmed across all contrast levels $\in [-1.0, 1.0]$.
