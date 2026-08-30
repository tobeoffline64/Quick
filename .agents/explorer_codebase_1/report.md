# 🔍 Comprehensive Codebase Architecture & Material You (M3) Implementation Blueprint

**Author**: Explorer Codebase Agent (`explorer_codebase_1`)  
**Date**: 2026-08-30  
**Target Project**: Quick Native UI Framework (`quick-silver`)  
**Reference Documents**: `ORIGINAL_REQUEST.md`, `material_you_full_theme_and_component_integration.md`

---

## 1. Executive Summary

An exhaustive exploration of the Quick UI Framework codebase (`quick-silver`) was conducted to evaluate the current architecture and formulate a concrete implementation blueprint for Google Material Design 3 (Material You / M3). 

The workspace consists of a high-performance, pure-Rust, modular crate architecture:
- `quick-core`: Geometry primitives (`Point`, `Rect`, `Size`, `Insets`, `BorderRadius`, `Color`, `Transform`), reactive fine-grained signal graph (`Signal<T>`, `create_computed`, `create_effect`, `batch`), event system (`Event`, `PointerEvent`, `KeyEvent`, `ScrollDelta`, `FocusEvent`).
- `quick-style`: Property definitions (`Style`, `Dimension`, `FlexDirection`, `JustifyContent`, `AlignItems`), selector specificity engine, SIMD-accelerated CSS parser (`parse_stylesheet`, `parse_inline_style`), rule matching, and `ThemePackage`.
- `quick-render`: Display list recording `Canvas` with an ephemeral per-frame Bump arena (`bumpalo`), software rasterizer (`SoftwareRasterizer`), and optional Skia 2D GPU pipeline (`RenderPipeline`).
- `quick-layout`: Flexbox layout computation backed by Taffy (`LayoutEngine`).
- `quick-widgets`: Native headless base widgets (`Button`, `Card`, `Switch`, `Checkbox`, `Slider`, `Chip`, `ProgressBar`, `TextInput`, `Text`, `Container`, `VStack`, `HStack`).
- `quick-markup`: Declarative `.quick` parser, TOML/XML parsers, and UI graph builder (`DataContext`, `build_ui_tree`).
- `quick-window` & `quick`: Native Wayland/X11 window runner (`winit`, `softbuffer`, `raw-window-handle`) and application coordinator (`App`).
- `apps/hello-world` & `examples/*`: Showcase applications.

While the foundation is highly solid, performant, and cleanly separated, the current theming in `quick-style` and component suite in `quick-widgets` only have preliminary placeholder implementations of Material You (e.g., hardcoded static colors in `ThemePackage::material_you()`, simplified button click opacity multipliers, single-layer box shadows on cards, and missing M3 variants). This report provides the full architectural analysis, gap assessment, and exact engineering blueprints for full M3 integration.

---

## 2. In-Depth Analysis of `quick-style`

### 2.1 Current Data Structures & Color Representations
- **`Color` (`quick_core::geometry::Color`)**:
  - Representation: 8-bit RGBA `(r: u8, g: u8, b: u8, a: u8)` with hex string parsing (`#RGB`, `#RGBA`, `#RRGGBB`, `#RRGGBBAA`, `rgb(...)`, `rgba(...)`, named CSS colors).
  - Conversions: `to_hex() -> String`, `to_argb_u32() -> u32`, `to_rgba_f32() -> [f32; 4]`.
  - Missing: Perceptual color spaces (CIELAB, CAM16, HCT), Linear RGB conversions, luminance / contrast ratio calculation (`(L1 + 0.05) / (L2 + 0.05)`).
- **`Style` (`quick_style::property::Style`)**:
  - Holds optional layout and presentation properties:
    - Layout: `width`, `height`, `min_width`, `min_height`, `max_width`, `max_height`, `padding`, `margin`, `flex_direction`, `justify_content`, `align_items`, `gap`.
    - Presentation: `background_color`, `text_color`, `border_color`, `border_width`, `border_radius`, `opacity`, `font_family`, `font_size`, `font_weight`, `text_align`.
  - Merging: `merge_with(&mut self, other: &Self)` performs property-by-property overwrite for `Some` fields.
- **`Dimension`**:
  - `Auto`, `Px(f32)`, `Percent(f32)`.
- **`Insets` & `BorderRadius`**:
  - `Insets`: `top`, `right`, `bottom`, `left` (supports `all`, `symmetric`, `new`).
  - `BorderRadius`: `top_left`, `top_right`, `bottom_right`, `bottom_left` (supports `all`, `new`).

