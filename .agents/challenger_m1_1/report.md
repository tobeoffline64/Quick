# Milestone 1 Adversarial Challenge Report: Dynamic HCT Engine & Tokens in `quick-style`

**Challenger**: Challenger 1 (Empirical Challenger)  
**Target Milestone**: Milestone 1 (Dynamic HCT Engine & Tokens in `quick-style`)  
**Verdict**: `REQUEST_CHANGES`  
**Overall Risk Assessment**: HIGH  

---

## 1. Executive Summary & Verdict

We executed an adversarial empirical stress-testing suite across the colorimetry, CAM16 transforms, tone-preserving gamut solver, tonal palette generation, scheme variants, dynamic contrast solvers, and design token generators in `crates/quick-style`.

While the CIE XYZ transforms, D65 viewing conditions, shape tokens, elevation tokens, state layer blending, and baseline light/dark `ColorScheme` generation are robust and compliant with Material Design 3 specifications, our empirical test harnesses identified **two concrete defects** in `quick-style`:

1. **[CRITICAL] Tone Collapse in Gamut Bisection Solver (`crates/quick-style/src/color/gamut.rs:20-22`)**:
   `test_gamut_point` improperly returns `Some(Color::from_rgb(0, 0, 0))` whenever `Cam16::to_xyz` produces $y \le 1e-9$ for unphysical/out-of-domain CAM16 coordinates at high chroma and low-to-moderate tones (predominantly in the cyan/blue hue range $200^\circ - 280^\circ$). Because `Some(...)` is returned when $target\_y > 0$, `solve_gamut` falsely treats pure black as an in-gamut match, causing **129 test configurations** across a 5-degree hue sweep to collapse to pure black `Color { r: 0, g: 0, b: 0 }` (Tone 0.0). This directly breaks tonal palette luminance monotonicity for high-chroma seeds (e.g. pure Blue `#0000FF` at Tone 4).
2. **[MEDIUM] Inverted Primary Contrast Calculation in Light Mode (`crates/quick-style/src/theme/color_scheme.rs:115`)**:
   In `ColorScheme::from_core_palette_with_contrast`, `primary_tone` in Light Mode uses `bg_tone(40.0, 80.0)`, which adds `+ c * 4.0`. When a user requests higher contrast ($c = +1.0$), `primary_tone` increases to 44.0 (lighter), reducing the contrast ratio against white `on_primary` (Tone 100) from $6.44:1$ down to $5.59:1$.

---

## 2. Adversarial Empirical Stress Test Results

| Test Dimension | Scenarios Tested | Status | Findings / Anomalies |
| :--- | :--- | :--- | :--- |
| **Gamut Boundaries & Extremes** | Tone 0.0, Tone 100.0, Negative Chroma, Huge Chroma (200, 500, 1000), Extreme Hues ($-1080^\circ \dots 1080^\circ$) | ⚠️ **FAILED** (Resolved by Oracle) | Solvers collapse to pure black (Tone 0.0) on out-of-gamut coordinates at low tones ($T=5, C=200, H=200..280$) due to gamut point check defect. |
| **Gamut Bisection Dense 360° Sweep** | Hue $0^\circ..360^\circ$ (1° and 5° steps) $\times$ Tones $1..99$ $\times$ Chromas $0..200$ | ⚠️ **FAILED** (129 Tone Violations) | 129 coordinate combinations collapse to Tone 0 instead of converging to target Tone $T$. |
| **Special Colors Roundtrip** | Pure Black (`#000000`), Pure White (`#FFFFFF`), Mid-Gray (`#808080`), RGB Primaries & Secondaries | ✅ **PASSED** | Perfect roundtrip fidelity for standard display boundaries. |
| **Tonal Palette Monotonicity** | 6 Tonal Palettes across 101 integer tones ($0..100$) for 15+ seed colors | ⚠️ **FAILED** (for Blue seed) | Non-monotonicity at Tone 4 for Blue seed due to Gamut Solver Tone Collapse bug. |
| **WCAG 2.1 Contrast Invariants** | `primary`/`on_primary`, `secondary`/`on_secondary`, `tertiary`/`on_tertiary`, `error`/`on_error`, `surface`/`on_surface` | ✅ **PASSED** (Baseline) | All role pairs achieve $\ge 4.5:1$ contrast ratio in baseline Light and Dark modes. |
| **Dynamic Contrast Adjustment** | Contrast levels $c \in [-1.0, 1.0]$ | ⚠️ **FAILED** (Light primary tone) | Light mode `primary_tone` increases with $+c$, diminishing contrast against white `on_primary`. |
| **7 Scheme Variants** | `TonalSpot`, `Vibrant`, `Expressive`, `Fidelity`, `Content`, `Monochrome`, `Neutral` across 60+ seed colors | ✅ **PASSED** | Palette derivation formulas, error palette invariant ($H=25, C=84$), and monochrome rules strictly adhere to M3 spec. |
| **Tokens & ThemePackage CSS** | `ShapeTokens` (0..9999px), `ElevationTokens` (Levels 0..5 dual shadows & tint), `StateLayerTokens` (hover, focus, pressed), CSS generator | ✅ **PASSED** | Generates valid CSS stylesheets with full M3 widget selectors, corner radiuses, and state layer overlays. |
| **NaN / Inf Robustness** | `f64::NAN`, `f64::INFINITY`, `f64::NEG_INFINITY` passed to `solve_gamut`, `Hct::new`, and contrast functions | ✅ **PASSED** | Zero panics; inputs sanitize gracefully without infinite loops. |

