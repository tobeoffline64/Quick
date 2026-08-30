# Material Design 3 (M3) Base Component Suite Implementation Blueprint
**Target Crate**: `quick-widgets` (`crates/quick-widgets`)  
**Target Milestone**: Milestone 2 (M2)  
**Author**: Explorer Agent (`explorer_m2_2`)  
**Date**: 2026-08-30  

---

## 1. Executive Summary & Architectural Overview

The Quick UI Framework's Material You (Material Design 3 / M3) subsystem provides a pure-Rust, hardware-accelerated component suite for native 60+ FPS desktop applications. Within this ecosystem, **`quick-widgets`** delivers the fundamental selection controls, form inputs, and interactive primitives:

1. **`Switch`**: M3 pill track ($52\times 32\text{px}$), asymmetric sliding thumb ($24\text{px}$ checked vs. $16\text{px}$ unchecked), state layer interaction halos, reactive signal binding, and change callbacks.
2. **`Checkbox`**: $24\times 24\text{px}$ touch target containing an $18\times 18\text{px}$ container box ($r=2\text{px}$), vector checkmark path, indeterminate horizontal dash, state layer halos, and tri-state reactivity.
3. **`Slider`**: $8\text{px}$ track pill ($r=4\text{px}$), $20\text{px}$ thumb ($r=10\text{px}$), continuous dragging, discrete step quantization with tick rendering, bounds clamping, and NaN safety.
4. **`Chip`**: 4 M3 variants (`Filter`, `Assist`, `Input`, `Suggestion`), $32\text{px}$ pill geometry, dynamic width estimation, selection toggle signal binding, and variant-specific container tones.

All four components adhere to the core framework architecture:
- **Layout**: Powered by Taffy (`quick_layout::engine::LayoutEngine`), computing exact node constraints, intrinsic dimensions, and bounds.
- **Rendering**: Vector commands emitted to `quick_render::canvas::Canvas` (`fill_rounded_rect`, `stroke_rounded_rect`, `draw_line`, `draw_text`).
- **Reactivity**: Integrated with `quick_core::signals::Signal<T>`, reacting immediately to external signal mutations without manual DOM reconciliation.
- **Event Pipeline**: Deterministic pointer hit testing (`PointerPhase::Down`, `Moved`, `Up`, `Cancel`), gesture capture, release-outside cancellation, and secondary button rejection.

---

## 2. Component Blueprint 1: `Switch` Selection Control

### 2.1 Specification & Geometry Contract

| Dimension / Property | Specification Value | Description |
| :--- | :--- | :--- |
| **Track Dimensions** | $52.0\text{px} \times 32.0\text{px}$ | Standard M3 switch track |
| **Track Corner Radius** | $16.0\text{px}$ (`corner-full` / $H/2$) | Perfectly rounded pill geometry |
| **Track Border Width** | $2.0\text{px}$ | Outlined in unchecked state |
| **Checked Thumb Size** | $24.0\text{px} \times 24.0\text{px}$ ($r=12.0\text{px}$) | Enlarged thumb indicating active state |
| **Unchecked Thumb Size**| $16.0\text{px} \times 16.0\text{px}$ ($r=8.0\text{px}$) | Reduced thumb indicating inactive state |
| **Pressed Thumb Size**  | $28.0\text{px} \times 28.0\text{px}$ ($r=14.0\text{px}$) | Transient expansion during pointer press |
| **Checked Thumb Offset**| $x = \text{origin.x} + \text{width} - 24.0 - 4.0 = \text{origin.x} + 24.0\text{px}$ | $4.0\text{px}$ margin from right edge |
| **Unchecked Thumb Offset**| $x = \text{origin.x} + 8.0\text{px}$ (or $7.0\text{px}$) | $8.0\text{px}$ margin from left edge |
| **Thumb Vertical Offset**| $y = \text{origin.y} + (\text{height} - \text{thumb\_size}) / 2.0$ | Vertically centered in track |
| **State Layer Halo**   | Diameter $40.0\text{px}$ centered on thumb | Hover (8%), Focus (12%), Pressed (12%) |

### 2.2 Color Role Mapping & State Layers

