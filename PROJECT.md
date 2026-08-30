# Project: Quick UI Framework — Material You (M3) Complete Integration

## Architecture
The Quick UI Framework is a high-performance, pure-Rust, modular native UI library designed for 60+ FPS desktop applications on Wayland and X11. The Material You (M3) subsystem spans the following layers:
- **`quick-style`**: Core colorimetry, dynamic HCT generator (CAM16, tone gamut bisection), 6 tonal palettes, 7 dynamic scheme variants, 32+ M3 color roles (light/dark), shape/elevation/state tokens, dynamic CSS generator, and `ThemePackage`.
- **`quick-render`**: Dual-pass elevation shadow rendering (key shadow + ambient shadow), surface tint overlays, and canvas drawing primitives.
- **`quick-widgets`**: Material 3 base components (`Button` with 5 variants, `Card` with 3 variants, `Switch`, `Checkbox`, `Slider`, `Chip`, `ProgressBar`, `TextInput`) with pill geometry, state layers (hover 8%, focus 12%, pressed 12%), and dynamic token styles.
- **`quick-markup`**: Declarative XML/TOML `.quick` parser, AST resolution, dynamic theme injection (`theme="material-you"`), component variant bindings (`variant="..."`), and reactive signal properties (`$sig`, `selected`, `checked`, `value`, `progress`).
- **`quick`**: Framework facade and application runner (`App::with_theme(ThemePackage)`).
- **`apps/hello-world`**: Interactive showcase desktop application demonstrating live seed color generation, scheme switching, dark mode toggling, and all M3 widget variants.

---

## Feature Inventory
Every feature from the survey phase is mapped to a designated milestone:

| # | Feature | Description | Milestone | Source |
|---|---------|-------------|-----------|--------|
| 1 | Pure Rust CAM16 & HCT Color Space | Forward sRGB $\to$ Linear $\to$ XYZ $\to$ CAM16/HCT conversion with D65 viewing conditions | M1 | Spec §2 |
| 2 | Tone-Preserving Gamut Solver | Binary search bisection over Chroma finding maximum realizable sRGB color preserving Hue and Tone | M1 | Spec §2.3 |
| 3 | Dynamic Contrast & Tone Inversion | WCAG 2.1 relative luminance, contrast ratio calculation, and tone inversion for dark mode | M1 | Spec §2.3 |
| 4 | 6 Tonal Palettes Generation | Generation of Primary, Secondary, Tertiary, Neutral, Neutral Variant, Error palettes (tones 0..100) | M1 | Spec §3 |
| 5 | 7 Dynamic Scheme Variants | TonalSpot, Vibrant, Expressive, Fidelity, Content, Monochrome, Neutral with exact hue/chroma rules | M1 | Spec §3.1 |
| 6 | 32+ M3 Color Roles (Light & Dark) | Derivation of 47 color roles (primary, surface_container_*, outline, error, scrim, etc.) | M1 | Spec §4 |
| 7 | Design Tokens (Shapes, Elevation, State) | Shape scale (0-9999px), Elevation Levels 0-5 dual shadows & tint, State layers (8%, 12%, 16%) | M1 | Spec §5 |
| 8 | Dynamic `ThemePackage` API | `from_seed_color`, `from_seed_color_with_contrast`, `material_you`, and dynamic `generate_css` | M1 | Spec §8 |
| 9 | M3 Button Component | Filled, Tonal, Elevated, Outlined, Text variants with pill geometry and state layer feedback | M2 | Spec §6.1 |
| 10 | M3 Card Component | Elevated (dual drop shadows + surface tint), Filled, Outlined variants with M3 corner radiuses | M2 | Spec §6.2 |
| 11 | M3 Switch Selection Control | $52\times 32\text{px}$ track, $24\text{px}/16\text{px}$ sliding thumb, state layer feedback | M2 | Spec §6.3 |
| 12 | M3 Checkbox Selection Control | $24\times 24\text{px}$ touch area, $18\times 18\text{px}$ box ($r=2\text{px}$), checkmark/dash, indeterminate state | M2 | Spec §6.4 |
| 13 | M3 Slider Selection Control | $8\text{px}$ track, $20\text{px}$ thumb, continuous & discrete step ticks, state layers | M2 | Spec §6.5 |
| 14 | M3 Chip Selection Control | Filter, Assist, Input, Suggestion variants, interactive pill geometry, state layers | M2 | Spec §6.6 |
| 15 | M3 ProgressBar Component | Determinate fill ratio and indeterminate animated pulse mode | M2 | Spec §6.7 |
| 16 | M3 TextInput Component | Filled and Outlined container variants, focus stroke indicator, placeholder, cursor editing | M2 | Spec §6.8 |
| 17 | Declarative `.quick` Markup Integration | Parser support for `theme="material-you"`, `variant`, `selected`, `checked`, `value`, `progress` | M3 | Spec §7 |
| 18 | Showcase Application & Live Demo | `apps/hello-world` interactive showcase with dynamic seed colors, scheme switcher, and widget gallery | M4 | Spec §11 |
| 19 | E2E Opaque-Box Test Verification | Comprehensive 4-tier test suite (Tiers 1-4) validating 100% requirement compliance | M5 | Spec §11 |
| 20 | Adversarial Coverage Hardening (Tier 5) | White-box stress tests, edge cases, and robustness verification | M5 | Spec §11 |

