# Milestone 2 Technical Blueprint: Material Design 3 (M3) Base Component Suite
## Components: `ProgressBar`, `TextInput`, and `StateLayer` Helper

---

## 1. Executive Summary & Problem Scope

This report provides the exhaustive, read-only architectural investigation and concrete Rust implementation blueprints for Milestone 2 (M3 Base Component Suite in `quick-widgets`), specifically focusing on:
1. **`crates/quick-widgets/src/state_layer.rs`**: A centralized, reusable state layer blending engine implementing Google Material You (M3) alpha compositing across interactive widget states (`Hover` 8%, `Focus` 12%, `Pressed` 12%, `Dragged` 16%, `Disabled Container` 12%, `Disabled Content` 38%).
2. **`crates/quick-widgets/src/progress.rs` (`ProgressBar`)**: Upgraded to support determinate progress fill ratio with custom ranges, robust boundary/NaN clamping, indeterminate animated pulse mode, and dynamic M3 token-based color styling (`surface_container_highest` track and `primary` indicator).
3. **`crates/quick-widgets/src/text_input.rs` (`TextInput`)**: Complete implementation supporting `Filled` and `Outlined` container variants, 2px dynamic focus indicator border, placeholder text, pointer-click cursor positioning, cursor navigation keys (`ArrowLeft`, `ArrowRight`, `Home`, `End`), text editing (`Backspace`, `Delete`, `Space`, UTF-8 Unicode typing with control character sanitization), and signal/callback reactivity.

All blueprints adhere strictly to the pure Rust, zero-overhead architecture of the Quick UI Framework, maintaining full compatibility with `quick-style` M3 theme tokens, `quick-render` 2D canvas primitives, `quick-layout` Taffy engine, and declarative `.quick` markup.

---

## 2. Architectural Context & Dependency Mapping

### 2.1 Crate Dependency Graph
```
┌───────────────────────────────────────────────────────────────────┐
│                           quick (facade)                         │
└─────────────────────────────────┬─────────────────────────────────┘
                                  │
         ┌────────────────────────┼────────────────────────┐
         ▼                        ▼                        ▼
┌──────────────────┐    ┌──────────────────┐    ┌──────────────────┐
│   quick-markup   │    │  quick-widgets   │    │   quick-render   │
│ (Declarative AST)│    │(Base Components) │    │ (2D Canvas/Skia) │
└────────┬─────────┘    └────────┬─────────┘    └────────┬─────────┘
         │                       │                       │
         └───────────────────────┼───────────────────────┘
                                 ▼
                     ┌───────────────────────┐
                     │      quick-style      │
                     │(M3 Tokens & Palettes) │
                     └───────────┬───────────┘
                                 ▼
                     ┌───────────────────────┐
                     │      quick-core       │
                     │ (Geometry & Signals)  │
                     └───────────────────────┘
```

### 2.2 Shared Types & Interfaces
- **`quick_core::geometry::Color`**: 32-bit RGBA color (`r`, `g`, `b`, `a: u8`). Methods: `from_rgb`, `from_rgba`, `from_hex`, `to_hex`.
- **`quick_core::geometry::Rect`**: 2D Rectangle (`origin: Point`, `size: Size`). Methods: `contains(Point)`, `min_x()`, `min_y()`, `max_x()`, `max_y()`.
- **`quick_core::geometry::BorderRadius`**: Per-corner corner radius (`top_left`, `top_right`, `bottom_left`, `bottom_right: f32`).
- **`quick_core::signals::Signal<T>`**: Reactive signal with `.get()`, `.set(val)`, `.get_untracked()`.
- **`quick_core::event::Event`**: `Pointer(PointerEvent)`, `Key(KeyEvent)`, `Focus(FocusEvent)`.
- **`quick_render::canvas::Canvas`**: Display list recorder (`fill_rect`, `fill_rounded_rect`, `stroke_rounded_rect`, `draw_text`, `draw_line`, `push_clip`, `pop_clip`).
- **`quick_style::property::Style`**: Layout and styling properties (`background_color`, `text_color`, `border_color`, `border_width`, `border_radius`, `padding`, `font_size`, `font_family`, `width`, `height`).
- **`quick_style::theme::tokens::StateLayerTokens`**: M3 interaction opacities (`hover: 0.08`, `focus: 0.12`, `pressed: 0.12`, `dragged: 0.16`, `disabled_container: 0.12`, `disabled_content: 0.38`).

