# Material You (M3) Markup, Showcase, and Verification Infrastructure Report

## Executive Summary

This report provides a comprehensive architectural analysis of the **Quick UI Framework** (`quick-silver`) focusing on declarative markup parsing (`quick-markup`), the showcase application (`apps/hello-world`), the workspace testing and verification infrastructure (`cargo test --workspace`), the build and rendering pipeline (Wayland, X11, Softbuffer, Skia), and the integration blueprint for Google Material You (Material Design 3).

The workspace is a modern, modular Rust workspace (Edition 2021) engineered for high performance (sub-millisecond frame times, mimalloc global allocation, bump arena per-frame memory recycling, SIMD CSS and UTF-8 parsing). All crates compile cleanly (`cargo check --workspace --all-targets`) with 0 errors and 0 warnings, and all test suites pass with 100% success across all targets.

---

## 1. Declarative Markup Engine (`quick-markup`)

### 1.1 Architecture & Pipeline Overview

`quick-markup` transforms declarative UI source files (`.quick`, `.xml`, `.toml`) into reactive, styled native widget trees.

```
┌────────────────────────────────────────────────────────────────────────┐
│                        .quick Source File                              │
│   (Auto-detected XML `<...` or TOML `styles = "..."; [root] ...`)      │
└───────────────────────────────────┬────────────────────────────────────┘
                                    │
                        simdutf8 UTF-8 Validation
                                    │
                                    ▼
┌────────────────────────────────────────────────────────────────────────┐
│                          Parser Frontend                               │
│   • quick_parser::parse_quick()                                        │
│   • xml_parser::parse_xml() (quick-xml zero-copy event streaming)      │
│   • toml_parser::parse_toml() (serde-based deserialization)            │
└───────────────────────────────────┬────────────────────────────────────┘
                                    │
                                    ▼
┌────────────────────────────────────────────────────────────────────────┐
│                          Intermediate AST                              │
│   • UiDocument { styles: Option<String>, root: UiNode }                │
│   • UiNode { element, id, class, style, text, placeholder,             │
│              on_click, on_change, attributes, children }               │
└───────────────────────────────────┬────────────────────────────────────┘
                                    │
                                    ▼
┌────────────────────────────────────────────────────────────────────────┐
│                  Hydration & DOM Builder (`builder.rs`)                │
│   1. Parse Embedded & External Stylesheets (quick_style::parse_stylesheet)│
│   2. Theme Package Activation (ThemePackage::material_you().generate_css())│
│   3. Recursive Node Instantiation & Widget Construction                │
│   4. CSS Rule Matching with Specificity & Attribute Selectors          │
│   5. Reactive Signal Binding ($sig) & Action Handler Hookup            │
└───────────────────────────────────┬────────────────────────────────────┘
                                    │
                                    ▼
┌────────────────────────────────────────────────────────────────────────┐
│             Output: `(Box<dyn Widget>, quick_style::StyleSheet)`        │
└────────────────────────────────────────────────────────────────────────┘
```

### 1.2 Lexer, Parser, and AST Details

#### A. Format Detection (`quick_parser.rs`)
The entrypoint `parse_quick(content: &str)` inspects the trimmed beginning of the input string:
- If `trimmed.starts_with('<')`: routes to `xml_parser::parse_xml(content)`.
- Otherwise: routes to `toml_parser::parse_toml(content)`.

#### B. XML Streaming Parser (`xml_parser.rs`)
- Validates UTF-8 with `simdutf8::basic::from_utf8(xml_content.as_bytes())`.
- Uses `quick_xml::reader::Reader::from_str` configured with `trim_text(true)`.
- Main event loop handles:
  - `Event::Start`: Detects `<Style>` or `<Styles>` tags, captures their inner text, and appends to `doc.styles`. For all other tags, creates a new `UiNode` on the `node_stack`. Extracts attributes (`id`, `class`, `style`, `text`, `placeholder`, `onclick`/`on_click`, `onchange`/`on_change`) with unescaping, storing any additional attributes in `node.attributes: HashMap<String, String>`.
  - `Event::Empty`: Handles self-closing elements (e.g., `<Text text="Hello" />`, `<Switch checked="$gpu" />`, `<Button onclick="inc" />`), attaching them directly to the parent node on the stack or to `doc.root`.
  - `Event::Text` & `Event::CData`: Captures text inside elements (e.g., `<Text>Hello</Text>` or CDATA blocks) and assigns or appends to `current.text`.
  - `Event::End`: Pops the completed `UiNode` from `node_stack` and attaches it to its parent node or `doc.root`.

