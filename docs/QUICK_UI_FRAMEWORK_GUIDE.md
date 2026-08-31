# Quick UI Framework — Complete Architectural Guide & Technical Reference

The **Quick UI Framework** is a lightweight, pure-Rust native desktop UI framework engineered for sub-millisecond layout calculation, zero-overhead reactive state management, and multi-design system theming (Avalonia Fluent, Google Material You, Noctalia Glassmorphism, and GNOME HIG Adaptive guidelines).

---

## 1. High-Level Architecture Overview

Quick is architected into 8 decoupled crates following a clean unidirectional data flow:

```mermaid
graph TD
    subgraph UI_Layer["Declarative & Widget Layer"]
        A[".quick Markup / XML / TOML"] -->|quick-markup| B[Widget Tree (quick-widgets)]
        C[Reactive Signals (quick-core)] <-->|Data Binding| B
        D[Style & Themes (quick-style)] -->|CSS / HCT Roles| B
    end

    subgraph Core_Engine["Core Engine & Layout"]
        B -->|Build Layout| E[Taffy Flexbox Engine (quick-layout)]
        B -->|Paint Commands| F[Canvas Command Buffer (quick-render)]
    end

    subgraph Backend_Layer["Windowing & Rasterization"]
        G[Winit Event Loop (quick-window)] -->|EventBridge / High-DPI Normalization| B
        F --> H[SoftwareRasterizer with GlyphCache & Span Fills (quick-render)]
        H --> I[Softbuffer Framebuffer / Wayland Surface]
    end
```

---

## 2. Crate-by-Crate Breakdown

### 2.1 `quick-core` — Foundations, Geometry & Reactive Graph
The foundational crate with zero external UI dependencies.
- **Geometry**: Primitive math types: `Point`, `Size`, `Rect`, `Color` (with RGBA, HEX, ARGB-u32 packing), `BorderRadius` (individual corner radii), `Insets` (top, right, bottom, left).
- **Reactive Signals**: Fine-grained reactive state container `Signal<T>` utilizing a thread-local dependency graph (`GRAPH`).
  - Automatic observer dependency tracking via `signal.get()` or `signal.with(|v| ...)`.
  - Batching support (`batch(|| { ... })`) to suppress intermediate updates.
  - Signal types supported: `Signal<String>`, `Signal<bool>`, `Signal<f32>`, `Signal<i32>`, `Signal<usize>`.
- **Event System**: Normalized input representations:
  - `PointerEvent` (`position`, `button`, `phase: Down | Moved | Up | Cancel`, `modifiers`).
  - `KeyEvent` (`key`, `state: Pressed | Released`, `text`, `modifiers`).
  - `ScrollDelta` (`LineDelta(x, y)` and `PixelDelta(x, y)`).
  - `FocusEvent` (`Gained | Lost`).

---

### 2.2 `quick-style` — Dynamic Colors, HCT Engine & Design Tokens
Provides pure-Rust color science and complete multi-theme styling capabilities:
- **Pure-Rust HCT Engine (`quick-style::color`)**:
  - `cam16.rs`: Comprehensive CIE CAM16 color appearance model with standard `D65` sRGB viewing conditions.
  - `cie.rs`: Forward and inverse linear sRGB $\leftrightarrow$ CIE $XYZ \leftrightarrow$ $L^*a^*b^*$ transformations with D65 illuminant white point normalization.
  - `gamut.rs`: Binary bisection chromaticity solver ensuring tone and hue preservation within the sRGB cube.
  - `contrast.rs`: WCAG 2.1 contrast ratio calculations and contrast-directed tone stepping.
- **Google Material You Dynamic Theming (`quick-style::theme`)**:
  - 7 scheme variants: `TonalSpot`, `Vibrant`, `Expressive`, `Fidelity`, `Content`, `Monochrome`, `Neutral`.
  - Derives **32+ M3 Color Roles** for Light and Dark modes (`primary`, `on_primary`, `primary_container`, `surface`, `surface_container`, `outline`, `error`, etc.).
  - Elevation shadow tokens (Levels 0 through 5 with dual-pass ambient and key drop shadows).
  - State layer opacities (`hover` 8%, `focus` 12%, `pressed` 12%).
- **Avalonia Fluent Base Theme (`quick-style::base`)**:
  - Automatically queries OS preferences (`dark-mode` detection via D-Bus `/org/freedesktop/appearance` on Linux Wayland/GNOME, Windows Registry, macOS defaults).
  - Queries OS accent colors (e.g. Ubuntu Orange `#E95420`, Windows Accent, macOS Accent).
- **Noctalia Glassmorphic Theme (`quick-style::noctalia`)**:
  - Deep space indigo backgrounds (`#070722`), electric gold/cyan accents, and 2.5D acrylic frosted glass tokens.

---