```text
Checked (Selected):
  ├── Track Fill:       ColorScheme.primary (#6750A4 / #D0BCFF)
  ├── Track Stroke:     None / Transparent
  ├── Thumb Fill:       ColorScheme.on_primary (#FFFFFF / #381E72)
  └── State Layer Halo: ColorScheme.primary @ 8% (Hover) / 12% (Pressed)

Unchecked (Unselected):
  ├── Track Fill:       ColorScheme.surface_container_highest (#36343B)
  ├── Track Stroke:     ColorScheme.outline (#79747E / #938F99), 2px width
  ├── Thumb Fill:       ColorScheme.outline (#79747E / #938F99)
  └── State Layer Halo: ColorScheme.on_surface @ 8% (Hover) / 12% (Pressed)

Disabled:
  ├── Track Opacity:    12% container opacity
  └── Thumb Opacity:    38% content opacity
```

### 2.3 Rust Struct & Method Signatures

```rust
pub struct Switch {
    pub id: Option<String>,
    pub classes: Vec<String>,
    pub style: Style,
    pub checked: Signal<bool>,
    pub on_change: Option<Box<dyn FnMut(bool)>>,
    pub is_disabled: bool,
    is_hovered: bool,
    is_pressed: bool,
}

impl Switch {
    pub fn new(checked: Signal<bool>) -> Self {
        let mut style = Style::default();
        style.width = Some(Dimension::Px(52.0));
        style.height = Some(Dimension::Px(32.0));

        Self {
            id: None,
            classes: Vec::new(),
            style,
            checked,
            on_change: None,
            is_disabled: false,
            is_hovered: false,
            is_pressed: false,
        }
    }

    pub fn on_change<F: FnMut(bool) + 'static>(mut self, handler: F) -> Self {
        self.on_change = Some(Box::new(handler));
        self
    }

    pub fn with_disabled(mut self, disabled: bool) -> Self {
        self.is_disabled = disabled;
        self
    }
}
```

### 2.4 Layout & Painting Implementation

```rust
impl Widget for Switch {
    fn build_layout(&mut self, engine: &mut LayoutEngine) -> Result<NodeId, TaffyError> {
        let mut computed_style = self.style.clone();
        if computed_style.width.is_none() {
            computed_style.width = Some(Dimension::Px(52.0));
        }
        if computed_style.height.is_none() {
            computed_style.height = Some(Dimension::Px(32.0));
        }
        engine.new_leaf(&computed_style)
    }

    fn paint(&self, canvas: &mut Canvas, bounds: Rect) {
        let is_on = self.checked.get();
        let track_radius = BorderRadius::all(bounds.size.height / 2.0);

        // 1. Resolve Colors
        let track_color = if is_on {
            self.style.background_color.unwrap_or(Color::from_hex("#6750A4").unwrap())
        } else {
            Color::from_hex("#36343B").unwrap()
        };

        let thumb_color = if is_on {
            Color::from_hex("#FFFFFF").unwrap()
        } else {
            Color::from_hex("#938F99").unwrap()
        };

        // 2. Render Track Pill
        canvas.fill_rounded_rect(bounds, track_radius, track_color);

        if !is_on {
            let outline_color = self.style.border_color.unwrap_or(Color::from_hex("#79747E").unwrap());
            canvas.stroke_rounded_rect(bounds, track_radius, outline_color, 2.0);
        }

        // 3. Compute Thumb Proportions
        let thumb_size = if self.is_pressed {
            28.0
        } else if is_on {
            24.0
        } else {
            16.0
        };

        let thumb_x = if is_on {
            bounds.origin.x + bounds.size.width - thumb_size - 4.0
        } else {
            bounds.origin.x + 8.0 - (thumb_size - 16.0) / 2.0
        };
        let thumb_y = bounds.origin.y + (bounds.size.height - thumb_size) / 2.0;
        let thumb_rect = Rect::new(thumb_x, thumb_y, thumb_size, thumb_size);

        // 4. Render State Layer Halo (Hover / Pressed)
        if self.is_hovered || self.is_pressed {
            let halo_size = 40.0;
            let halo_x = thumb_x + (thumb_size - halo_size) / 2.0;
            let halo_y = thumb_y + (thumb_size - halo_size) / 2.0;
            let halo_rect = Rect::new(halo_x, halo_y, halo_size, halo_size);
            let halo_alpha = if self.is_pressed { 0.12 } else { 0.08 };
            let halo_color = if is_on {
                Color::from_rgba(255, 255, 255, (halo_alpha * 255.0) as u8)
            } else {
                Color::from_rgba(103, 80, 164, (halo_alpha * 255.0) as u8)
            };
            canvas.fill_rounded_rect(halo_rect, BorderRadius::all(halo_size / 2.0), halo_color);
        }

        // 5. Render Sliding Thumb
        canvas.fill_rounded_rect(thumb_rect, BorderRadius::all(thumb_size / 2.0), thumb_color);
    }
}
```

