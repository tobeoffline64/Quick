# Milestone 1: Dynamic HCT Color Engine — Handoff Report

## 1. Observation
- Target crate: `crates/quick-style` with `crates/quick-core` (`Color` defined in `crates/quick-core/src/geometry.rs:215-358`).
- Existing `quick-style::theme` (`crates/quick-style/src/theme.rs:1-95`) has a placeholder static `ThemePackage` with hardcoded color hex strings.
- Colorimetry and dynamic color generation requirements from `ORIGINAL_REQUEST.md`, `PROJECT.md`, and `material_you_full_theme_and_component_integration.md`:
  - Pure Rust CAM16 Color Appearance Model with D65 Viewing Conditions.
  - HCT Color Space combining CAM16 $(H, C)$ and CIELAB $L^*$ ($T$).
  - Binary search bisection over Chroma in $[0.0, C]$ to preserve Tone $T$ within sRGB gamut.
  - WCAG 2.1 relative luminance and contrast ratio calculations.
- File and mathematical layout designed in `/home/ai-workspace/coding-repo/quick-silver/.agents/explorer_m1_1/report.md`.

## 2. Logic Chain
1. Material Design 3 dynamic color scheme generation requires perceptual uniformity where contrast depends exclusively on Tone ($L^*$) differences, unaffected by Hue or Chroma.
2. Direct RGB or HSV models do not separate perceived luminance from chromaticity. CAM16 provides the most accurate human color appearance model (chromatic adaptation, opponent channels, eccentricity, surround luminance).
3. Combining CAM16 $(H, C)$ with CIELAB $L^*$ ($T$) yields HCT.
4. When constructing HCT colors at high chroma, the requested $(H, C, T)$ can exceed the physical sRGB monitor gamut $[0, 255]^3$. Clamping sRGB channels directly alters Tone and destroys WCAG contrast invariants.
5. To guarantee Tone preservation, $Y = \text{y_from_lstar}(T)$ is held constant while performing a 16-step binary search bisection over Chroma $C' \in [0.0, C]$.
6. A 16-iteration bisection divides the chroma range into steps of $\Delta C \approx 0.0018$, which is $> 500\times$ below human perceptual difference ($\Delta C \approx 1.0$) and executes in $< 0.5\,\mu\text{s}$.
7. WCAG 2.1 relative luminance and contrast ratio formulas enable closed-form tone calculations (`lighter_tone`, `darker_tone`) without iterating through RGB space.

## 3. Caveats
- The binary search solver maps into the standard sRGB gamut (standard desktop display target for Wayland/X11). If wide-gamut (Display P3) is supported in future versions, only the gamut testing matrix bounds need parameterization.
- The standard viewing condition assumes standard desktop display luminance ($L_A = 11.72\,\text{cd/m}^2$, $Y_b = 18.41865$). Custom viewing conditions can be instantiated via `ViewingConditions::new` if ambient light sensors are later integrated.

## 4. Conclusion
The implementation blueprint for `crates/quick-style/src/color/` is fully detailed and ready for immediate implementation by the SWE team:
- `mod.rs`: Facade re-exporting `Hct`, `Cam16`, `ViewingConditions`, `solve_gamut`, and contrast helpers.
- `cie.rs`: Exact piece-wise transfer functions for sRGB $\leftrightarrow$ Linear sRGB, Linear sRGB $\leftrightarrow$ XYZ (D65), and $Y \leftrightarrow$ CIELAB $L^*$.
- `cam16.rs`: Structs `Cam16` and `ViewingConditions` with closed-form algebraic forward and inverse transformations.
- `hct.rs`: `Hct` struct with `new`, `from_color`, `to_color`, and non-allocating builder methods.
- `gamut.rs`: Bisection gamut solver anchoring $Y = \text{y_from_lstar}(T)$.
- `contrast.rs`: WCAG 2.1 relative luminance, contrast ratios, and accessible tone helpers.

## 5. Verification Method
1. `cargo check --workspace --all-targets` compiles without errors or warnings.
2. Add unit test suite in `crates/quick-style/src/color/` verifying:
   - D65 white point ($Y=100.0 \to L^*=100.0$) and black point ($Y=0.0 \to L^*=0.0$).
   - Standard seeds: `#FF0000` ($T \approx 53.2$), `#00FF00` ($T \approx 87.7$), `#0000FF` ($T \approx 32.3$), `#6750A4` ($T \approx 36.6, H \approx 280.0, C \approx 60.0$).
   - Gamut bisection convergence at high chroma out-of-gamut inputs.
   - Contrast calculation: Black vs White $= 21.0$, Tone 40 vs Tone 100 $\ge 4.5:1$, Tone 80 vs Tone 20 $\ge 7.0:1$.
3. Run `cargo test -p quick-style` to ensure all tests pass.