### 2.2 Selectors, Rules, and Stylesheet Resolution
- **`Selector` (`quick_style::selector::Selector`)**:
  - Matches element name, class name, ID, pseudo-state (`Hover`, `Active`, `Focused`, `Disabled`), and attribute key-value pairs (e.g., `Button[variant="filled"]`, `Card[variant="elevated"]`).
  - Specificity calculation: ID (+100), Class (+10), Attribute (+10), PseudoState (+10), Element (+1).
- **`StyleSheet` & `StyleRule` (`quick_style::rule::StyleSheet`)**:
  - `resolve_with_attrs(element, classes, id, state, attributes) -> Style` finds all matching rules, sorts by specificity ascending, and merges styles into a computed `Style`.
- **`ResourceDictionary`**:
  - Key-value store mapping strings to `serde_json::Value` with typed getters (`get<T>`) and setters (`set<T>`).

### 2.3 Current `ThemePackage` Implementation
Currently located in `crates/quick-style/src/theme.rs`:
```rust
pub struct ThemePackage {
    pub name: String,
    pub colors: HashMap<String, Color>,
    pub shapes: HashMap<String, f32>,
}
```
- Only provides static hardcoded methods: `ThemePackage::material_you()` and `ThemePackage::nord()`.
- `generate_css(&self) -> String` emits a 3-rule CSS string for `Button.btn-primary`, `Card`, and `Text`.
- **Deficiencies**:
  - No dynamic HCT (Hue, Chroma, Tone) generation algorithm.
  - No Scheme variants (`TonalSpot`, `Vibrant`, `Expressive`, `Fidelity`, `Content`, `Monochrome`, `Neutral`, `FruitSalad`).
  - No `ThemePackage::from_seed_color(seed, variant, is_dark)`.
  - No derivation of all 32+ M3 color roles.
  - No Elevation token system (Levels 0 through 5 with key/ambient shadow definitions).
  - No State Layer opacity tokens (Hover 8%, Focus 12%, Pressed 12%, Dragged 16%, Disabled 38%/12%).
  - No Shape Scale registry (`corner-none` 0px, `corner-extra-small` 4px, `corner-small` 8px, `corner-medium` 12px, `corner-large` 16px, `corner-extra-large` 28px, `corner-full` 9999px).

---

## 3. In-Depth Analysis of `quick-widgets`

### 3.1 The `Widget` Trait (`crates/quick-widgets/src/widget.rs`)
The universal headless interface for all UI elements:
```rust
pub trait Widget {
    fn widget_type(&self) -> &'static str;
    fn id(&self) -> Option<&str> { None }
    fn classes(&self) -> &[String] { &[] }
    fn style(&self) -> &Style;
    fn style_mut(&mut self) -> &mut Style;

    fn build_layout(&mut self, engine: &mut LayoutEngine) -> Result<NodeId, TaffyError>;
    fn update_layout(&mut self, engine: &LayoutEngine, parent_origin: Point) { ... }
    fn paint(&self, canvas: &mut Canvas, bounds: Rect);
    fn handle_event(&mut self, event: &Event, bounds: Rect) -> bool { false }
}
```

### 3.2 Existing Base Widgets Breakdown