### 2.5 Event Lifecycle & State Transitions

```text
[Pointer Down (Primary, Inside)] ──► is_pressed = true, return true
                                           │
         ┌─────────────────────────────────┴─────────────────────────────────┐
         ▼                                                                   ▼
[Pointer Up (Inside Bounds)]                                        [Pointer Up (Outside Bounds)]
  ├── is_pressed = false                                              ├── is_pressed = false
  ├── new_state = !checked.get()                                      └── return false (Toggle aborted)
  ├── checked.set(new_state)
  ├── on_change(new_state)
  └── return true

[Pointer Cancel] ──► is_pressed = false, return false
```

---

## 3. Component Blueprint 2: `Checkbox` Selection Control

### 3.1 Specification & Geometry Contract

| Dimension / Property | Specification Value | Description |
| :--- | :--- | :--- |
| **Touch / Target Bounds** | $24.0\text{px} \times 24.0\text{px}$ | Layout allocation and minimum touch area |
| **Container Box Size**  | $18.0\text{px} \times 18.0\text{px}$ (or $20.0\text{px}$) | Visual rounded rectangle box |
| **Box Corner Radius**   | $2.0\text{px}$ (`corner-extra-small` / $r=2\text{px}$) | Slight corner curve per M3 spec |
| **Box Position**        | Centered: $x = \text{origin.x} + 3.0\text{px}$, $y = \text{origin.y} + 3.0\text{px}$ | $3.0\text{px}$ padding on all sides |
| **Border Width**        | $2.0\text{px}$ | Stroke thickness in unchecked state |
| **Checkmark Path**      | Polyline: $(4.5, 9.5) \to (7.5, 13.5) \to (14.5, 5.5)$ | Standard M3 checkmark vertex proportions |
| **Checkmark Stroke Width** | $2.0\text{px}$ | Clean high-contrast white vector line |
| **Indeterminate Dash**  | Horizontal line: $(4.0, 9.0) \to (14.0, 9.0)$ | Centered dash across box width |
| **Indeterminate Stroke**| $2.0\text{px}$ | High-contrast white vector line |
| **State Layer Halo**    | Diameter $40.0\text{px}$ (or $24.0\text{px}$ bounded) | Hover 8%, Focus 12%, Pressed 12% |

### 3.2 Color Role Mapping & Vector Glyph Paths

```text
Checked / Selected:
  ├── Box Fill:         ColorScheme.primary (#6750A4 / #D0BCFF)
  ├── Box Border:       None / Transparent
  ├── Checkmark Stroke: ColorScheme.on_primary (#FFFFFF / #381E72), 2px width
  └── Vector Segments:  P1(box_x + 4.5, box_y + 10.0) ──► P2(box_x + 8.5, box_y + 14.5)
                        P2(box_x + 8.5, box_y + 14.5) ──► P3(box_x + 15.5, box_y + 5.5)

Indeterminate:
  ├── Box Fill:         ColorScheme.primary (#6750A4 / #D0BCFF)
  ├── Box Border:       None / Transparent
  ├── Dash Stroke:      ColorScheme.on_primary (#FFFFFF / #381E72), 2px width
  └── Vector Segment:   P1(box_x + 4.0, box_y + 10.0) ──► P2(box_x + 16.0, box_y + 10.0)

Unchecked:
  ├── Box Fill:         Transparent
  ├── Box Border:       ColorScheme.outline (#79747E / #938F99), 2px width
  └── Vector Glyph:     None
```

### 3.3 Rust Struct & Method Signatures

