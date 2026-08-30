use quick::prelude::*;

#[cfg(feature = "mimalloc")]
#[global_allocator]
static GLOBAL: quick::core::MiMalloc = quick::core::MiMalloc;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("⚡ Starting Standalone Quick Native Application...");

    // 1. Reactive State Signals
    let click_count = Signal::new(0);
    let count_sig = click_count.clone();

    let greeting = create_computed(move || {
        let n = count_sig.get();
        if n == 0 {
            "Welcome to your standalone Quick application!".to_string()
        } else {
            format!("🎉 You clicked the button {} times! (Zero-latency reactivity)", n)
        }
    });

    let count_desc = click_count.clone();
    let description = create_computed(move || {
        let n = count_desc.get();
        if n == 0 {
            "Edit app.quick or src/main.rs to build fast, beautiful native desktop UIs.".to_string()
        } else {
            format!("Rendering with Skia 2D in frame bump arena • {} state mutations", n)
        }
    });

    // 2. Bind signals and actions to DataContext
    let mut data_ctx = DataContext::new();
    data_ctx.bind_signal("greeting", greeting.clone());
    data_ctx.bind_signal("description", description.clone());

    let count_inc = click_count.clone();
    data_ctx.bind_action("on_click", move || {
        count_inc.update(|v| *v += 1);
        println!("👉 Clicked! Total clicks: {}", count_inc.get());
    });

    let count_reset = click_count.clone();
    data_ctx.bind_action("on_reset", move || {
        count_reset.set(0);
        println!("🔄 State reset!");
    });

    // 3. Load UI from app.quick
    let quick_content = include_str!("../app.quick");
    let mut app = App::new(
        WindowOptions::new()
            .title("Standalone Quick Application")
            .size(680.0, 520.0),
    )
    .from_quick(quick_content, &mut data_ctx)
    .map_err(|e| format!("Failed to parse app.quick: {}", e))?;

    println!("✅ Loaded app.quick successfully!");

    // 4. Render initial frame
    let canvas = app.render_frame(Size::new(680.0, 520.0));
    println!("🎨 Rendered frame with {} draw commands.", canvas.commands().len());

    Ok(())
}
