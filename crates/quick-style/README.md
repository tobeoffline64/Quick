# quick-style

**Dynamic styling engine for the Quick UI Framework** — Material You (M3) design tokens, HCT color generation, CSS parsing, and theming.

## Overview

`quick-style` implements the complete [Google Material Design 3](https://m3.material.io/) design system in pure Rust. It provides:

- **HCT Color Space** — Hue, Chroma, Tone color science matching Google's `material-color-utilities`
- **Dynamic Tonal Palettes** — Derive mathematically harmonious tonal palettes from any seed color
- **M3 Color Roles** — 32+ named roles (`primary`, `surface`, `on_surface`, `outline`, `error`, …) for Light and Dark modes
- **Design Tokens** — Shape scales, elevation shadows (0–5), state layer opacities, and motion curves
- **CSS Generator** — Serialize any `ThemePackage` to a CSS variable sheet for runtime injection
- **CSS Parser** — Parse inline and stylesheet CSS into typed `StyleSheet` / `Rule` / `Style` structures

## Architecture

```
quick-style
├── color/               # Pure-Rust HCT Color Science
│   ├── cam16.rs         # CAM16 perceptual color appearance model
│   ├── cie.rs           # CIE XYZ ↔ linear sRGB conversions
│   ├── gamut.rs         # sRGB gamut mapping and clamping
│   ├── hct.rs           # HCT (Hue, Chroma, Tone) color space
│   └── contrast.rs      # WCAG contrast ratio calculations
├── theme/               # M3 Design System
│   ├── palette.rs       # TonalPalette — 13 standard tonal steps (0…100)
│   ├── scheme.rs        # 7 SchemeVariants (TonalSpot, Vibrant, Expressive, …)
│   ├── color_scheme.rs  # 32-field ColorScheme (all M3 color roles)
│   ├── tokens.rs        # ShapeTokens, ElevationTokens, StateLayerTokens, MotionTokens
│   └── package.rs       # ThemePackage — unified theme API + CSS generator
├── parser.rs            # CSS stylesheet & inline style parser
├── property.rs          # Typed style properties (Color, Dimension, FlexDirection, …)
├── rule.rs              # StyleSheet, Rule, Selector structures
└── selector.rs          # CSS selector matching
```

## Quick Start

```rust
use quick_style::theme::{ThemePackage, SchemeVariant};
use quick_core::geometry::Color;

// Generate a full Material You theme from any seed color
let seed = Color::from_hex("#6750A4").unwrap();
let theme = ThemePackage::from_seed_color(seed, SchemeVariant::TonalSpot, false);

println!("primary = {}", theme.color_scheme.primary.to_hex());
println!("surface = {}", theme.color_scheme.surface.to_hex());

// Generate CSS variables for runtime injection
let css = theme.generate_css();

// Use built-in Material You preset
let preset = ThemePackage::material_you();
```

## Scheme Variants

| Variant | Character |
|---------|-----------|
| `TonalSpot` *(default)* | Balanced, calm pastel accents |
| `Vibrant` | High-saturation, punchy |
| `Expressive` | Playful contrasting harmonies |
| `Fidelity` | Closely matches seed color |
| `Content` | Muted, content-forward |
| `Monochrome` | Pure grayscale |
| `Neutral` | Subtle, low-chroma |
