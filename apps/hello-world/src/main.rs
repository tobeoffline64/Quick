use quick::prelude::*;

#[cfg(feature = "mimalloc")]
#[global_allocator]
static GLOBAL: quick::core::MiMalloc = quick::core::MiMalloc;

fn main() -> Result<(), Box<dyn std::error::Error>> {
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

    // 2. Bind signals and actions to DataContext
    let mut data_ctx = DataContext::new();
    data_ctx.bind_signal("greeting", greeting);
    data_ctx.bind_signal("description", description);
    data_ctx.bind_bool_signal("gpu_enabled", gpu_enabled.clone());
    data_ctx.bind_f32_signal("brightness", brightness.clone());
    data_ctx.bind_bool_signal("chip_wayland", chip_wayland.clone());
    data_ctx.bind_bool_signal("chip_rust", chip_rust.clone());
    data_ctx.bind_bool_signal("chip_skia", chip_skia.clone());

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

    let s_chip = chip_skia.clone();
    data_ctx.bind_action("toggle_skia", move || {
        println!("🏷️ Chip 'Skia 2D' clicked! Active: {}", s_chip.get());
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