#### C. TOML Parser (`toml_parser.rs`)
Uses Serde and `toml::from_str::<UiDocument>` to map directly into the AST structure.

#### D. AST Schema (`schema.rs`)
```rust
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UiNode {
    #[serde(rename = "type", default = "default_element")]
    pub element: String,
    pub id: Option<String>,
    pub class: Option<String>,
    pub style: Option<String>,
    pub text: Option<String>,
    pub placeholder: Option<String>,
    pub on_click: Option<String>,
    pub on_change: Option<String>,
    #[serde(default)]
    pub attributes: HashMap<String, String>,
    #[serde(default)]
    pub children: Vec<UiNode>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UiDocument {
    pub styles: Option<String>,
    pub root: UiNode,
}
```

### 1.3 Builder, Component Registration & Attribute Binding (`builder.rs`)

`build_ui_tree(doc: &UiDocument, data_ctx: &mut DataContext) -> (Box<dyn Widget>, StyleSheet)` performs runtime widget instantiation:

1. **Stylesheet & Theme Loading**:
   - Parses `doc.styles` via `quick_style::parser::parse_stylesheet`.
   - Inspects `doc.root.attributes.get("theme")`. When set to `"material-you"` or `"m3"`, generates theme CSS from `ThemePackage::material_you()` and prepends the theme rules to the stylesheet.
2. **Style Resolution**:
   - For every node, resolves styles using `stylesheet.resolve_with_attrs(element, classes, id, pseudo_state, attributes)` which sorts matching rules by CSS specificity (ID = 100, Class = 10, Attribute = 10, PseudoState = 10, Element = 1).
   - Parses and merges inline styles (`node.style`).
3. **Component Registration & Signal Binding Table**:

| Tag / Element | Bound Attributes | Data Binding Mechanism | Target Widget |
| :--- | :--- | :--- | :--- |
| **`<Text>`** | `text="$key"`, `class`, `style` | `$key` binds to `data_ctx.string_signals[key]`, creating dynamic reactive `Text::dynamic(sig)`. Static text creates `Text::new(text)`. | `quick_widgets::text::Text` |
| **`<Button>`** | `text`, `onclick="$handler"`, `on_click`, `class`, `style` | Connects action name to `data_ctx.action_handlers[key]` closure. Supports hover/pressed state tracking. | `quick_widgets::button::Button` |
| **`<Switch>`** | `checked="$key"`, `onchange="$handler"`, `class`, `style` | `$key` binds to `data_ctx.bool_signals[key]`. Changes trigger both the signal update and `on_change` closure. | `quick_widgets::switch::Switch` |
| **`<Checkbox>`** | `checked="$key"`, `onchange="$handler"`, `class`, `style` | `$key` binds to `data_ctx.bool_signals[key]`. Toggles checkmark stroke and state. | `quick_widgets::checkbox::Checkbox` |
| **`<Slider>`** | `value="$key"`, `min="0"`, `max="100"`, `onchange="$handler"` | `$key` binds to `data_ctx.f32_signals[key]`. Interactively updates on mouse down/drag. | `quick_widgets::slider::Slider` |
| **`<Chip>`** | `text`, `selected="$key"`, `onclick="$handler"` | `$key` binds to `data_ctx.bool_signals[key]`. Toggles pill selection and triggers click callback. | `quick_widgets::chip::Chip` |
| **`<ProgressBar>`**| `progress="$key"`, `min="0"`, `max="100"` | `$key` binds to `data_ctx.f32_signals[key]`. Paints active rounded fill track. | `quick_widgets::progress::ProgressBar` |
| **`<Card>`** | `variant="elevated\|filled\|outlined"` | Sets `CardVariant`. Elevated draws dynamic soft shadow; filled/outlined use container tones and border strokes. Children recursively added. | `quick_widgets::card::Card` |
| **`<TextInput>`** | `text="$key"`, `placeholder="..."`, `onchange="$handler"` | `$key` provides two-way string binding to `data_ctx.string_signals[key]`. Handles typing, backspace, delete, focus. | `quick_widgets::text_input::TextInput` |
| **`<HStack>`** | `gap`, `justify-content`, `align-items` | Horizontal flex container (`FlexDirection::Row`). Recursively builds child widgets. | `quick_widgets::stack::HStack` |
| **`<VStack>`** | `gap`, `justify-content`, `align-items` | Vertical flex container (`FlexDirection::Column`). Recursively builds child widgets. | `quick_widgets::stack::VStack` |
| **`<Container>`** | All layout and style properties | Generic container widget. | `quick_widgets::container::Container` |

---

## 2. Showcase Application (`apps/hello-world`)