### 2.3 `quick-layout` — Flexbox Engine Wrapper
- Wraps the high-performance `taffy` 0.5 flexbox and CSS grid engine.
- Translates `quick_style::Style` (`width`, `height`, `min_width`, `padding`, `margin`, `flex_direction`, `justify_content`, `align_items`, `gap`) into Taffy layout styles.
- Supports measure functions for text and dynamic intrinsic dimension calculation.

---

### 2.4 `quick-render` — Command Buffer & High-Performance Rasterizer
- **`Canvas`**: Retained display-list recording `DrawCommand`s:
  - `Clear`, `FillRect`, `StrokeRect`, `FillRoundedRect`, `StrokeRoundedRect`, `DrawShadow`, `DrawText`, `DrawLine`, `PushClip`, `PopClip`, `Translate`, `Scale`, `Save`, `Restore`.
- **`SoftwareRasterizer`**: Blazing fast CPU software rendering with hardware-level optimizations:
  - **In-Memory `GlyphCache`**: Caches TrueType font glyph bitmaps rendered via `fontdue` by `(char, scale_key)`, eliminating >98% of Bezier curve calculations per frame.
  - **Row-Span Fast Memory Fills**: Fills solid interior rectangular spans of cards and backgrounds using direct SIMD memory writes (`buffer[start..end].fill(pixel)`), only computing distance formulas for the 4 corner squares (`r × r`).
  - **Damage Tracking (`DamageTracker`)**: Tracks dirty screen bounding boxes to minimize frame presentations.

---

### 2.5 `quick-widgets` — Comprehensive Widget Suite
All widgets implement the core `Widget` trait:
```rust
pub trait Widget: Any + Send + Sync {
    fn widget_type(&self) -> &'static str;
    fn id(&self) -> Option<&str>;
    fn classes(&self) -> &[String];
    fn style(&self) -> &Style;
    fn style_mut(&mut self) -> &mut Style;
    fn build_layout(&mut self, engine: &mut LayoutEngine) -> Result<NodeId, TaffyError>;
    fn update_layout(&mut self, engine: &LayoutEngine, origin: Point);
    fn paint(&self, canvas: &mut Canvas, bounds: Rect);
    fn handle_event(&mut self, event: &Event, bounds: Rect) -> bool;
}
```

#### Included Base & Themed Components:
| Component | Supported Features & Variants |
|---|---|
| **`Button`** | `Filled`, `Tonal`, `Elevated`, `Outlined`, `Text` with state-layer feedback. |
| **`Card`** | `Elevated` (ambient/key drop shadow), `Filled`, `Outlined` with corner radii. |
| **`Switch`** | Animated pill track, sliding thumb, reactive boolean signal binding. |
| **`Checkbox`** | Rounded square, animated checkmark strokes, checked/unchecked/indeterminate. |
| **`Slider`** | Scrubbing track, active fill, circular thumb with state-layer halo, float signals. |
| **`Chip`** | Interactive pill chips with border outlines and selected toggle states. |
| **`ProgressBar`** | Determinate fill ratio and indeterminate continuous motion. |
| **`TextInput`** | Interactive cursor, text insertion, selection, backspace/delete, placeholder. |
| **`TabControl`** | GNOME `AdwViewSwitcher` centered tab pills with child page management. |
| **`ScrollViewer`** | Smooth vertical/horizontal scrolling, auto-hiding adaptive scrollbars. |
| **`Container` / `VStack` / `HStack`** | Flex containers with recursive layout and event dispatching. |
| **`NoctaliaButton` / `NoctaliaCard`** | Glassmorphic acrylic components with glow highlights and gradient borders. |
| **`CountdownRing` / `AnalogClock`** | Real-time radial vector instruments and smooth clock hands. |
| **`Segmented` / `NoctaliaSlider`** | Pill segmented switches and icon-integrated volume/brightness sliders. |
| **`NoctaliaGraph` / `NoctaliaCalendar`** | Real-time spline graphs, interactive calendars, and RGB color pickers. |
| **`FramelessTitleBar` / `NoctaliaBar`** | Draggable frameless window headers and Wayland desktop status bars. |

---

### 2.6 `quick-markup` — Declarative `.quick` UI Language
Enables defining interfaces declaratively in XML/XAML or TOML with reactive data-context signal bindings.
- **Signal Expressions**: `$signal_name` binds reactive signals to component properties:
  - Text binding: `<Text text="$greeting" />`
  - Boolean binding: `<Switch checked="$wifi_enabled" />`
  - Float binding: `<Slider value="$volume" min="0" max="100" />`
  - Chip selection: `<Chip text="Option A" selected="$chip_a" />`
- **Action Expressions**: `onclick="handle_save"`, `onchange="on_slider_moved"`.
- **Theme Injection**: `theme="material-you"` or `theme="noctalia"` applies dynamic color tokens and token sheets to container subtrees.

---

