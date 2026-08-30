//! E2E Material You (M3) Declarative Markup Integration Test Suite
//!
//! Covers:
//! - Feature 17: Declarative `.quick` Markup Integration (Spec §7)
//!   - XML and TOML Format Parsing
//!   - Dynamic `theme="material-you"` Injection
//!   - Component Variant Attributes (`variant="filled|tonal|elevated|outlined"`)
//!   - Two-Way Reactive Signal Bindings (`$sig`, `checked`, `value`, `progress`, `text`)
//!   - Event Handler Bindings (`onclick`, `onchange`)
//!   - Cascading CSS Resolution with Specificity
//! - Tier 3: Pairwise Combinations in Declarative Runtime

use quick::app::App;
use quick::core::event::{Event, PointerButton, PointerEvent, PointerPhase};
use quick::core::geometry::{Point, Size};
use quick::core::signals::{create_computed, Signal};
use quick::markup::builder::{build_ui_tree, DataContext};
use quick::markup::quick_parser::parse_quick;
use quick::render::canvas::Canvas;
use quick::window::window::WindowOptions;
use std::cell::RefCell;
use std::rc::Rc;

// ============================================================================
// FEATURE 17: DECLARATIVE MARKUP PARSER & BUILDER (Spec §7)
// ============================================================================

#[test]
fn test_f17_parse_quick_xml_syntax_with_theme_attribute() {
    let xml_src = r#"
    <VStack id="app-root" theme="material-you" style="padding: 24px;">
        <Card variant="elevated">
            <Text id="heading" text="Material You Title" />
            <Button id="submit-btn" text="Submit" />
        </Card>
    </VStack>
    "#;

    let doc = parse_quick(xml_src).expect("Should parse XML syntax");
    assert_eq!(doc.root.element, "VStack");
    assert_eq!(doc.root.attributes.get("theme").map(|s| s.as_str()), Some("material-you"));
    assert_eq!(doc.root.children.len(), 1);
    assert_eq!(doc.root.children[0].element, "Card");
}

#[test]
fn test_f17_parse_quick_toml_syntax_with_root() {
    let toml_src = r#"
    styles = "Text.title { font-size: 20px; }"

    [root]
    type = "VStack"
    id = "root-container"

    [[root.children]]
    type = "Text"
    class = "title"
    text = "TOML Material Title"
    "#;

    let doc = parse_quick(toml_src).expect("Should parse TOML syntax");
    assert_eq!(doc.root.element, "VStack");
    assert_eq!(doc.root.id.as_deref(), Some("root-container"));
    assert_eq!(doc.root.children.len(), 1);
    assert_eq!(doc.root.children[0].element, "Text");
}

#[test]
fn test_f17_builder_card_and_button_variant_attributes() {
    let xml_src = r#"
    <VStack>
        <Card id="card-el" variant="elevated">
            <Button id="btn-filled" variant="filled" text="Filled" />
            <Button id="btn-tonal" variant="tonal" text="Tonal" />
            <Button id="btn-outlined" variant="outlined" text="Outlined" />
        </Card>
    </VStack>
    "#;

    let doc = parse_quick(xml_src).unwrap();
    let mut ctx = DataContext::new();
    let (mut root, stylesheet) = build_ui_tree(&doc, &mut ctx);

    let mut engine = quick::layout::engine::LayoutEngine::new();
    let node = root.build_layout(&mut engine).unwrap();
    engine.compute_layout(node, Size::new(400.0, 300.0)).unwrap();
    root.update_layout(&engine, Point::ZERO);

    let bounds = engine.get_layout(node).unwrap();
    let mut canvas = Canvas::new();
    root.paint(&mut canvas, bounds);

    assert!(canvas.commands().len() >= 5);
    assert_eq!(stylesheet.rules.len(), 0);
}