| Widget | Internal Structure | Layout Lifecycle | Paint / Rendering | Event Handling | Current Limitations |
| :--- | :--- | :--- | :--- | :--- | :--- |
| **`Button`** | `text: String`, `on_click: Option<Box<dyn FnMut()>>`, `is_hovered: bool`, `is_pressed: bool`, `style: Style` | Estimates width based on char count $\times$ font_size $\times 0.55 + \text{padding}$, creates Taffy leaf. | Fills rounded/sharp rect, strokes border if specified, draws centered text. Darkens RGB by 30% on press, brightens by 15% on hover. | Captures `PointerPhase::Down` on left click, fires `on_click` on `PointerPhase::Up` within bounds. | Lacks M3 variants (`Filled`, `Tonal`, `Elevated`, `Outlined`, `Text`), icon slot, pill geometry (`corner-full`), and true M3 state layer overlays. |
| **`Card`** | Wraps `container: Container`, `variant: CardVariant` (`Elevated`, `Filled`, `Outlined`). | Delegates layout creation and bounds updates to inner `Container`. | If `Elevated`, draws single drop-shadow rect `(0, 3px)` with alpha 80, then paints `container`. | Delegates pointer events to `container.handle_event`. | Single-pass shadow is crude; lacks M3 elevation levels (0-5), ambient + key dual shadows, surface tinting, and M3 corner scales. |
| **`Switch`** | `checked: Signal<bool>`, `on_change: Option<Box<dyn FnMut(bool)>>`, `is_hovered: bool`, `is_pressed: bool`. | Fixed $52 \times 32\text{px}$ leaf in layout engine. | Paints pill track (`radius = height / 2`), border stroke if unselected, sliding thumb circle (size 24px when checked, 18px when unchecked). | `PointerPhase::Up` toggles `checked` signal and calls `on_change`. | Hardcoded track/thumb colors, lacks state layer ripple, icon inside thumb, and disabled state handling. |
| **`Checkbox`** | `checked: Signal<bool>`, `on_change: Option<Box<dyn FnMut(bool)>>`, `is_hovered: bool`, `is_pressed: bool`. | Fixed $24 \times 24\text{px}$ leaf. | Draws $20 \times 20\text{px}$ rounded rect (`r = 4px`). If checked, fills and draws 2-segment checkmark line; if unchecked, draws 2px border. | `PointerPhase::Up` toggles `checked` and calls `on_change`. | Missing indeterminate state (dash icon), error state, and state layer feedback. |
| **`Slider`** | `value: Signal<f32>`, `min: f32`, `max: f32`, `on_change: Option<Box<dyn FnMut(f32)>>`, `is_dragging: bool`. | Height 36px, width 100% leaf. | Draws 8px inactive track pill, active track fill proportional to value ratio, and 20px diameter thumb circle (`r = 10px`). | Tracks pointer drag (`Down`, `Moved`, `Up`), computes ratio `(x - left) / width`, updates signal & triggers `on_change`. | Lacks discrete step ticks, value tooltip / bubble, state layers on thumb, and theme token styling. |
| **`Chip`** | `text: String`, `selected: Option<Signal<bool>>`, `on_click: Option<Box<dyn FnMut()>>`, `is_hovered: bool`, `is_pressed: bool`. | Width computed from text length + padding, height 32px. | Pill container (`corner-full`), border stroke (1px), centered text. Active/selected state switches background to `#4A4458` and border to `#CCC2DC`. | `PointerPhase::Up` toggles `selected` signal (if present) and fires `on_click`. | Lacks M3 chip categories (Filter, Assist, Input, Suggestion), leading icon, trailing remove button, and state layer blending. |
| **`ProgressBar`** | `progress: Signal<f32>`, `min: f32`, `max: f32`. | Height 8px, width 100% leaf. | Draws 8px inactive background track pill, active progress bar pill filled with primary color. | None (presentation-only widget). | Missing indeterminate animation mode, buffer progress, and dynamic theme tokens. |
| **`TextInput`** | `value: String`, `placeholder: String`, `on_change: Option<Box<dyn FnMut(String)>>`, `is_focused: bool`, `cursor_pos: usize`. | Width 180px, height 34px. | Background rect, border (accented when `is_focused`), text / placeholder rendered with bitmap font. | Click toggles `is_focused`. `KeyEvent` updates string on text input and Backspace/Delete. | Lacks M3 Outlined / Filled container styles, floating label animation, helper / error text, leading / trailing icons. |
| **`Text`** | `source: TextSource` (`Static(String)` or `Dynamic(Signal<String>)`). | Width/height estimated from character count and font size. | Paints background/border if styled, draws text with horizontal alignment (`Left`, `Center`, `Right`). | None. | Fully functional; needs M3 typography scale tokens (display, headline, title, body, label). |
| **`Container` / `VStack` / `HStack`** | `children: Vec<Box<dyn Widget>>`, `child_bounds: Vec<Rect>`. | Recursively builds Taffy layout for children with `FlexDirection::Column` / `Row`, updates absolute coordinates. | Paints background / border, then paints children in order. | Hit-tests children in reverse z-order, propagates pointer and focus clearing events. | Solid layout primitive. |

---

## 4. How Widgets Access Theme Colors, Shadows, Shapes, and States

### 4.1 The Current Styling Pipeline
```mermaid
flowchart LR
    QuickMarkup["app.quick / XML / TOML"] --> Parser["quick-markup::quick_parser"]
    Parser --> Document["UiDocument (AST)"]
    Document --> Builder["quick-markup::builder::build_ui_tree"]
    ThemePackage["ThemePackage (material_you / nord)"] -->|generate_css()| CSS["StyleSheet Rules"]
    CSS --> Builder
    Builder --> Resolve["StyleSheet::resolve_with_attrs()"]
    Resolve --> WidgetStyle["Widget.style (Merged)"]
    WidgetStyle --> WidgetPaint["Widget::paint(canvas, bounds)"]
```
1. In `quick-markup::builder::build_ui_tree`, if `theme="material-you"` is present on the root node, `ThemePackage::material_you()` generates static CSS rules via `generate_css()`.
2. For each node, `stylesheet.resolve_with_attrs(element, classes, id, state, attributes)` evaluates matching rules and populates `widget.style`.
3. During `paint(&self, canvas: &mut Canvas, bounds: Rect)`, the widget reads properties directly from `self.style` (e.g. `self.style.background_color`, `self.style.border_radius`). If a style property is `None`, the widget falls back to hardcoded default color constants in its source file.

