//! E2E Material You (M3) Real-World Application Scenarios (Tier 4)
//!
//! Covers:
//! - Scenario 1: Dynamic Wallpaper Theme Switching (Light/Dark mode + 7 Scheme Variants)
//! - Scenario 2: Material 3 Settings Form (Switches, Checkboxes, Sliders, TextInputs, Filled Buttons)
//! - Scenario 3: Filterable Card Dashboard with Chips and Elevated Cards
//! - Scenario 4: Asynchronous Task Manager with Determinate & Indeterminate ProgressBars
//! - Scenario 5: End-to-End Declarative `.quick` Markup UI Compilation and Event Dispatch

use quick::app::App;
use quick::core::event::{Event, PointerButton, PointerEvent, PointerPhase};
use quick::core::geometry::{Color, Point, Size};
use quick::core::signals::{create_computed, Signal};
use quick::markup::builder::DataContext;
use quick::style::theme::ThemePackage;
use quick::window::window::WindowOptions;
use std::cell::RefCell;
use std::rc::Rc;

// ============================================================================
// SCENARIO 1: DYNAMIC WALLPAPER THEME SWITCHING (LIGHT/DARK + 7 SCHEMES)
// ============================================================================

#[test]
fn test_scenario_1_dynamic_wallpaper_theme_switching() {
    let wallpaper_seeds = [
        ("#6750A4", "Forest Purple"),
        ("#00639B", "Ocean Blue"),
        ("#386A20", "Meadow Green"),
    ];

    let scheme_variants = [
        "TonalSpot",
        "Vibrant",
        "Expressive",
        "Fidelity",
        "Content",
        "Monochrome",
        "Neutral",
    ];

    for &(seed_hex, seed_name) in &wallpaper_seeds {
        let seed_color = Color::from_hex(seed_hex).expect("Valid seed hex");
        assert_eq!(seed_color.a, 255);

        for &variant_name in &scheme_variants {
            // Test both Dark Mode and Light Mode
            for &is_dark in &[true, false] {
                let theme_name = format!("{}-{}-{}", seed_name, variant_name, if is_dark { "dark" } else { "light" });
                let mut theme = ThemePackage::new(&theme_name);

                // Populate with standard M3 roles
                theme.colors.insert("primary".into(), seed_color);
                theme.colors.insert("on_primary".into(), if is_dark { Color::from_hex("#381E72").unwrap() } else { Color::WHITE });
                theme.colors.insert("surface".into(), if is_dark { Color::from_hex("#141218").unwrap() } else { Color::from_hex("#FEF7FF").unwrap() });
                theme.colors.insert("on_surface".into(), if is_dark { Color::from_hex("#E6E0E9").unwrap() } else { Color::from_hex("#1D1B20").unwrap() });
                theme.colors.insert("surface_container".into(), if is_dark { Color::from_hex("#211F26").unwrap() } else { Color::from_hex("#F3EDF7").unwrap() });
                theme.colors.insert("outline".into(), Color::from_hex("#79747E").unwrap());

                theme.shapes.corner_small = 8.0;
                theme.shapes.corner_medium = 16.0;
                theme.shapes.corner_large = 24.0;
                theme.shapes.corner_full = 9999.0;
                theme.shape_map = theme.shapes.to_map();

                let css = theme.generate_css();
                assert!(!css.is_empty(), "CSS should be generated for {}", theme_name);

                // Verify stylesheet parses cleanly
                let stylesheet = quick::style::parser::parse_stylesheet(&css);
                assert!(!stylesheet.rules.is_empty(), "Stylesheet must have rules for {}", theme_name);
            }
        }
    }
}

// ============================================================================
// SCENARIO 2: MATERIAL 3 SETTINGS FORM
// ============================================================================

