use quick::prelude::*;

#[cfg(feature = "mimalloc")]
#[global_allocator]
static GLOBAL: quick::core::MiMalloc = quick::core::MiMalloc;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("⚡ Starting Quick Hello World Application on Wayland/X11...");

    // 1. Reactive state signals
    let click_count = Signal::new(0);
    let count_sig = click_count.clone();

    let greeting = create_computed(move || {
        let n = count_sig.get();
        if n == 0 {
            "Welcome to your first Quick native application on Linux!".to_string()
        } else {
            format!("🎉 You clicked the button {} times! (Zero-latency reactivity)", n)
        }
    });

    let count_desc = click_count.clone();
    let description = create_computed(move || {
        let n = count_desc.get();
        if n == 0 {
            "Click the button below to trigger reactive UI updates in real-time.".to_string()
        } else {
            format!("Rendering with Skia 2D in frame bump arena • {} state mutations", n)
        }
    });

    // 2. Bind reactive signals and actions to DataContext
    let mut data_ctx = DataContext::new();
    data_ctx.bind_signal("greeting", greeting);
    data_ctx.bind_signal("description", description);

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

    // 3. Load UI from app.quick
    let quick_content = include_str!("../app.quick");
    let app = App::new(
        WindowOptions::new()
            .title("Hello World - Quick Native UI")
            .size(680.0, 520.0),
    )
    .from_quick(quick_content, &mut data_ctx)
    .map_err(|e| format!("Failed to parse app.quick: {}", e))?;

    println!("🚀 Opening desktop window...");
    // 4. Launch interactive desktop window & event loop
    app.run()?;

    Ok(())
}
