use quick::prelude::*;

#[cfg(feature = "mimalloc")]
#[global_allocator]
static GLOBAL: quick::core::MiMalloc = quick::core::MiMalloc;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("⚡ Starting Quick 'Hello World' Project Application...");

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
            "Edit app.quick or src/main.rs to build fast, beautiful native desktop UIs.".to_string()
        } else {
            format!("Rendering with Skia 2D in frame bump arena • {} state mutations processed", n)
        }
    });

    // 2. Bind reactive signals and action callbacks
    let mut data_ctx = DataContext::new();
    data_ctx.bind_signal("greeting", greeting.clone());
    data_ctx.bind_signal("description", description.clone());

    let count_inc = click_count.clone();
    data_ctx.bind_action("on_click", move || {
        count_inc.update(|v| *v += 1);
        println!("👉 Interaction recorded! Clicks: {}", count_inc.get());
    });

    let count_reset = click_count.clone();
    data_ctx.bind_action("on_reset", move || {
        count_reset.set(0);
        println!("🔄 State reset to initial values.");
    });

    // 3. Load UI from app.quick declarative file
    let quick_content = include_str!("../app.quick");
    let mut app = App::new(
        WindowOptions::new()
            .title("Hello World - Quick Native Project")
            .size(680.0, 520.0),
    )
    .from_quick(quick_content, &mut data_ctx)
    .map_err(|e| format!("Failed to parse app.quick: {}", e))?;

    println!("✅ Successfully loaded UI from 'app.quick'!");
    println!("💬 Initial Greeting: '{}'", greeting.get());

    // 4. Initial Frame Render (Layout + Skia 2D Canvas Display List in Arena)
    let canvas = app.render_frame(Size::new(680.0, 520.0));
    println!("🎨 Initial frame rendered ({} draw commands in display list).", canvas.commands().len());

    // 5. Simulate interaction
    println!("\n🔄 Simulating click event on '✨ Click Me!' button...");
    click_count.update(|v| *v += 1);
    println!("💬 Updated Greeting: '{}'", greeting.get());

    let canvas_updated = app.render_frame(Size::new(680.0, 520.0));
    println!("🎨 Re-rendered frame in arena ({} draw commands).", canvas_updated.commands().len());

    println!("\n✨ Hello World project initialized and ready for development!");
    Ok(())
}