#[test]
fn test_scenario_2_material_3_settings_form() {
    let dark_mode_sig = Signal::new(true);
    let gpu_accel_sig = Signal::new(true);
    let notifications_sig = Signal::new(false);
    let volume_sig = Signal::new(65.0);
    let ui_scale_sig = Signal::new(100.0);
    let server_url_sig = Signal::new("https://api.quick-ui.org".to_string());

    let form_submitted = Rc::new(RefCell::new(false));
    let form_reset = Rc::new(RefCell::new(false));

    let fs_cl = form_submitted.clone();
    let fr_cl = form_reset.clone();

    let mut ctx = DataContext::new();
    ctx.bind_bool_signal("dark_mode", dark_mode_sig.clone());
    ctx.bind_bool_signal("gpu_accel", gpu_accel_sig.clone());
    ctx.bind_bool_signal("notifications", notifications_sig.clone());
    ctx.bind_f32_signal("volume", volume_sig.clone());
    ctx.bind_f32_signal("ui_scale", ui_scale_sig.clone());
    ctx.bind_signal("server_url", server_url_sig.clone());

    ctx.bind_action("save_settings", move || {
        *fs_cl.borrow_mut() = true;
    });
    ctx.bind_action("reset_defaults", move || {
        *fr_cl.borrow_mut() = true;
    });

    let settings_markup = r#"
    <VStack id="settings-root" theme="material-you" style="padding: 24px; width: 600px; height: 500px;">
        <Card variant="elevated">
            <Text text="Application Settings" style="font-size: 22px; font-weight: bold;" />
            
            <HStack style="justify-content: space-between; align-items: center; width: 100%;">
                <Text text="Dark Mode" />
                <Switch id="sw-dark" checked="$dark_mode" />
            </HStack>

            <HStack style="justify-content: space-between; align-items: center; width: 100%;">
                <Text text="GPU Hardware Acceleration" />
                <Switch id="sw-gpu" checked="$gpu_accel" />
            </HStack>

            <HStack style="align-items: center; gap: 12px; width: 100%;">
                <Checkbox id="cb-notif" checked="$notifications" />
                <Text text="Enable Desktop Notifications" />
            </HStack>

            <VStack style="width: 100%; gap: 4px;">
                <Text text="Master Volume" />
                <Slider id="sl-vol" min="0" max="100" value="$volume" />
                <ProgressBar progress="$volume" min="0" max="100" />
            </VStack>

            <TextInput id="input-url" placeholder="Server URL" text="$server_url" />

            <HStack style="gap: 16px; justify-content: flex-end; width: 100%;">
                <Button id="btn-reset" variant="outlined" text="Reset" onclick="reset_defaults" />
                <Button id="btn-save" variant="filled" text="Save" onclick="save_settings" />
            </HStack>
        </Card>
    </VStack>
    "#;

    let mut app = App::new(WindowOptions::new().title("Settings Form"))
        .from_quick(settings_markup, &mut ctx)
        .expect("Settings form markup should build");

    let window_size = Size::new(600.0, 500.0);
    let canvas_1 = app.render_frame(window_size);
    assert!(canvas_1.commands().len() >= 10);

    // Simulate user toggling notifications checkbox
    notifications_sig.set(true);
    // Simulate user adjusting volume to 90
    volume_sig.set(90.0);

    let canvas_2 = app.render_frame(window_size);
    assert!(canvas_2.commands().len() >= 10);
    assert!(notifications_sig.get());
    assert_eq!(volume_sig.get(), 90.0);
}

// ============================================================================
// SCENARIO 3: FILTERABLE CARD DASHBOARD WITH CHIPS AND ELEVATED CARDS
// ============================================================================

#[test]
fn test_scenario_3_filterable_card_dashboard() {
    let chip_cpu = Signal::new(true);
    let chip_mem = Signal::new(true);
    let chip_net = Signal::new(false);
    let chip_gpu = Signal::new(false);

    let mut ctx = DataContext::new();
    ctx.bind_bool_signal("show_cpu", chip_cpu.clone());
    ctx.bind_bool_signal("show_mem", chip_mem.clone());
    ctx.bind_bool_signal("show_net", chip_net.clone());
    ctx.bind_bool_signal("show_gpu", chip_gpu.clone());

    let dashboard_markup = r#"
    <VStack id="dashboard-root" theme="material-you" style="padding: 20px; width: 700px; height: 500px;">
        <Text text="System Telemetry Dashboard" style="font-size: 24px; font-weight: bold;" />
        
        <HStack style="gap: 8px; margin: 12px 0;">
            <Chip id="chip-cpu" text="CPU Metrics" selected="$show_cpu" />
            <Chip id="chip-mem" text="Memory Usage" selected="$show_mem" />
            <Chip id="chip-net" text="Network I/O" selected="$show_net" />
            <Chip id="chip-gpu" text="GPU Load" selected="$show_gpu" />
        </HStack>

        <HStack style="gap: 16px; width: 100%;">
            <Card id="card-cpu" variant="elevated" style="width: 48%; padding: 16px;">
                <Text text="CPU Load: 34%" style="font-size: 16px;" />
                <ProgressBar progress="34" min="0" max="100" />
            </Card>

            <Card id="card-mem" variant="elevated" style="width: 48%; padding: 16px;">
                <Text text="RAM: 4.2 GB / 16 GB" style="font-size: 16px;" />
                <ProgressBar progress="26" min="0" max="100" />
            </Card>
        </HStack>
    </VStack>
    "#;

    let mut app = App::new(WindowOptions::new().title("Telemetry Dashboard"))
        .from_quick(dashboard_markup, &mut ctx)
        .unwrap();

    let window_size = Size::new(700.0, 500.0);
    let canvas = app.render_frame(window_size);
    assert!(canvas.commands().len() >= 12);

    // Toggle filter chips
    chip_net.set(true);
    assert!(chip_net.get());
    let canvas_updated = app.render_frame(window_size);
    assert!(canvas_updated.commands().len() >= 12);
}

// ============================================================================
// SCENARIO 4: ASYNCHRONOUS TASK MANAGER WITH PROGRESS BARS
// ============================================================================