---

## Milestones

| # | Name | Scope | Dependencies | Status |
|---|------|-------|-------------|--------|
| **M1** | Dynamic HCT Engine & Tokens (`quick-style`) | Pure Rust CAM16/HCT, gamut bisection, contrast ratio, 6 tonal palettes, 7 scheme variants, 32+ color roles (light/dark), shape/elevation/state tokens, `ThemePackage` dynamic APIs, and CSS generator. | None | DONE |
| **M2** | Material 3 Base Component Suite (`quick-widgets` & `quick-render`) | Button (5 variants), Card (3 variants + dual shadows), Switch, Checkbox (with indeterminate), Slider (with steps), Chip (4 variants), ProgressBar (determinate/indeterminate), TextInput (Filled/Outlined), and state layer rendering. | M1 | PLANNED |
| **M3** | Declarative `.quick` Markup Integration (`quick-markup` & `quick`) | Declarative parsing & AST binding of `theme="material-you"`, `variant`, `selected`, `checked`, `value`, `progress` attributes, dynamic theme loading, and `App::with_theme`. | M1, M2 | PLANNED |
| **M4** | Showcase Application & Live Integration (`apps/hello-world`) | Full interactive Material You demo app in `apps/hello-world` showcasing dynamic theme generation, scheme picker, dark mode switch, and all widget variants. | M1, M2, M3 | PLANNED |
| **M5** | Final Milestone: 100% E2E Test Suite & Adversarial Hardening | Pass 100% of E2E test suite (Tiers 1-4) published by E2E Testing Track, followed by Tier 5 adversarial stress testing and coverage hardening. | M1, M2, M3, M4, TEST_READY | PLANNED |

---

## Interface Contracts

### 1. `quick-style` $\leftrightarrow$ `quick-widgets` & `quick-markup`
- **HCT & Palettes**:
  ```rust
  pub struct Hct { pub hue: f64, pub chroma: f64, pub tone: f64 }
  impl Hct {
      pub fn new(hue: f64, chroma: f64, tone: f64) -> Self;
      pub fn from_color(color: Color) -> Self;
      pub fn to_color(&self) -> Color;
  }
  pub struct TonalPalette { ... }
  impl TonalPalette {
      pub fn from_hue_and_chroma(hue: f64, chroma: f64) -> Self;
      pub fn get(&self, tone: f64) -> Color;
  }
  ```
- **Scheme Variants & Color Scheme**:
  ```rust
  pub enum SchemeVariant { TonalSpot, Vibrant, Expressive, Fidelity, Content, Monochrome, Neutral }
  pub struct ColorScheme {
      pub primary: Color, pub on_primary: Color, pub primary_container: Color, pub on_primary_container: Color,
      pub secondary: Color, pub on_secondary: Color, pub secondary_container: Color, pub on_secondary_container: Color,
      pub tertiary: Color, pub on_tertiary: Color, pub tertiary_container: Color, pub on_tertiary_container: Color,
      pub error: Color, pub on_error: Color, pub error_container: Color, pub on_error_container: Color,
      pub surface: Color, pub on_surface: Color, pub surface_variant: Color, pub on_surface_variant: Color,
      pub surface_dim: Color, pub surface_bright: Color,
      pub surface_container_lowest: Color, pub surface_container_low: Color,
      pub surface_container: Color, pub surface_container_high: Color, pub surface_container_highest: Color,
      pub outline: Color, pub outline_variant: Color, pub shadow: Color, pub scrim: Color,
      pub inverse_surface: Color, pub inverse_on_surface: Color, pub inverse_primary: Color,
      pub surface_tint: Color, pub background: Color, pub on_background: Color,
  }
  ```
