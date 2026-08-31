use quick::prelude::*;
use quick_style::theme::{SchemeVariant, ThemePackage};

#[cfg(feature = "mimalloc")]
#[global_allocator]
static GLOBAL: quick::core::MiMalloc = quick::core::MiMalloc;

/// Headless showcase: parse app.quick, generate M3 color roles + widget tree, print to stdout.
/// Activated when `QUICK_HEADLESS=1` env var is set (ideal for CI / servers without a display).
fn run_headless() -> Result<(), Box<dyn std::error::Error>> {
    println!("⚡ Quick Framework — Headless Material You Showcase");
    println!("══════════════════════════════════════════════════");

    // ── 1. Generate M3 color roles from default seed ──────────────────────────
    let seed = quick_core::geometry::Color::from_hex("#6750A4")?; // M3 baseline purple
    for &dark in &[false, true] {
        let mode = if dark { "Dark" } else { "Light" };
        let pkg = ThemePackage::from_seed_color(seed, SchemeVariant::TonalSpot, dark);
        let cs = &pkg.color_scheme;
        println!("\n🎨 Material You Color Roles — {mode} Mode (seed #6750A4, TonalSpot)");
        println!("  primary            = {}", cs.primary.to_hex());
        println!("  on_primary         = {}", cs.on_primary.to_hex());
        println!("  primary_container  = {}", cs.primary_container.to_hex());
        println!("  secondary          = {}", cs.secondary.to_hex());
        println!("  tertiary           = {}", cs.tertiary.to_hex());
        println!("  surface            = {}", cs.surface.to_hex());
        println!("  on_surface         = {}", cs.on_surface.to_hex());
        println!("  surface_container  = {}", cs.surface_container.to_hex());
        println!("  outline            = {}", cs.outline.to_hex());
        println!("  outline_variant    = {}", cs.outline_variant.to_hex());
        println!("  error              = {}", cs.error.to_hex());
        println!("  on_error           = {}", cs.on_error.to_hex());
        println!("  inverse_surface    = {}", cs.inverse_surface.to_hex());
    }

    // ── 2. Build UI widget tree from app.quick ────────────────────────────────
    let quick_content = include_str!("../app.quick");

    let greeting = Signal::new("Welcome to your Material You themed Quick application!".to_string());
    let description = Signal::new("Unified base widgets skinned dynamically via Material You theme package.".to_string());
    let gpu_enabled = Signal::new(true);
    let brightness = Signal::new(75.0f32);
    let chip_wayland = Signal::new(true);
    let chip_rust = Signal::new(true);
    let chip_skia = Signal::new(false);

    let mut data_ctx = DataContext::new();
    data_ctx.bind_signal("greeting", greeting);
    data_ctx.bind_signal("description", description);
    data_ctx.bind_bool_signal("gpu_enabled", gpu_enabled);
    data_ctx.bind_f32_signal("brightness", brightness);
    data_ctx.bind_bool_signal("chip_wayland", chip_wayland);
    data_ctx.bind_bool_signal("chip_rust", chip_rust);
    data_ctx.bind_bool_signal("chip_skia", chip_skia);
    data_ctx.bind_action("on_click", || {});
    data_ctx.bind_action("on_reset", || {});
    data_ctx.bind_action("toggle_gpu", || {});
    data_ctx.bind_action("on_slider", || {});
    data_ctx.bind_action("toggle_wayland", || {});
    data_ctx.bind_action("toggle_rust", || {});
    data_ctx.bind_action("toggle_skia", || {});

    let mut app = App::new(
        WindowOptions::new()
            .title("Material You - Quick Framework [headless]")
            .size(680.0, 560.0),
    )
    .from_quick(quick_content, &mut data_ctx)
    .map_err(|e| format!("Failed to parse app.quick: {e}"))?;

    // ── 3. Render one frame without a window ─────────────────────────────────
    let canvas = app.render_frame(Size::new(680.0, 560.0));
    let cmd_count = canvas.commands().len();

    println!("\n🌲 Widget Tree (headless render)");
    println!("  app.quick parsed     ✓");
    println!("  signals bound        ✓  (greeting, description, gpu_enabled, brightness, chips×3)");
    println!("  canvas commands      {cmd_count}  (paint calls across all widgets)");
    println!("  components present   Button×2, Card, Switch, Slider, Chip×3, ProgressBar, TextInput");

    println!("\n✅ Headless showcase complete — exit 0");
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // ── Headless CI mode: QUICK_HEADLESS=1 ────────────────────────────────────
    if std::env::var("QUICK_HEADLESS").as_deref() == Ok("1") {
        return run_headless();
    }

    println!("⚡ Starting Quick Material You Theme Application on Wayland/X11...");

    // 1. Reactive state signals
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

    let chip_wayland = Signal::new(true);
    let chip_rust = Signal::new(true);
    let chip_vello = Signal::new(true);

    let count_desc = click_count.clone();
    let description = create_computed(move || {
        let n = count_desc.get();
        if n == 0 {
            "Unified base widgets skinned dynamically via Material You theme package.".to_string()
        } else {
            format!("Rendering with Vello GPU Compute (WGPU) & SIMD Software fallback • {} state mutations", n)
        }
    });

    let gpu_enabled = Signal::new(true);
    let brightness = Signal::new(75.0);

    // 2. Bind signals and actions to DataContext
    let mut data_ctx = DataContext::new();
    data_ctx.bind_signal("greeting", greeting);
    data_ctx.bind_signal("description", description);
    data_ctx.bind_bool_signal("gpu_enabled", gpu_enabled.clone());
    data_ctx.bind_f32_signal("brightness", brightness.clone());
    data_ctx.bind_bool_signal("chip_wayland", chip_wayland.clone());
    data_ctx.bind_bool_signal("chip_rust", chip_rust.clone());
    data_ctx.bind_bool_signal("chip_vello", chip_vello.clone());
    data_ctx.bind_bool_signal("chip_skia", chip_vello.clone()); // backwards compatibility

    let count_inc = click_count.clone();
    data_ctx.bind_action("on_click", move || {
        count_inc.update(|v| *v += 1);
        println!("👉 Button Clicked! Count: {}", count_inc.get());
    });

    let count_reset = click_count.clone();
    data_ctx.bind_action("on_reset", move || {
        count_reset.set(0);
        println!("🔄 State Reset!");
    });

    let gpu_toggle = gpu_enabled.clone();
    data_ctx.bind_action("toggle_gpu", move || {
        let s = !gpu_toggle.get();
        println!("⚡ Switch toggled -> GPU Acceleration: {}", s);
    });

    let b_sig = brightness.clone();
    data_ctx.bind_action("on_slider", move || {
        println!("🎚️ Slider adjusted -> Brightness: {:.1}%", b_sig.get());
    });

    let w_chip = chip_wayland.clone();
    data_ctx.bind_action("toggle_wayland", move || {
        println!("🏷️ Chip 'Wayland EGL' clicked! Active: {}", w_chip.get());
    });

    let r_chip = chip_rust.clone();
    data_ctx.bind_action("toggle_rust", move || {
        println!("🏷️ Chip 'Pure Rust' clicked! Active: {}", r_chip.get());
    });

    let s_chip = chip_vello.clone();
    data_ctx.bind_action("toggle_skia", move || {
        println!("🏷️ Chip 'Vello GPU' clicked! Active: {}", s_chip.get());
    });

    // 3. Load UI from app.quick (with theme="material-you")
    let quick_content = include_str!("../app.quick");
    let app = App::new(
        WindowOptions::new()
            .title("Material You - Quick Framework")
            .size(680.0, 560.0),
    )
    .from_quick(quick_content, &mut data_ctx)
    .map_err(|e| format!("Failed to parse app.quick: {}", e))?;

    println!("🚀 Opening desktop window with Material You theme package...");
    // 4. Launch interactive desktop window & event loop
    app.run()?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::rc::Rc;

    #[test]
    fn test_hello_world_app_lifecycle_and_interactions() {

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

        let count_desc = click_count.clone();
        let description = create_computed(move || {
            let n = count_desc.get();
            if n == 0 {
                "Unified base widgets skinned dynamically via Material You theme package.".to_string()
            } else {
                format!("Rendering with Skia 2D in frame bump arena • {} state mutations", n)
            }
        });

        let gpu_enabled = Signal::new(true);
        let brightness = Signal::new(75.0);

        let chip_wayland = Signal::new(true);
        let chip_rust = Signal::new(true);
        let chip_skia = Signal::new(false);

        let mut data_ctx = DataContext::new();
        data_ctx.bind_signal("greeting", greeting.clone());
        data_ctx.bind_signal("description", description.clone());
        data_ctx.bind_bool_signal("gpu_enabled", gpu_enabled.clone());
        data_ctx.bind_f32_signal("brightness", brightness.clone());
        data_ctx.bind_bool_signal("chip_wayland", chip_wayland.clone());
        data_ctx.bind_bool_signal("chip_rust", chip_rust.clone());
        data_ctx.bind_bool_signal("chip_skia", chip_skia.clone());

        let count_inc = click_count.clone();
        let count_inc_cl = count_inc.clone();
        data_ctx.bind_action("on_click", move || {
            count_inc_cl.update(|v| *v += 1);
        });

        let count_reset = click_count.clone();
        data_ctx.bind_action("on_reset", move || {
            count_reset.set(0);
        });

        let gpu_toggle = gpu_enabled.clone();
        data_ctx.bind_action("toggle_gpu", move || {
            gpu_toggle.set(!gpu_toggle.get());
        });

        let slider_called = Rc::new(std::cell::RefCell::new(false));
        let sl_cl = slider_called.clone();
        data_ctx.bind_action("on_slider", move || {
            *sl_cl.borrow_mut() = true;
        });

        let wayland_called = Rc::new(std::cell::RefCell::new(false));
        let w_cl = wayland_called.clone();
        data_ctx.bind_action("toggle_wayland", move || {
            *w_cl.borrow_mut() = true;
        });

        let quick_content = include_str!("../app.quick");
        let mut app = App::new(
            WindowOptions::new()
                .title("Material You - Quick Framework")
                .size(680.0, 560.0),
        )
        .from_quick(quick_content, &mut data_ctx)
        .expect("Failed to parse app.quick");

        let window_size = Size::new(680.0, 560.0);
        let canvas = app.render_frame(window_size);
        assert!(canvas.commands().len() >= 10, "Canvas must record render commands for all components");

        assert_eq!(click_count.get(), 0);
        assert!(greeting.get().contains("Welcome"));

        count_inc.update(|v| *v += 1);
        assert_eq!(click_count.get(), 1);
        assert!(greeting.get().contains("1 times"));

        let canvas2 = app.render_frame(window_size);
        assert!(canvas2.commands().len() >= 10);

        // Test event handling across app bounds
        let dummy_down = quick_core::event::Event::Pointer(quick_core::event::PointerEvent {
            position: quick_core::geometry::Point::new(340.0, 280.0),
            button: Some(quick_core::event::PointerButton::Primary),
            phase: quick_core::event::PointerPhase::Down,
            modifiers: Default::default(),
        });
        let _ = app.handle_event(&dummy_down, window_size);

        let canvas3 = app.render_frame(window_size);
        assert!(canvas3.commands().len() >= 10);
    }

    /// Verifies that the headless showcase (QUICK_HEADLESS=1 mode) runs to completion
    /// without panicking: parses app.quick, generates M3 color roles, and renders a frame.
    #[test]
    fn test_headless_showcase_runs_to_completion() {
        // run_headless() is the same code path as QUICK_HEADLESS=1 — call it directly.
        run_headless().expect("Headless showcase must complete without error");
    }

    /// Verifies that ThemePackage::from_seed_color produces non-empty, valid M3 hex roles.
    #[test]
    fn test_headless_m3_color_roles_are_valid_hex() {
        use quick_style::theme::{SchemeVariant, ThemePackage};
        let seed = quick_core::geometry::Color::from_hex("#6750A4").unwrap();
        for &dark in &[false, true] {
            let pkg = ThemePackage::from_seed_color(seed, SchemeVariant::TonalSpot, dark);
            let cs = &pkg.color_scheme;
            // All hex strings must be 7 chars (#RRGGBB) and start with '#'
            for hex in [
                cs.primary.to_hex(),
                cs.on_primary.to_hex(),
                cs.secondary.to_hex(),
                cs.surface.to_hex(),
                cs.on_surface.to_hex(),
                cs.outline.to_hex(),
                cs.error.to_hex(),
                cs.inverse_surface.to_hex(),
            ] {
                assert!(hex.starts_with('#'), "hex role must start with '#': {hex}");
                assert_eq!(hex.len(), 7, "hex role must be #RRGGBB format: {hex}");
            }
        }
    }
}
