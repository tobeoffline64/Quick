# quick-widgets

**Material Design 3 base component suite for the Quick UI Framework** — headless, renderer-agnostic, reactive widgets.

## Overview

`quick-widgets` provides the complete M3 base widget library for Quick. Every widget is:

- **Headless** — pure geometry, style, and event logic; no platform dependencies
- **Renderer-agnostic** — paints via `quick-render::Canvas` draw commands
- **Reactive** — driven by `Signal<T>` values from `quick-core::signals`
- **M3-compliant** — geometry, corner radii, state layers, and color roles match Material Design 3

## Component Suite

| Widget | Variants | M3 Spec |
|--------|----------|---------|
| `Button` | `Filled`, `Tonal`, `Elevated`, `Outlined`, `Text` | Pill geometry, state layer ripple |
| `Card` | `Elevated`, `Filled`, `Outlined` | corner-medium radius, elevation tint |
| `Switch` | On / Off | Pill track, sliding thumb |
| `Checkbox` | Checked / Unchecked / Indeterminate | Rounded-square, checkmark stroke |
| `Slider` | Continuous scrub | Active/inactive track split, circular thumb |
| `Chip` | Selected / Unselected | Interactive pill, toggle feedback |
| `ProgressBar` | Determinate / Indeterminate | Animated track |
| `TextInput` | Single-line | Placeholder, cursor, on_change callback |
| `Text` | Label / Body / Title / Display | Inline text rendering |
| `Container` / `HStack` / `VStack` | Layout containers | Flexbox-style via `quick-layout` |
| `StateLayer` | Hover / Focus / Pressed | M3 opacity overlays (8%, 12%, 12%) |

## Quick Start

```rust
use quick_widgets::button::{Button, ButtonVariant};
use quick_widgets::card::{Card, CardVariant};
use quick_widgets::switch::Switch;
use quick_core::signals::Signal;

let mut btn = Button::new("Get Started");
btn.variant = ButtonVariant::Filled;
btn.on_click = Some(Box::new(|| println!("Clicked!")));

let mut card = Card::new(CardVariant::Elevated);
card.add_child(Box::new(btn));

let enabled = Signal::new(false);
let mut sw = Switch::new(enabled.clone());
```

## Declarative Usage via `.quick` Markup

```xml
<VStack theme="material-you" style="padding: 32px;">
    <Card variant="elevated">
        <Button text="Submit" variant="filled" onclick="on_submit" />
        <Switch checked="$is_enabled" onchange="on_toggle" />
        <Slider min="0" max="100" value="$brightness" />
        <Chip text="Wayland" selected="$chip_active" onclick="on_chip" />
        <ProgressBar progress="$loading" min="0" max="1" />
        <TextInput placeholder="Enter text…" value="$input_text" />
    </Card>
</VStack>
```

## Widget Trait

```rust
pub trait Widget {
    fn build_layout(&mut self, engine: &mut LayoutEngine) -> Result<NodeId, LayoutError>;
    fn update_layout(&mut self, engine: &LayoutEngine, origin: Point);
    fn paint(&self, canvas: &mut Canvas, bounds: Rect);
    fn handle_event(&mut self, event: &Event, bounds: Rect) -> EventResult;
}
```