- **Tokens**:
  ```rust
  pub struct ShapeTokens { pub corner_none: f32, pub corner_extra_small: f32, pub corner_small: f32, pub corner_medium: f32, pub corner_large: f32, pub corner_extra_large: f32, pub corner_full: f32 }
  pub struct ElevationTokens { ... }
  pub struct StateLayerTokens { pub hover: f32, pub focus: f32, pub pressed: f32, pub dragged: f32, pub disabled_container: f32, pub disabled_content: f32 }
  ```
- **Theme Package API**:
  ```rust
  impl ThemePackage {
      pub fn from_seed_color(seed: Color, variant: SchemeVariant, is_dark: bool) -> Self;
      pub fn from_seed_color_with_contrast(seed: Color, variant: SchemeVariant, is_dark: bool, contrast_level: f64) -> Self;
      pub fn material_you() -> Self;
      pub fn generate_css(&self) -> String;
  }
  ```

### 2. `quick-widgets` Component Signatures
- `Button::new(text)` / `with_variant(ButtonVariant)` (`Filled`, `Tonal`, `Elevated`, `Outlined`, `Text`)
- `Card::new()` / `with_variant(CardVariant)` (`Elevated`, `Filled`, `Outlined`) / `with_elevation(u8)`
- `Switch::new(checked_signal)`
- `Checkbox::new(checked_signal)` / `with_indeterminate(bool_signal)`
- `Slider::new(value_signal, min, max)` / `with_steps(Option<u32>)`
- `Chip::new(text)` / `with_variant(ChipVariant)` (`Filter`, `Assist`, `Input`, `Suggestion`) / `with_selected(signal)`
- `ProgressBar::new(progress_signal)` / `indeterminate(bool)`
- `TextInput::new()` / `with_variant(InputVariant)` (`Filled`, `Outlined`) / `with_placeholder(text)`

---

## Code Layout

```
crates/
├── quick-core/src/               # Primitives: Point, Rect, Color, Signal, Events
├── quick-style/src/
│   ├── lib.rs
│   ├── color/                    # HCT, CAM16, CIELAB L*, gamut bisection, contrast
│   │   ├── mod.rs
│   │   ├── cam16.rs
│   │   ├── hct.rs
│   │   ├── gamut.rs
│   │   └── contrast.rs
│   ├── theme/                    # Dynamic schemes, tonal palettes, tokens, ThemePackage
│   │   ├── mod.rs
│   │   ├── palette.rs
│   │   ├── scheme.rs
│   │   ├── color_scheme.rs
│   │   ├── tokens.rs
│   │   └── package.rs
│   ├── property.rs
│   ├── rule.rs
│   └── selector.rs
├── quick-render/src/             # Canvas primitives, dual drop-shadows, surface tinting
├── quick-widgets/src/
│   ├── lib.rs
│   ├── button.rs                 # 5 M3 variants, pill shape, state layers
│   ├── card.rs                   # 3 M3 variants, elevation levels 0-5, dual shadows
│   ├── switch.rs                 # M3 track & thumb proportions, state layers
│   ├── checkbox.rs               # Checkmark, indeterminate dash, state layers
│   ├── slider.rs                 # Track, thumb, discrete step ticks, state layers
│   ├── chip.rs                   # Filter, assist, input, suggestion pill chips
│   ├── progress.rs               # Determinate fill, indeterminate animation
│   ├── text_input.rs             # Filled & outlined styles, focus indicator
│   └── state_layer.rs            # Reusable state layer blending helper
├── quick-markup/src/             # Parser, AST, builder, theme loading, attribute bindings
├── quick/src/                    # App facade: App::with_theme(ThemePackage)
apps/
└── hello-world/                  # Interactive M3 showcase application
    ├── src/main.rs
    └── ui/app.quick
themes/
└── material-you.theme.toml       # M3 default token definition
tests/                            # Integration and E2E test suites
```