```rust
pub struct Checkbox {
    pub id: Option<String>,
    pub classes: Vec<String>,
    pub style: Style,
    pub checked: Signal<bool>,
    pub indeterminate: Option<Signal<bool>>,
    pub on_change: Option<Box<dyn FnMut(bool)>>,
    pub is_disabled: bool,
    is_hovered: bool,
    is_pressed: bool,
}

impl Checkbox {
    pub fn new(checked: Signal<bool>) -> Self {
        let mut style = Style::default();
        style.width = Some(Dimension::Px(24.0));
        style.height = Some(Dimension::Px(24.0));

        Self {
            id: None,
            classes: Vec::new(),
            style,
            checked,
            indeterminate: None,
            on_change: None,
            is_disabled: false,
            is_hovered: false,
            is_pressed: false,
        }
    }

    pub fn with_indeterminate(mut self, indeterminate: Signal<bool>) -> Self {
        self.indeterminate = Some(indeterminate);
        self
    }

    pub fn on_change<F: FnMut(bool) + 'static>(mut self, handler: F) -> Self {
        self.on_change = Some(Box::new(handler));
        self
    }
}
```

### 3.4 Painting & Vector Rendering Implementation

```rust
impl Widget for Checkbox {
    fn build_layout(&mut self, engine: &mut LayoutEngine) -> Result<NodeId, TaffyError> {
        let mut computed_style = self.style.clone();
        if computed_style.width.is_none() {
            computed_style.width = Some(Dimension::Px(24.0));
        }
        if computed_style.height.is_none() {
            computed_style.height = Some(Dimension::Px(24.0));
        }
        engine.new_leaf(&computed_style)
    }

    fn paint(&self, canvas: &mut Canvas, bounds: Rect) {
        let is_on = self.checked.get();
        let is_indet = self.indeterminate.as_ref().map(|s| s.get()).unwrap_or(false);

        let box_size = 20.0; // or 18.0
        let box_x = bounds.origin.x + (bounds.size.width - box_size) / 2.0;
        let box_y = bounds.origin.y + (bounds.size.height - box_size) / 2.0;
        let box_rect = Rect::new(box_x, box_y, box_size, box_size);
        let radius = BorderRadius::all(4.0); // or 2.0

        if is_indet {
            // Indeterminate state: filled container + horizontal dash line
            let fill_color = self.style.background_color.unwrap_or(Color::from_hex("#6750A4").unwrap());
            canvas.fill_rounded_rect(box_rect, radius, fill_color);

            let dash_y = box_y + box_size / 2.0;
            let p_start = Point::new(box_x + 4.0, dash_y);
            let p_end = Point::new(box_x + box_size - 4.0, dash_y);
            canvas.draw_line(p_start, p_end, Color::WHITE, 2.0);
        } else if is_on {
            // Checked state: filled container + 2 checkmark strokes
            let fill_color = self.style.background_color.unwrap_or(Color::from_hex("#6750A4").unwrap());
            canvas.fill_rounded_rect(box_rect, radius, fill_color);

            let p1 = Point::new(box_x + 4.5, box_y + 10.0);
            let p2 = Point::new(box_x + 8.5, box_y + 14.5);
            let p3 = Point::new(box_x + 15.5, box_y + 5.5);
            canvas.draw_line(p1, p2, Color::WHITE, 2.0);
            canvas.draw_line(p2, p3, Color::WHITE, 2.0);
        } else {
            // Unchecked state: outlined container stroke only
            let border_color = self.style.border_color.unwrap_or(Color::from_hex("#79747E").unwrap());
            canvas.stroke_rounded_rect(box_rect, radius, border_color, 2.0);
        }
    }
}
```

---

## 4. Component Blueprint 3: `Slider` Selection Control

### 4.1 Specification & Geometry Contract

| Dimension / Property | Specification Value | Description |
| :--- | :--- | :--- |
| **Total Widget Height** | $36.0\text{px}$ (or $48.0\text{px}$ with touch margin) | Touch scrubbing area |
| **Track Height**        | $8.0\text{px}$ ($r=4.0\text{px}$ pill) | Pill track container |
| **Track Side Padding**  | $12.0\text{px}$ left and right | Clearance for $20\text{px}$ thumb overhang |
| **Effective Track Width**| $\text{bounds.width} - 24.0\text{px}$ | Active scrubbing range |
| **Thumb Diameter**      | $20.0\text{px}$ ($r=10.0\text{px}$) | Circular draggable thumb |
| **Thumb Interaction Halo**| $40.0\text{px}$ diameter | Dragged 16%, Hover 8%, Focus 12% |
| **Discrete Step Ticks** | $2.0\text{px}$ dots along track (when `steps` configured) | Visual indicators for discrete quantization |
| **Default Range**       | `min: 0.0`, `max: 100.0` (or `1.0`) | Fully customizable range |