### 4.2 Deficiencies in the Current Pipeline
1. **No Runtime Token Propagation**: If the theme changes at runtime (e.g., light $\leftrightarrow$ dark mode, wallpaper seed color update), widgets have no reference to the active `ThemePackage` unless the entire UI tree is rebuilt.
2. **Missing Token Roles in Selectors**: The current `generate_css()` outputs only 3 basic CSS rules. All other widgets fall back to static fallback colors in `paint()`.
3. **No Elevation Shader / Dual-Pass Shadow Pipeline**: `quick-render`'s `Canvas` provides `FillRect`, `FillRoundedRect`, `StrokeRoundedRect`, etc., but lacks a dedicated shadow primitive or composite shadow commands that handle M3 ambient + key shadows with surface tinting.
4. **Ad-Hoc State Highlights**: Widgets simulate hover/pressed states by manually multiplying RGB values (e.g. `(r * 0.7)`) instead of layering an `on_surface` / `on_primary` state layer at 8% (hover), 12% (focus), 12% (pressed).

---

## 5. Comprehensive Gap Analysis against Material Design 3 (M3)

| M3 Requirement | Specification Target | Current Codebase State | Critical Gaps to Bridge |
| :--- | :--- | :--- | :--- |
| **HCT Dynamic Color Engine** | Pure Rust Matugen / `material-colors` algorithm deriving tones 0–100 from any seed HEX or image. | Hardcoded static hex values in `crates/quick-style/src/theme.rs`. | Implement HCT color space conversions (CAM16, CIELAB $L^*$, sRGB), tone searching bisection, and `TonalPalette`. |
| **Scheme Variants** | `TonalSpot`, `Vibrant`, `Expressive`, `Fidelity`, `Content`, `Monochrome`, `Neutral`, `FruitSalad`. | None. | Implement `SchemeVariant` enum and palette generation rules for each variant in `quick-style::theme`. |
| **32+ M3 Color Roles** | Primary, Secondary, Tertiary, Neutral, NeutralVariant, Error, Surfaces (Lowest to Highest), Outlines, Scrim, Shadow. | Only 11 hardcoded colors in `ThemePackage`. | Define `ColorScheme` struct containing all 32+ light & dark roles with contrast calculation. |
| **Theme Creation API** | `ThemePackage::from_seed_color(seed, variant, is_dark)` & `from_toml(...)` & `from_image(...)`. | Only `ThemePackage::material_you()` and `ThemePackage::nord()`. | Add full builder APIs and TOML deserialization matching `themes/material-you.theme.toml`. |
| **Shape Scale System** | `corner-none` (0), `corner-extra-small` (4), `corner-small` (8), `corner-medium` (12), `corner-large` (16), `corner-extra-large` (28), `corner-full` (9999). | Partial `shapes` HashMap with 4 values (`corner_small`, `medium`, `large`, `full`). | Implement `ShapeTokens` and map to all standard component radiuses. |
| **Elevation & Dual Shadows** | Levels 0–5 with ambient + key drop shadows and dynamic surface tinting (0%–14%). | Hardcoded single rect offset on `Card::Elevated`. | Implement `ElevationTokens`, `Shadow` descriptor, and dual-pass shadow drawing helpers in `quick-render` / `quick-style`. |
| **State Layer Engine** | Hover (8%), Focus (12%), Pressed (12%), Dragged (16%), Disabled (38%/12%). | Primitive RGB scaling (`* 0.7` / `* 1.15`). | Implement `StateLayerTokens` and state layer alpha blending in widget drawing. |
| **Button Variants** | `Filled`, `Tonal`, `Elevated`, `Outlined`, `Text` with pill shape and state layer feedback. | Only generic Button with hardcoded blue background. | Implement `ButtonVariant` enum, attribute parsing (`variant="..."`), and full CSS selector mappings. |
| **Card Variants** | `Elevated` (dynamic shadow), `Filled`, `Outlined` with M3 container tones. | Minimal `CardVariant` with hardcoded colors and primitive shadow. | Connect variants to `surface_container_*`, M3 elevation levels, and proper borders. |
| **Selection Controls** | `Switch` (pill track + thumb + icons), `Checkbox` (rounded square + check/indeterminate mark), `Slider` (track + thumb + steps), `Chip` (filter/assist pills). | Implemented but using hardcoded colors and missing variants/states. | Upgrade with M3 dimensions, state layers, indeterminate state, chip variants, and theme color binding. |
| **Progress & Text Inputs** | `ProgressBar` (determinate & indeterminate), `TextInput` (filled & outlined with floating label / helper). | Basic progress bar and plain text input. | Add indeterminate animation support, M3 input variant styling, and theme color roles. |
| **Declarative Markup & Runtime** | `theme="material-you"` loading, `App::with_theme`, reactive signal attribute binding. | Partial support in `quick-markup`, missing `App::with_theme`. | Implement `App::with_theme`, update `build_ui_tree` to inject full M3 CSS, and support all M3 widget attributes. |
| **Verification & Showcase** | Zero errors/warnings, 100% test pass, updated `apps/hello-world`. | Existing tests pass, but hello-world uses manual CSS overrides. | Add comprehensive test suite for HCT color math and widget rendering; update `apps/hello-world`. |