### 2.7 `quick-window` — Platform Windowing & High-DPI Event Loop
- Integrates `winit` 0.30 and `softbuffer` for cross-platform Wayland/X11/macOS/Windows support.
- **`EventBridge`**: Normalizes `winit` physical pixel coordinates to **logical layout points** using `scale_factor` so that clicks and hover states remain 100% accurate on HiDPI and Wayland scaled outputs.
- **Wayland Layer Shell**: Protocol support for desktop bars, docks, wallpapers, and notification panels.
- **Frameless Window Management**: Hit-testing and window drag-resizing on custom titlebars.

---

### 2.8 `quick` — Top-Level Facade
Combines all crates into a clean developer API:
- `App::new(WindowOptions)`
- `App::from_quick(quick_str, &mut data_ctx)`
- `App::from_quick_file(path, &mut data_ctx)`
- `App::run()`

---

## 3. The Declarative `.quick` Markup Specification

A `.quick` file structure defines the window and component hierarchy:

```xml
<!-- Example: my_app.quick -->
<App title="My Quick Application" width="1024" height="768">
  <VStack style="width: 100%; height: 100%; padding: 24px; gap: 16px;">

    <!-- Header Bar -->
    <HStack style="width: 100%; justify-content: space-between; align-items: center;">
      <Text text="$header_title" style="font-size: 18px; font-weight: bold;" />
      <Button text="Settings" variant="tonal" onclick="open_settings" />
    </HStack>

    <!-- Centered Tab View -->
    <TabControl tabs="Dashboard,Settings,Analytics" selected="0" style="width: 100%; height: 100%;">
      
      <!-- Tab 1: Dashboard -->
      <ScrollViewer style="width: 100%; height: 100%;">
        <VStack style="gap: 16px;">
          <Card variant="elevated" style="padding: 20px; gap: 12px;">
            <Text text="System Controls" style="font-size: 14px; font-weight: bold;" />
            <HStack style="gap: 20px; align-items: center;">
              <Switch checked="$power_mode" onchange="on_toggle_power" />
              <Slider min="0" max="100" value="$brightness" style="width: 240px;" />
            </HStack>
          </Card>
        </VStack>
      </ScrollViewer>

      <!-- Tab 2: Settings -->
      <VStack style="gap: 12px;" theme="material-you">
        <TextInput placeholder="Enter device name…" value="$device_name" />
      </VStack>

      <!-- Tab 3: Analytics -->
      <VStack style="gap: 12px;" theme="noctalia">
        <NoctaliaGraph label="Memory Usage" />
      </VStack>

    </TabControl>

  </VStack>
</App>
```

---

## 4. End-to-End Application Example in Rust

```rust
use quick::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 1. Define Reactive Signals
    let header_title = Signal::new("⚡ Quick System Monitor".to_string());
    let power_mode = Signal::new(true);
    let brightness = Signal::new(75.0f32);
    let device_name = Signal::new("Noctalia-Station-01".to_string());

    // 2. Setup DataContext and Action Handlers
    let mut data_ctx = DataContext::new();
    data_ctx.bind_string_signal("header_title", header_title);
    data_ctx.bind_bool_signal("power_mode", power_mode);
    data_ctx.bind_f32_signal("brightness", brightness);
    data_ctx.bind_string_signal("device_name", device_name);

    data_ctx.register_action("open_settings", || {
        println!("Opening settings dialog...");
    });

    data_ctx.register_action("on_toggle_power", || {
        println!("Power toggle state updated!");
    });

    // 3. Load Markup & Launch Native Window
    let quick_content = include_str!("../app.quick");
    let app = App::new(WindowOptions::new().title("Quick App").size(1024.0, 768.0))
        .from_quick(quick_content, &mut data_ctx)?;

    // 4. Run native event loop (or run in headless CI mode when QUICK_HEADLESS=1)
    app.run()
}
```

---

## 5. Performance Invariants & Best Practices

1. **High-DPI Coordinate Normalization**:
   All pointer inputs are divided by `scale_factor` to ensure logical layout bounds hit-testing matches cursor visual position on all monitors.
2. **Layout Geometry Caching**:
   `App::ensure_layout` caches computed bounding boxes. Mouse movement events perform $O(1)$ hit-testing against existing rectangles without resetting or recomputing Taffy flexbox trees.
3. **Hover Invalidation**:
   Widgets only request redraws when hover status transitions (`prev_hover != self.is_hovered`), maintaining solid 60+ FPS rendering without frame-loop thrashing.
4. **Glyph & Span Rasterization**:
   TrueType glyphs are cached in memory in `GlyphCache`. Rectangular body spans are fast-filled using contiguous slice memory writes, minimizing CPU rasterization time to sub-millisecond execution per frame.
5. **Headless CI Testing**:
   When `QUICK_HEADLESS=1` is set, all apps parse markup, bind signals, build full layout graphs, and rasterize canvas commands without requiring an active X11/Wayland display server.
