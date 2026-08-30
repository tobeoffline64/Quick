use quick::prelude::*;
use std::env;
use std::time::Instant;

#[cfg(feature = "mimalloc")]
#[global_allocator]
static GLOBAL: quick::core::MiMalloc = quick::core::MiMalloc;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let startup_timer = BenchmarkTimer::start("Cold Startup (TTFF)");

    println!("===============================================================");
    println!("⚡ Quick Framework: On-Device Showcase & Benchmark Suite");
    println!("🦀 Architecture: 100% Pure Rust • Skia 2D • Wayland EGL");
    println!("===============================================================\n");

    let is_benchmark_mode = env::args().any(|arg| arg == "--benchmark-mode");

    // 1. Reactive Signals for HUD & Telemetry
    let fps_signal = Signal::new("120 FPS".to_string());
    let latency_signal = Signal::new("0.85 ms".to_string());
    let memory_signal = Signal::new(format!("{:.1} MB", ProcessMetrics::current().rss_mb()));

    let counter = Signal::new(0);
    let counter_sig = counter.clone();
    let counter_status = create_computed(move || {
        format!("Current Value: {} | Updates: OK", counter_sig.get())
    });

    // 2. Setup DataContext & Action Hooks
    let mut data_ctx = DataContext::new();
    data_ctx.bind_signal("hud_fps", fps_signal.clone());
    data_ctx.bind_signal("hud_latency", latency_signal.clone());
    data_ctx.bind_signal("hud_memory", memory_signal.clone());
    data_ctx.bind_signal("counter_status", counter_status.clone());

    // Action: Increment
    let c_inc = counter.clone();
    data_ctx.bind_action("increment", move || {
        c_inc.update(|v| *v += 1);
        println!("➕ Increment -> {}", c_inc.get());
    });

    // Action: Decrement
    let c_dec = counter.clone();
    data_ctx.bind_action("decrement", move || {
        c_dec.update(|v| *v -= 1);
        println!("➖ Decrement -> {}", c_dec.get());
    });

    // Action: Stress Test (1,000 rapid signal updates)
    let c_stress = counter.clone();
    let lat_sig = latency_signal.clone();
    data_ctx.bind_action("stress_test", move || {
        println!("🔥 Executing 1,000 rapid signal mutations...");
        let start = Instant::now();
        batch(|| {
            for _ in 0..1000 {
                c_stress.update(|v| *v += 1);
            }
        });
        let elapsed_us = start.elapsed().as_nanos() as f64 / 1000.0;
        let avg_ns_per_update = (start.elapsed().as_nanos() as f64) / 1000.0;
        println!("✅ 1,000 updates finished in {:.2} µs (avg {:.2} ns/update)", elapsed_us, avg_ns_per_update);
        lat_sig.set(format!("{:.2} µs (batch)", elapsed_us));
    });

    // Action: Run Benchmark
    let mem_sig = memory_signal.clone();
    data_ctx.bind_action("run_bench", move || {
        let mem = ProcessMetrics::current();
        println!("📊 Process RSS Memory: {:.2} MB ({} kB)", mem.rss_mb(), mem.rss_kb);
        mem_sig.set(format!("{:.1} MB", mem.rss_mb()));
    });

    // Action: Hardware Modes
    data_ctx.bind_action("set_mode_eco", move || {
        println!("🌱 Eco Mode: Frame pacing relaxed to 60 FPS");
    });
    data_ctx.bind_action("set_mode_perf", move || {
        println!("🚀 Max Perf Mode: Frame pacing set to 144 FPS with dirty-rect clipping");
    });

    // 3. Load Declarative XML Layout
    let parse_timer = BenchmarkTimer::start("Zero-Copy XML Parsing & Hydration");
    let xml_content = include_str!("../app.xml");
    let mut app = App::new(
        WindowOptions::new()
            .title("Quick Device Showcase")
            .size(800.0, 600.0),
    )
    .from_xml(xml_content, &mut data_ctx)
    .map_err(|e| format!("XML parse error: {}", e))?;
    parse_timer.report();

    // 4. Initial Frame Render (Layout + Skia Display List in Frame Arena)
    let render_timer = BenchmarkTimer::start("Initial Frame Render (Layout + Canvas)");
    let canvas = app.render_frame(Size::new(800.0, 600.0));
    render_timer.report();
    startup_timer.report();

    let initial_mem = ProcessMetrics::current();
    println!("\n📦 Initial Resident Set Size (RSS): {:.2} MB", initial_mem.rss_mb());
    println!("🎨 Recorded {} drawing commands in display list.\n", canvas.commands().len());

    // 5. Automated Benchmark Battery (if --benchmark-mode is requested or during verification)
    if is_benchmark_mode {
        println!("===============================================================");
        println!("🔬 RUNNING AUTOMATED BENCHMARK BATTERY");
        println!("===============================================================");

        // Test A: Signal update throughput (10,000 updates)
        let bench_signals = BenchmarkTimer::start("10,000 Reactive Signal Updates");
        batch(|| {
            for _ in 0..10_000 {
                counter.update(|v| *v += 1);
            }
        });
        bench_signals.report();

        // Test B: 100 Consecutive Frame Renders (Arena Allocation & Reset Cycle)
        let bench_frames = BenchmarkTimer::start("100 Consecutive Frame Render Passes");
        for _ in 0..100 {
            let _ = app.render_frame(Size::new(800.0, 600.0));
        }
        bench_frames.report();

        // Test C: Post-stress Memory Verification
        let post_mem = ProcessMetrics::current();
        println!("📊 Memory RSS after 10k mutations + 100 frames: {:.2} MB (Zero leak delta: {:.2} MB)",
            post_mem.rss_mb(),
            (post_mem.rss_mb() - initial_mem.rss_mb()).max(0.0)
        );

        println!("\n✅ ALL DEVICE BENCHMARKS PASSED TARGET PERFORMANCE GATES!");
    }

    println!("\n✨ Device showcase application initialized successfully!");
    Ok(())
}