#[test]
fn test_f17_builder_two_way_signal_attribute_bindings() {
    let gpu_checked = Signal::new(true);
    let volume = Signal::new(75.0);
    let chip_selected = Signal::new(true);
    let title_text = Signal::new("Reactive Header".to_string());

    let mut ctx = DataContext::new();
    ctx.bind_bool_signal("gpu_on", gpu_checked.clone());
    ctx.bind_f32_signal("vol_level", volume.clone());
    ctx.bind_bool_signal("chip_state", chip_selected.clone());
    ctx.bind_signal("header_title", title_text.clone());

    let xml_src = r#"
    <VStack>
        <Text text="$header_title" />
        <Switch checked="$gpu_on" />
        <Slider value="$vol_level" min="0" max="100" />
        <ProgressBar progress="$vol_level" min="0" max="100" />
        <Chip text="Tag" selected="$chip_state" />
    </VStack>
    "#;

    let doc = parse_quick(xml_src).unwrap();
    let (mut root, _) = build_ui_tree(&doc, &mut ctx);

    let mut engine = quick::layout::engine::LayoutEngine::new();
    let node = root.build_layout(&mut engine).unwrap();
    engine.compute_layout(node, Size::new(400.0, 400.0)).unwrap();
    root.update_layout(&engine, Point::ZERO);

    let bounds = engine.get_layout(node).unwrap();
    let mut canvas = Canvas::new();
    root.paint(&mut canvas, bounds);

    assert!(canvas.commands().len() >= 8);
}

#[test]
fn test_f17_builder_action_handler_attribute_bindings() {
    let btn_action_fired = Rc::new(RefCell::new(false));
    let baf_cl = btn_action_fired.clone();

    let switch_action_fired = Rc::new(RefCell::new(false));
    let saf_cl = switch_action_fired.clone();

    let mut ctx = DataContext::new();
    ctx.bind_action("on_btn_click", move || {
        *baf_cl.borrow_mut() = true;
    });
    ctx.bind_action("on_switch_toggle", move || {
        *saf_cl.borrow_mut() = true;
    });

    let xml_src = r#"
    <VStack>
        <Button text="Click" onclick="on_btn_click" />
        <Switch onchange="on_switch_toggle" />
    </VStack>
    "#;

    let doc = parse_quick(xml_src).unwrap();
    let (mut root, _) = build_ui_tree(&doc, &mut ctx);

    let mut engine = quick::layout::engine::LayoutEngine::new();
    let node = root.build_layout(&mut engine).unwrap();
    engine.compute_layout(node, Size::new(400.0, 200.0)).unwrap();
    root.update_layout(&engine, Point::ZERO);

    let bounds = engine.get_layout(node).unwrap();

    // Click button
    let click_btn_down = Event::Pointer(PointerEvent {
        position: Point::new(50.0, 20.0),
        button: Some(PointerButton::Primary),
        phase: PointerPhase::Down,
        modifiers: Default::default(),
    });
    let click_btn_up = Event::Pointer(PointerEvent {
        position: Point::new(50.0, 20.0),
        button: Some(PointerButton::Primary),
        phase: PointerPhase::Up,
        modifiers: Default::default(),
    });
    let _ = root.handle_event(&click_btn_down, bounds);
    let _ = root.handle_event(&click_btn_up, bounds);

    assert!(*btn_action_fired.borrow());
}

#[test]
fn test_f17_builder_dynamic_css_resolution_with_theme() {
    let xml_src = r#"
    <VStack theme="material-you">
        <Style>
            Button.custom-btn { font-size: 18px; }
        </Style>
        <Button class="custom-btn" text="Customized" />
    </VStack>
    "#;

    let doc = parse_quick(xml_src).unwrap();
    let mut ctx = DataContext::new();
    let (mut root, stylesheet) = build_ui_tree(&doc, &mut ctx);

    assert!(!stylesheet.rules.is_empty());
    let mut engine = quick::layout::engine::LayoutEngine::new();
    let node = root.build_layout(&mut engine).unwrap();
    engine.compute_layout(node, Size::new(300.0, 100.0)).unwrap();
    root.update_layout(&engine, Point::ZERO);

    let bounds = engine.get_layout(node).unwrap();
    let mut canvas = Canvas::new();
    root.paint(&mut canvas, bounds);
    assert!(!canvas.commands().is_empty());
}

#[test]
fn test_f17_bva_malformed_xml_returns_clean_err() {
    assert!(parse_quick("<VStack><Text></VStack>").is_err());
    assert!(parse_quick("<Button unclosed_attr=").is_err());
}

#[test]
fn test_f17_bva_malformed_toml_returns_clean_err() {
    assert!(parse_quick("[root\ntype = VStack").is_err());
}