---

## 3. Reusable State Layer Blending Engine (`crates/quick-widgets/src/state_layer.rs`)

### 3.1 Design Specification & Mathematical Model
Material Design 3 specifies that interactive state feedback is achieved by overlaying a state layer with fixed opacity on top of the container's resting background color. The state layer color is typically the matching "on-" color (e.g., `on_surface` over `surface`, `on_primary` over `primary`, or `on_secondary_container` over `secondary_container`).

#### Alpha Compositing Formula
Given a base color $C_{base} = (R_b, G_b, B_b, A_b)$, an overlay color $C_{overlay} = (R_o, G_o, B_o)$, and an opacity factor $\alpha \in [0.0, 1.0]$:
$$R_{out} = \text{round}\left(R_b \cdot (1 - \alpha) + R_o \cdot \alpha\right)$$
$$G_{out} = \text{round}\left(G_b \cdot (1 - \alpha) + G_o \cdot \alpha\right)$$
$$B_{out} = \text{round}\left(B_b \cdot (1 - \alpha) + B_o \cdot \alpha\right)$$
$$A_{out} = A_b$$

For disabled content (text/icons), opacity is scaled directly:
$$A_{content} = \text{round}(A_{initial} \cdot 0.38)$$
For disabled container backgrounds:
$$A_{container} = \text{round}(A_{initial} \cdot 0.12)$$

#### Opacity Level Matrix
| Interaction State | M3 Opacity Token | Standard Overlay Role | Applied To |
|:---|:---:|:---|:---|
| **Hover** | `8%` (`0.08`) | `on_surface` / `on_primary` | Pointer inside widget bounds |
| **Focus** | `12%` (`0.12`) | `on_surface` / `on_primary` | Keyboard or accessibility focus active |
| **Pressed** | `12%` (`0.12`) | `on_surface` / `on_primary` | Pointer button active down inside bounds |
| **Dragged** | `16%` (`0.16`) | `on_surface` / `on_primary` | Active thumb/handle dragging |
| **Disabled Container**| `12%` (`0.12`) | Container background | Widget interactivity disabled |
| **Disabled Content** | `38%` (`0.38`) | Text / Icon foreground | Text/icon in disabled widget |

### 3.2 Concrete Rust Implementation Blueprint