---

## 6. Concrete Implementation Blueprint

### 6.1 Architectural Map of Required Changes

```text
crates/
├── quick-core/
│   └── src/geometry.rs                     # Add Color blending & luminance / contrast helpers
├── quick-style/
│   ├── Cargo.toml
│   └── src/
│       ├── lib.rs                          # Export new modules & tokens
│       ├── hct/                            # [NEW] Pure Rust HCT & CAM16 Engine
│       │   ├── mod.rs                      # Module root, HCT coordinate struct
│       │   ├── cam16.rs                    # CAM16 viewing conditions & forward/inverse transforms
│       │   ├── color_utils.rs              # sRGB <-> Linear <-> XYZ <-> L*a*b* conversions
│       │   ├── palette.rs                  # TonalPalette (Tones 0..100)
│       │   └── scheme.rs                   # Scheme variants (TonalSpot, Vibrant, Expressive, etc.)
│       ├── tokens/                         # [NEW] Design Tokens
│       │   ├── mod.rs
│       │   ├── color_roles.rs              # Complete 32+ M3 Light & Dark Color Roles
│       │   ├── elevation.rs                # Elevation Levels 0..5 (Dual ambient & key shadows + tint)
│       │   ├── shapes.rs                   # Shape Scale System (corner_none to corner_full)
│       │   └── state_layers.rs             # State Layer Opacity Tokens
│       ├── theme.rs                        # Enhanced ThemePackage (from_seed_color, from_toml, generate_css)
│       └── parser.rs                       # Enhanced CSS parser for M3 tokens and attribute selectors
├── quick-render/
│   └── src/
│       ├── canvas.rs                       # Add DrawCommand::DrawShadow and DrawCommand::FillStateLayer
│       ├── rasterizer.rs                   # Software rasterization for shadows & alpha overlays
│       └── pipeline.rs                     # Skia implementation for dual-pass shadows and state layers
├── quick-widgets/
│   └── src/
│       ├── button.rs                       # Button variants (Filled, Tonal, Elevated, Outlined, Text) + State Layers
│       ├── card.rs                         # Card variants (Elevated, Filled, Outlined) with M3 Elevation Levels
│       ├── switch.rs                       # M3 Switch pill track, thumb ratios, state layers
│       ├── checkbox.rs                     # M3 Checkbox with check & indeterminate dash strokes
│       ├── slider.rs                       # M3 Slider with active/inactive track, thumb, and step ticks
│       ├── chip.rs                         # M3 Chip variants (Filter, Assist, Input, Suggestion)
│       ├── progress.rs                     # M3 ProgressBar (Determinate and Indeterminate)
│       └── text_input.rs                   # M3 TextInput (Filled and Outlined variants)
├── quick-markup/
│   └── src/
│       └── builder.rs                      # Bind variant attributes, wire complete M3 CSS rules
├── quick/
│   └── src/
│       └── app.rs                          # Add App::with_theme(ThemePackage)
├── themes/
│   └── material-you.theme.toml             # Full M3 configuration file
└── apps/hello-world/
    ├── app.quick                           # Comprehensive M3 Showcase UI
    └── src/main.rs                         # Integration with ThemePackage::from_seed_color
```

---

### 6.2 Module-by-Module Implementation Blueprint