#[test]
fn test_f17_bva_missing_signal_binding_fallback() {
    let xml_src = r#"
    <VStack>
        <Text text="$non_existent_string_signal" />
        <Switch checked="$non_existent_bool_signal" />
        <Slider value="$non_existent_f32_signal" />
    </VStack>
    "#;

    let doc = parse_quick(xml_src).unwrap();
    let mut ctx = DataContext::new();
    let (mut root, _) = build_ui_tree(&doc, &mut ctx);

    let mut engine = quick::layout::engine::LayoutEngine::new();
    let node = root.build_layout(&mut engine).unwrap();
    engine.compute_layout(node, Size::new(300.0, 200.0)).unwrap();
    root.update_layout(&engine, Point::ZERO);

    let bounds = engine.get_layout(node).unwrap();
    let mut canvas = Canvas::new();
    root.paint(&mut canvas, bounds);
    assert!(!canvas.commands().is_empty());
}

#[test]
fn test_f17_bva_missing_action_handler_ignored() {
    let xml_src = r#"
    <VStack>
        <Button text="Click" onclick="unregistered_handler" />
    </VStack>
    "#;

    let doc = parse_quick(xml_src).unwrap();
    let mut ctx = DataContext::new();
    let (mut root, _) = build_ui_tree(&doc, &mut ctx);

    let mut engine = quick::layout::engine::LayoutEngine::new();
    let node = root.build_layout(&mut engine).unwrap();
    engine.compute_layout(node, Size::new(300.0, 100.0)).unwrap();
    root.update_layout(&engine, Point::ZERO);

    let bounds = engine.get_layout(node).unwrap();
    let click = Event::Pointer(PointerEvent {
        position: Point::new(50.0, 20.0),
        button: Some(PointerButton::Primary),
        phase: PointerPhase::Down,
        modifiers: Default::default(),
    });
    // Should not panic
    let _ = root.handle_event(&click, bounds);
}

#[test]
fn test_f17_bva_deeply_nested_markup_thirty_levels() {
    let mut xml = String::from("<VStack>");
    for _ in 0..30 {
        xml.push_str("<VStack class=\"nested-box\">");
    }
    xml.push_str("<Text text=\"Deep Content\" />");
    for _ in 0..30 {
        xml.push_str("</VStack>");
    }
    xml.push_str("</VStack>");

    let doc = parse_quick(&xml).unwrap();
    let mut ctx = DataContext::new();
    let (mut root, _) = build_ui_tree(&doc, &mut ctx);

    let mut engine = quick::layout::engine::LayoutEngine::new();
    let node = root.build_layout(&mut engine).unwrap();
    engine.compute_layout(node, Size::new(400.0, 400.0)).unwrap();
    root.update_layout(&engine, Point::ZERO);

    let bounds = engine.get_layout(node).unwrap();
    let mut canvas = Canvas::new();
    root.paint(&mut canvas, bounds);
    assert!(!canvas.commands().is_empty());
}

#[test]
fn test_f17_bva_empty_markup_string_handling() {
    assert!(parse_quick("").is_err());
    assert!(parse_quick("   \n\t  ").is_err());
}

#[test]
fn test_f17_bva_cdata_and_special_character_unescaping() {
    let xml_src = r#"
    <VStack>
        <Text text="&lt;Material &amp; Rust&gt;" />
    </VStack>
    "#;

    let doc = parse_quick(xml_src).unwrap();
    assert_eq!(doc.root.children[0].text.as_deref(), Some("<Material & Rust>"));
}

// ============================================================================
// TIER 3: CROSS-FEATURE COMBINATIONS IN MARKUP
// ============================================================================

#[test]
fn test_f17_f8_theme_package_injection_into_quick_markup() {
    let xml_src = r#"
    <VStack theme="material-you">
        <Button class="btn-primary" text="Dynamic Themed" />
    </VStack>
    "#;
    let doc = parse_quick(xml_src).unwrap();
    let mut ctx = DataContext::new();
    let (root, stylesheet) = build_ui_tree(&doc, &mut ctx);
    assert_eq!(root.widget_type(), "Container");
    assert!(!stylesheet.rules.is_empty());
}

#[test]
fn test_f17_f9_f10_markup_widget_tree_layout_and_paint() {
    let xml_src = r#"
    <VStack theme="material-you" style="padding: 16px;">
        <Card variant="elevated">
            <Button variant="filled" text="Proceed" />
            <Button variant="outlined" text="Cancel" />
        </Card>
    </VStack>
    "#;
    let doc = parse_quick(xml_src).unwrap();
    let mut ctx = DataContext::new();
    let (mut root, _) = build_ui_tree(&doc, &mut ctx);

    let mut engine = quick::layout::engine::LayoutEngine::new();
    let node = root.build_layout(&mut engine).unwrap();
    engine.compute_layout(node, Size::new(500.0, 300.0)).unwrap();
    root.update_layout(&engine, Point::ZERO);

    let bounds = engine.get_layout(node).unwrap();
    let mut canvas = Canvas::new();
    root.paint(&mut canvas, bounds);
    assert!(canvas.commands().len() >= 5);
}