```rust
//! Reusable Material Design 3 State Layer Blending Helper
//!
//! Provides alpha compositing for interactive widget states according to M3 specifications.

use quick_core::geometry::Color;
use quick_style::theme::tokens::StateLayerTokens;

/// Represents the interactive state of a widget.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct WidgetState {
    pub is_hovered: bool,
    pub is_focused: bool,
    pub is_pressed: bool,
    pub is_dragged: bool,
    pub is_disabled: bool,
}

impl WidgetState {
    pub const NORMAL: Self = Self {
        is_hovered: false,
        is_focused: false,
        is_pressed: false,
        is_dragged: false,
        is_disabled: false,
    };

    pub fn hovered() -> Self {
        Self { is_hovered: true, ..Self::NORMAL }
    }

    pub fn pressed() -> Self {
        Self { is_pressed: true, ..Self::NORMAL }
    }

    pub fn focused() -> Self {
        Self { is_focused: true, ..Self::NORMAL }
    }

    pub fn dragged() -> Self {
        Self { is_dragged: true, ..Self::NORMAL }
    }

    pub fn disabled() -> Self {
        Self { is_disabled: true, ..Self::NORMAL }
    }
}

/// Helper for computing state layer alpha compositing on colors.
pub struct StateLayer;

impl StateLayer {
    /// Blend an overlay color onto a base color using a given alpha factor [0.0, 1.0].
    #[inline]
    pub fn blend(base: Color, overlay: Color, alpha: f32) -> Color {
        let a = if alpha.is_nan() { 0.0 } else { alpha.clamp(0.0, 1.0) };
        let r = (base.r as f32 * (1.0 - a) + overlay.r as f32 * a).round() as u8;
        let g = (base.g as f32 * (1.0 - a) + overlay.g as f32 * a).round() as u8;
        let b = (base.b as f32 * (1.0 - a) + overlay.b as f32 * a).round() as u8;
        Color::from_rgba(r, g, b, base.a)
    }

    /// Computes the effective background color given base color, on-surface/on-primary overlay,
    /// and current widget state based on M3 state priority.
    pub fn apply_state(
        base: Color,
        on_color: Color,
        state: WidgetState,
        tokens: &StateLayerTokens,
    ) -> Color {
        if state.is_disabled {
            return tokens.apply_disabled_container(base);
        }
        if state.is_pressed {
            return tokens.apply_pressed(base, on_color);
        }
        if state.is_dragged {
            return tokens.apply_dragged(base, on_color);
        }
        if state.is_hovered {
            return tokens.apply_hover(base, on_color);
        }
        if state.is_focused {
            return tokens.apply_focus(base, on_color);
        }
        base
    }

    /// Convenience helper using standard M3 state tokens.
    #[inline]
    pub fn apply_m3_state(base: Color, on_color: Color, state: WidgetState) -> Color {
        Self::apply_state(base, on_color, state, &StateLayerTokens::M3)
    }

    /// Apply hover state layer (8% overlay).
    #[inline]
    pub fn apply_hover(base: Color, on_color: Color) -> Color {
        StateLayerTokens::M3.apply_hover(base, on_color)
    }

    /// Apply pressed state layer (12% overlay).
    #[inline]
    pub fn apply_pressed(base: Color, on_color: Color) -> Color {
        StateLayerTokens::M3.apply_pressed(base, on_color)
    }

    /// Apply focus state layer (12% overlay).
    #[inline]
    pub fn apply_focus(base: Color, on_color: Color) -> Color {
        StateLayerTokens::M3.apply_focus(base, on_color)
    }

    /// Apply dragged state layer (16% overlay).
    #[inline]
    pub fn apply_dragged(base: Color, on_color: Color) -> Color {
        StateLayerTokens::M3.apply_dragged(base, on_color)
    }

    /// Apply disabled styling to container background (12% opacity) or content (38% opacity).
    #[inline]
    pub fn apply_disabled(color: Color, is_container: bool) -> Color {
        if is_container {
            StateLayerTokens::M3.apply_disabled_container(color)
        } else {
            StateLayerTokens::M3.apply_disabled_content(color)
        }
    }
}
```

---

## 4. Material 3 ProgressBar Blueprint (`crates/quick-widgets/src/progress.rs`)

### 4.1 Component Analysis & M3 Token Mapping
The M3 Linear Progress Indicator comprises:
1. **Inactive Track**: Rounded pill spanning the full width of the widget bounds.
   - Default M3 Color: `surface_container_highest` (Dark: `#36343B` / Light: `#E7E0EC`).
   - Default Height: `8.0px` (or `4.0px` indicator with `8.0px` bounding box).
   - Corner Radius: `BorderRadius::all(bounds.size.height / 2.0)` (full pill).
2. **Active Indicator**:
   - Default M3 Color: `primary` (Dark: `#D0BCFF` / Light: `#6750A4`).
   - Fill Ratio: Scaled linearly across $[min, max]$.
   - When $\text{ratio} == 0.0$: Active rect has width $0.0$ and is not drawn, resulting in exactly **1 paint command** (track only).
   - When $\text{ratio} > 0.0$: Active rounded rect is drawn over track, resulting in exactly **2 paint commands** (track + active indicator).
3. **Indeterminate Animated Mode**:
   - When `is_indeterminate == true`, paints a moving pulse indicator across the inactive track.
   - Pulse width: $0.35 \times \text{bounds.width}$.
   - Position offset: $\text{phase} \times (\text{bounds.width} + \text{pulse\_width}) - \text{pulse\_width}$, clamped/clipped to track.

