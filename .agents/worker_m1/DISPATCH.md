## 2026-08-30T14:07:14Z
You are the Worker implementing Milestone 1: Dynamic HCT Engine & Tokens in `quick-style` for the Quick UI Framework.
Your working directory is: /home/ai-workspace/coding-repo/quick-silver/.agents/worker_m1

Your exclusive write ownership for this milestone:
- `crates/quick-style/src/color/*` (`mod.rs`, `cam16.rs`, `hct.rs`, `gamut.rs`, `contrast.rs`)
- `crates/quick-style/src/theme/*` (`mod.rs`, `palette.rs`, `scheme.rs`, `color_scheme.rs`, `tokens.rs`, `package.rs`)
- `crates/quick-style/src/lib.rs`
- `crates/quick-style/src/theme.rs`
- `crates/quick-core/src/geometry/color.rs`

Your tasks:
1. Implement pure Rust CAM16 and CIELAB L* color model in `crates/quick-style/src/color/cam16.rs` and `hct.rs`:
   - Standard D65 viewing conditions ($L_A=11.7257$, $Y_b=18.4187$, $F=0.8$, $c=0.69$, $N_c=1.0$).
   - sRGB to Linear sRGB to CIE XYZ to CAM16 forward pipeline.
   - CAM16 inverse to XYZ to Linear sRGB to 8-bit sRGB.
   - Tone $T = L^*$ calculation from CIE $Y$.
2. Implement tone-preserving gamut mapping solver in `crates/quick-style/src/color/gamut.rs`:
   - Binary search bisection over Chroma $C$ (16 iterations, tolerance 0.01) finding maximum realizable sRGB color strictly preserving Hue $h$ and Tone $T$.
3. Implement dynamic contrast and WCAG 2.1 calculations in `crates/quick-style/src/color/contrast.rs`:
   - Relative luminance and contrast ratio $(L1 + 0.05) / (L2 + 0.05)$.
4. Implement 6 Tonal Palettes in `crates/quick-style/src/theme/palette.rs`:
   - `TonalPalette` and `CorePalette` (primary, secondary, tertiary, neutral, neutral_variant, error) across tones $0..100$.
5. Implement 7 Scheme Variants in `crates/quick-style/src/theme/scheme.rs`:
   - `SchemeVariant`: `TonalSpot`, `Vibrant`, `Expressive`, `Fidelity`, `Content`, `Monochrome`, `Neutral` with exact hue rotation and chroma formulas.
6. Implement 32+ M3 Color Roles in `crates/quick-style/src/theme/color_scheme.rs`:
   - `ColorScheme` struct with all 47 light and dark roles (primary, on_primary, primary_container, on_primary_container, secondary, tertiary, surface, surface_dim, surface_bright, surface_container_lowest..highest, outline, outline_variant, error, scrim, shadow, etc.).
7. Implement Design Tokens in `crates/quick-style/src/theme/tokens.rs`:
   - `ShapeTokens` (`corner_none` 0px, `corner_extra_small` 4px, `corner_small` 8px, `corner_medium` 12px, `corner_large` 16px, `corner_extra_large` 28px, `corner_full` 9999px).
   - `ElevationTokens` (Levels 0..5 dual shadows and dynamic surface tint).
   - `StateLayerTokens` (hover 8%, focus 12%, pressed 12%, dragged 16%, disabled 38%/12%).
8. Implement Dynamic `ThemePackage` in `crates/quick-style/src/theme/package.rs`:
   - `from_seed_color(seed, variant, is_dark)`, `from_seed_color_with_contrast`, `material_you()`, `nord()`, `generate_css()`.
9. Update `crates/quick-style/src/lib.rs` and re-exports.
10. Run build and test verification:
    - `cargo check --workspace --all-targets`
    - `cargo test -p quick-style`
    - Add comprehensive unit tests in `crates/quick-style` verifying all algorithms, contrast calculations, and CSS generation.