### 4.2 Mathematical Formulas & Quantization Logic

1. **Normalized Ratio Calculation**:
   $$\text{ratio} = \begin{cases} 
   0.0 & \text{if } |\text{max} - \text{min}| \le 10^{-5} \text{ or } \text{value.is\_nan()} \\
   \text{clamp}\left(\frac{\text{val} - \text{min}}{\text{max} - \text{min}}, 0.0, 1.0\right) & \text{otherwise}
   \end{cases}$$

2. **Pointer Position to Value Mapping**:
   $$\text{pos\_ratio} = \text{clamp}\left(\frac{x - \text{track\_left}}{\text{track\_width}}, 0.0, 1.0\right)$$
   $$\text{raw\_value} = \text{min} + \text{pos\_ratio} \times (\text{max} - \text{min})$$

3. **Discrete Step Snapping (when $\text{steps} = \text{Some}(N)$)**:
   $$\text{step\_size} = \frac{\text{max} - \text{min}}{N}$$
   $$\text{snapped\_value} = \text{min} + \text{round}\left(\frac{\text{raw\_value} - \text{min}}{\text{step\_size}}\right) \times \text{step\_size}$$

### 4.3 Color Role Mapping

```text
Active Track (Left of Thumb):
  └── ColorScheme.primary (#6750A4 / #D0BCFF), height 8px, r = 4px

Inactive Track (Right of Thumb):
  └── ColorScheme.surface_container_highest (#36343B / #E6E0E9), height 8px, r = 4px

Thumb:
  ├── Core Fill:        ColorScheme.primary / inverse_primary (#D0BCFF)
  └── Dragged Halo:     ColorScheme.primary @ 16% alpha (40px circle)

Discrete Step Ticks:
  ├── In Active Track:  ColorScheme.on_primary (#FFFFFF) dot (2px)
  └── In Inactive Track:ColorScheme.on_surface_variant (#938F99) dot (2px)
```

### 4.4 Rust Struct & Painting Implementation

```rust
pub struct Slider {
    pub id: Option<String>,
    pub classes: Vec<String>,
    pub style: Style,
    pub value: Signal<f32>,
    pub min: f32,
    pub max: f32,
    pub steps: Option<u32>,
    pub on_change: Option<Box<dyn FnMut(f32)>>,
    is_dragging: bool,
    is_hovered: bool,
}

impl Slider {
    pub fn new(value: Signal<f32>, min: f32, max: f32) -> Self {
        let mut style = Style::default();
        style.height = Some(Dimension::Px(36.0));
        style.width = Some(Dimension::Percent(100.0));

        Self {
            id: None,
            classes: Vec::new(),
            style,
            value,
            min,
            max,
            steps: None,
            on_change: None,
            is_dragging: false,
            is_hovered: false,
        }
    }

    pub fn with_steps(mut self, steps: Option<u32>) -> Self {
        self.steps = steps;
        self
    }

    pub fn on_change<F: FnMut(f32) + 'static>(mut self, handler: F) -> Self {
        self.on_change = Some(Box::new(handler));
        self
    }

    fn update_from_pos(&mut self, pos_x: f32, bounds: Rect) {
        let pad = 12.0;
        let track_left = bounds.origin.x + pad;
        let track_width = (bounds.size.width - pad * 2.0).max(1.0);
        let ratio = ((pos_x - track_left) / track_width).clamp(0.0, 1.0);
        let mut new_val = self.min + ratio * (self.max - self.min);

        // Discrete step quantization
        if let Some(steps) = self.steps {
            if steps > 0 {
                let step_size = (self.max - self.min) / steps as f32;
                new_val = self.min + ((new_val - self.min) / step_size).round() * step_size;
            }
        }

        self.value.set(new_val);
        if let Some(ref mut handler) = self.on_change {
            handler(new_val);
        }
    }
}
```

---

## 5. Component Blueprint 4: `Chip` Selection Control

### 5.1 Specification & Variant Matrix

| Variant | Role & Interaction | Default Background | Selected Background | Leading/Trailing Icons |
| :--- | :--- | :--- | :--- | :--- |
| **`Filter`** | Interactive toggle filter | `surface_container_low` (`#1D1B20`) | `secondary_container` (`#4A4458`) | Optional leading checkmark on selected |
| **`Assist`** | Action initiator | `surface` with `outline` border | N/A (Non-toggle) | Optional leading action icon |
| **`Input`**  | User input tag / entity | `surface_container_low` | `secondary_container` | Optional trailing dismiss '×' icon |
| **`Suggestion`** | Recommendation chips | `surface` with `outline` border | N/A (Non-toggle) | Optional leading icon |