### 4.2 Robust Numerical Safety & Boundary Handling
- **NaN Handling**: If `progress.get().is_nan()`, clamp value safely to `min` ($\text{ratio} = 0.0$).
- **Inverted Range**: If `min > max`, swap `min` and `max` so calculations never divide by negative numbers or invert ratios.
- **Zero Range**: If $|max - min| < 10^{-4}$, ratio evaluates to $0.0$.
- **Zero Width Layout**: If `bounds.size.width <= 0.0`, safely no-ops drawing active fill.

### 4.3 Concrete Rust Implementation Blueprint

```rust
use crate::widget::Widget;
use quick_core::geometry::{BorderRadius, Color, Rect};
use quick_core::signals::Signal;
use quick_layout::engine::LayoutEngine;
use quick_render::canvas::Canvas;
use quick_style::property::{Dimension, Style};
use taffy::prelude::NodeId;
use taffy::TaffyError;

pub struct ProgressBar {
    pub id: Option<String>,
    pub classes: Vec<String>,
    pub style: Style,
    pub progress: Signal<f32>,
    pub min: f32,
    pub max: f32,
    pub is_indeterminate: bool,
    pub animation_phase: f32,
}

impl ProgressBar {
    /// Create a new determinate progress bar bound to a reactive signal (default range 0.0..1.0).
    pub fn new(progress: Signal<f32>) -> Self {
        let mut style = Style::default();
        style.height = Some(Dimension::Px(8.0));
        style.width = Some(Dimension::Percent(100.0));

        Self {
            id: None,
            classes: Vec::new(),
            style,
            progress,
            min: 0.0,
            max: 1.0,
            is_indeterminate: false,
            animation_phase: 0.0,
        }
    }

    /// Configure custom minimum and maximum range bounds.
    pub fn with_range(mut self, min: f32, max: f32) -> Self {
        self.min = min;
        self.max = max;
        self
    }

    /// Set indeterminate mode on or off.
    pub fn with_indeterminate(mut self, indeterminate: bool) -> Self {
        self.is_indeterminate = indeterminate;
        self
    }

    /// Set animation phase for indeterminate pulse [0.0, 1.0].
    pub fn with_phase(mut self, phase: f32) -> Self {
        self.animation_phase = if phase.is_nan() { 0.0 } else { phase.fract().abs() };
        self
    }
}

impl Widget for ProgressBar {
    fn widget_type(&self) -> &'static str {
        "ProgressBar"
    }

    fn id(&self) -> Option<&str> {
        self.id.as_deref()
    }

    fn classes(&self) -> &[String] {
        &self.classes
    }

    fn style(&self) -> &Style {
        &self.style
    }

    fn style_mut(&mut self) -> &mut Style {
        &mut self.style
    }

    fn build_layout(&mut self, engine: &mut LayoutEngine) -> Result<NodeId, TaffyError> {
        let mut computed_style = self.style.clone();
        if computed_style.height.is_none() {
            computed_style.height = Some(Dimension::Px(8.0));
        }
        engine.new_leaf(&computed_style)
    }

    fn paint(&self, canvas: &mut Canvas, bounds: Rect) {
        if bounds.size.width <= 0.0 || bounds.size.height <= 0.0 {
            return;
        }

        let radius = self
            .style
            .border_radius
            .unwrap_or_else(|| BorderRadius::all(bounds.size.height / 2.0));

        // 1. Inactive Background Track (M3 surface_container_highest)
        let track_color = self
            .style
            .border_color
            .unwrap_or_else(|| Color::from_hex("#36343B").unwrap_or(Color::from_rgb(54, 52, 59)));
        canvas.fill_rounded_rect(bounds, radius, track_color);

        // 2. Active Indicator
        let fill_color = self
            .style
            .background_color
            .or(self.style.text_color)
            .unwrap_or_else(|| Color::from_hex("#6750A4").unwrap_or(Color::from_rgb(103, 80, 164)));

        if self.is_indeterminate {
            // Indeterminate animated pulse mode
            let pulse_w = (bounds.size.width * 0.35).max(12.0);
            let total_travel = bounds.size.width + pulse_w;
            let current_x = bounds.origin.x - pulse_w + total_travel * self.animation_phase;
            
            // Clip active indicator to track bounds
            let visible_x = current_x.max(bounds.origin.x);
            let visible_right = (current_x + pulse_w).min(bounds.origin.x + bounds.size.width);
            let visible_w = (visible_right - visible_x).max(0.0);

            if visible_w > 0.0 {
                let active_rect = Rect::new(visible_x, bounds.origin.y, visible_w, bounds.size.height);
                canvas.fill_rounded_rect(active_rect, radius, fill_color);
            }
        } else {
            // Determinate Fill Ratio mode
            let (min_val, max_val) = if self.min <= self.max {
                (self.min, self.max)
            } else {
                (self.max, self.min)
            };

            let raw_val = self.progress.get();
            let val = if raw_val.is_nan() { min_val } else { raw_val.clamp(min_val, max_val) };

            let pct = if (max_val - min_val).abs() > 0.0001 {
                ((val - min_val) / (max_val - min_val)).clamp(0.0, 1.0)
            } else {
                0.0
            };

            let active_w = bounds.size.width * pct;
            if active_w > 0.0 {
                let active_rect = Rect::new(bounds.origin.x, bounds.origin.y, active_w, bounds.size.height);
                canvas.fill_rounded_rect(active_rect, radius, fill_color);
            }
        }
    }
}
```