#[test]
fn test_f17_f11_f12_f13_markup_selection_controls_signals() {
    let sw_sig = Signal::new(true);
    let cb_sig = Signal::new(false);
    let sl_sig = Signal::new(42.0);

    let mut ctx = DataContext::new();
    ctx.bind_bool_signal("sw", sw_sig.clone());
    ctx.bind_bool_signal("cb", cb_sig.clone());
    ctx.bind_f32_signal("sl", sl_sig.clone());

    let xml_src = r#"
    <VStack>
        <Switch checked="$sw" />
        <Checkbox checked="$cb" />
        <Slider value="$sl" min="0" max="100" />
    </VStack>
    "#;
    let doc = parse_quick(xml_src).unwrap();
    let (mut root, _) = build_ui_tree(&doc, &mut ctx);

    let mut engine = quick::layout::engine::LayoutEngine::new();
    let node = root.build_layout(&mut engine).unwrap();
    engine.compute_layout(node, Size::new(400.0, 300.0)).unwrap();
    root.update_layout(&engine, Point::ZERO);

    let bounds = engine.get_layout(node).unwrap();
    let mut canvas = Canvas::new();
    root.paint(&mut canvas, bounds);
    assert!(canvas.commands().len() >= 5);
}

#[test]
fn test_f17_f16_markup_text_input_two_way_binding() {
    let user_name = Signal::new("QuickUser".to_string());
    let mut ctx = DataContext::new();
    ctx.bind_signal("user_name", user_name.clone());

    let xml_src = r#"
    <VStack>
        <TextInput placeholder="Name" text="$user_name" />
    </VStack>
    "#;
    let doc = parse_quick(xml_src).unwrap();
    let (mut root, _) = build_ui_tree(&doc, &mut ctx);

    let mut engine = quick::layout::engine::LayoutEngine::new();
    let node = root.build_layout(&mut engine).unwrap();
    engine.compute_layout(node, Size::new(300.0, 100.0)).unwrap();
    root.update_layout(&engine, Point::ZERO);

    let bounds = engine.get_layout(node).unwrap();
    let mut canvas = Canvas::new();
    root.paint(&mut canvas, bounds);
    assert!(!canvas.commands().is_empty());
}

#[test]
fn test_f17_f18_app_from_quick_event_dispatch_and_render() {
    let click_count = Signal::new(0);
    let count_sig = click_count.clone();
    let greeting = create_computed(move || format!("Count: {}", count_sig.get()));

    let inc = click_count.clone();
    let mut ctx = DataContext::new();
    ctx.bind_signal("greeting", greeting.clone());
    ctx.bind_action("increment", move || {
        inc.update(|v| *v += 1);
    });

    let quick_src = r#"
    <VStack theme="material-you" style="width: 400px; height: 300px; padding: 20px;">
        <Text id="label" text="$greeting" />
        <Button id="btn" text="Add" onclick="increment" />
    </VStack>
    "#;

    let mut app = App::new(WindowOptions::new().title("App Test"))
        .from_quick(quick_src, &mut ctx)
        .unwrap();

    let window_size = Size::new(400.0, 300.0);
    let c1 = app.render_frame(window_size);
    assert!(c1.commands().len() >= 4);
    assert_eq!(greeting.get(), "Count: 0");

    let click_down = Event::Pointer(PointerEvent {
        position: Point::new(50.0, 55.0),
        button: Some(PointerButton::Primary),
        phase: PointerPhase::Down,
        modifiers: Default::default(),
    });
    let click_up = Event::Pointer(PointerEvent {
        position: Point::new(50.0, 55.0),
        button: Some(PointerButton::Primary),
        phase: PointerPhase::Up,
        modifiers: Default::default(),
    });
    let _ = app.handle_event(&click_down, window_size);
    let _ = app.handle_event(&click_up, window_size);

    assert_eq!(greeting.get(), "Count: 1");
    let c2 = app.render_frame(window_size);
    assert!(c2.commands().len() >= 4);
}
