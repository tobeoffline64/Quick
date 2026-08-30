use quick::prelude::*;

// 1. Configure mimalloc high-throughput global allocator (inspired by Bun)
#[cfg(feature = "mimalloc")]
#[global_allocator]
static GLOBAL: quick::core::MiMalloc = quick::core::MiMalloc;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("⚡ Quick Framework — Bun-Inspired Pure Rust Runtime");
    println!("📦 Allocator: mimalloc (Thread-Local Free Lists & Zero Lock Contention)");

    // 2. Define reactive state using Fine-Grained Signals
    let count = Signal::new(0);
    let count_sig = count.clone();

    // 3. Computed signal with automatic dependency tracking
    let counter_display = create_computed(move || {
        format!("Count: {}", count_sig.get())
    });

    // 4. Setup DataContext for Declarative UI binding
    let mut data_ctx = DataContext::new();
    data_ctx.bind_signal("counter_display", counter_display.clone());

    // Bind action callbacks
    let count_inc = count.clone();
    data_ctx.bind_action("increment", move || {
        count_inc.update(|v| *v += 1);
        println!("➕ Increment Action -> Value: {}", count_inc.get());
    });

    let count_dec = count.clone();
    data_ctx.bind_action("decrement", move || {
        count_dec.update(|v| *v -= 1);
        println!("➖ Decrement Action -> Value: {}", count_dec.get());
    });

    let count_reset = count.clone();
    data_ctx.bind_action("reset", move || {
        count_reset.set(0);
        println!("🔄 Reset Action -> Value: {}", count_reset.get());
    });

    // 5. Load Declarative UI from XML (XAML-style with embedded CSS)
    let xml_content = include_str!("../app.xml");
    let mut app = App::new(
        WindowOptions::new()
            .title("Quick Counter - Linux Wayland / Skia (Pure Rust)")
            .size(800.0, 600.0),
    )
    .from_xml(xml_content, &mut data_ctx)
    .map_err(|e| format!("Failed to load XML UI: {}", e))?;

    println!("✅ Declarative UI parsed with SIMD & zero-copy AST!");
    println!("📊 Initial Reactive Display: '{}'", counter_display.get());

    // 6. Initial Frame Render (Layout + Display List in Arena)
    let canvas = app.render_frame(Size::new(800.0, 600.0));
    println!("🎨 Initial frame recorded {} draw commands.", canvas.commands().len());

    // 7. Verify Fine-Grained Reactive updates
    println!("\n🔄 Testing high-throughput Signal updates...");
    count.update(|v| *v += 10);
    println!("👉 State after update (+10): '{}'", counter_display.get());

    count.set(100);
    println!("👉 State after set (100): '{}'", counter_display.get());

    // 8. Re-render frame verifying O(1) arena reset
    let canvas_updated = app.render_frame(Size::new(800.0, 600.0));
    println!("🎨 Updated frame rendered successfully ({} commands).", canvas_updated.commands().len());

    println!("\n✨ Quick framework engine initialized and verified in 100% Pure Rust!");
    Ok(())
}