---

## 3. Detailed Challenges & Defect Analysis

### Challenge 1 (CRITICAL): Unphysical CAM16 Coordinate Tone Collapse in `solve_gamut`

- **Affected File**: `crates/quick-style/src/color/gamut.rs` (lines 16–23)
- **Observed Behavior**:
  ```rust
  pub fn test_gamut_point(hue: f64, chroma: f64, j: f64, target_y: f64) -> Option<Color> {
      let cam = Cam16::from_jch(j, chroma, hue);
      let [x, y, z] = cam.to_xyz(ViewingConditions::standard());

      if y <= 1e-9 {
          return Some(Color::from_rgb(0, 0, 0)); // <--- BUG: Returns Some(BLACK) even when target_y > 0!
      }
      ...
  }
  ```
- **Empirical Evidence**:
  Executing `solve_gamut(200.0, 200.0, 5.0)` (Hue 200, Chroma 200, Tone 5.0) returns `Color { r: 0, g: 0, b: 0, a: 255 }` (Tone 0.0, luminance 0.0) instead of searching for the in-gamut color with Tone 5.0.
  Across a dense test sweep (5° hue steps), **129 coordinate combinations** exhibit this tone collapse.
- **Root Cause**:
  For unphysical CAM16 coordinates (e.g. high chroma at low tone where CAM16 denominator is negative), `cam.to_xyz()` returns $y \le 0$. If `target_y > 1e-9`, returning `Some(Color(0,0,0))` tells the caller that pure black $(Y=0)$ satisfies the requested non-zero target luminance `target_y`.
- **Recommended Remediation**:
  In `crates/quick-style/src/color/gamut.rs`:
  ```rust
  if y <= 1e-9 {
      if target_y <= 1e-9 {
          return Some(Color::from_rgb(0, 0, 0));
      } else {
          return None;
      }
  }
  ```
  Our verified oracle test (`test_oracle_gamut_solver_tone_preservation` in `tests/adversarial_hct_stress_tests.rs`) proves that applying this change eliminates all 129 tone violations across the entire 360-degree color space ($0$ violations remaining).

---

### Challenge 2 (MEDIUM): Inverted Tone Direction for Primary Role in Light Mode Contrast Adjustment

- **Affected File**: `crates/quick-style/src/theme/color_scheme.rs` (lines 107–115)
- **Observed Behavior**:
  ```rust
  let bg_tone = |base_light: f64, base_dark: f64| -> f64 {
      if is_dark {
          (base_dark - c * 6.0).clamp(0.0, 100.0)
      } else {
          (base_light + c * 4.0).clamp(0.0, 100.0)
      }
  };

  let primary_tone = if is_dark { fg_tone(40.0, 80.0) } else { bg_tone(40.0, 80.0) };
  let on_primary_tone = if is_dark { 20.0 } else { 100.0 };
  ```
- **Empirical Evidence**:
  For baseline seed `#6750A4` in Light Mode:
  - Normal Contrast ($c=0.0$): `primary_tone = 40.0`, `on_primary_tone = 100.0` $\to$ Contrast Ratio = **6.44:1**
  - High Contrast ($c=+1.0$): `primary_tone = 44.0`, `on_primary_tone = 100.0` $\to$ Contrast Ratio = **5.59:1**
- **Root Cause**:
  In Light Mode, `primary` acts as a foreground element against `surface` (Tone 98) and a background for `on_primary` (Tone 100). Making `primary` lighter ($40 \to 44$) reduces contrast against white text ($100$) and reduces contrast against light background ($98$).
- **Recommended Remediation**:
  In Light Mode, `primary_tone` should be calculated using `fg_tone(40.0, 80.0)` or `(40.0 - c * 10.0)` so high contrast deepens the primary tone to 30.0, boosting contrast against `on_primary` to $\ge 7.0:1$.

---

## 4. Test Artifacts Delivered

The following empirical test harnesses were created in `crates/quick-style/tests/`:
1. `tests/adversarial_hct_stress_tests.rs`:
   - `test_reproduce_gamut_solver_unphysical_black_collapse`: Reproduces the black collapse bug in `solve_gamut`.
   - `test_oracle_gamut_solver_tone_preservation`: Demonstrates the exact oracle solution with 0 tone violations across 360 degrees.
2. `tests/adversarial_m1_comprehensive_tests.rs`:
   - `test_adversarial_cie_pipeline_extremes`
   - `test_adversarial_hct_extremes_and_boundary_handling`
   - `test_adversarial_special_colors_fidelity`
   - `test_adversarial_scheme_variants_parsing_and_contract`
   - `test_adversarial_color_scheme_contrast_and_accessibility_matrix`
   - `test_adversarial_contrast_ratio_monotonicity`
   - `test_adversarial_design_tokens_and_theme_package`

---

## 5. Conclusion & Action Items

We issue a formal verdict of **`REQUEST_CHANGES`** for Milestone 1.

### Required Actions Before Approval:
1. Update `test_gamut_point` in `crates/quick-style/src/color/gamut.rs` to return `None` when $y \le 1e-9$ and $target\_y > 1e-9$.
2. Update `primary_tone` calculation in `crates/quick-style/src/theme/color_scheme.rs` for light mode contrast adjustment.
3. Re-run `cargo test -p quick-style` to confirm 100% test success rate across all adversarial suites.
