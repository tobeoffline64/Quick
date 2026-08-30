# ⚡ Quick

**Quick** is a blazing-fast, lightweight, and modern native application and UI framework written in **100% Pure Rust**.

Designed with the performance and memory principles of **Bun's Rust architecture**, Quick combines high-throughput memory allocators (`mimalloc`), per-frame bump arenas (`bumpalo`), zero-copy SIMD parsing (`memchr`), fine-grained reactive signals, hardware-accelerated 2D graphics via Skia, and native Linux Wayland windowing.

---

## 🚀 Key Features

- **🦀 100% Pure Rust**: Zero memory leaks, zero use-after-free bugs, and compile-time thread-safety.
- **⚡ Bun-Inspired Performance**:
  - **`mimalloc` Global Allocator**: Lock-free thread-local memory pools eliminating allocation contention.
  - **Per-Frame Arena Allocators (`bumpalo`)**: $O(1)$ ephemeral layout and display list allocation with instant bulk resets.
  - **Zero-Copy SIMD Parsing (`memchr`, `simdutf8`)**: Vectorized `.quick`, XML, and CSS parsing scanning up to 64 bytes/cycle directly over memory-mapped assets.
- **🎨 Skia 2D Canvas Graphics Pipeline**: GPU-accelerated DirectContext (EGL / OpenGL ES) with software raster fallback.
- **🐧 Linux Wayland Native**: First-class Wayland protocol integration with dirty-rect damage region tracking and **0.0% CPU usage at idle**.
- **🔄 Fine-Grained Reactive Signals**: Arena-backed `Signal<T>`, `create_computed`, and `create_effect` updating target widgets directly with zero lock overhead.
- **📄 Declarative UI (`.quick` Format)**: Modern declarative format with embedded CSS styling, class selectors (`.btn`), pseudo-states (`:hover`, `:active`), and dynamic signal data-binding (`$state`).
- **📦 Compact Binary Footprint**: Configured with whole-program Fat LTO, symbol stripping, and `panic = "abort"` targeting release binaries under 8MB.

---

## 📖 Documentation & Guides

- 📘 **[How to Run Guide](HOW_TO_RUN.md)**: Step-by-step instructions to build, run examples, and deploy on Linux Wayland devices.
- 🔬 **[Efficiency Benchmark Plan](https://github.com/tobeoffline64/Quick)**: Performance metrics and device testing battery.

---

## 🛠️ Quick Start: Hello World (`.quick` format)

### 1. Declarative UI (`app.quick`)

```xml
<!-- app.quick -->
<VStack id="app-root" class="surface" style="padding: 32px; gap: 16px; align-items: center; justify-content: center; background: #0f111a;">
    <Style>
        Text.title { font-size: 24px; color: #c0caf5; font-weight: bold; }
        Text.greeting { font-size: 16px; color: #7dcfff; font-weight: bold; }
        Button.btn-greet { background: #7aa2f7; color: #0f111a; padding: 10px 24px; border-radius: 8px; font-weight: bold; }
        Button.btn-greet:hover { background: #89b4fa; }
    </Style>

    <Text class="title" text="⚡ Hello, Quick World!" />
    <Text id="greeting" class="greeting" text="$greeting" />

    <Button id="btn-say-hello" class="btn-greet" text="✨ Click Me!" onclick="greet" />
</VStack>
```

### 2. Rust Application Code (`src/main.rs`)

```rust
use quick::prelude::*;

// Optional: High-throughput mimalloc global allocator
#[cfg(feature = "mimalloc")]
#[global_allocator]
static GLOBAL: quick::core::MiMalloc = quick::core::MiMalloc;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 1. Reactive State
    let clicks = Signal::new(0);
    let clicks_sig = clicks.clone();

    let greeting = create_computed(move || {
        let n = clicks_sig.get();
        if n == 0 {
            "Welcome to the fastest native UI framework on Linux!".to_string()
        } else {
            format!("🎉 You clicked the button {} times!", n)
        }
    });

    // 2. DataContext Binding
    let mut data_ctx = DataContext::new();
    data_ctx.bind_signal("greeting", greeting);

    let count_inc = clicks.clone();
    data_ctx.bind_action("greet", move || count_inc.update(|v| *v += 1));

    // 3. Build App from .quick file
    let quick_content = include_str!("app.quick");
    let mut app = App::new(
        WindowOptions::new()
            .title("Hello World - Quick")
            .size(640.0, 480.0),
    )
    .from_quick(quick_content, &mut data_ctx)?;

    // 4. Render Frame (Runs layout & Skia canvas pipeline in frame arena)
    let canvas = app.render_frame(Size::new(640.0, 480.0));
    println!("Rendered frame with {} draw commands.", canvas.commands().len());

    Ok(())
}
```

### Run the App:
```bash
cargo run -p hello_world
```

---

## 📁 Workspace Crates

| Crate | Description |
| :--- | :--- |
| **[`quick-core`](crates/quick-core)** | Reactive Signals engine (`Signal<T>`, `create_computed`), `mimalloc` integration, geometry, and event definitions. |
| **[`quick-style`](crates/quick-style)** | CSS and XAML stylesheet parser with SIMD delimiter scanning, selector specificity, and property cascades. |
| **[`quick-render`](crates/quick-render)** | Skia 2D Canvas pipeline, display list recording, per-frame bump arena, and dirty-rect damage tracking. |
| **[`quick-window`](crates/quick-window)** | Linux Wayland windowing backend (via `winit` + `raw-window-handle`) and event translation bridge. |
| **[`quick-layout`](crates/quick-layout)** | High-speed Flexbox and CSS Grid layout solver powered by Taffy with frame arena memory. |
| **[`quick-widgets`](crates/quick-widgets)** | Core interactive widgets (`Container`, `Text`, `Button`, `TextInput`, `VStack`, `HStack`). |
| **[`quick-markup`](crates/quick-markup)** | Zero-copy Serde deserializer for `.quick`, XML, and TOML declarative UI documents with dynamic `DataContext` bindings. |
| **[`quick`](crates/quick)** | Unified umbrella crate and `App` lifecycle coordinator. |

---

## ⚡ Performance Profiles

Configured in `Cargo.toml`:

```toml
[profile.release]
opt-level = 3
lto = "fat"
codegen-units = 1
panic = "abort"
strip = true
overflow-checks = false
```

---

## 📄 License

Dual-licensed under either:
- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.