---

## 5. Material 3 TextInput Blueprint (`crates/quick-widgets/src/text_input.rs`)

### 5.1 Component Architecture & Feature Matrix
The M3 Text Field specification defines:
1. **Container Variants**:
   - `InputVariant::Filled`: Background `surface_container_highest` (e.g. `#2B2930` / `#1E1E2E`), top rounded corners ($r=4\text{px}$) with active bottom border line or filled container stroke.
   - `InputVariant::Outlined`: Background `Color::TRANSPARENT`, full rounded perimeter border ($r=4\text{px}$, `corner_extra_small`).
2. **Focus Indicator Stroke**:
   - Unfocused: 1.0px width, `outline` / `outline_variant` color (`#79747E` / `#45475A`).
   - Focused: 2.0px width, `primary` / active focus color (`#6750A4` / `#89B4FA`).
3. **Placeholder & Content Text**:
   - Rendered with vertical baseline centering: $Y_{origin} = bounds.origin.y + \frac{bounds.height + font\_size \times 0.8}{2.0}$.
   - Placeholder color: `on_surface_variant` with 180 alpha.
   - Value color: `on_surface` (or `style.text_color`).
4. **Focused Cursor Line**:
   - When focused, paints vertical bar at measured text offset $X_{cursor} = X_{origin} + \text{char\_count} \times \text{font\_size} \times 0.55$.
5. **Interactive Text Editing Engine**:
   - **Pointer Click**: Updates `is_focused = true` and calculates nearest character index for `cursor_pos`.
   - **Arrow Keys**: `ArrowLeft`/`Left` decrements cursor; `ArrowRight`/`Right` increments cursor.
   - **Home / End**: Jumps cursor to index $0$ or `chars.len()`.
   - **Backspace**: Deletes character before cursor.
   - **Delete**: Deletes character after cursor.
   - **Space & Unicode Characters**: Inserts text at `cursor_pos`, advances cursor, ignores control characters.
   - **Signal / Callback**: Invokes `on_change` with updated text value.

### 5.2 Concrete Rust Implementation Blueprint