### 5.2 Geometry & Dynamic Sizing Rules

- **Height**: $32.0\text{px}$ fixed layout height (`corner-small` 8px radius or `corner-full` 999px pill).
- **Padding**: $6.0\text{px}$ top/bottom, $14.0\text{px}$ left/right ($28.0\text{px}$ horizontal total).
- **Typography**: $13.0\text{px}$ or $14.0\text{px}$ font size.
- **Estimated Width Formula**:
  $$\text{estimated\_width} = \max\left(48.0, \text{char\_count} \times \text{font\_size} \times 0.60 + \text{pad\_horizontal} + 10.0\right)$$
- **Minimum Width**: Guaranteed $\ge 48.0\text{px}$ even for empty or single-character strings.

### 5.3 Rust Struct & Method Signatures

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChipVariant {
    Filter,
    Assist,
    Input,
    Suggestion,
}

pub struct Chip {
    pub id: Option<String>,
    pub classes: Vec<String>,
    pub style: Style,
    pub text: String,
    pub variant: ChipVariant,
    pub selected: Option<Signal<bool>>,
    pub on_click: Option<Box<dyn FnMut()>>,
    is_hovered: bool,
    is_pressed: bool,
}

impl Chip {
    pub fn new(text: impl Into<String>) -> Self {
        let mut style = Style::default();
        style.border_radius = Some(BorderRadius::all(999.0));
        style.padding = Some(Insets::symmetric(6.0, 14.0));
        style.font_size = Some(13.0);

        Self {
            id: None,
            classes: Vec::new(),
            style,
            text: text.into(),
            variant: ChipVariant::Filter,
            selected: None,
            on_click: None,
            is_hovered: false,
            is_pressed: false,
        }
    }

    pub fn with_variant(mut self, variant: ChipVariant) -> Self {
        self.variant = variant;
        self
    }

    pub fn with_selected(mut self, selected: Signal<bool>) -> Self {
        self.selected = Some(selected);
        self
    }

    pub fn on_click<F: FnMut() + 'static>(mut self, handler: F) -> Self {
        self.on_click = Some(Box::new(handler));
        self
    }
}
```

### 5.4 Painting & Typography Centering Implementation

```rust
impl Widget for Chip {
    fn build_layout(&mut self, engine: &mut LayoutEngine) -> Result<NodeId, TaffyError> {
        let font_size = self.style.font_size.unwrap_or(13.0);
        let char_count = self.text.chars().count() as f32;
        let pad_h = self.style.padding.map(|p| p.left + p.right).unwrap_or(28.0);
        let pad_v = self.style.padding.map(|p| p.top + p.bottom).unwrap_or(12.0);

        let estimated_width = (char_count * font_size * 0.60 + pad_h + 10.0).max(48.0);
        let estimated_height = font_size * 1.4 + pad_v;

        let mut computed_style = self.style.clone();
        if computed_style.width.is_none() {
            computed_style.width = Some(Dimension::Px(estimated_width));
        }
        if computed_style.height.is_none() {
            computed_style.height = Some(Dimension::Px(estimated_height));
        }

        engine.new_leaf(&computed_style)
    }

