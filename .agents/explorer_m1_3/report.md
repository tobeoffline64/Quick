# 🎨 Material You Design Tokens, Dynamic `ThemePackage` API & Dynamic CSS Generator Specification

**Milestone**: Milestone 1 (Dynamic HCT Engine & Tokens in `quick-style`)  
**Target Module**: `crates/quick-style/src/theme/` and `crates/quick-style/src/lib.rs`  
**Author**: Explorer M1.3 (`explorer_m1_3`)  
**Date**: 2026-08-30  
**Status**: Investigation Complete / Ready for Implementation  

---

## 📑 Table of Contents
1. [Executive Summary](#1-executive-summary)
2. [Design Tokens Specification & Data Structures](#2-design-tokens-specification--data-structures)
   - 2.1 [Shape Scale System (`md-sys-shape`)](#21-shape-scale-system-md-sys-shape)
   - 2.2 [Elevation & Dual-Pass Shadow System (`md-sys-elevation`)](#22-elevation--dual-pass-shadow-system-md-sys-elevation)
   - 2.3 [State Layer Opacity System (`md-sys-state`)](#23-state-layer-opacity-system-md-sys-state)
   - 2.4 [Motion & Timing Tokens (`md-sys-motion`)](#24-motion--timing-tokens-md-sys-motion)
3. [Dynamic `ThemePackage` Architecture & API Specification](#3-dynamic-themepackage-architecture--api-specification)
   - 3.1 [`ThemePackage` Struct & Invariants](#31-themepackage-struct--invariants)
   - 3.2 [Constructors & Factory Methods](#32-constructors--factory-methods)
   - 3.3 [Contrast Level & Dark Mode Handling](#33-contrast-level--dark-mode-handling)
4. [Dynamic CSS Generator (`generate_css`) Engine](#4-dynamic-css-generator-generate_css-engine)
   - 4.1 [Architecture & Selector Cascade](#41-architecture--selector-cascade)
   - 4.2 [Comprehensive Component Styling Rules](#42-comprehensive-component-styling-rules)
     - Button Rules (Filled, Tonal, Elevated, Outlined, Text, Secondary)
     - Card Rules (Elevated, Filled, Outlined)
     - Selection Controls (Switch, Checkbox, Slider, Chip)
     - Feedback & Inputs (ProgressBar, TextInput)
     - Typography & Surfaces (Text, Badges, App Roots)
5. [Codebase Organization & Integration Plan](#5-codebase-organization--integration-plan)
   - 5.1 [File Structure in `crates/quick-style/src/theme/`](#51-file-structure-in-cratesquick-stylesrctheme)
   - 5.2 [`crates/quick-style/src/lib.rs` Integration](#52-cratesquick-stylesrclibrs-integration)
   - 5.3 [Integration with `quick-markup`, `quick-widgets`, and `quick`](#53-integration-with-quick-markup-quick-widgets-and-quick)
6. [Edge Cases & Error Handling Matrix](#6-edge-cases--error-handling-matrix)
7. [Unit & Integration Test Verification Plan](#7-unit--integration-test-verification-plan)

---

## 1. Executive Summary

This report establishes the authoritative implementation specification for the **Design Tokens**, **Dynamic `ThemePackage` API**, and **Dynamic CSS Generator** in `quick-style` for Milestone 1.

Key deliverables analyzed:
1. **Design Tokens (`crates/quick-style/src/theme/tokens.rs`)**:
   - **`ShapeTokens`**: Google M3 shape scale from `corner_none` ($0\text{px}$) to `corner_full` ($9999\text{px}$).
   - **`ElevationTokens`**: Levels 0 through 5 supporting dual-pass drop shadows (directional key shadow + diffuse ambient shadow) and mathematical surface tint overlay percentages ($0\%$ to $14\%$).
   - **`StateLayerTokens`**: Standard pointer interaction alpha overlays: Hover ($8\%$), Focus ($12\%$), Pressed ($12\%$), Dragged ($16\%$), Disabled Container ($12\%$), Disabled Content ($38\%$).
2. **Dynamic `ThemePackage` API (`crates/quick-style/src/theme/package.rs`)**:
   - Programmatic constructors: `from_seed_color`, `from_seed_color_with_contrast`, `material_you`, `material_you_light`, and `nord`.
   - Full synthesis of `ColorScheme` (47 roles), `ShapeTokens`, `ElevationTokens`, and `StateLayerTokens`.
   - Complete backward compatibility with key-value map queries (`theme.colors.get("primary")`, `theme.shapes.get("corner_large")`).
3. **Dynamic CSS Generator (`generate_css`)**:
   - Generates fully qualified CSS rules for all M3 base components (`Button`, `Card`, `Switch`, `Checkbox`, `Slider`, `Chip`, `ProgressBar`, `TextInput`, `Text`) with interactive pseudo-classes (`:hover`, `:active`/`:pressed`, `:focus`).

---

## 2. Design Tokens Specification & Data Structures

### 2.1 Shape Scale System (`md-sys-shape`)

The Material Design 3 shape scale provides standardized corner radii for all interactive and container components.

#### Specification Matrix:
| Shape Token | CSS Equivalent | Radius | Typical Applications |
| :--- | :--- | :--- | :--- |
| `corner_none` | `--md-sys-shape-corner-none` | $0.0\text{ px}$ | Edge-to-edge root containers, full-bleed images/canvases |
| `corner_extra_small` | `--md-sys-shape-corner-extra-small` | $4.0\text{ px}$ | Snackbars, checkboxes, text field top corners, progress bars |
| `corner_small` | `--md-sys-shape-corner-small` | $8.0\text{ px}$ | Small chips, tooltip overlays, small dialog badges |
| `corner_medium` | `--md-sys-shape-corner-medium` | $12.0\text{ px}$ | Small dialogs, sub-cards, menus |
| `corner_large` | `--md-sys-shape-corner-large` | $16.0\text{ px}$ | Standard cards, modal bottom sheets, floating dialogs |
| `corner_extra_large`| `--md-sys-shape-corner-extra-large`| $28.0\text{ px}$ | Floating Action Buttons (FAB), search bars, nav drawers |
| `corner_full` | `--md-sys-shape-corner-full` | $9999.0\text{ px}$ | Action buttons, filter chips, pill badges, switch tracks |

#### Rust Implementation (`tokens.rs`):
```rust
use quick_core::geometry::BorderRadius;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct ShapeTokens {
    pub corner_none: f32,
    pub corner_extra_small: f32,
    pub corner_small: f32,
    pub corner_medium: f32,
    pub corner_large: f32,
    pub corner_extra_large: f32,
    pub corner_full: f32,
}

impl Default for ShapeTokens {
    fn default() -> Self {
        Self {
            corner_none: 0.0,
            corner_extra_small: 4.0,
            corner_small: 8.0,
            corner_medium: 12.0,
            corner_large: 16.0,
            corner_extra_large: 28.0,
            corner_full: 9999.0,
        }
    }
}

impl ShapeTokens {
    pub const M3: Self = Self {
        corner_none: 0.0,
        corner_extra_small: 4.0,
        corner_small: 8.0,
        corner_medium: 12.0,
        corner_large: 16.0,
        corner_extra_large: 28.0,
        corner_full: 9999.0,
    };

    pub fn to_border_radius(radius: f32) -> BorderRadius {
        BorderRadius::all(radius)
    }

    pub fn to_map(&self) -> HashMap<String, f32> {
        let mut map = HashMap::with_capacity(7);
        map.insert("corner_none".into(), self.corner_none);
        map.insert("corner_extra_small".into(), self.corner_extra_small);
        map.insert("corner_small".into(), self.corner_small);
        map.insert("corner_medium".into(), self.corner_medium);
        map.insert("corner_large".into(), self.corner_large);
        map.insert("corner_extra_large".into(), self.corner_extra_large);
        map.insert("corner_full".into(), self.corner_full);
        map
    }

    pub fn get(&self, name: &str) -> Option<f32> {
        match name {
            "corner_none" | "none" => Some(self.corner_none),
            "corner_extra_small" | "extra_small" | "xs" => Some(self.corner_extra_small),
            "corner_small" | "small" | "sm" => Some(self.corner_small),
            "corner_medium" | "medium" | "md" => Some(self.corner_medium),
            "corner_large" | "large" | "lg" => Some(self.corner_large),
            "corner_extra_large" | "extra_large" | "xl" => Some(self.corner_extra_large),
            "corner_full" | "full" | "pill" => Some(self.corner_full),
            _ => None,
        }
    }
}
```

---

### 2.2 Elevation & Dual-Pass Shadow System (`md-sys-elevation`)

Material Design 3 defines elevation as a combination of **two shadow layers** (Key + Ambient) and a **dynamic surface tint overlay**:
- **Key Shadow**: Directional, simulates direct light, sharper blur.
- **Ambient Shadow**: Omnidirectional, simulates scattered ambient light, softer and larger blur.
- **Surface Tint**: In dark and light themes, higher elevation levels receive an increasing percentage overlay of the `surface_tint` role (Primary Tone 40/80) atop the base container.

#### Specification Matrix:
| Level | Elevation ($dp$) | Key Shadow Parameters ($x, y, \text{blur}, \text{spread}, \alpha$) | Ambient Shadow Parameters ($x, y, \text{blur}, \text{spread}, \alpha$) | Surface Tint $\%$ |
| :--- | :--- | :--- | :--- | :--- |
| **Level 0** | $0\text{ dp}$ | `none` | `none` | $0\%$ ($0.0$) |
| **Level 1** | $1\text{ dp}$ | $0\text{px}, 1\text{px}, 2\text{px}, 0\text{px}, \text{rgba}(0,0,0,0.30)$ | $0\text{px}, 1\text{px}, 3\text{px}, 1\text{px}, \text{rgba}(0,0,0,0.15)$ | $5\%$ ($0.05$) |
| **Level 2** | $3\text{ dp}$ | $0\text{px}, 1\text{px}, 2\text{px}, 0\text{px}, \text{rgba}(0,0,0,0.30)$ | $0\text{px}, 2\text{px}, 6\text{px}, 2\text{px}, \text{rgba}(0,0,0,0.15)$ | $8\%$ ($0.08$) |
| **Level 3** | $6\text{ dp}$ | $0\text{px}, 1\text{px}, 3\text{px}, 0\text{px}, \text{rgba}(0,0,0,0.30)$ | $0\text{px}, 4\text{px}, 8\text{px}, 3\text{px}, \text{rgba}(0,0,0,0.15)$ | $11\%$ ($0.11$) |
| **Level 4** | $8\text{ dp}$ | $0\text{px}, 2\text{px}, 3\text{px}, 0\text{px}, \text{rgba}(0,0,0,0.30)$ | $0\text{px}, 6\text{px}, 10\text{px}, 4\text{px}, \text{rgba}(0,0,0,0.15)$ | $12\%$ ($0.12$) |
| **Level 5** | $12\text{ dp}$ | $0\text{px}, 4\text{px}, 4\text{px}, 0\text{px}, \text{rgba}(0,0,0,0.30)$ | $0\text{px}, 8\text{px}, 12\text{px}, 6\text{px}, \text{rgba}(0,0,0,0.15)$ | $14\%$ ($0.14$) |

#### Rust Implementation (`tokens.rs`):
```rust
use quick_core::geometry::Color;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Shadow {
    pub offset_x: f32,
    pub offset_y: f32,
    pub blur_radius: f32,
    pub spread_radius: f32,
    pub color: Color,
}

impl Shadow {
    pub const fn new(offset_x: f32, offset_y: f32, blur_radius: f32, spread_radius: f32, color: Color) -> Self {
        Self { offset_x, offset_y, blur_radius, spread_radius, color }
    }

    pub fn to_css(&self) -> String {
        format!(
            "{:.0}px {:.0}px {:.0}px {:.0}px rgba({}, {}, {}, {:.2})",
            self.offset_x,
            self.offset_y,
            self.blur_radius,
            self.spread_radius,
            self.color.r,
            self.color.g,
            self.color.b,
            self.color.a as f32 / 255.0
        )
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ElevationLevel {
    pub level: u8,
    pub elevation_dp: f32,
    pub key_shadow: Option<Shadow>,
    pub ambient_shadow: Option<Shadow>,
    pub surface_tint_opacity: f32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ElevationTokens {
    pub level_0: ElevationLevel,
    pub level_1: ElevationLevel,
    pub level_2: ElevationLevel,
    pub level_3: ElevationLevel,
    pub level_4: ElevationLevel,
    pub level_5: ElevationLevel,
}

impl Default for ElevationTokens {
    fn default() -> Self {
        let key_color = Color::from_rgba(0, 0, 0, 77);      // 30% alpha (76.5)
        let ambient_color = Color::from_rgba(0, 0, 0, 38);  // 15% alpha (38.25)

        Self {
            level_0: ElevationLevel {
                level: 0,
                elevation_dp: 0.0,
                key_shadow: None,
                ambient_shadow: None,
                surface_tint_opacity: 0.0,
            },
            level_1: ElevationLevel {
                level: 1,
                elevation_dp: 1.0,
                key_shadow: Some(Shadow::new(0.0, 1.0, 2.0, 0.0, key_color)),
                ambient_shadow: Some(Shadow::new(0.0, 1.0, 3.0, 1.0, ambient_color)),
                surface_tint_opacity: 0.05,
            },
            level_2: ElevationLevel {
                level: 2,
                elevation_dp: 3.0,
                key_shadow: Some(Shadow::new(0.0, 1.0, 2.0, 0.0, key_color)),
                ambient_shadow: Some(Shadow::new(0.0, 2.0, 6.0, 2.0, ambient_color)),
                surface_tint_opacity: 0.08,
            },
            level_3: ElevationLevel {
                level: 3,
                elevation_dp: 6.0,
                key_shadow: Some(Shadow::new(0.0, 1.0, 3.0, 0.0, key_color)),
                ambient_shadow: Some(Shadow::new(0.0, 4.0, 8.0, 3.0, ambient_color)),
                surface_tint_opacity: 0.11,
            },
            level_4: ElevationLevel {
                level: 4,
                elevation_dp: 8.0,
                key_shadow: Some(Shadow::new(0.0, 2.0, 3.0, 0.0, key_color)),
                ambient_shadow: Some(Shadow::new(0.0, 6.0, 10.0, 4.0, ambient_color)),
                surface_tint_opacity: 0.12,
            },
            level_5: ElevationLevel {
                level: 5,
                elevation_dp: 12.0,
                key_shadow: Some(Shadow::new(0.0, 4.0, 4.0, 0.0, key_color)),
                ambient_shadow: Some(Shadow::new(0.0, 8.0, 12.0, 6.0, ambient_color)),
                surface_tint_opacity: 0.14,
            },
        }
    }
}

impl ElevationTokens {
    pub fn get(&self, level: u8) -> &ElevationLevel {
        match level {
            0 => &self.level_0,
            1 => &self.level_1,
            2 => &self.level_2,
            3 => &self.level_3,
            4 => &self.level_4,
            _ => &self.level_5,
        }
    }

    /// Blends surface tint overlay with base surface color based on elevation level
    pub fn calculate_surface_tint(&self, level: u8, base_surface: Color, surface_tint: Color) -> Color {
        let opacity = self.get(level).surface_tint_opacity;
        if opacity <= 0.0 {
            return base_surface;
        }

        let r = (base_surface.r as f32 * (1.0 - opacity) + surface_tint.r as f32 * opacity).round() as u8;
        let g = (base_surface.g as f32 * (1.0 - opacity) + surface_tint.g as f32 * opacity).round() as u8;
        let b = (base_surface.b as f32 * (1.0 - opacity) + surface_tint.b as f32 * opacity).round() as u8;

        Color::from_rgba(r, g, b, base_surface.a)
    }

    /// Convert dual-pass shadow to standard CSS box-shadow declaration string
    pub fn to_css_box_shadow(&self, level: u8) -> String {
        let elev = self.get(level);
        match (&elev.key_shadow, &elev.ambient_shadow) {
            (Some(k), Some(a)) => format!("{}, {}", k.to_css(), a.to_css()),
            (Some(k), None) => k.to_css(),
            (None, Some(a)) => a.to_css(),
            (None, None) => "none".to_string(),
        }
    }
}
```

---

### 2.3 State Layer Opacity System (`md-sys-state`)

State layers provide interactive visual feedback across components through semi-transparent color overlays of the corresponding on-surface or on-accent color.

#### Specification Matrix:
| State | Opacity Float | Opacity $\%$ | Semantic Target Applied |
| :--- | :--- | :--- | :--- |
| **Hover** | `0.08` | $8\%$ | Pointer hovered over interactive element |
| **Focus** | `0.12` | $12\%$ | Focused interactive element (in addition to focus ring) |
| **Pressed** | `0.12` | $12\%$ | Active pointer down / click state |
| **Dragged** | `0.16` | $16\%$ | Element actively dragged (e.g. slider thumb, reorderable item) |
| **Disabled Container** | `0.12` | $12\%$ | Opacity of disabled surface/container background |
| **Disabled Content** | `0.38` | $38\%$ | Opacity of disabled label, text, or icon content |

#### Rust Implementation (`tokens.rs`):
```rust
use quick_core::geometry::Color;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct StateLayerTokens {
    pub hover: f32,
    pub focus: f32,
    pub pressed: f32,
    pub dragged: f32,
    pub disabled_container: f32,
    pub disabled_content: f32,
}

impl Default for StateLayerTokens {
    fn default() -> Self {
        Self {
            hover: 0.08,
            focus: 0.12,
            pressed: 0.12,
            dragged: 0.16,
            disabled_container: 0.12,
            disabled_content: 0.38,
        }
    }
}

impl StateLayerTokens {
    pub const M3: Self = Self {
        hover: 0.08,
        focus: 0.12,
        pressed: 0.12,
        dragged: 0.16,
        disabled_container: 0.12,
        disabled_content: 0.38,
    };

    /// Mathematical alpha blending of an overlay color onto a base color
    pub fn blend(&self, base: Color, overlay: Color, alpha: f32) -> Color {
        let alpha = alpha.clamp(0.0, 1.0);
        let r = (base.r as f32 * (1.0 - alpha) + overlay.r as f32 * alpha).round() as u8;
        let g = (base.g as f32 * (1.0 - alpha) + overlay.g as f32 * alpha).round() as u8;
        let b = (base.b as f32 * (1.0 - alpha) + overlay.b as f32 * alpha).round() as u8;
        Color::from_rgba(r, g, b, base.a)
    }

    pub fn apply_hover(&self, base: Color, on_color: Color) -> Color {
        self.blend(base, on_color, self.hover)
    }

    pub fn apply_pressed(&self, base: Color, on_color: Color) -> Color {
        self.blend(base, on_color, self.pressed)
    }

    pub fn apply_focus(&self, base: Color, on_color: Color) -> Color {
        self.blend(base, on_color, self.focus)
    }

    pub fn apply_dragged(&self, base: Color, on_color: Color) -> Color {
        self.blend(base, on_color, self.dragged)
    }

    pub fn apply_disabled_container(&self, base: Color) -> Color {
        Color::from_rgba(base.r, base.g, base.b, (base.a as f32 * self.disabled_container).round() as u8)
    }

    pub fn apply_disabled_content(&self, content: Color) -> Color {
        Color::from_rgba(content.r, content.g, content.b, (content.a as f32 * self.disabled_content).round() as u8)
    }
}
```

---

### 2.4 Motion & Timing Tokens (`md-sys-motion`)

Standard transition durations and easing curves:
```rust
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct MotionTokens {
    pub duration_short_1: u32, // 50ms
    pub duration_short_2: u32, // 100ms
    pub duration_short_3: u32, // 150ms
    pub duration_short_4: u32, // 200ms
    pub duration_medium_1: u32, // 250ms
    pub duration_medium_2: u32, // 300ms
    pub duration_medium_3: u32, // 350ms
    pub duration_medium_4: u32, // 400ms
    pub duration_long_1: u32, // 450ms
    pub duration_long_2: u32, // 500ms
}

impl Default for MotionTokens {
    fn default() -> Self {
        Self {
            duration_short_1: 50,
            duration_short_2: 100,
            duration_short_3: 150,
            duration_short_4: 200,
            duration_medium_1: 250,
            duration_medium_2: 300,
            duration_medium_3: 350,
            duration_medium_4: 400,
            duration_long_1: 450,
            duration_long_2: 500,
        }
    }
}
```

---

## 3. Dynamic `ThemePackage` Architecture & API Specification

### 3.1 `ThemePackage` Struct & Invariants

The `ThemePackage` coordinates the entire styling model. It holds:
1. Strongly typed `ColorScheme` (47 roles).
2. Strongly typed `ShapeTokens`, `ElevationTokens`, `StateLayerTokens`.
3. `colors` HashMap and `shapes` HashMap for O(1) dynamic lookups and backward compatibility.
4. Metadata: `name`, `is_dark`, and `contrast` level.

```rust
use crate::theme::color_scheme::ColorScheme;
use crate::theme::scheme::{DynamicScheme, SchemeVariant};
use crate::theme::tokens::{ElevationTokens, ShapeTokens, StateLayerTokens};
use quick_core::geometry::Color;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThemePackage {
    pub name: String,
    pub color_scheme: ColorScheme,
    pub shapes: ShapeTokens,
    pub elevation: ElevationTokens,
    pub state_layers: StateLayerTokens,
    pub is_dark: bool,
    pub contrast: f64,
    pub colors: HashMap<String, Color>,
    pub shape_map: HashMap<String, f32>,
}
```

### 3.2 Constructors & Factory Methods

```rust
impl ThemePackage {
    pub fn new(name: impl Into<String>) -> Self {
        let name_str = name.into();
        let default_color = Color::BLACK;
        let color_scheme = ColorScheme::default();
        let shapes = ShapeTokens::default();
        let elevation = ElevationTokens::default();
        let state_layers = StateLayerTokens::default();
        let colors = HashMap::new();
        let shape_map = shapes.to_map();

        Self {
            name: name_str,
            color_scheme,
            shapes,
            elevation,
            state_layers,
            is_dark: true,
            contrast: 0.0,
            colors,
            shape_map,
        }
    }

    /// Dynamic Material You generator from a seed Color
    pub fn from_seed_color(seed: Color, variant: SchemeVariant, is_dark: bool) -> Self {
        Self::from_seed_color_with_contrast(seed, variant, is_dark, 0.0)
    }

    /// Dynamic Material You generator from a seed hex string
    pub fn from_seed_hex(hex: &str, variant: SchemeVariant, is_dark: bool) -> Result<Self, String> {
        let color = Color::from_hex(hex)?;
        Ok(Self::from_seed_color(color, variant, is_dark))
    }

    /// Dynamic Material You generator with explicit WCAG contrast adjustment (-1.0 to 1.0)
    pub fn from_seed_color_with_contrast(
        seed: Color,
        variant: SchemeVariant,
        is_dark: bool,
        contrast: f64,
    ) -> Self {
        let scheme = DynamicScheme::new(seed, variant, is_dark, contrast);
        let color_scheme = scheme.to_color_scheme();
        let shapes = ShapeTokens::default();
        let elevation = ElevationTokens::default();
        let state_layers = StateLayerTokens::default();

        let colors = color_scheme.to_map();
        let shape_map = shapes.to_map();

        Self {
            name: "material-you".to_string(),
            color_scheme,
            shapes,
            elevation,
            state_layers,
            is_dark,
            contrast,
            colors,
            shape_map,
        }
    }

    /// Built-in baseline Material You Dark Theme (Default Seed #6750A4)
    pub fn material_you() -> Self {
        let seed = Color::from_rgb(103, 80, 164); // #6750A4
        Self::from_seed_color(seed, SchemeVariant::TonalSpot, true)
    }

    /// Built-in baseline Material You Light Theme (Default Seed #6750A4)
    pub fn material_you_light() -> Self {
        let seed = Color::from_rgb(103, 80, 164); // #6750A4
        Self::from_seed_color(seed, SchemeVariant::TonalSpot, false)
    }

    /// Built-in Nord Arctic Palette for backward compatibility
    pub fn nord() -> Self {
        let mut theme = Self::new("nord");
        theme.is_dark = true;

        theme.colors.insert("primary".into(), Color::from_hex("#88C0D0").unwrap());
        theme.colors.insert("on_primary".into(), Color::from_hex("#2E3440").unwrap());
        theme.colors.insert("surface".into(), Color::from_hex("#2E3440").unwrap());
        theme.colors.insert("on_surface".into(), Color::from_hex("#ECEFF4").unwrap());
        theme.colors.insert("surface_container".into(), Color::from_hex("#3B4252").unwrap());
        theme.colors.insert("surface_container_high".into(), Color::from_hex("#434C5E").unwrap());
        theme.colors.insert("outline".into(), Color::from_hex("#4C566A").unwrap());
        theme.colors.insert("outline_variant".into(), Color::from_hex("#3B4252").unwrap());
        theme.colors.insert("error".into(), Color::from_hex("#BF616A").unwrap());

        theme.shapes.corner_small = 6.0;
        theme.shapes.corner_medium = 12.0;
        theme.shapes.corner_large = 16.0;
        theme.shapes.corner_full = 9999.0;
        theme.shape_map = theme.shapes.to_map();

        theme
    }

    pub fn get_color(&self, role: &str) -> Option<Color> {
        self.colors.get(role).copied()
    }

    pub fn get_shape(&self, name: &str) -> Option<f32> {
        self.shapes.get(name)
    }
}
```

---

## 4. Dynamic CSS Generator (`generate_css`) Engine

### 4.1 Architecture & Selector Cascade

The `generate_css(&self) -> String` method evaluates the theme's `ColorScheme`, `ShapeTokens`, `ElevationTokens`, and `StateLayerTokens` to dynamically construct a CSS stylesheet.

When parsed by `quick_style::parser::parse_stylesheet`, these rules are resolved according to specificity:
1. Universal / Tag selectors: `Button`, `Card`, `Text`, `Switch`
2. Attribute selectors: `Button[variant="filled"]`, `Card[variant="elevated"]`
3. Class selectors: `Button.btn-primary`, `Card.main-card`
4. Pseudo-state selectors: `Button[variant="filled"]:hover`, `Button:active`, `TextInput:focus`

### 4.2 Comprehensive Component Styling Rules

```rust
impl ThemePackage {
    pub fn generate_css(&self) -> String {
        let mut css = String::with_capacity(4096);
        let cs = &self.color_scheme;
        let shapes = &self.shapes;
        let states = &self.state_layers;
        let elev = &self.elevation;

        // 1. Root & Base Containers
        css.push_str(&format!(
            "VStack#app-root, HStack#app-root, Container#app-root {{ background: {}; }}\n",
            cs.surface.to_hex()
        ));

        // 2. Typography
        css.push_str(&format!(
            "Text {{ color: {}; font-size: 14px; }}\n\
             Text.title {{ color: {}; font-size: 24px; font-weight: bold; }}\n\
             Text.subtitle {{ color: {}; font-size: 14px; }}\n\
             Text.body {{ color: {}; font-size: 14px; }}\n\
             Text.label {{ color: {}; font-size: 12px; }}\n\
             Text.greeting {{ color: {}; font-size: 16px; font-weight: bold; }}\n\
             Text.description {{ color: {}; font-size: 13px; }}\n\
             Text.pill-badge {{ background: {}; color: {}; border-radius: {:.0}px; font-size: 11px; font-weight: bold; padding: 6px 14px; text-align: center; }}\n",
            cs.on_surface.to_hex(),
            cs.on_surface.to_hex(),
            cs.on_surface_variant.to_hex(),
            cs.on_surface.to_hex(),
            cs.on_surface_variant.to_hex(),
            cs.primary.to_hex(),
            cs.on_surface_variant.to_hex(),
            cs.primary_container.to_hex(),
            cs.on_primary_container.to_hex(),
            shapes.corner_full
        ));

        // 3. Buttons
        // A. Filled Button (Default)
        let filled_hover = states.apply_hover(cs.primary, cs.on_primary);
        let filled_pressed = states.apply_pressed(cs.primary, cs.on_primary);
        css.push_str(&format!(
            "Button, Button[variant=\"filled\"], Button.btn-primary, Button.filled {{\n\
                 background: {};\n\
                 color: {};\n\
                 border-radius: {:.0}px;\n\
                 padding: 10px 24px;\n\
                 font-size: 14px;\n\
                 font-weight: 500;\n\
             }}\n\
             Button:hover, Button[variant=\"filled\"]:hover, Button.btn-primary:hover, Button.filled:hover {{\n\
                 background: {};\n\
             }}\n\
             Button:active, Button:pressed, Button[variant=\"filled\"]:active, Button[variant=\"filled\"]:pressed, Button.btn-primary:active, Button.btn-primary:pressed {{\n\
                 background: {};\n\
             }}\n",
            cs.primary.to_hex(),
            cs.on_primary.to_hex(),
            shapes.corner_full,
            filled_hover.to_hex(),
            filled_pressed.to_hex()
        ));

        // B. Tonal Button
        let tonal_hover = states.apply_hover(cs.secondary_container, cs.on_secondary_container);
        let tonal_pressed = states.apply_pressed(cs.secondary_container, cs.on_secondary_container);
        css.push_str(&format!(
            "Button[variant=\"tonal\"], Button.btn-tonal, Button.tonal {{\n\
                 background: {};\n\
                 color: {};\n\
                 border-radius: {:.0}px;\n\
                 padding: 10px 24px;\n\
                 font-size: 14px;\n\
                 font-weight: 500;\n\
             }}\n\
             Button[variant=\"tonal\"]:hover, Button.btn-tonal:hover, Button.tonal:hover {{\n\
                 background: {};\n\
             }}\n\
             Button[variant=\"tonal\"]:active, Button[variant=\"tonal\"]:pressed, Button.btn-tonal:active {{\n\
                 background: {};\n\
             }}\n",
            cs.secondary_container.to_hex(),
            cs.on_secondary_container.to_hex(),
            shapes.corner_full,
            tonal_hover.to_hex(),
            tonal_pressed.to_hex()
        ));

        // C. Elevated Button
        let elevated_hover = states.apply_hover(cs.surface_container_low, cs.primary);
        let elevated_pressed = states.apply_pressed(cs.surface_container_low, cs.primary);
        css.push_str(&format!(
            "Button[variant=\"elevated\"], Button.btn-elevated, Button.elevated {{\n\
                 background: {};\n\
                 color: {};\n\
                 border-radius: {:.0}px;\n\
                 padding: 10px 24px;\n\
                 font-size: 14px;\n\
                 font-weight: 500;\n\
             }}\n\
             Button[variant=\"elevated\"]:hover, Button.btn-elevated:hover, Button.elevated:hover {{\n\
                 background: {};\n\
             }}\n\
             Button[variant=\"elevated\"]:active, Button[variant=\"elevated\"]:pressed, Button.btn-elevated:active {{\n\
                 background: {};\n\
             }}\n",
            cs.surface_container_low.to_hex(),
            cs.primary.to_hex(),
            shapes.corner_full,
            elevated_hover.to_hex(),
            elevated_pressed.to_hex()
        ));

        // D. Outlined Button
        let outlined_hover = states.apply_hover(cs.surface, cs.primary);
        css.push_str(&format!(
            "Button[variant=\"outlined\"], Button.btn-outlined, Button.outlined {{\n\
                 background: transparent;\n\
                 color: {};\n\
                 border-color: {};\n\
                 border-width: 1px;\n\
                 border-radius: {:.0}px;\n\
                 padding: 10px 24px;\n\
                 font-size: 14px;\n\
                 font-weight: 500;\n\
             }}\n\
             Button[variant=\"outlined\"]:hover, Button.btn-outlined:hover, Button.outlined:hover {{\n\
                 background: {};\n\
             }}\n",
            cs.primary.to_hex(),
            cs.outline.to_hex(),
            shapes.corner_full,
            outlined_hover.to_hex()
        ));

        // E. Text Button
        let text_hover = states.apply_hover(cs.surface, cs.primary);
        css.push_str(&format!(
            "Button[variant=\"text\"], Button.btn-text, Button.text {{\n\
                 background: transparent;\n\
                 color: {};\n\
                 border-radius: {:.0}px;\n\
                 padding: 10px 16px;\n\
                 font-size: 14px;\n\
                 font-weight: 500;\n\
             }}\n\
             Button[variant=\"text\"]:hover, Button.btn-text:hover, Button.text:hover {{\n\
                 background: {};\n\
             }}\n",
            cs.primary.to_hex(),
            shapes.corner_full,
            text_hover.to_hex()
        ));

        // F. Secondary / Accent Buttons (Backward Compatibility)
        let sec_hover = states.apply_hover(cs.surface_variant, cs.on_surface_variant);
        css.push_str(&format!(
            "Button.btn-secondary {{\n\
                 background: {};\n\
                 color: {};\n\
                 padding: 10px 20px;\n\
                 border-radius: {:.0}px;\n\
                 font-size: 14px;\n\
             }}\n\
             Button.btn-secondary:hover {{\n\
                 background: {};\n\
             }}\n",
            cs.surface_variant.to_hex(),
            cs.on_surface_variant.to_hex(),
            shapes.corner_full,
            sec_hover.to_hex()
        ));

        // 4. Cards
        // Elevated Card (Default)
        css.push_str(&format!(
            "Card, Card[variant=\"elevated\"], Card.elevated, Card.main-card {{\n\
                 background: {};\n\
                 border-radius: {:.0}px;\n\
                 border-color: {};\n\
                 border-width: 1px;\n\
                 padding: 24px;\n\
                 gap: 16px;\n\
             }}\n\
             Card[variant=\"filled\"], Card.filled {{\n\
                 background: {};\n\
                 border-radius: {:.0}px;\n\
                 border-width: 0px;\n\
                 padding: 24px;\n\
                 gap: 16px;\n\
             }}\n\
             Card[variant=\"outlined\"], Card.outlined {{\n\
                 background: {};\n\
                 border-color: {};\n\
                 border-width: 1px;\n\
                 border-radius: {:.0}px;\n\
                 padding: 24px;\n\
                 gap: 16px;\n\
             }}\n",
            cs.surface_container_low.to_hex(),
            shapes.corner_large,
            cs.outline_variant.to_hex(),
            cs.surface_container_highest.to_hex(),
            shapes.corner_large,
            cs.surface.to_hex(),
            cs.outline_variant.to_hex(),
            shapes.corner_large
        ));

        // 5. Selection Controls
        // A. Switch
        css.push_str(&format!(
            "Switch {{\n\
                 background: {};\n\
                 border-color: {};\n\
                 border-width: 2px;\n\
                 border-radius: {:.0}px;\n\
             }}\n\
             Switch[checked=\"true\"], Switch.checked {{\n\
                 background: {};\n\
                 border-color: {};\n\
             }}\n",
            cs.surface_container_highest.to_hex(),
            cs.outline.to_hex(),
            shapes.corner_full,
            cs.primary.to_hex(),
            cs.primary.to_hex()
        ));

        // B. Checkbox
        css.push_str(&format!(
            "Checkbox {{\n\
                 background: transparent;\n\
                 border-color: {};\n\
                 border-width: 2px;\n\
                 border-radius: {:.0}px;\n\
             }}\n\
             Checkbox[checked=\"true\"], Checkbox.checked {{\n\
                 background: {};\n\
                 border-color: {};\n\
             }}\n",
            cs.outline.to_hex(),
            shapes.corner_extra_small,
            cs.primary.to_hex(),
            cs.primary.to_hex()
        ));

        // C. Slider
        css.push_str(&format!(
            "Slider {{\n\
                 background: {};\n\
                 color: {};\n\
                 border-radius: {:.0}px;\n\
             }}\n",
            cs.surface_container_highest.to_hex(),
            cs.primary.to_hex(),
            shapes.corner_full
        ));

        // D. Chip
        let chip_hover = states.apply_hover(cs.surface_container_low, cs.on_surface_variant);
        css.push_str(&format!(
            "Chip {{\n\
                 background: {};\n\
                 border-color: {};\n\
                 border-width: 1px;\n\
                 border-radius: {:.0}px;\n\
                 color: {};\n\
                 padding: 6px 14px;\n\
             }}\n\
             Chip:hover {{\n\
                 background: {};\n\
             }}\n\
             Chip[selected=\"true\"], Chip.selected {{\n\
                 background: {};\n\
                 color: {};\n\
                 border-color: {};\n\
             }}\n",
            cs.surface_container_low.to_hex(),
            cs.outline_variant.to_hex(),
            shapes.corner_small,
            cs.on_surface_variant.to_hex(),
            chip_hover.to_hex(),
            cs.secondary_container.to_hex(),
            cs.on_secondary_container.to_hex(),
            cs.secondary_container.to_hex()
        ));

        // 6. Progress Bar
        css.push_str(&format!(
            "ProgressBar {{\n\
                 background: {};\n\
                 color: {};\n\
                 border-radius: {:.0}px;\n\
             }}\n",
            cs.surface_container_highest.to_hex(),
            cs.primary.to_hex(),
            shapes.corner_full
        ));

        // 7. Text Input
        css.push_str(&format!(
            "TextInput, TextInput[variant=\"filled\"] {{\n\
                 background: {};\n\
                 color: {};\n\
                 border-radius: {:.0}px;\n\
                 border-width: 1px;\n\
                 border-color: transparent;\n\
                 padding: 8px 12px;\n\
             }}\n\
             TextInput[variant=\"outlined\"] {{\n\
                 background: transparent;\n\
                 color: {};\n\
                 border-color: {};\n\
                 border-width: 1px;\n\
                 border-radius: {:.0}px;\n\
                 padding: 8px 12px;\n\
             }}\n\
             TextInput:focus, TextInput:focused {{\n\
                 border-color: {};\n\
                 border-width: 2px;\n\
             }}\n",
            cs.surface_container_highest.to_hex(),
            cs.on_surface.to_hex(),
            shapes.corner_extra_small,
            cs.on_surface.to_hex(),
            cs.outline.to_hex(),
            shapes.corner_extra_small,
            cs.primary.to_hex()
        ));

        css
    }

    /// Converts generated CSS directly into a parsed StyleSheet instance
    pub fn to_stylesheet(&self) -> crate::rule::StyleSheet {
        crate::parser::parse_stylesheet(&self.generate_css())
    }
}
```

---

## 5. Codebase Organization & Integration Plan

### 5.1 File Structure in `crates/quick-style/src/theme/`

```
crates/quick-style/src/theme/
├── mod.rs             # Module declarations and unified re-exports
├── color_scheme.rs    # 47 M3 Color Roles struct, maps, and tone definitions
├── palette.rs         # TonalPalette struct & tone generation algorithms
├── scheme.rs          # SchemeVariant enum, DynamicScheme, and hue/chroma rules
├── tokens.rs          # ShapeTokens, ElevationTokens, Shadow, StateLayerTokens, MotionTokens
└── package.rs         # ThemePackage struct, constructors, and generate_css engine
```

#### `crates/quick-style/src/theme/mod.rs`:
```rust
pub mod color_scheme;
pub mod palette;
pub mod scheme;
pub mod tokens;
pub mod package;

pub use color_scheme::*;
pub use palette::*;
pub use scheme::*;
pub use tokens::*;
pub use package::*;
```

### 5.2 `crates/quick-style/src/lib.rs` Integration

```rust
pub mod color;
pub mod parser;
pub mod property;
pub mod rule;
pub mod selector;
pub mod theme;

pub use color::*;
pub use parser::*;
pub use property::*;
pub use rule::*;
pub use selector::*;
pub use theme::*;

pub mod prelude {
    pub use crate::color::*;
    pub use crate::parser::*;
    pub use crate::property::*;
    pub use crate::rule::*;
    pub use crate::selector::*;
    pub use crate::theme::*;
}
```

### 5.3 Integration with `quick-markup`, `quick-widgets`, and `quick`

1. **`quick-markup` (`crates/quick-markup/src/builder.rs`)**:
   ```rust
   // When theme attribute is present in markup:
   if let Some(ref theme_name) = doc.root.attributes.get("theme") {
       let theme = match theme_name.as_str() {
           "material-you" | "m3" => ThemePackage::material_you(),
           "nord" => ThemePackage::nord(),
           _ => ThemePackage::material_you(),
       };
       let theme_css = theme.generate_css();
       let theme_sheet = parse_stylesheet(&theme_css);
       stylesheet.rules.splice(0..0, theme_sheet.rules);
   }
   ```
2. **`quick` Facade (`crates/quick/src/app.rs`)**:
   ```rust
   impl App {
       pub fn with_theme(mut self, theme: ThemePackage) -> Self {
           let theme_css = theme.generate_css();
           let theme_sheet = parse_stylesheet(&theme_css);
           self.stylesheet.rules.splice(0..0, theme_sheet.rules);
           self
       }
   }
   ```

---

## 6. Edge Cases & Error Handling Matrix

| # | Edge Case / Scenario | Input Condition | Specified Engine Behavior | Rationale / Mitigation |
|---|----------------------|-----------------|---------------------------|------------------------|
| 1 | **Invalid Hex Color String** | `from_seed_hex("not-a-color")` or `"#zzz"` | Returns `Err(String)` error describing invalid hex syntax. Does not panic. | Safe input parsing in dynamic scripts and CLI tools. |
| 2 | **Out-of-Gamut Elevation Levels** | `elevation.get(99)` | Clamps to Level 5 (`elevation.level_5`). | Prevents index out of bounds or unhandled state. |
| 3 | **Negative or Non-Finite Opacities** | `states.blend(base, overlay, -0.5)` or `NaN` | Clamps opacity to `[0.0, 1.0]`. If `NaN`, treats as `0.0`. | Prevents arithmetic overflows in color blending. |
| 4 | **Monochrome Theme Shape & Shadows** | `from_seed_color(Color::BLACK, SchemeVariant::Monochrome, true)` | Tokens remain active; all tonal palettes are pure grayscale ($C = 0.0$). | Ensures high readability and consistency. |
| 5 | **Extreme Contrast Levels** | `contrast = 1.0` or `-1.0` | Tone calculation scales accordingly; contrast checks maintain WCAG AA compliance ($\ge 4.5:1$). | Accessibility for low-vision users. |
| 6 | **Backward Compatibility Access** | Code accessing `theme.colors.get("primary")` | `colors` HashMap contains all 47 roles in lowercase with underscores. | Preserves existing codebase integrations. |

---

## 7. Unit & Integration Test Verification Plan

### Test Matrix for `crates/quick-style/tests/` and unit modules:

1. **Shape Tokens Verification (`test_shape_tokens`)**:
   - Verify `corner_none == 0.0`, `corner_small == 8.0`, `corner_large == 16.0`, `corner_full == 9999.0`.
   - Verify `shapes.get("corner_large") == Some(16.0)` and `shapes.to_map().len() == 7`.

2. **Elevation & Dual Shadow Verification (`test_elevation_tokens`)**:
   - Verify `level_0` shadows are `None`, `level_1` through `level_5` have both key and ambient shadows.
   - Verify surface tint percentages: Level 0 (0%), Level 1 (5%), Level 2 (8%), Level 3 (11%), Level 4 (12%), Level 5 (14%).
   - Verify `calculate_surface_tint` produces mathematically correct RGB values.
   - Verify `to_css_box_shadow` produces standard CSS dual-shadow strings.

3. **State Layer Blending Verification (`test_state_layer_blending`)**:
   - Verify `apply_hover(Color::BLACK, Color::WHITE)` produces `Color::from_rgb(20, 20, 20)` ($8\%$ of 255 ≈ 20.4).
   - Verify `apply_pressed` produces $12\%$ blend.
   - Verify `apply_disabled_content` scales content alpha to $38\%$.

4. **Dynamic `ThemePackage` Verification (`test_theme_package_from_seed`)**:
   - Verify `ThemePackage::from_seed_color(Color::from_hex("#6750A4").unwrap(), SchemeVariant::TonalSpot, true)` computes complete `ColorScheme`.
   - Verify `theme.colors.get("primary")` returns non-empty Color.
   - Verify `theme.generate_css()` outputs valid CSS rules containing `Button`, `Card`, `Switch`, `TextInput`.
   - Verify `theme.to_stylesheet()` parses into non-empty `StyleSheet` rules.