```rust
use crate::widget::Widget;
use quick_core::event::{Event, FocusEvent, KeyEvent, KeyState, PointerButton, PointerEvent, PointerPhase};
use quick_core::geometry::{BorderRadius, Color, Insets, Point, Rect};
use quick_layout::engine::LayoutEngine;
use quick_render::canvas::Canvas;
use quick_style::property::{Dimension, Style};
use taffy::prelude::NodeId;
use taffy::TaffyError;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InputVariant {
    Filled,
    Outlined,
}

pub struct TextInput {
    pub id: Option<String>,
    pub classes: Vec<String>,
    pub style: Style,
    pub value: String,
    pub placeholder: String,
    pub variant: InputVariant,
    pub on_change: Option<Box<dyn FnMut(String)>>,
    pub is_focused: bool,
    pub cursor_pos: usize,
}

impl TextInput {
    pub fn new(placeholder: impl Into<String>) -> Self {
        let mut style = Style::default();
        style.background_color = Some(Color::from_hex("#1e1e2e").unwrap_or(Color::from_rgb(30, 30, 46)));
        style.text_color = Some(Color::WHITE);
        style.border_color = Some(Color::from_hex("#45475a").unwrap_or(Color::from_rgb(69, 71, 90)));
        style.border_width = Some(1.0);
        style.border_radius = Some(BorderRadius::all(4.0));
        style.padding = Some(Insets::symmetric(6.0, 10.0));
        style.font_size = Some(14.0);

        Self {
            id: None,
            classes: Vec::new(),
            style,
            value: String::new(),
            placeholder: placeholder.into(),
            variant: InputVariant::Filled,
            on_change: None,
            is_focused: false,
            cursor_pos: 0,
        }
    }

    pub fn with_variant(mut self, variant: InputVariant) -> Self {
        self.variant = variant;
        if variant == InputVariant::Outlined {
            self.style.background_color = Some(Color::TRANSPARENT);
            self.style.border_color = Some(Color::from_hex("#79747E").unwrap_or(Color::from_rgb(121, 116, 126)));
        }
        self
    }

    pub fn with_placeholder(mut self, placeholder: impl Into<String>) -> Self {
        self.placeholder = placeholder.into();
        self
    }

    pub fn with_value(mut self, value: impl Into<String>) -> Self {
        self.value = value.into();
        self.cursor_pos = self.value.chars().count();
        self
    }

    pub fn on_change<F: FnMut(String) + 'static>(mut self, handler: F) -> Self {
        self.on_change = Some(Box::new(handler));
        self
    }

    fn clamp_cursor(&mut self) {
        let char_count = self.value.chars().count();
        if self.cursor_pos > char_count {
            self.cursor_pos = char_count;
        }
    }
}

impl Widget for TextInput {
    fn widget_type(&self) -> &'static str {
        "TextInput"
    }

    fn id(&self) -> Option<&str> {
        self.id.as_deref()
    }

    fn classes(&self) -> &[String] {
        &self.classes
    }

    fn style(&self) -> &Style {
        &self.style
    }

    fn style_mut(&mut self) -> &mut Style {
        &mut self.style
    }

    fn build_layout(&mut self, engine: &mut LayoutEngine) -> Result<NodeId, TaffyError> {
        let mut computed_style = self.style.clone();
        if computed_style.width.is_none() {
            computed_style.width = Some(Dimension::Px(180.0));
        }
        if computed_style.height.is_none() {
            computed_style.height = Some(Dimension::Px(35.0));
        }
        engine.new_leaf(&computed_style)
    }

    fn paint(&self, canvas: &mut Canvas, bounds: Rect) {
        let radius = self.style.border_radius.unwrap_or(BorderRadius::all(4.0));

        // 1. Container Background
        let bg_color = if self.variant == InputVariant::Outlined {
            self.style.background_color.unwrap_or(Color::TRANSPARENT)
        } else {
            self.style.background_color.unwrap_or(Color::from_rgb(30, 30, 46))
        };
        canvas.fill_rounded_rect(bounds, radius, bg_color);

        // 2. Border Stroke (1.0px unfocused, 2.0px focused M3 active indicator)
        let (border_color, border_width) = if self.is_focused {
            (Color::from_hex("#89b4fa").unwrap_or(Color::BLUE), 2.0)
        } else {
            (self.style.border_color.unwrap_or(Color::from_hex("#45475a").unwrap_or(Color::GRAY)), self.style.border_width.unwrap_or(1.0))
        };
        canvas.stroke_rounded_rect(bounds, radius, border_color, border_width);

        // 3. Text & Placeholder Rendering
        let font_size = self.style.font_size.unwrap_or(14.0);
        let pad_left = self.style.padding.map(|p| p.left).unwrap_or(8.0);
        let origin_x = bounds.origin.x + pad_left;
        let origin_y = bounds.origin.y + ((bounds.size.height + font_size * 0.8) / 2.0);

        if self.value.is_empty() && !self.placeholder.is_empty() {
            let placeholder_color = Color::from_rgba(150, 150, 150, 180);
            canvas.draw_text(
                &self.placeholder,
                Point::new(origin_x, origin_y),
                placeholder_color,
                font_size,
                self.style.font_family.clone(),
            );
        } else {
            let text_color = self.style.text_color.unwrap_or(Color::WHITE);
            canvas.draw_text(
                &self.value,
                Point::new(origin_x, origin_y),
                text_color,
                font_size,
                self.style.font_family.clone(),
            );
        }

        // 4. Cursor Rendering (Active Focus)
        if self.is_focused {
            let char_count = self.value.chars().take(self.cursor_pos).count() as f32;
            let cursor_x = origin_x + (char_count * font_size * 0.55);
            let cursor_top = bounds.origin.y + (bounds.size.height - font_size * 1.2) / 2.0;
            let cursor_bottom = cursor_top + font_size * 1.2;
            let cursor_color = Color::from_hex("#89b4fa").unwrap_or(Color::WHITE);

            canvas.draw_line(
                Point::new(cursor_x, cursor_top),
                Point::new(cursor_x, cursor_bottom),
                cursor_color,
                1.5,
            );
        }
    }

    fn handle_event(&mut self, event: &Event, bounds: Rect) -> bool {
        match event {
            Event::Pointer(PointerEvent { position, button, phase, .. }) => {
                if *phase == PointerPhase::Down && *button == Some(PointerButton::Primary) {
                    if bounds.contains(*position) {
                        self.is_focused = true;
                        // Calculate cursor position from click X coordinate
                        let pad_left = self.style.padding.map(|p| p.left).unwrap_or(8.0);
                        let font_size = self.style.font_size.unwrap_or(14.0);
                        let char_width = font_size * 0.55;
                        let relative_x = (position.x - (bounds.origin.x + pad_left)).max(0.0);
                        let clicked_idx = (relative_x / char_width).round() as usize;
                        let total_chars = self.value.chars().count();
                        self.cursor_pos = clicked_idx.min(total_chars);
                        return true;
                    } else {
                        self.is_focused = false;
                        return false;
                    }
                }
            }
            Event::Key(KeyEvent { key, state, text, .. }) if self.is_focused && *state == KeyState::Pressed => {
                let mut chars: Vec<char> = self.value.chars().collect();
                self.clamp_cursor();

                match key.as_str() {
                    "Left" | "ArrowLeft" => {
                        self.cursor_pos = self.cursor_pos.saturating_sub(1);
                        return true;
                    }
                    "Right" | "ArrowRight" => {
                        self.cursor_pos = (self.cursor_pos + 1).min(chars.len());
                        return true;
                    }
                    "Home" => {
                        self.cursor_pos = 0;
                        return true;
                    }
                    "End" => {
                        self.cursor_pos = chars.len();
                        return true;
                    }
                    "Backspace" => {
                        if self.cursor_pos > 0 && !chars.is_empty() {
                            chars.remove(self.cursor_pos - 1);
                            self.cursor_pos -= 1;
                            self.value = chars.into_iter().collect();
                            if let Some(ref mut handler) = self.on_change {
                                handler(self.value.clone());
                            }
                        }
                        return true;
                    }
                    "Delete" => {
                        if self.cursor_pos < chars.len() {
                            chars.remove(self.cursor_pos);
                            self.value = chars.into_iter().collect();
                            if let Some(ref mut handler) = self.on_change {
                                handler(self.value.clone());
                            }
                        }
                        return true;
                    }
                    "Space" if text.is_none() => {
                        chars.insert(self.cursor_pos, ' ');
                        self.cursor_pos += 1;
                        self.value = chars.into_iter().collect();
                        if let Some(ref mut handler) = self.on_change {
                            handler(self.value.clone());
                        }
                        return true;
                    }
                    _ => {
                        if let Some(ref ch_str) = text {
                            let insert_chars: Vec<char> = ch_str.chars().filter(|c| !c.is_control()).collect();
                            if !insert_chars.is_empty() {
                                let insert_count = insert_chars.len();
                                for (i, c) in insert_chars.into_iter().enumerate() {
                                    chars.insert(self.cursor_pos + i, c);
                                }
                                self.cursor_pos += insert_count;
                                self.value = chars.into_iter().collect();
                                if let Some(ref mut handler) = self.on_change {
                                    handler(self.value.clone());
                                }
                                return true;
                            }
                        }
                    }
                }
            }
            Event::Focus(FocusEvent::Lost) => {
                self.is_focused = false;
            }
            _ => {}
        }
        false
    }
}
```