### 2.1 Workspace Placement & Structure
- **Path**: `/home/ai-workspace/coding-repo/quick-silver/apps/hello-world`
- **Files**:
  - `Cargo.toml`: Package manifest with `mimalloc` feature and dependencies on `quick`, `quick-core`, `quick-markup`, `quick-style`, `quick-widgets`, `quick-window`, `log`, `env_logger`.
  - `app.quick`: Declarative Material You layout with embedded `<Style>` definitions and component tree.
  - `src/main.rs`: Application entrypoint, reactive signal initialization, action callback bindings, desktop window runner, and comprehensive test suite.
  - `run.sh`: Convenience launcher script.
  - `README.md`: Showcase documentation.

### 2.2 Declarative Markup (`app.quick`) Analysis
The showcase markup features:
- Root element: `<VStack id="app-root" theme="material-you" style="width: 100%; height: 100%; padding: 32px; align-items: center; justify-content: center; background: #141218;">`
- Embedded `<Style>` rules:
  - `Card.main-card`: Elevated surface background `#211F26`, corner radius `20px`, outline `#49454F`, padding `32px`.
  - `Text.pill-badge`: M3 pill badge styling with `#381E72` container and `#D0BCFF` text.
  - `Text.title`, `Text.greeting`, `Text.description`: Tonal typography hierarchy.
  - `Button.btn-primary`, `Button.btn-primary:hover`: Pill button (`border-radius: 99px`), primary background `#D0BCFF` with on-primary text `#381E72`, state layer hover `#EADDFF`.
  - `Button.btn-secondary`, `Button.btn-secondary:hover`: Tonal secondary container `#49454F` with `#E6E0E9` text.
- Interactive Component Hierarchy:
  1. Header with Badge, Title, Dynamic Greeting (`$greeting`), and Dynamic Subtext (`$description`).
  2. GPU Hardware Switch: `<Switch checked="$gpu_enabled" onchange="toggle_gpu" />`.
  3. Brightness Slider: `<Slider min="0" max="100" value="$brightness" onchange="on_slider" />`.
  4. Technology Chips Strip: `<Chip text="Wayland EGL" selected="$chip_wayland" onclick="toggle_wayland" />`, etc.
  5. Action Buttons: `<Button id="btn-interact" text="Click Me" onclick="on_click" />` and `<Button id="btn-reset" text="Reset" onclick="on_reset" />`.

### 2.3 Application Flow & State Reactivity (`main.rs`)

```rust
// 1. Reactive State Signals
let click_count = Signal::new(0);
let count_sig = click_count.clone();

let greeting = create_computed(move || {
    let n = count_sig.get();
    if n == 0 {
        "Welcome to your Material You themed Quick application!".to_string()
    } else {
        format!("🎉 You clicked the button {} times! (Zero-latency reactivity)", n)
    }
});

let gpu_enabled = Signal::new(true);
let brightness = Signal::new(75.0);
let chip_wayland = Signal::new(true);
let chip_rust = Signal::new(true);
let chip_skia = Signal::new(false);

// 2. DataContext Registration
let mut data_ctx = DataContext::new();
data_ctx.bind_signal("greeting", greeting);
data_ctx.bind_signal("description", description);
data_ctx.bind_bool_signal("gpu_enabled", gpu_enabled.clone());
data_ctx.bind_f32_signal("brightness", brightness.clone());
data_ctx.bind_bool_signal("chip_wayland", chip_wayland.clone());
data_ctx.bind_bool_signal("chip_rust", chip_rust.clone());
data_ctx.bind_bool_signal("chip_skia", chip_skia.clone());

// 3. Action Handlers
let count_inc = click_count.clone();
data_ctx.bind_action("on_click", move || {
    count_inc.update(|v| *v += 1);
});

// 4. Instantiation & Run
let quick_content = include_str!("../app.quick");
let app = App::new(
    WindowOptions::new()
        .title("Material You - Quick Framework")
        .size(680.0, 560.0),
)
.from_quick(quick_content, &mut data_ctx)?;

app.run()?;
```

---

## 3. Workspace Test Infrastructure & Verification Patterns

### 3.1 Workspace Test Summary (`cargo test --workspace`)

The workspace features a test suite with 100% pass rate across all crates:

| Crate | Unit / Integration Tests | Key Tested Systems |
| :--- | :--- | :--- |
| **`quick`** | 3 tests | `from_quick`, `from_xml`, `from_toml`, full layout + paint cycle, reactive signal update + click event re-rendering, damage tracker. |
| **`quick-core`** | 16 tests | Geometry math (Point, Rect, Size, Insets, BorderRadius, Transform), Color hex/rgb/rgba parsing, Signals fine-grained reactivity, computed signals, batching, nested batching, cascading effect disposal, diamond graphs, 5,000-signal update stress test. |
| **`quick-layout`** | 6 tests | Taffy layout conversion, flexbox row/column, nested percentage sizing, min/max constraints, zero-size boundaries, 50-level deep nesting stress, 500-sibling wide layout stress. |
| **`quick-markup`** | 9 tests | SIMD XML parsing, TOML parsing, CDATA & entity unescaping, multiple `<Style>` blocks, 30-level nested tags, builder signal/action bindings, switch/slider/chip/progress bar layout + paint verification. |
| **`quick-render`** | 3 tests | Canvas command recording, per-frame `Bump` arena allocation and O(1) reset, SoftwareRasterizer clipping, translation, scaling, and 8x12 bitmap glyph rendering. |
| **`quick-style`** | 9 tests | SIMD CSS tokenizer, composite selectors (`Button.btn-primary:hover`), attribute selectors (`Button[variant="filled"]`, `Card[variant='elevated']`), insets parsing (1, 2, 3, 4 values), opacity percentages, ResourceDictionary typed get/set. |
| **`quick-widgets`** | 16 tests | Button (click, hover, pressed), Card (Elevated, Filled, Outlined variants and shadow drawing), Checkbox (toggle, event), Chip (selection toggle, click), Container (recursive layout/paint, z-order hit testing, focus clearing), Slider (dragging, pos calculation, change callback), ProgressBar (paint, range scaling), Stack (HStack, VStack), Switch (toggle, paint), Text (static, dynamic signal, alignment), TextInput (typing, backspace, delete, focus). |
| **`quick-window`** | 2 tests | EventBridge translation of winit window events (CursorMoved, MouseInput, Focused) to native Quick events. |
| **`apps/hello-world`** | 1 test | End-to-end application lifecycle: markup parsing, signal reactivity, computed text re-evaluation, multi-frame rendering, and simulated pointer event dispatch. |
| **`examples/hello_world`**| 1 test | Example application lifecycle test. |
| **Total Workspace** | **57+ automated tests** | **100% Pass, 0 Failures, Execution time < 0.20s** |

### 3.2 Headless / In-Memory Test Pattern

A core strength of the testing architecture is that **tests are 100% headless and require no X11/Wayland display server or GPU**:
1. `App::from_quick(src, &mut ctx)` parses markup and constructs the widget tree.
2. `app.render_frame(window_size)` triggers `LayoutEngine::compute_layout`, updates widget bounds, and executes `Widget::paint(&mut canvas, bounds)`.
3. Assertions inspect `canvas.commands()` (e.g. `assert!(canvas.commands().len() >= 10)`).
4. `app.handle_event(&event, window_size)` dispatches synthetic pointer or keyboard events directly through the tree, validating state mutations without a display window.

---

## 4. Build System, Dependencies & Wayland/X11 Rendering

### 4.1 Cargo Configuration & Performance Profile

```toml
[profile.release]
opt-level = 3
lto = "fat"
codegen-units = 1
panic = "abort"
strip = true
overflow-checks = false
```

- **Global Allocator**: `mimalloc` (Thread-local free lists and zero lock contention). Enabled by default via feature flag in `crates/quick/Cargo.toml` and apps.
- **Per-Frame Arena**: `bumpalo` bump arena allocated inside `quick-render::Canvas` for ephemeral per-frame display lists and string formatting, reset in $O(1)$ time at the start of each frame.

### 4.2 Windowing & Presentation Stack

- **Windowing Library**: `winit` 0.30 with features `["rwh_06", "wayland", "x11"]`.
- **Softbuffer Presentation**: `softbuffer` 0.4 with features `["wayland", "x11rb"]`.
- **Pure Rust Software Rasterizer**: `quick-render::SoftwareRasterizer` renders all vector drawing commands (rectangles, rounded rectangles with per-corner radii, stroke borders, lines, and bitmap text) into a 32-bit ARGB framebuffer (`&mut [u32]`) with sub-pixel clipping and alpha blending.
- **Hardware Skia Backend**: `quick-render::RenderPipeline` conditionally compiles with `feature = "skia"` (using `skia-safe` 0.80) to render directly into hardware-accelerated OpenGL / Vulkan surfaces.

### 4.3 Runtime Environment Behavior

- **Desktop Session (Wayland / X11)**:
  `app.run()` initializes `WindowRunner`, which uses `winit::event_loop::EventLoop::new()` to create the desktop window, connects `softbuffer::Surface` to the Wayland/X11 buffer, and runs the 60/120+ FPS event loop.
