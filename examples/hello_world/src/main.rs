use quick::prelude::*;

#[cfg(feature = "mimalloc")]
#[global_allocator]
static GLOBAL: quick::core::MiMalloc = quick::core::MiMalloc;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("⚡ Starting Quick 'Hello World' Application on Wayland/X11...");

    // 1. Reactive state signals
    let click_count = Signal::new(0);
    let count_sig = click_count.clone();

    let greeting = create_computed(move || {
        let n = count_sig.get();
        if n == 0 {
            "Welcome to the fastest native UI framework on Linux!".to_string()
        } else {
            format!("🎉 You clicked the button {} times! (Zero-latency reactivity)", n)
        }
    });

    let count_sub = click_count.clone();
    let subtext = create_computed(move || {
        let n = count_sub.get();
        if n == 0 {
            "Click the button below to trigger high-speed reactive updates.".to_string()
        } else {
            format!("Running Skia 2D Canvas pipeline in frame arena • {} state mutations", n)
        }
    });

    // 2. Bind reactive signals & actions to DataContext
    let mut data_ctx = DataContext::new();
    data_ctx.bind_signal("greeting", greeting);
    data_ctx.bind_signal("subtext", subtext);

    let inc_count = click_count.clone();
    data_ctx.bind_action("greet", move || {
        inc_count.update(|v| *v += 1);
        println!("👉 Button clicked! Total clicks: {}", inc_count.get());
    });

    let reset_count = click_count.clone();
    data_ctx.bind_action("reset", move || {
        reset_count.set(0);
        println!("🔄 State reset to initial!");
    });

    // 3. Load UI from the `.quick` declarative file
    let quick_content = include_str!("../app.quick");
    let app = App::new(
        WindowOptions::new()
            .title("Hello World - Quick Framework (.quick format)")
            .size(640.0, 480.0),
    )
    .from_quick(quick_content, &mut data_ctx)
    .map_err(|e| format!("Failed to parse .quick file: {}", e))?;

    println!("🚀 Opening desktop window...");
    // 4. Launch interactive desktop window & event loop
    app.run()?;

    Ok(())
}