#### Crate 1: `quick-core`
- **File: `crates/quick-core/src/geometry.rs`**
  - Add color manipulation methods to `Color`:
    - `with_alpha(self, alpha: u8) -> Color`
    - `blend_over(self, background: Color) -> Color` (standard alpha compositing)
    - `linear_rgb(self) -> (f32, f32, f32)` (sRGB gamma expansion)
    - `relative_luminance(self) -> f32` ($Y = 0.2126 R_{\text{lin}} + 0.7152 G_{\text{lin}} + 0.0722 B_{\text{lin}}$)
    - `contrast_ratio(self, other: Color) -> f32` ($(\max(Y_1, Y_2) + 0.05) / (\min(Y_1, Y_2) + 0.05)$)

#### Crate 2: `quick-style`
- **Module: `crates/quick-style/src/hct/`**
  - `color_utils.rs`:
    - `rgb_to_xyz(r, g, b) -> (f32, f32, f32)` using D65 reference white.
    - `xyz_to_rgb(x, y, z) -> (u8, u8, u8)` with clamping.
    - `y_to_lstar(y: f32) -> f32` and `lstar_to_y(lstar: f32) -> f32`.
  - `cam16.rs`:
    - CAM16 color appearance model calculations: viewing conditions, chromatic adaptation, cone response $RGB_c$, hue angle $h$, chroma $C$, lightness $J$, tone $Q$.
  - `mod.rs` (`Hct` struct):
    - `Hct::from_rgb(color: Color) -> Hct`
    - `Hct::from_hct(hue: f32, chroma: f32, tone: f32) -> Hct`
    - `to_color(&self) -> Color` using bisection search on Chroma for in-gamut sRGB color at target Tone ($L^*$).
  - `palette.rs` (`TonalPalette`):
    - `TonalPalette::from_hue_and_chroma(hue: f32, chroma: f32) -> TonalPalette`
    - `tone(&self, tone: u32) -> Color` (precomputes tones 0, 10, 20, 30, 40, 50, 60, 70, 80, 90, 95, 99, 100).
  - `scheme.rs` (`SchemeVariant` & `DynamicScheme`):
    - Enum `SchemeVariant`: `TonalSpot`, `Vibrant`, `Expressive`, `Fidelity`, `Content`, `Monochrome`, `Neutral`, `FruitSalad`.
    - Generates 6 key palettes: `primary`, `secondary`, `tertiary`, `neutral`, `neutral_variant`, `error`.
    - Generates all 32+ roles for both Light and Dark modes.

- **Module: `crates/quick-style/src/tokens/`**
  - `color_roles.rs`:
    - Struct `ColorScheme` containing all 32+ named `Color` roles (`primary`, `on_primary`, `primary_container`, `on_primary_container`, `inverse_primary`, `secondary`, `on_secondary`, `secondary_container`, `on_secondary_container`, `tertiary`, `on_tertiary`, `tertiary_container`, `on_tertiary_container`, `surface`, `on_surface`, `surface_variant`, `on_surface_variant`, `surface_container_lowest`, `surface_container_low`, `surface_container`, `surface_container_high`, `surface_container_highest`, `surface_dim`, `surface_bright`, `surface_tint`, `outline`, `outline_variant`, `error`, `on_error`, `error_container`, `on_error_container`, `shadow`, `scrim`, `inverse_surface`, `inverse_on_surface`).
  - `shapes.rs`:
    - Struct `ShapeTokens` defining:
      - `corner_none: f32 = 0.0`
      - `corner_extra_small: f32 = 4.0`
      - `corner_small: f32 = 8.0`
      - `corner_medium: f32 = 12.0`
      - `corner_large: f32 = 16.0`
      - `corner_extra_large: f32 = 28.0`
      - `corner_full: f32 = 9999.0`
  - `elevation.rs`:
    - Struct `ElevationLevel` (Levels 0 through 5) defining key shadow offset/blur/alpha, ambient shadow offset/blur/alpha, and surface tint factor (0%, 5%, 8%, 11%, 12%, 14%).
  - `state_layers.rs`:
    - Struct `StateLayerTokens`: `hover = 0.08`, `focus = 0.12`, `pressed = 0.12`, `dragged = 0.16`, `disabled_content = 0.38`, `disabled_container = 0.12`.

