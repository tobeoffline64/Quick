//! Quick Component Gallery — Two-column showcase of all Quick UI widgets.
//!
//! Runs in two modes:
//!   - QUICK_HEADLESS=1  → prints gallery summary table + color tokens, exits 0 (CI-safe)
//!   - normal            → opens a 1200×900 gallery window (requires display)

use quick::prelude::*;
use quick_style::base::{init_base_theme, ColorScheme};
use quick_style::theme::{SchemeVariant, ThemePackage};

#[cfg(feature = "mimalloc")]
#[global_allocator]
static GLOBAL: quick::core::MiMalloc = quick::core::MiMalloc;

fn run_headless() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize OS-adaptive base theme
    let bt = init_base_theme();
    let scheme_name = match bt.scheme { ColorScheme::Light => "Light", ColorScheme::Dark => "Dark" };

    println!("⚡ Quick Component Gallery — Headless Mode");
    println!("═══════════════════════════════════════════════════════════════");
    println!();
    println!("🎨 Base Theme (Avalonia Fluent — OS Adaptive)");
    println!("   Color scheme   : {scheme_name}");
    println!("   Accent         : {}", bt.colors.accent.normal.to_hex());
    println!("   Background     : {}", bt.colors.bg.to_hex());
    println!("   Surface        : {}", bt.colors.surface.to_hex());
    println!("   Border         : {}", bt.colors.border.to_hex());
    println!("   Text primary   : {}", bt.colors.text_primary.to_hex());
    println!("   Text secondary : {}", bt.colors.text_secondary.to_hex());
    println!("   Error          : {}", bt.colors.error.to_hex());
    println!();
    println!("   Radius scale   : NONE={} XS={} SM={} MD={} LG={} PILL={}",
        bt.radius.none, bt.radius.xs, bt.radius.sm,
        bt.radius.md,   bt.radius.lg, bt.radius.pill);
    println!("   Spacing scale  : XS={} SM={} MD={} LG={} XL={} XXL={}",
        bt.spacing.xs, bt.spacing.sm, bt.spacing.md,
        bt.spacing.lg, bt.spacing.xl, bt.spacing.xxl);
    println!("   Type scale     : caption={} body={} title={} display={}",
        bt.type_scale.caption, bt.type_scale.body,
        bt.type_scale.title, bt.type_scale.display);
    println!("   Font stack     : primary='{}', fallback=[{}]",
        bt.font_stack.primary,
        bt.font_stack.families.join(", "));
    println!();

    // Generate CSS custom properties
    let css = bt.generate_css();
    let var_count = css.matches("--q-").count();
    println!("   CSS vars generated  : {var_count} (--q-* custom properties)");
    println!();

    // Material You theme
    let seed = quick_core::geometry::Color::from_hex("#6750A4")?;
    for &dark in &[false, true] {
        let pkg = ThemePackage::from_seed_color(seed, SchemeVariant::TonalSpot, dark);
        let mode = if dark { "Dark" } else { "Light" };
        println!("🎨 Material You Color Roles — {mode} Mode");
        println!("   primary   : {}  surface   : {}",
            pkg.color_scheme.primary.to_hex(), pkg.color_scheme.surface.to_hex());
        println!("   secondary : {}  error     : {}",
            pkg.color_scheme.secondary.to_hex(), pkg.color_scheme.error.to_hex());
        println!("   outline   : {}  on_surface: {}",
            pkg.color_scheme.outline.to_hex(), pkg.color_scheme.on_surface.to_hex());
        println!();
    }

    // Noctalia Glass Theme
    let noctalia_dark = quick_style::noctalia::NoctaliaPalette::noctalia_dark();
    println!("🌙 Noctalia Brand Palette — Dark Mode");
    println!("   primary   : {}  surface         : {}", noctalia_dark.primary.to_hex(), noctalia_dark.surface.to_hex());
    println!("   secondary : {}  surface_variant : {}", noctalia_dark.secondary.to_hex(), noctalia_dark.surface_variant.to_hex());
    println!("   tertiary  : {}  outline         : {}", noctalia_dark.tertiary.to_hex(), noctalia_dark.outline.to_hex());
    println!("   error     : {}  hover           : {}", noctalia_dark.error.to_hex(), noctalia_dark.hover.to_hex());
    println!();

    // Parse and render the gallery layout
    let gallery_content = include_str!("../gallery.quick");
    let mut ctx = DataContext::new();

    // Base column signals
    ctx.bind_bool_signal("sw_base",  Signal::new(true));
    ctx.bind_bool_signal("cb_base",  Signal::new(false));
    ctx.bind_bool_signal("chip1",    Signal::new(true));
    ctx.bind_bool_signal("chip2",    Signal::new(false));
    ctx.bind_f32_signal("slider_base",   Signal::new(65.0f32));
    ctx.bind_f32_signal("progress_base", Signal::new(0.65f32));
    ctx.bind_signal("input_base", Signal::new(String::new()));

    // M3 column signals
    ctx.bind_bool_signal("sw_m3",  Signal::new(true));
    ctx.bind_bool_signal("cb_m3",  Signal::new(false));
    ctx.bind_bool_signal("chip3",  Signal::new(true));
    ctx.bind_bool_signal("chip4",  Signal::new(false));
    ctx.bind_f32_signal("slider_m3",   Signal::new(65.0f32));
    ctx.bind_f32_signal("progress_m3", Signal::new(0.65f32));
    ctx.bind_signal("input_m3", Signal::new(String::new()));

    // Actions
    for &action in &["on_switch_base","on_check_base","on_chip1","on_chip2",
                    "on_switch_m3","on_check_m3","on_chip3","on_chip4"] {
        ctx.bind_action(action, || {});
    }

    let mut app = App::new(
        WindowOptions::new()
            .title("Quick Gallery [headless]")
            .size(1200.0, 900.0),
    )
    .from_quick(gallery_content, &mut ctx)
    .map_err(|e| format!("Failed to parse gallery.quick: {e}"))?;

    let canvas = app.render_frame(Size::new(1200.0, 900.0));
    let cmd_count = canvas.commands().len();

    println!("🌲 Gallery Widget Tree (headless render)");
    println!("   gallery.quick parsed  ✓");
    println!("   signals bound         ✓  (14 signals across both columns)");
    println!("   canvas commands       {cmd_count}  (paint calls across all gallery sections)");
    println!();
    println!("   Sections rendered:");
    println!("   ├── Base & M3 Buttons       (5 variants × 2 columns)");
    println!("   ├── Cards                   (3 variants × 2 columns)");
    println!("   ├── Selection Controls      (Switch + Checkbox + Chip×2 × 2 columns)");
    println!("   ├── Inputs & Sliders        (TextInput + Slider × 2 columns)");
    println!("   ├── Progress Bars           (ProgressBar × 2 columns)");
    println!("   ├── Typography Scale        (Caption→Display × 2 columns)");
    println!("   └── 🌙 Noctalia UI Suite    (FramelessTitleBar, NoctaliaBar, NoctaliaButton×5,");
    println!("                                NoctaliaCard, CountdownRing, AnalogClock,");
    println!("                                Segmented, NoctaliaSlider, NoctaliaGraph,");
    println!("                                NoctaliaCalendar, NoctaliaColorPicker)");
    println!();
    println!("✅ Gallery headless showcase complete — exit 0");
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    // Initialize OS-adaptive base theme at startup
    init_base_theme();

    if std::env::var("QUICK_HEADLESS").as_deref() == Ok("1") {
        return run_headless();
    }

    println!("⚡ Quick Component Gallery — launching window (1200×900)…");
    println!("   Set QUICK_HEADLESS=1 to run without a display.");

    let mut ctx = DataContext::new();

    // Base column signals
    ctx.bind_bool_signal("sw_base",  Signal::new(true));
    ctx.bind_bool_signal("cb_base",  Signal::new(false));
    ctx.bind_bool_signal("chip1",    Signal::new(true));
    ctx.bind_bool_signal("chip2",    Signal::new(false));
    ctx.bind_f32_signal("slider_base",   Signal::new(65.0f32));
    ctx.bind_f32_signal("progress_base", Signal::new(0.65f32));
    ctx.bind_signal("input_base", Signal::new(String::new()));

    // M3 column signals
    ctx.bind_bool_signal("sw_m3",  Signal::new(true));
    ctx.bind_bool_signal("cb_m3",  Signal::new(false));
    ctx.bind_bool_signal("chip3",  Signal::new(true));
    ctx.bind_bool_signal("chip4",  Signal::new(false));
    ctx.bind_f32_signal("slider_m3",   Signal::new(65.0f32));
    ctx.bind_f32_signal("progress_m3", Signal::new(0.65f32));
    ctx.bind_signal("input_m3", Signal::new(String::new()));

    for &action in &["on_switch_base","on_check_base","on_chip1","on_chip2",
                    "on_switch_m3","on_check_m3","on_chip3","on_chip4"] {
        ctx.bind_action(action, || {});
    }

    let gallery_content = include_str!("../gallery.quick");

    let app = App::new(
        WindowOptions::new()
            .title("⚡ Quick Component Gallery")
            .size(1200.0, 900.0),
    )
    .from_quick(gallery_content, &mut ctx)
    .map_err(|e| format!("Failed to parse gallery.quick: {e}"))?;

    app.run()?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gallery_base_theme_initializes() {
        let bt = init_base_theme();
        assert!(matches!(bt.scheme, ColorScheme::Light | ColorScheme::Dark));
        assert!(bt.colors.accent.normal.r as u32 + bt.colors.accent.normal.g as u32 + bt.colors.accent.normal.b as u32 > 0);
    }

    #[test]
    fn test_gallery_generates_css_vars() {
        let bt = init_base_theme();
        let css = bt.generate_css();
        assert!(css.contains("--q-bg:"));
        assert!(css.contains("--q-accent:"));
        assert!(css.contains("--q-radius-md:"));
        assert!(css.contains("--q-font-body:"));
    }

    #[test]
    fn test_gallery_headless_runs_to_completion() {
        std::env::set_var("QUICK_HEADLESS", "1");
        let result = run_headless();
        std::env::remove_var("QUICK_HEADLESS");
        assert!(result.is_ok(), "headless run failed: {:?}", result.err());
    }
}