---

## 6. Unit & Integration Verification Suite

### 6.1 State Layer Blending Verification
```rust
#[test]
fn test_state_layer_alpha_blending_matrix() {
    let base = Color::from_rgb(100, 100, 100);
    let overlay = Color::WHITE;

    let hover = StateLayer::apply_hover(base, overlay);
    assert_eq!(hover.r, 112); // 100 * 0.92 + 255 * 0.08 = 112.4 -> 112

    let pressed = StateLayer::apply_pressed(base, overlay);
    assert_eq!(pressed.r, 119); // 100 * 0.88 + 255 * 0.12 = 118.6 -> 119

    let dragged = StateLayer::apply_dragged(base, overlay);
    assert_eq!(dragged.r, 125); // 100 * 0.84 + 255 * 0.16 = 124.8 -> 125
}
```

### 6.2 ProgressBar Range & Animation Verification
```rust
#[test]
fn test_progressbar_indeterminate_and_clamping() {
    let sig = Signal::new(0.5);
    let bar = ProgressBar::new(sig).with_indeterminate(true).with_phase(0.25);
    let mut canvas = Canvas::new();
    bar.paint(&mut canvas, Rect::new(0.0, 0.0, 200.0, 8.0));
    assert_eq!(canvas.commands().len(), 2);
}
```