- **Enhanced `ThemePackage` (`crates/quick-style/src/theme.rs`)**:
  - Struct fields:
    ```rust
    pub struct ThemePackage {
        pub name: String,
        pub is_dark: bool,
        pub variant: SchemeVariant,
        pub color_scheme: ColorScheme,
        pub shapes: ShapeTokens,
        pub state_layers: StateLayerTokens,
        pub elevations: [ElevationLevel; 6],
    }
    ```
  - Factory methods:
    - `ThemePackage::from_seed_color(seed: &str, variant: SchemeVariant, is_dark: bool) -> Self`
    - `ThemePackage::from_image(path: &str, variant: SchemeVariant, is_dark: bool) -> Result<Self, String>`
    - `ThemePackage::from_toml(toml_str: &str) -> Result<Self, String>`
    - `ThemePackage::material_you() -> Self` (defaults to `#6750A4`, `TonalSpot`, `is_dark: true`)
  - Full CSS Generator (`generate_css(&self) -> String`):
    - Emits comprehensive CSS rules for all widget elements and variant attribute selectors (`Button[variant="filled"]`, `Button[variant="tonal"]`, `Button[variant="elevated"]`, `Button[variant="outlined"]`, `Button[variant="text"]`, `Card[variant="elevated"]`, `Card[variant="filled"]`, `Card[variant="outlined"]`, `Switch`, `Checkbox`, `Slider`, `Chip`, `ProgressBar`, `TextInput[variant="outlined"]`, `TextInput[variant="filled"]`, `Text`, etc.) using the dynamically generated M3 tokens.

---

#### Crate 3: `quick-render`
- **File: `crates/quick-render/src/canvas.rs`**
  - Add draw commands:
    - `DrawCommand::DrawShadow { rect: Rect, radius: BorderRadius, elevation: u8, tint_color: Option<Color> }`
    - `DrawCommand::DrawStateLayer { rect: Rect, radius: BorderRadius, color: Color, opacity: f32 }`
  - Add helper methods on `Canvas`:
    - `draw_elevation_shadow(&mut self, rect: Rect, radius: BorderRadius, level: u8)`
    - `draw_state_layer(&mut self, rect: Rect, radius: BorderRadius, content_color: Color, opacity: f32)`
- **File: `crates/quick-render/src/rasterizer.rs`**
  - Implement dual-pass shadow rasterization (key shadow pass + ambient shadow pass) using soft alpha bounding rects.
- **File: `crates/quick-render/src/pipeline.rs`**
  - Implement Skia dual-pass shadow rendering using `skia_safe::Point::new(dx, dy)` and `skia_safe::Paint` blur masks.

---

#### Crate 4: `quick-widgets`
- **`button.rs`**:
  - Add `pub enum ButtonVariant { Filled, Tonal, Elevated, Outlined, Text }`.
  - Add `pub variant: ButtonVariant`, `pub icon: Option<String>`.
  - Default shape: `BorderRadius::all(9999.0)` (pill geometry).
  - In `paint()`:
    1. For `Elevated`, draw elevation shadow Level 1 (or Level 2 on hover).
    2. Draw base container fill according to variant (`primary`, `secondary_container`, `surface_container_low`, `transparent`).
    3. Draw border if `Outlined` (`outline`).
    4. Draw state layer overlay if `is_hovered` (8%), `is_focused` (12%), or `is_pressed` (12%) using `on_surface` / `on_primary` color.
    5. Draw centered text and optional icon with correct M3 `on_*` content color.
- **`card.rs`**:
  - Support `CardVariant::Elevated`, `CardVariant::Filled`, `CardVariant::Outlined`.
  - Add `pub elevation: u8` (default 1 for `Elevated`, 0 for `Filled`/`Outlined`).
  - In `paint()`:
    1. If `variant == CardVariant::Elevated`, draw dual-pass shadow at `elevation` level.
    2. Fill container with `surface_container_low` (Elevated), `surface_container_highest` (Filled), or `surface` (Outlined).
    3. If `Outlined`, stroke 1px border with `outline_variant`.
    4. Paint child widgets.
- **`switch.rs`**:
  - Dimensions: $52 \times 32\text{px}$.
  - Track: Pill with `corner-full`. Unselected: `surface_container_highest` with 2px `outline` border. Selected: `primary` fill.
  - Thumb: Unselected: 16px diameter, `outline` color. Selected: 24px diameter, `on_primary` color.
  - State layer: 40px diameter circle centered on thumb with 8% (hover) / 12% (pressed) opacity.
- **`checkbox.rs`**:
  - Box size: $18 \times 18\text{px}$ with `corner_extra_small` (4px).
  - Add `pub is_indeterminate: bool`.
  - Unchecked: 2px `outline` border.
  - Checked: `primary` fill, white checkmark stroke.
  - Indeterminate: `primary` fill, white horizontal bar stroke.
  - State layer: 40px diameter circular feedback on interaction.