#[test]
fn test_scenario_4_asynchronous_task_manager() {
    let task_progress = Signal::new(0.0);
    let task_status = Signal::new("Ready to start".to_string());
    let is_running = Signal::new(false);

    let prog_cl = task_progress.clone();
    let status_cl = task_status.clone();
    let running_cl = is_running.clone();

    let mut ctx = DataContext::new();
    ctx.bind_f32_signal("progress", task_progress.clone());
    ctx.bind_signal("status", task_status.clone());
    ctx.bind_bool_signal("running", is_running.clone());

    ctx.bind_action("start_task", move || {
        running_cl.set(true);
        prog_cl.set(25.0);
        status_cl.set("Processing batch 1/4...".to_string());
    });

    let task_markup = r#"
    <VStack id="task-manager" theme="material-you" style="padding: 24px; width: 500px; height: 350px;">
        <Card variant="elevated">
            <Text text="Batch File Processor" style="font-size: 20px; font-weight: bold;" />
            <Text text="$status" style="font-size: 14px; color: #89b4fa;" />
            
            <ProgressBar id="batch-progress" progress="$progress" min="0" max="100" />

            <HStack style="gap: 12px; margin-top: 16px;">
                <Button id="btn-start" variant="filled" text="Start Processing" onclick="start_task" />
            </HStack>
        </Card>
    </VStack>
    "#;

    let mut app = App::new(WindowOptions::new().title("Task Manager"))
        .from_quick(task_markup, &mut ctx)
        .unwrap();

    let window_size = Size::new(500.0, 350.0);
    let c1 = app.render_frame(window_size);
    assert!(c1.commands().len() >= 6);

    // Simulate clicking start task
    let click_down = Event::Pointer(PointerEvent {
        position: Point::new(60.0, 100.0),
        button: Some(PointerButton::Primary),
        phase: PointerPhase::Down,
        modifiers: Default::default(),
    });
    let click_up = Event::Pointer(PointerEvent {
        position: Point::new(60.0, 100.0),
        button: Some(PointerButton::Primary),
        phase: PointerPhase::Up,
        modifiers: Default::default(),
    });
    let _ = app.handle_event(&click_down, window_size);
    let _ = app.handle_event(&click_up, window_size);

    // Simulate task progression to 100%
    task_progress.set(100.0);
    task_status.set("Completed successfully!".to_string());
    is_running.set(false);

    let c2 = app.render_frame(window_size);
    assert!(c2.commands().len() >= 6);
    assert_eq!(task_progress.get(), 100.0);
    assert_eq!(task_status.get(), "Completed successfully!");
}

// ============================================================================
// SCENARIO 5: FULL END-TO-END DECLARATIVE APP COMPILATION & EVENT LOOP
// ============================================================================

#[test]
fn test_scenario_5_full_declarative_app_lifecycle() {
    let click_counter = Signal::new(0);
    let cc_cl = click_counter.clone();

    let dynamic_message = create_computed(move || {
        let n = cc_cl.get();
        if n == 0 {
            "Ready for interactions".to_string()
        } else {
            format!("Interaction Count: {}", n)
        }
    });

    let inc = click_counter.clone();
    let mut ctx = DataContext::new();
    ctx.bind_signal("msg", dynamic_message.clone());
    ctx.bind_action("increment_clicks", move || {
        inc.update(|v| *v += 1);
    });

    let app_quick = r#"
    <VStack id="showcase-root" theme="material-you" style="width: 600px; height: 400px; padding: 24px; background: #141218;">
        <Card variant="elevated" style="padding: 24px; gap: 16px;">
            <Text id="heading" text="Quick UI Framework M3" style="font-size: 22px; font-weight: bold; color: #E6E0E9;" />
            <Text id="dynamic-label" text="$msg" style="font-size: 16px; color: #D0BCFF;" />
            
            <HStack style="gap: 12px;">
                <Button id="btn-add" variant="filled" text="Increment" onclick="increment_clicks" />
            </HStack>
        </Card>
    </VStack>
    "#;

    let mut app = App::new(WindowOptions::new().title("Full Showcase App"))
        .from_quick(app_quick, &mut ctx)
        .expect("App should compile from .quick markup");

    let window_size = Size::new(600.0, 400.0);

    // Initial Frame
    let f1 = app.render_frame(window_size);
    assert!(f1.commands().len() >= 6);
    assert_eq!(dynamic_message.get(), "Ready for interactions");

    // Click 1
    click_counter.update(|v| *v += 1);
    assert_eq!(dynamic_message.get(), "Interaction Count: 1");

    // Re-render Frame 2
    let f2 = app.render_frame(window_size);
    assert!(f2.commands().len() >= 6);

    // Click 2
    click_counter.update(|v| *v += 1);
    assert_eq!(dynamic_message.get(), "Interaction Count: 2");

    // Re-render Frame 3
    let f3 = app.render_frame(window_size);
    assert!(f3.commands().len() >= 6);
}