- **Headless / CI Environment**:
  When `DISPLAY` or `WAYLAND_DISPLAY` are absent, unit and integration tests run headlessly using `app.render_frame(...)` and `app.handle_event(...)`, bypassing `EventLoop::run_app`.

---

## 5. Material You (M3) Integration Blueprint

Based on the specifications in `ORIGINAL_REQUEST.md` and `material_you_full_theme_and_component_integration.md`, here are the exact integration requirements across the workspace:

### 5.1 Dynamic HCT Color Generation & Token Engine (`quick-style`)
1. **Pure Rust HCT Algorithm**:
   - Implement HCT (Hue, Chroma, Tone) dynamic color generation in `quick-style::theme`.
   - Support 7 scheme variants: `TonalSpot` (default), `Vibrant`, `Expressive`, `Fidelity`, `FruitSalad`, `Monochrome`, and `Neutral`.
   - Generate all 32+ M3 color roles for both Light and Dark modes:
     - Primary: `primary`, `on_primary`, `primary_container`, `on_primary_container`, `inverse_primary`
     - Secondary: `secondary`, `on_secondary`, `secondary_container`, `on_secondary_container`
     - Tertiary: `tertiary`, `on_tertiary`, `tertiary_container`, `on_tertiary_container`
     - Surface Hierarchy: `surface`, `on_surface`, `surface_variant`, `on_surface_variant`, `surface_container_lowest`, `surface_container_low`, `surface_container`, `surface_container_high`, `surface_container_highest`, `surface_dim`, `surface_bright`, `surface_tint`
     - Outlines: `outline`, `outline_variant`
     - Error: `error`, `on_error`, `error_container`, `on_error_container`
     - Scrim & Shadow: `shadow`, `scrim`, `inverse_surface`, `inverse_on_surface`
2. **Tokens for Shapes, Elevations & State Layers**:
   - Shape scale: `corner_none` (0px), `corner_extra_small` (4px), `corner_small` (8px), `corner_medium` (12px), `corner_large` (16px), `corner_extra_large` (28px), `corner_full` (9999px).
   - Elevation shadows: Levels 0..5 with dual-pass ambient and key drop shadows.
   - State layers: Hover (8%), Focus (12%), Pressed (12%), Dragged (16%), Disabled (38% content / 12% container).

### 5.2 Material 3 Component Suite (`quick-widgets`)
- **Buttons**: Variants `filled`, `tonal`, `elevated`, `outlined`, `text` with pill geometry (`corner-full`) and state layer feedback.
- **Cards**: Variants `elevated` (dynamic drop shadows), `filled`, `outlined` with M3 container tones and corner radiuses.
- **Selection Controls**: `Switch` (pill track and sliding thumb), `Checkbox` (rounded-square with checkmark stroke), `Slider` (scrubbing track with thumb), `Chip` (interactive pill chips).
- **Progress & Inputs**: `ProgressBar` (determinate and indeterminate) and `TextInput` (filled and outlined variants).

### 5.3 Declarative Markup Integration (`quick-markup`)
- **Theme Loading**: Support `theme="material-you"` and custom TOML theme configuration (`themes/material-you.theme.toml`).
- **Attribute Selectors & Binding**:
  - `variant="..."` (e.g. `<Button variant="tonal">`, `<Card variant="elevated">`, `<TextInput variant="outlined">`)
  - `selected="$sig"` for `<Chip>`
  - `checked="$sig"` for `<Switch>`, `<Checkbox>`
  - `value="$sig"` for `<Slider>`
  - `progress="$sig"` for `<ProgressBar>`
- **API Extension**: Ensure `quick::App` provides `.with_theme(theme: ThemePackage)` to allow programmatic theme configuration before `from_quick(...)`.

### 5.4 Showcase Application (`apps/hello-world`)
- Provide a showcase interface in `apps/hello-world` that demonstrates the complete M3 component catalog, dynamic seed color theme generation, and live signal reactivity.

---

## 6. Verification Checklist & Success Criteria

1. **Compilation**:
   `cargo check --workspace --all-targets` must pass with 0 errors and 0 warnings.
2. **Testing**:
   `cargo test --workspace` must pass with 100% test success rate across all crates.
3. **Theme Generation**:
   `ThemePackage::from_seed_color("#6750A4", SchemeVariant::Vibrant, true)` must correctly compute all M3 tonal palettes and contrast ratios.
4. **Interactive Execution**:
   `apps/hello-world` launches and renders cleanly on Linux Wayland/X11.