    fn paint(&self, canvas: &mut Canvas, bounds: Rect) {
        let is_sel = self.selected.as_ref().map(|s| s.get()).unwrap_or(false);
        let radius = BorderRadius::all(bounds.size.height / 2.0);

        // 1. Container Background
        let bg_color = if is_sel {
            Color::from_hex("#4A4458").unwrap()
        } else if self.is_hovered {
            Color::from_hex("#2B2930").unwrap()
        } else {
            Color::from_hex("#1D1B20").unwrap()
        };
        canvas.fill_rounded_rect(bounds, radius, bg_color);

        // 2. Container Border
        let border_color = if is_sel {
            Color::from_hex("#CCC2DC").unwrap()
        } else {
            Color::from_hex("#49454F").unwrap()
        };
        canvas.stroke_rounded_rect(bounds, radius, border_color, 1.0);

        // 3. Centered Text Label
        let text_color = if is_sel {
            Color::from_hex("#E8DEF8").unwrap()
        } else {
            Color::from_hex("#CAC4D0").unwrap()
        };

        let font_size = self.style.font_size.unwrap_or(13.0);
        let char_count = self.text.chars().count() as f32;
        let text_w = char_count * font_size * 0.60;
        let origin_x = bounds.origin.x + ((bounds.size.width - text_w) / 2.0).max(0.0);
        let origin_y = bounds.origin.y + ((bounds.size.height + font_size * 0.8) / 2.0);

        canvas.draw_text(&self.text, Point::new(origin_x, origin_y), text_color, font_size, None);
    }
}
```

---

## 6. Declarative `.quick` Markup Integration Architecture

In `quick-markup` (`crates/quick-markup/src/builder.rs`), developers declare these widgets with reactive attribute bindings:

```xml
<VStack theme="material-you" style="padding: 24px; gap: 16px;">
    <!-- Switch Binding -->
    <Switch id="sw-dark" checked="$dark_mode" onchange="toggle_theme" />

    <!-- Checkbox Binding -->
    <Checkbox id="cb-notif" checked="$notifications" onchange="update_prefs" />

    <!-- Slider Binding -->
    <Slider id="sl-volume" min="0" max="100" value="$volume" onchange="set_volume" />

    <!-- Chip Variants -->
    <HStack style="gap: 8px;">
        <Chip id="chip-filter" variant="filter" text="Filter Option" selected="$filter_active" />
        <Chip id="chip-assist" variant="assist" text="Assist Action" onclick="trigger_assist" />
        <Chip id="chip-input" variant="input" text="Input Tag" onclick="remove_tag" />
        <Chip id="chip-sugg" variant="suggestion" text="Suggestion" onclick="apply_suggestion" />
    </HStack>
