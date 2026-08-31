use quick::prelude::*;

fn run_headless(silver_src: &str, quick_src: &str) -> Result<(), Box<dyn std::error::Error>> {
    println!("⚡ Quick + Silver (.silver) Headless Showcase");
    println!("══════════════════════════════════════════════════════");

    // 1. Initialize Silver script runtime
    let silver_script = SilverScript::new(silver_src)
        .map_err(|e| format!("Silver compile error: {e}"))?;
    println!("✓ Silver (.silver) script compiled to bytecode & initialized");

    // 2. Bind Silver signals and actions to Quick DataContext
    let mut data_ctx = DataContext::new();
    silver_script.bind_to_data_context(&mut data_ctx);
    println!("✓ Bound Silver signals and action handlers to DataContext");

    // 3. Load App from quick markup
    let mut app = App::new(
        WindowOptions::new()
            .title("Quick + Silver Reactive App [headless]")
            .size(800.0, 600.0),
    )
    .from_quick(quick_src, &mut data_ctx)
    .map_err(|e| format!("Failed to parse app.quick: {e}"))?;

    // 4. Render a frame in headless mode
    let canvas = app.render_frame(Size::new(800.0, 600.0));
    let cmd_count = canvas.commands().len();

    println!("\n🌲 Widget Tree (headless render)");
    println!("  app.silver executed  ✓");
    println!("  app.quick parsed     ✓");
    println!("  signals bound        ✓  ($count, $status, $is_active, $brightness)");
    println!("  canvas commands      {cmd_count}  (paint calls across all widgets)");

    println!("\n✅ Headless Silver + Quick showcase complete — exit 0");
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("── Quick + Silver (.silver) Reactive Engine ──");

    let silver_src = include_str!("../app.silver");
    let quick_src = include_str!("../app.quick");

    if std::env::var("QUICK_HEADLESS").as_deref() == Ok("1")
        || (std::env::var("WAYLAND_DISPLAY").is_err() && std::env::var("DISPLAY").is_err())
    {
        return run_headless(silver_src, quick_src);
    }

    // 1. Initialize Silver script runtime
    let silver_script = SilverScript::new(silver_src)
        .map_err(|e| format!("Silver compile error: {e}"))?;

    // 2. Bind Silver signals and actions to Quick DataContext
    let mut data_ctx = DataContext::new();
    silver_script.bind_to_data_context(&mut data_ctx);

    // 3. Parse .quick declarative markup and construct UI tree
    let app = App::new(
        WindowOptions::new()
            .title("Quick + Silver Reactive App")
            .size(800.0, 600.0),
    )
    .from_quick(quick_src, &mut data_ctx)
    .map_err(|e| format!("Failed to parse app.quick: {e}"))?;

    println!("🚀 Launching interactive desktop window...");
    app.run()?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_silver_app_execution_and_rendering() {
        let silver_src = include_str!("../app.silver");
        let quick_src = include_str!("../app.quick");
        run_headless(silver_src, quick_src).expect("Silver app headless execution failed");
    }
}