### 6.3 TextInput Cursor Navigation & Multi-Character Insertion Verification
```rust
#[test]
fn test_text_input_cursor_navigation_and_insert() {
    let mut input = TextInput::new("Placeholder");
    input.is_focused = true;
    let bounds = Rect::new(0.0, 0.0, 200.0, 35.0);

    // Type "Hello"
    for c in ["H", "e", "l", "l", "o"] {
        input.handle_event(&Event::Key(KeyEvent {
            key: c.to_string(),
            state: KeyState::Pressed,
            text: Some(c.to_string()),
            modifiers: Default::default(),
        }), bounds);
    }
    assert_eq!(input.value, "Hello");
    assert_eq!(input.cursor_pos, 5);

    // Move cursor left twice
    for _ in 0..2 {
        input.handle_event(&Event::Key(KeyEvent {
            key: "ArrowLeft".to_string(),
            state: KeyState::Pressed,
            text: None,
            modifiers: Default::default(),
        }), bounds);
    }
    assert_eq!(input.cursor_pos, 3);

    // Insert 'p' -> "Helplo"
    input.handle_event(&Event::Key(KeyEvent {
        key: "p".to_string(),
        state: KeyState::Pressed,
        text: Some("p".to_string()),
        modifiers: Default::default(),
    }), bounds);
    assert_eq!(input.value, "Helplo");
    assert_eq!(input.cursor_pos, 4);
}
```

---

## 7. Implementation Checklist for Milestone 2

- [ ] Create `crates/quick-widgets/src/state_layer.rs` with `WidgetState`, `StateLayer`, and M3 blending methods.
- [ ] Export `pub mod state_layer; pub use state_layer::*;` in `crates/quick-widgets/src/lib.rs`.
- [ ] Update `crates/quick-widgets/src/progress.rs` with indeterminate pulse mode, animation phase, and numerical guards.
- [ ] Update `crates/quick-widgets/src/text_input.rs` with `InputVariant`, cursor navigation (`ArrowLeft`, `ArrowRight`, `Home`, `End`), click-to-index calculation, and multi-char insertion.
- [ ] Run `cargo test --test e2e_m3_widgets` and `cargo test --workspace` to confirm 100% test success rate with 0 warnings.