</VStack>
```

### Markup Builder Binding Rules:
1. **`Switch`**:
   - `checked="$sig"` binds to `data_ctx.bool_signals.get("sig")`.
   - `onchange="handler"` or `on_change="handler"` invokes `data_ctx.action_handlers.get("handler")`.
2. **`Checkbox`**:
   - `checked="$sig"` binds to `data_ctx.bool_signals.get("sig")`.
   - `indeterminate="$sig"` binds to `data_ctx.bool_signals.get("sig")`.
   - `onchange="handler"` binds to `data_ctx.action_handlers.get("handler")`.
3. **`Slider`**:
   - `value="$sig"` binds to `data_ctx.f32_signals.get("sig")`.
   - `min="0"`, `max="100"`, `steps="10"` parse into numeric floats/u32.
   - `onchange="handler"` binds to `data_ctx.action_handlers.get("handler")`.
4. **`Chip`**:
   - `selected="$sig"` binds to `data_ctx.bool_signals.get("sig")`.
   - `variant="filter|assist|input|suggestion"` parses to `ChipVariant`.
   - `onclick="handler"` or `on_click="handler"` binds to `data_ctx.action_handlers.get("handler")`.

---

## 7. Comprehensive Verification Matrix & Edge Case Strategy

### 7.1 Test Matrix Across Tiers

| Test Suite | Widget Focus | Test Names | Expected Verification Criteria |
| :--- | :--- | :--- | :--- |
| **Tier 1: Feature Coverage** | Switch | `test_f11_switch_checked_state_rendering`<br>`test_f11_switch_unchecked_state_rendering`<br>`test_f11_switch_toggle_signal_reactivity`<br>`test_f11_switch_on_change_callback_dispatch`<br>`test_f11_switch_dimensions_and_pill_geometry` | $52\times 32\text{px}$ track, thumb moves correctly, signal updates, `on_change` fires. |
| **Tier 1: Feature Coverage** | Checkbox | `test_f12_checkbox_checked_paint_checkmark`<br>`test_f12_checkbox_unchecked_paint_border`<br>`test_f12_checkbox_toggle_signal_reactivity`<br>`test_f12_checkbox_on_change_callback_dispatch`<br>`test_f12_checkbox_touch_bounds_and_box_size` | $24\times 24\text{px}$ bounds, checkmark painted with $\ge 3$ commands, unchecked is 1 stroke command. |
| **Tier 1: Feature Coverage** | Slider | `test_f13_slider_drag_and_value_update`<br>`test_f13_slider_active_and_inactive_track_paint`<br>`test_f13_slider_on_change_callback_stream`<br>`test_f13_slider_custom_range_scaling`<br>`test_f13_slider_layout_dimensions` | Inactive + active tracks + thumb painted ($\ge 3$ commands), continuous scrubbing, bounds scaling. |
| **Tier 1: Feature Coverage** | Chip | `test_f14_chip_selected_paint_style`<br>`test_f14_chip_unselected_paint_style`<br>`test_f14_chip_toggle_selected_signal`<br>`test_f14_chip_pill_geometry_and_layout`<br>`test_f14_chip_action_only_without_selection_signal` | Toggle selection, background/border color shift, non-toggle action chip handling. |
| **Tier 2: Boundary / BVA** | Switch | `test_f11_bva_switch_multiple_rapid_clicks`<br>`test_f11_bva_switch_click_outside_bounds_ignored`<br>`test_f11_bva_switch_pointer_cancel_resets_pressed`<br>`test_f11_bva_switch_custom_style_override`<br>`test_f11_bva_switch_external_signal_mutation` | Rapid toggling parity (10 toggles $\to$ original state), pointer cancel safety, external reactivity. |
| **Tier 2: Boundary / BVA** | Checkbox | `test_f12_bva_checkbox_rapid_toggling`<br>`test_f12_bva_checkbox_drag_out_cancels_toggle`<br>`test_f12_bva_checkbox_secondary_button_ignored`<br>`test_f12_bva_checkbox_custom_border_color`<br>`test_f12_bva_checkbox_keyboard_space_activation` | Drag out cancelation, secondary button rejection, 15 rapid toggles parity check. |
| **Tier 2: Boundary / BVA** | Slider | `test_f13_bva_slider_drag_beyond_left_edge_clamped`<br>`test_f13_bva_slider_drag_beyond_right_edge_clamped`<br>`test_f13_bva_slider_nan_and_infinity_resilience`<br>`test_f13_bva_slider_zero_range_min_equals_max`<br>`test_f13_bva_slider_negative_range_support` | Left/right edge clamping, NaN/Infinity division safety, zero-range resilience ($min==max$). |
| **Tier 2: Boundary / BVA** | Chip | `test_f14_bva_chip_empty_text_label`<br>`test_f14_bva_chip_very_long_label_layout`<br>`test_f14_bva_chip_click_released_outside`<br>`test_f14_bva_chip_custom_font_size_and_padding`<br>`test_f14_bva_chip_multiple_selection_group` | Width $\ge 48\text{px}$ on empty text, $500\text{px}+$ width on long text, outside release abort. |
| **Tier 3: Pairwise Combinations**| Composite | `test_f11_f13_switch_and_slider_signal_coupling`<br>`test_f12_f14_checkbox_and_chip_filter_group`<br>`test_f13_f15_slider_scrubbing_updates_progressbar` | Cross-widget signal cascading, master checkboxes controlling chip groups, slider driving progress bar. |
| **Tier 4: Scenario Integration** | App Workloads | `test_scenario_2_material_settings_form`<br>`test_scenario_3_filterable_card_dashboard` | Full `.quick` declarative UI parsing, data context binding, live rendering, multi-widget interactions. |

---

## 8. Summary of Findings & Next Steps for Implementer

1. **Current Codebase State**:
   - `quick-widgets` has foundational implementations of `Switch`, `Checkbox`, `Slider`, and `Chip` with basic tests passing.
   - All 213 E2E test cases across the workspace currently pass `cargo test --workspace`.
2. **Enhancement Recommendations for Milestone 2 Implementation Track**:
   - **`Switch`**: Ensure exact $52\times 32\text{px}$ track and $24\text{px}/16\text{px}$ thumb sliding geometry, and state layer halo blending.
   - **`Checkbox`**: Verify $18\times 18\text{px}$ centered container inside $24\times 24\text{px}$ touch area, checkmark vector coordinates, and explicit `indeterminate` dash support.
   - **`Slider`**: Add discrete `steps: Option<u32>` snapping and tick markers along the $8\text{px}$ pill track.
   - **`Chip`**: Add `ChipVariant` enum (`Filter`, `Assist`, `Input`, `Suggestion`) and variant-specific container tokens.
   - **`quick-markup`**: Validate that XML/TOML attributes (`variant`, `selected`, `checked`, `value`, `min`, `max`, `steps`, `onchange`, `onclick`) route flawlessly into the enhanced widget constructors.