- **`slider.rs`**:
  - Track height: 8px (or 16px active track in M3 expressiveness). Inactive track: `surface_container_highest`. Active track: `primary`.
  - Thumb: 20px pill/circle `primary` with state layer ring.
  - Add support for `steps: Option<u32>` and stop indicators.
- **`chip.rs`**:
  - Add `pub enum ChipVariant { Assist, Filter, Input, Suggestion }`.
  - Height: 32px, `corner-small` (8px) or `corner-full` (pill).
  - Unselected: `surface` with 1px `outline_variant` border, `on_surface_variant` text.
  - Selected: `secondary_container` fill, `on_secondary_container` text, checkmark icon.
- **`progress.rs`**:
  - Track: 4px height, `surface_container_highest`.
  - Add `pub is_indeterminate: bool`.
  - Determinate: `primary` fill pill matching progress ratio.
  - Indeterminate: pulsating / animating sliding pulse.
- **`text_input.rs`**:
  - Add `pub enum TextInputVariant { Outlined, Filled }`.
  - Outlined: 1px `outline` border (2px `primary` on focus), 4px corner radius.
  - Filled: `surface_container_highest` background, active indicator line bottom (2px `primary` on focus).

---

#### Crate 5: `quick-markup`
- **File: `crates/quick-markup/src/builder.rs`**:
  - Parse `variant="..."` attribute on `Button`, `Card`, `Chip`, `TextInput`.
  - Parse `indeterminate="true"` on `Checkbox` and `ProgressBar`.
  - In `build_ui_tree`:
    - Check if root node contains `theme="material-you"`, `seed="..."`, `variant="..."`, or `mode="..."`.
    - Generate complete M3 stylesheet rules from the resolved `ThemePackage` and prepend them into the document stylesheet.

---

#### Crate 6: `quick` & `apps/hello-world`
- **File: `crates/quick/src/app.rs`**:
  - Add `pub fn with_theme(mut self, theme: ThemePackage) -> Self`:
    - Generates theme CSS from `theme.generate_css()`.
    - Merges theme rules into `self.stylesheet`.
- **File: `apps/hello-world/src/main.rs` & `app.quick`**:
  - Instantiate `ThemePackage::from_seed_color("#6750A4", SchemeVariant::Vibrant, true)`.
  - Launch `App::new(...).with_theme(m3_theme).from_quick(...)`.
  - Showcase all M3 widgets with live interaction.

---

## 7. Verification and Testing Plan

1. **Unit Tests for HCT & Color Math (`quick-style`)**:
   - Verify sRGB $\leftrightarrow$ Linear $\leftrightarrow$ XYZ roundtrip precision ($< 10^{-4}$).
   - Verify CAM16 forward and inverse transforms.
   - Verify `Hct` tone accuracy: $L^*(Color) \approx \text{target tone} \pm 0.5$.
   - Verify all 8 Scheme variants produce complete 32+ color role sets.
   - Verify WCAG contrast ratios: `on_primary` vs `primary` $\ge 4.5:1$, `on_surface` vs `surface` $\ge 4.5:1$.
   - Verify `ThemePackage::from_seed_color` and `ThemePackage::from_toml`.
2. **Component & Layout Tests (`quick-widgets`)**:
   - Verify Button variants (`Filled`, `Tonal`, `Elevated`, `Outlined`, `Text`) layout and paint.
   - Verify Card elevation levels and dual-pass shadows.
   - Verify Selection controls (`Switch`, `Checkbox`, `Slider`, `Chip`) state toggles and drawing.
   - Verify `ProgressBar` and `TextInput` variants.
3. **Integration & Markup Tests (`quick-markup`, `quick`)**:
   - Verify `App::with_theme` loads and applies dynamic M3 tokens to `.quick` files.
   - Verify declarative variant attribute bindings.
4. **Workspace Acceptance Command**:
   - `cargo check --workspace --all-targets` $\rightarrow$ 0 errors, 0 warnings.
   - `cargo test --workspace` $\rightarrow$ 100% test pass rate.
   - `cargo run -p hello-world` $\rightarrow$ Successfully renders the M3 dynamic UI.

---

## 8. Conclusion

The Quick codebase architecture is well-structured, modular, and optimized for native presentation with zero runtime overhead. Implementing the Material You dynamic theming and component suite according to this blueprint requires:
1. Adding the pure-Rust HCT color engine and M3 token system to `quick-style`.
2. Enhancing `quick-widgets` with M3 component variants, dual-pass elevation shadows, and state layer overlays.
3. Adding `App::with_theme` and markup attribute support in `quick` and `quick-markup`.
4. Updating `apps/hello-world` to showcase the full Material You design system.
