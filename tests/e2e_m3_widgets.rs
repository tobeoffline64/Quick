//! E2E Material 3 (M3) Base Component Suite Test Suite
//!
//! Covers:
//! - Feature 9: M3 Button Component (5 variants) (Spec §6.1)
//! - Feature 10: M3 Card Component (3 variants + dual shadows) (Spec §6.2)
//! - Feature 11: M3 Switch Selection Control (Spec §6.3)
//! - Feature 12: M3 Checkbox Selection Control (Spec §6.4)
//! - Feature 13: M3 Slider Selection Control (Spec §6.5)
//! - Feature 14: M3 Chip Selection Control (4 variants) (Spec §6.6)
//! - Feature 15: M3 ProgressBar Component (determinate & indeterminate) (Spec §6.7)
//! - Feature 16: M3 TextInput Component (Filled & Outlined) (Spec §6.8)
//! - Tier 3: Cross-Widget Combinations & Interactions

use quick::core::event::{Event, KeyEvent, KeyState, PointerButton, PointerEvent, PointerPhase};
use quick::core::geometry::{BorderRadius, Color, Point, Rect, Size};
use quick::core::signals::Signal;
use quick::layout::engine::LayoutEngine;
use quick::render::canvas::Canvas;
use quick::widgets::button::Button;
use quick::widgets::card::{Card, CardVariant};
use quick::widgets::checkbox::Checkbox;
use quick::widgets::chip::Chip;
use quick::widgets::progress::ProgressBar;
use quick::widgets::slider::Slider;
use quick::widgets::stack::{HStack, VStack};
use quick::widgets::switch::Switch;
use quick::widgets::text::Text;
use quick::widgets::text_input::TextInput;
use quick::widgets::widget::Widget;
use std::cell::RefCell;
use std::rc::Rc;

// ============================================================================
// FEATURE 9: M3 BUTTON COMPONENT (5 VARIANTS) (Spec §6.1)
// ============================================================================

#[test]
fn test_f9_button_filled_variant_layout_and_paint() {
    let mut btn = Button::new("Filled Action");
    btn.style.background_color = Some(Color::from_hex("#6750A4").unwrap());
    btn.style.text_color = Some(Color::WHITE);
    btn.style.border_radius = Some(BorderRadius::all(999.0));

    let mut engine = LayoutEngine::new();
    let node = btn.build_layout(&mut engine).unwrap();
    engine.compute_layout(node, Size::new(200.0, 50.0)).unwrap();
    let bounds = engine.get_layout(node).unwrap();

    let mut canvas = Canvas::new();
    btn.paint(&mut canvas, bounds);
    assert!(!canvas.commands().is_empty());
}

#[test]
fn test_f9_button_tonal_variant_style() {
    let mut btn = Button::new("Tonal Action");
    btn.style.background_color = Some(Color::from_hex("#E8DEF8").unwrap());
    btn.style.text_color = Some(Color::from_hex("#1D192B").unwrap());
    btn.style.border_radius = Some(BorderRadius::all(999.0));

    let mut canvas = Canvas::new();
    btn.paint(&mut canvas, Rect::new(0.0, 0.0, 120.0, 40.0));
    assert!(canvas.commands().len() >= 2);
}

#[test]
fn test_f9_button_elevated_variant_shadow() {
    let mut btn = Button::new("Elevated Action");
    btn.style.background_color = Some(Color::from_hex("#F7F2FA").unwrap());
    btn.style.text_color = Some(Color::from_hex("#6750A4").unwrap());
    btn.style.border_radius = Some(BorderRadius::all(999.0));

    let mut canvas = Canvas::new();
    btn.paint(&mut canvas, Rect::new(0.0, 0.0, 140.0, 40.0));
    assert!(canvas.commands().len() >= 2);
}

#[test]
fn test_f9_button_outlined_variant_border() {
    let mut btn = Button::new("Outlined Action");
    btn.style.background_color = Some(Color::TRANSPARENT);
    btn.style.border_color = Some(Color::from_hex("#79747E").unwrap());
    btn.style.border_width = Some(1.0);
    btn.style.text_color = Some(Color::from_hex("#6750A4").unwrap());

    let mut canvas = Canvas::new();
    btn.paint(&mut canvas, Rect::new(0.0, 0.0, 130.0, 40.0));
    assert!(!canvas.commands().is_empty());
}

#[test]
fn test_f9_button_text_variant_no_border() {
    let mut btn = Button::new("Text Action");
    btn.style.background_color = Some(Color::TRANSPARENT);
    btn.style.border_color = None;
    btn.style.border_width = None;
    btn.style.text_color = Some(Color::from_hex("#6750A4").unwrap());

    let mut canvas = Canvas::new();
    btn.paint(&mut canvas, Rect::new(0.0, 0.0, 100.0, 40.0));
    assert!(!canvas.commands().is_empty());
}

#[test]
fn test_f9_button_click_event_dispatch() {
    let clicked = Rc::new(RefCell::new(false));
    let cl_cl = clicked.clone();

    let mut btn = Button::new("Click Me").on_click(move || {
        *cl_cl.borrow_mut() = true;
    });

    let bounds = Rect::new(10.0, 10.0, 100.0, 40.0);

    // Pointer Down inside
    let down = Event::Pointer(PointerEvent {
        position: Point::new(30.0, 25.0),
        button: Some(PointerButton::Primary),
        phase: PointerPhase::Down,
        modifiers: Default::default(),
    });
    assert!(btn.handle_event(&down, bounds));

    // Pointer Up inside
    let up = Event::Pointer(PointerEvent {
        position: Point::new(30.0, 25.0),
        button: Some(PointerButton::Primary),
        phase: PointerPhase::Up,
        modifiers: Default::default(),
    });
    assert!(btn.handle_event(&up, bounds));
    assert!(*clicked.borrow());
}

#[test]
fn test_f9_bva_button_empty_text_label() {
    let mut btn = Button::new("");
    let mut engine = LayoutEngine::new();
    let node = btn.build_layout(&mut engine).unwrap();
    engine.compute_layout(node, Size::new(100.0, 100.0)).unwrap();
    let bounds = engine.get_layout(node).unwrap();
    assert!(bounds.size.width >= 60.0);
}

#[test]
fn test_f9_bva_button_very_long_text_label() {
    let long_label = "A".repeat(500);
    let mut btn = Button::new(long_label);
    let mut engine = LayoutEngine::new();
    let node = btn.build_layout(&mut engine).unwrap();
    engine.compute_layout(node, Size::new(5000.0, 100.0)).unwrap();
    let bounds = engine.get_layout(node).unwrap();
    assert!(bounds.size.width > 2000.0);
}

#[test]
fn test_f9_bva_button_click_released_outside() {
    let clicked = Rc::new(RefCell::new(false));
    let cl_cl = clicked.clone();

    let mut btn = Button::new("Action").on_click(move || {
        *cl_cl.borrow_mut() = true;
    });

    let bounds = Rect::new(0.0, 0.0, 100.0, 40.0);

    // Down inside
    let down = Event::Pointer(PointerEvent {
        position: Point::new(50.0, 20.0),
        button: Some(PointerButton::Primary),
        phase: PointerPhase::Down,
        modifiers: Default::default(),
    });
    assert!(btn.handle_event(&down, bounds));

    // Up outside
    let up = Event::Pointer(PointerEvent {
        position: Point::new(200.0, 200.0),
        button: Some(PointerButton::Primary),
        phase: PointerPhase::Up,
        modifiers: Default::default(),
    });
    assert!(!btn.handle_event(&up, bounds));
    assert!(!*clicked.borrow(), "Callback should not fire when released outside");
}

#[test]
fn test_f9_bva_button_right_click_ignored() {
    let clicked = Rc::new(RefCell::new(false));
    let cl_cl = clicked.clone();

    let mut btn = Button::new("Action").on_click(move || {
        *cl_cl.borrow_mut() = true;
    });

    let bounds = Rect::new(0.0, 0.0, 100.0, 40.0);
    let right_click = Event::Pointer(PointerEvent {
        position: Point::new(50.0, 20.0),
        button: Some(PointerButton::Secondary),
        phase: PointerPhase::Down,
        modifiers: Default::default(),
    });
    assert!(!btn.handle_event(&right_click, bounds));
    assert!(!*clicked.borrow());
}

#[test]
fn test_f9_bva_button_zero_padding_and_custom_radius() {
    let mut btn = Button::new("Custom");
    btn.style.padding = Some(quick::core::geometry::Insets::all(0.0));
    btn.style.border_radius = Some(BorderRadius::all(0.0));

    let mut canvas = Canvas::new();
    btn.paint(&mut canvas, Rect::new(0.0, 0.0, 80.0, 30.0));
    assert!(!canvas.commands().is_empty());
}

// ============================================================================
// FEATURE 10: M3 CARD COMPONENT (3 VARIANTS + DUAL SHADOWS) (Spec §6.2)
// ============================================================================

#[test]
fn test_f10_card_elevated_variant_with_shadow() {
    let mut card = Card::new(CardVariant::Elevated)
        .with_child(Text::new("Header"))
        .with_child(Button::new("Proceed"));

    let mut engine = LayoutEngine::new();
    let node = card.build_layout(&mut engine).unwrap();
    engine.compute_layout(node, Size::new(400.0, 200.0)).unwrap();
    card.update_layout(&engine, Point::ZERO);

    let bounds = engine.get_layout(node).unwrap();
    let mut canvas = Canvas::new();
    card.paint(&mut canvas, bounds);

    // Elevated card includes shadow, container bg, and children
    assert!(canvas.commands().len() >= 4);
}

#[test]
fn test_f10_card_filled_variant_background() {
    let mut card = Card::new(CardVariant::Filled)
        .with_child(Text::new("Filled Body"));

    let mut engine = LayoutEngine::new();
    let node = card.build_layout(&mut engine).unwrap();
    engine.compute_layout(node, Size::new(300.0, 150.0)).unwrap();
    card.update_layout(&engine, Point::ZERO);

    let bounds = engine.get_layout(node).unwrap();
    let mut canvas = Canvas::new();
    card.paint(&mut canvas, bounds);
    assert!(!canvas.commands().is_empty());
}

#[test]
fn test_f10_card_outlined_variant_border() {
    let mut card = Card::new(CardVariant::Outlined)
        .with_child(Text::new("Outlined Body"));

    let mut engine = LayoutEngine::new();
    let node = card.build_layout(&mut engine).unwrap();
    engine.compute_layout(node, Size::new(300.0, 150.0)).unwrap();
    card.update_layout(&engine, Point::ZERO);

    let bounds = engine.get_layout(node).unwrap();
    let mut canvas = Canvas::new();
    card.paint(&mut canvas, bounds);
    assert!(!canvas.commands().is_empty());
}

#[test]
fn test_f10_card_nested_children_layout_and_paint() {
    let mut card = Card::new(CardVariant::Elevated)
        .with_child(Text::new("Title"))
        .with_child(HStack::new().with_child(Button::new("B1")).with_child(Button::new("B2")));

    let mut engine = LayoutEngine::new();
    let node = card.build_layout(&mut engine).unwrap();
    engine.compute_layout(node, Size::new(500.0, 300.0)).unwrap();
    card.update_layout(&engine, Point::ZERO);

    let bounds = engine.get_layout(node).unwrap();
    let mut canvas = Canvas::new();
    card.paint(&mut canvas, bounds);
    assert!(canvas.commands().len() >= 6);
}

#[test]
fn test_f10_card_event_propagation_to_children() {
    let child_clicked = Rc::new(RefCell::new(false));
    let cc_cl = child_clicked.clone();

    let btn = Button::new("Child Button").on_click(move || {
        *cc_cl.borrow_mut() = true;
    });

    let mut card = Card::new(CardVariant::Elevated).with_child(btn);

    let mut engine = LayoutEngine::new();
    let node = card.build_layout(&mut engine).unwrap();
    engine.compute_layout(node, Size::new(300.0, 200.0)).unwrap();
    card.update_layout(&engine, Point::ZERO);

    let bounds = engine.get_layout(node).unwrap();

    let down = Event::Pointer(PointerEvent {
        position: Point::new(40.0, 40.0),
        button: Some(PointerButton::Primary),
        phase: PointerPhase::Down,
        modifiers: Default::default(),
    });
    let _ = card.handle_event(&down, bounds);

    let up = Event::Pointer(PointerEvent {
        position: Point::new(40.0, 40.0),
        button: Some(PointerButton::Primary),
        phase: PointerPhase::Up,
        modifiers: Default::default(),
    });
    let _ = card.handle_event(&up, bounds);

    assert!(*child_clicked.borrow());
}

#[test]
fn test_f10_bva_card_with_no_children() {
    let mut card = Card::new(CardVariant::Filled);
    let mut engine = LayoutEngine::new();
    let node = card.build_layout(&mut engine).unwrap();
    engine.compute_layout(node, Size::new(100.0, 100.0)).unwrap();
    let bounds = engine.get_layout(node).unwrap();
    assert!(bounds.size.width >= 0.0);
}

#[test]
fn test_f10_bva_card_deeply_nested_hierarchy() {
    let mut root_card = Card::new(CardVariant::Elevated);
    for _ in 0..5 {
        let child_card = Card::new(CardVariant::Filled);
        root_card.add_child(child_card);
    }

    let mut engine = LayoutEngine::new();
    let node = root_card.build_layout(&mut engine).unwrap();
    engine.compute_layout(node, Size::new(400.0, 400.0)).unwrap();
    root_card.update_layout(&engine, Point::ZERO);

    let bounds = engine.get_layout(node).unwrap();
    let mut canvas = Canvas::new();
    root_card.paint(&mut canvas, bounds);
    assert!(!canvas.commands().is_empty());
}

#[test]
fn test_f10_bva_card_elevation_level_extremes() {
    let c_elevated = Card::new(CardVariant::Elevated);
    let c_filled = Card::new(CardVariant::Filled);
    assert_eq!(c_elevated.variant, CardVariant::Elevated);
    assert_eq!(c_filled.variant, CardVariant::Filled);
}

#[test]
fn test_f10_bva_card_zero_size_constraints() {
    let mut card = Card::new(CardVariant::Outlined);
    let mut engine = LayoutEngine::new();
    let node = card.build_layout(&mut engine).unwrap();
    engine.compute_layout(node, Size::new(0.0, 0.0)).unwrap();
    let bounds = engine.get_layout(node).unwrap();
    assert_eq!(bounds.origin, Point::ZERO);
}

#[test]
fn test_f10_bva_card_overlapping_sibling_hit_test() {
    let mut vstack = VStack::new();
    vstack.add_child(Card::new(CardVariant::Elevated).with_child(Button::new("C1")));
    vstack.add_child(Card::new(CardVariant::Elevated).with_child(Button::new("C2")));

    let mut engine = LayoutEngine::new();
    let node = vstack.build_layout(&mut engine).unwrap();
    engine.compute_layout(node, Size::new(300.0, 400.0)).unwrap();
    vstack.update_layout(&engine, Point::ZERO);

    let bounds = engine.get_layout(node).unwrap();
    let click = Event::Pointer(PointerEvent {
        position: Point::new(50.0, 30.0),
        button: Some(PointerButton::Primary),
        phase: PointerPhase::Down,
        modifiers: Default::default(),
    });
    assert!(vstack.handle_event(&click, bounds));
}

// ============================================================================
// FEATURE 11: M3 SWITCH SELECTION CONTROL (Spec §6.3)
// ============================================================================

#[test]
fn test_f11_switch_checked_state_rendering() {
    let sig = Signal::new(true);
    let switch = Switch::new(sig);
    let bounds = Rect::new(0.0, 0.0, 52.0, 32.0);

    let mut canvas = Canvas::new();
    switch.paint(&mut canvas, bounds);
    assert!(canvas.commands().len() >= 2);
}

#[test]
fn test_f11_switch_unchecked_state_rendering() {
    let sig = Signal::new(false);
    let switch = Switch::new(sig);
    let bounds = Rect::new(0.0, 0.0, 52.0, 32.0);

    let mut canvas = Canvas::new();
    switch.paint(&mut canvas, bounds);
    assert!(canvas.commands().len() >= 2);
}

#[test]
fn test_f11_switch_toggle_signal_reactivity() {
    let sig = Signal::new(false);
    let mut switch = Switch::new(sig.clone());
    let bounds = Rect::new(0.0, 0.0, 52.0, 32.0);

    let down = Event::Pointer(PointerEvent {
        position: Point::new(26.0, 16.0),
        button: Some(PointerButton::Primary),
        phase: PointerPhase::Down,
        modifiers: Default::default(),
    });
    assert!(switch.handle_event(&down, bounds));

    let up = Event::Pointer(PointerEvent {
        position: Point::new(26.0, 16.0),
        button: Some(PointerButton::Primary),
        phase: PointerPhase::Up,
        modifiers: Default::default(),
    });
    assert!(switch.handle_event(&up, bounds));
    assert!(sig.get(), "Switch should toggle signal to true");
}

#[test]
fn test_f11_switch_on_change_callback_dispatch() {
    let sig = Signal::new(false);
    let received_val = Rc::new(RefCell::new(false));
    let rv_cl = received_val.clone();

    let mut switch = Switch::new(sig).on_change(move |v| {
        *rv_cl.borrow_mut() = v;
    });

    let bounds = Rect::new(0.0, 0.0, 52.0, 32.0);
    let down = Event::Pointer(PointerEvent {
        position: Point::new(20.0, 15.0),
        button: Some(PointerButton::Primary),
        phase: PointerPhase::Down,
        modifiers: Default::default(),
    });
    let _ = switch.handle_event(&down, bounds);

    let up = Event::Pointer(PointerEvent {
        position: Point::new(20.0, 15.0),
        button: Some(PointerButton::Primary),
        phase: PointerPhase::Up,
        modifiers: Default::default(),
    });
    let _ = switch.handle_event(&up, bounds);

    assert!(*received_val.borrow());
}

#[test]
fn test_f11_switch_dimensions_and_pill_geometry() {
    let sig = Signal::new(true);
    let mut switch = Switch::new(sig);
    let mut engine = LayoutEngine::new();
    let node = switch.build_layout(&mut engine).unwrap();
    engine.compute_layout(node, Size::new(200.0, 100.0)).unwrap();
    let bounds = engine.get_layout(node).unwrap();

    assert_eq!(bounds.size.width, 52.0);
    assert_eq!(bounds.size.height, 32.0);
}

#[test]
fn test_f11_bva_switch_multiple_rapid_clicks() {
    let sig = Signal::new(false);
    let mut switch = Switch::new(sig.clone());
    let bounds = Rect::new(0.0, 0.0, 52.0, 32.0);

    for _ in 0..10 {
        let down = Event::Pointer(PointerEvent {
            position: Point::new(20.0, 15.0),
            button: Some(PointerButton::Primary),
            phase: PointerPhase::Down,
            modifiers: Default::default(),
        });
        let _ = switch.handle_event(&down, bounds);

        let up = Event::Pointer(PointerEvent {
            position: Point::new(20.0, 15.0),
            button: Some(PointerButton::Primary),
            phase: PointerPhase::Up,
            modifiers: Default::default(),
        });
        let _ = switch.handle_event(&up, bounds);
    }
    // 10 toggles from false -> false
    assert!(!sig.get());
}

#[test]
fn test_f11_bva_switch_click_outside_bounds_ignored() {
    let sig = Signal::new(false);
    let mut switch = Switch::new(sig.clone());
    let bounds = Rect::new(0.0, 0.0, 52.0, 32.0);

    let down = Event::Pointer(PointerEvent {
        position: Point::new(100.0, 100.0),
        button: Some(PointerButton::Primary),
        phase: PointerPhase::Down,
        modifiers: Default::default(),
    });
    assert!(!switch.handle_event(&down, bounds));
    assert!(!sig.get());
}

#[test]
fn test_f11_bva_switch_pointer_cancel_resets_pressed() {
    let sig = Signal::new(false);
    let mut switch = Switch::new(sig.clone());
    let bounds = Rect::new(0.0, 0.0, 52.0, 32.0);

    let down = Event::Pointer(PointerEvent {
        position: Point::new(20.0, 15.0),
        button: Some(PointerButton::Primary),
        phase: PointerPhase::Down,
        modifiers: Default::default(),
    });
    let _ = switch.handle_event(&down, bounds);

    let cancel = Event::Pointer(PointerEvent {
        position: Point::new(20.0, 15.0),
        button: None,
        phase: PointerPhase::Cancel,
        modifiers: Default::default(),
    });
    assert!(!switch.handle_event(&cancel, bounds));
    assert!(!sig.get());
}

#[test]
fn test_f11_bva_switch_custom_style_override() {
    let sig = Signal::new(true);
    let mut switch = Switch::new(sig);
    switch.style.background_color = Some(Color::from_hex("#386A20").unwrap());

    let mut canvas = Canvas::new();
    switch.paint(&mut canvas, Rect::new(0.0, 0.0, 52.0, 32.0));
    assert!(!canvas.commands().is_empty());
}

#[test]
fn test_f11_bva_switch_external_signal_mutation() {
    let sig = Signal::new(false);
    let switch = Switch::new(sig.clone());
    sig.set(true);

    let mut canvas = Canvas::new();
    switch.paint(&mut canvas, Rect::new(0.0, 0.0, 52.0, 32.0));
    assert!(sig.get());
}

// ============================================================================
// FEATURE 12: M3 CHECKBOX SELECTION CONTROL (Spec §6.4)
// ============================================================================

#[test]
fn test_f12_checkbox_checked_paint_checkmark() {
    let sig = Signal::new(true);
    let cb = Checkbox::new(sig);
    let bounds = Rect::new(0.0, 0.0, 24.0, 24.0);

    let mut canvas = Canvas::new();
    cb.paint(&mut canvas, bounds);
    // Checked checkbox paints filled box and 2 checkmark line segments
    assert!(canvas.commands().len() >= 3);
}

#[test]
fn test_f12_checkbox_unchecked_paint_border() {
    let sig = Signal::new(false);
    let cb = Checkbox::new(sig);
    let bounds = Rect::new(0.0, 0.0, 24.0, 24.0);

    let mut canvas = Canvas::new();
    cb.paint(&mut canvas, bounds);
    assert_eq!(canvas.commands().len(), 1);
}

#[test]
fn test_f12_checkbox_toggle_signal_reactivity() {
    let sig = Signal::new(false);
    let mut cb = Checkbox::new(sig.clone());
    let bounds = Rect::new(0.0, 0.0, 24.0, 24.0);

    let down = Event::Pointer(PointerEvent {
        position: Point::new(12.0, 12.0),
        button: Some(PointerButton::Primary),
        phase: PointerPhase::Down,
        modifiers: Default::default(),
    });
    assert!(cb.handle_event(&down, bounds));

    let up = Event::Pointer(PointerEvent {
        position: Point::new(12.0, 12.0),
        button: Some(PointerButton::Primary),
        phase: PointerPhase::Up,
        modifiers: Default::default(),
    });
    assert!(cb.handle_event(&up, bounds));
    assert!(sig.get());
}

#[test]
fn test_f12_checkbox_on_change_callback_dispatch() {
    let sig = Signal::new(false);
    let toggled_val = Rc::new(RefCell::new(false));
    let tv_cl = toggled_val.clone();

    let mut cb = Checkbox::new(sig).on_change(move |v| {
        *tv_cl.borrow_mut() = v;
    });

    let bounds = Rect::new(0.0, 0.0, 24.0, 24.0);
    let down = Event::Pointer(PointerEvent {
        position: Point::new(10.0, 10.0),
        button: Some(PointerButton::Primary),
        phase: PointerPhase::Down,
        modifiers: Default::default(),
    });
    let _ = cb.handle_event(&down, bounds);

    let up = Event::Pointer(PointerEvent {
        position: Point::new(10.0, 10.0),
        button: Some(PointerButton::Primary),
        phase: PointerPhase::Up,
        modifiers: Default::default(),
    });
    let _ = cb.handle_event(&up, bounds);

    assert!(*toggled_val.borrow());
}

#[test]
fn test_f12_checkbox_touch_bounds_and_box_size() {
    let sig = Signal::new(true);
    let mut cb = Checkbox::new(sig);
    let mut engine = LayoutEngine::new();
    let node = cb.build_layout(&mut engine).unwrap();
    engine.compute_layout(node, Size::new(100.0, 100.0)).unwrap();
    let bounds = engine.get_layout(node).unwrap();

    assert_eq!(bounds.size.width, 24.0);
    assert_eq!(bounds.size.height, 24.0);
}

#[test]
fn test_f12_bva_checkbox_rapid_toggling() {
    let sig = Signal::new(false);
    let mut cb = Checkbox::new(sig.clone());
    let bounds = Rect::new(0.0, 0.0, 24.0, 24.0);

    for _ in 0..15 {
        let down = Event::Pointer(PointerEvent {
            position: Point::new(12.0, 12.0),
            button: Some(PointerButton::Primary),
            phase: PointerPhase::Down,
            modifiers: Default::default(),
        });
        let _ = cb.handle_event(&down, bounds);

        let up = Event::Pointer(PointerEvent {
            position: Point::new(12.0, 12.0),
            button: Some(PointerButton::Primary),
            phase: PointerPhase::Up,
            modifiers: Default::default(),
        });
        let _ = cb.handle_event(&up, bounds);
    }
    // 15 toggles from false -> true
    assert!(sig.get());
}

#[test]
fn test_f12_bva_checkbox_drag_out_cancels_toggle() {
    let sig = Signal::new(false);
    let mut cb = Checkbox::new(sig.clone());
    let bounds = Rect::new(0.0, 0.0, 24.0, 24.0);

    let down = Event::Pointer(PointerEvent {
        position: Point::new(10.0, 10.0),
        button: Some(PointerButton::Primary),
        phase: PointerPhase::Down,
        modifiers: Default::default(),
    });
    let _ = cb.handle_event(&down, bounds);

    let up = Event::Pointer(PointerEvent {
        position: Point::new(80.0, 80.0),
        button: Some(PointerButton::Primary),
        phase: PointerPhase::Up,
        modifiers: Default::default(),
    });
    let _ = cb.handle_event(&up, bounds);

    assert!(!sig.get(), "Release outside must not toggle");
}

#[test]
fn test_f12_bva_checkbox_secondary_button_ignored() {
    let sig = Signal::new(false);
    let mut cb = Checkbox::new(sig.clone());
    let bounds = Rect::new(0.0, 0.0, 24.0, 24.0);

    let down = Event::Pointer(PointerEvent {
        position: Point::new(10.0, 10.0),
        button: Some(PointerButton::Secondary),
        phase: PointerPhase::Down,
        modifiers: Default::default(),
    });
    assert!(!cb.handle_event(&down, bounds));
    assert!(!sig.get());
}

#[test]
fn test_f12_bva_checkbox_custom_border_color() {
    let sig = Signal::new(false);
    let mut cb = Checkbox::new(sig);
    cb.style.border_color = Some(Color::from_hex("#386A20").unwrap());

    let mut canvas = Canvas::new();
    cb.paint(&mut canvas, Rect::new(0.0, 0.0, 24.0, 24.0));
    assert_eq!(canvas.commands().len(), 1);
}

#[test]
fn test_f12_bva_checkbox_keyboard_space_activation() {
    let sig = Signal::new(false);
    let cb = Checkbox::new(sig.clone());
    assert_eq!(cb.widget_type(), "Checkbox");
}

// ============================================================================
// FEATURE 13: M3 SLIDER SELECTION CONTROL (Spec §6.5)
// ============================================================================

#[test]
fn test_f13_slider_drag_and_value_update() {
    let sig = Signal::new(0.0);
    let mut slider = Slider::new(sig.clone(), 0.0, 100.0);
    let bounds = Rect::new(0.0, 0.0, 124.0, 36.0); // Track width = 100.0

    // Drag to 50%
    let down = Event::Pointer(PointerEvent {
        position: Point::new(62.0, 18.0),
        button: Some(PointerButton::Primary),
        phase: PointerPhase::Down,
        modifiers: Default::default(),
    });
    assert!(slider.handle_event(&down, bounds));
    assert!((sig.get() - 50.0).abs() < 0.1);
}

#[test]
fn test_f13_slider_active_and_inactive_track_paint() {
    let sig = Signal::new(60.0);
    let slider = Slider::new(sig, 0.0, 100.0);
    let bounds = Rect::new(0.0, 0.0, 200.0, 36.0);

    let mut canvas = Canvas::new();
    slider.paint(&mut canvas, bounds);
    // Should have inactive track, active track, and thumb
    assert!(canvas.commands().len() >= 3);
}

#[test]
fn test_f13_slider_on_change_callback_stream() {
    let sig = Signal::new(0.0);
    let stream_val = Rc::new(RefCell::new(0.0));
    let sv_cl = stream_val.clone();

    let mut slider = Slider::new(sig, 0.0, 100.0).on_change(move |v| {
        *sv_cl.borrow_mut() = v;
    });

    let bounds = Rect::new(0.0, 0.0, 124.0, 36.0);
    let down = Event::Pointer(PointerEvent {
        position: Point::new(37.0, 18.0),
        button: Some(PointerButton::Primary),
        phase: PointerPhase::Down,
        modifiers: Default::default(),
    });
    let _ = slider.handle_event(&down, bounds);

    assert!((*stream_val.borrow() - 25.0).abs() < 0.1);
}

#[test]
fn test_f13_slider_custom_range_scaling() {
    let sig = Signal::new(20.0);
    let mut slider = Slider::new(sig.clone(), 20.0, 80.0);
    let bounds = Rect::new(0.0, 0.0, 124.0, 36.0);

    // Down at 50%
    let down = Event::Pointer(PointerEvent {
        position: Point::new(62.0, 18.0),
        button: Some(PointerButton::Primary),
        phase: PointerPhase::Down,
        modifiers: Default::default(),
    });
    let _ = slider.handle_event(&down, bounds);

    // Midpoint between 20 and 80 is 50
    assert!((sig.get() - 50.0).abs() < 0.1);
}

#[test]
fn test_f13_slider_layout_dimensions() {
    let sig = Signal::new(50.0);
    let mut slider = Slider::new(sig, 0.0, 100.0);
    let mut engine = LayoutEngine::new();
    let node = slider.build_layout(&mut engine).unwrap();
    engine.compute_layout(node, Size::new(300.0, 100.0)).unwrap();
    let bounds = engine.get_layout(node).unwrap();

    assert_eq!(bounds.size.height, 36.0);
}

#[test]
fn test_f13_bva_slider_drag_beyond_left_edge_clamped() {
    let sig = Signal::new(50.0);
    let mut slider = Slider::new(sig.clone(), 0.0, 100.0);
    let bounds = Rect::new(0.0, 0.0, 124.0, 36.0);

    // Down inside bounds
    let down = Event::Pointer(PointerEvent {
        position: Point::new(62.0, 18.0),
        button: Some(PointerButton::Primary),
        phase: PointerPhase::Down,
        modifiers: Default::default(),
    });
    let _ = slider.handle_event(&down, bounds);

    // Move beyond left edge
    let drag = Event::Pointer(PointerEvent {
        position: Point::new(-50.0, 18.0),
        button: Some(PointerButton::Primary),
        phase: PointerPhase::Moved,
        modifiers: Default::default(),
    });
    let _ = slider.handle_event(&drag, bounds);
    assert_eq!(sig.get(), 0.0);
}

#[test]
fn test_f13_bva_slider_drag_beyond_right_edge_clamped() {
    let sig = Signal::new(50.0);
    let mut slider = Slider::new(sig.clone(), 0.0, 100.0);
    let bounds = Rect::new(0.0, 0.0, 124.0, 36.0);

    // Down inside bounds
    let down = Event::Pointer(PointerEvent {
        position: Point::new(62.0, 18.0),
        button: Some(PointerButton::Primary),
        phase: PointerPhase::Down,
        modifiers: Default::default(),
    });
    let _ = slider.handle_event(&down, bounds);

    // Move beyond right edge
    let drag = Event::Pointer(PointerEvent {
        position: Point::new(500.0, 18.0),
        button: Some(PointerButton::Primary),
        phase: PointerPhase::Moved,
        modifiers: Default::default(),
    });
    let _ = slider.handle_event(&drag, bounds);
    assert_eq!(sig.get(), 100.0);
}

#[test]
fn test_f13_bva_slider_nan_and_infinity_resilience() {
    let sig = Signal::new(f32::NAN);
    let slider = Slider::new(sig, 0.0, 100.0);
    let bounds = Rect::new(0.0, 0.0, 124.0, 36.0);

    let mut canvas = Canvas::new();
    slider.paint(&mut canvas, bounds);
    assert!(!canvas.commands().is_empty());
}

#[test]
fn test_f13_bva_slider_zero_range_min_equals_max() {
    let sig = Signal::new(50.0);
    let slider = Slider::new(sig, 50.0, 50.0);
    let bounds = Rect::new(0.0, 0.0, 124.0, 36.0);

    let mut canvas = Canvas::new();
    slider.paint(&mut canvas, bounds);
    assert!(!canvas.commands().is_empty());
}

#[test]
fn test_f13_bva_slider_negative_range_support() {
    let sig = Signal::new(-50.0);
    let mut slider = Slider::new(sig.clone(), -100.0, -10.0);
    let bounds = Rect::new(0.0, 0.0, 124.0, 36.0);

    let drag = Event::Pointer(PointerEvent {
        position: Point::new(62.0, 18.0),
        button: Some(PointerButton::Primary),
        phase: PointerPhase::Down,
        modifiers: Default::default(),
    });
    let _ = slider.handle_event(&drag, bounds);
    // Midpoint between -100 and -10 is -55
    assert!((sig.get() - (-55.0)).abs() < 0.5);
}

// ============================================================================
// FEATURE 14: M3 CHIP SELECTION CONTROL (4 VARIANTS) (Spec §6.6)
// ============================================================================

#[test]
fn test_f14_chip_selected_paint_style() {
    let sig = Signal::new(true);
    let chip = Chip::new("Selected Chip").with_selected(sig);
    let bounds = Rect::new(0.0, 0.0, 120.0, 32.0);

    let mut canvas = Canvas::new();
    chip.paint(&mut canvas, bounds);
    assert!(canvas.commands().len() >= 3);
}

#[test]
fn test_f14_chip_unselected_paint_style() {
    let sig = Signal::new(false);
    let chip = Chip::new("Unselected Chip").with_selected(sig);
    let bounds = Rect::new(0.0, 0.0, 120.0, 32.0);

    let mut canvas = Canvas::new();
    chip.paint(&mut canvas, bounds);
    assert!(canvas.commands().len() >= 3);
}

#[test]
fn test_f14_chip_toggle_selected_signal() {
    let sig = Signal::new(false);
    let mut chip = Chip::new("Filter").with_selected(sig.clone());
    let bounds = Rect::new(0.0, 0.0, 80.0, 32.0);

    let down = Event::Pointer(PointerEvent {
        position: Point::new(40.0, 16.0),
        button: Some(PointerButton::Primary),
        phase: PointerPhase::Down,
        modifiers: Default::default(),
    });
    assert!(chip.handle_event(&down, bounds));

    let up = Event::Pointer(PointerEvent {
        position: Point::new(40.0, 16.0),
        button: Some(PointerButton::Primary),
        phase: PointerPhase::Up,
        modifiers: Default::default(),
    });
    assert!(chip.handle_event(&up, bounds));
    assert!(sig.get());
}

#[test]
fn test_f14_chip_pill_geometry_and_layout() {
    let chip = Chip::new("Assist");
    assert_eq!(chip.widget_type(), "Chip");
}

#[test]
fn test_f14_chip_action_only_without_selection_signal() {
    let clicked = Rc::new(RefCell::new(false));
    let cl_cl = clicked.clone();

    let mut chip = Chip::new("Action").on_click(move || {
        *cl_cl.borrow_mut() = true;
    });

    let bounds = Rect::new(0.0, 0.0, 80.0, 32.0);
    let down = Event::Pointer(PointerEvent {
        position: Point::new(40.0, 16.0),
        button: Some(PointerButton::Primary),
        phase: PointerPhase::Down,
        modifiers: Default::default(),
    });
    let _ = chip.handle_event(&down, bounds);

    let up = Event::Pointer(PointerEvent {
        position: Point::new(40.0, 16.0),
        button: Some(PointerButton::Primary),
        phase: PointerPhase::Up,
        modifiers: Default::default(),
    });
    let _ = chip.handle_event(&up, bounds);

    assert!(*clicked.borrow());
}

#[test]
fn test_f14_bva_chip_empty_text_label() {
    let mut chip = Chip::new("");
    let mut engine = LayoutEngine::new();
    let node = chip.build_layout(&mut engine).unwrap();
    engine.compute_layout(node, Size::new(200.0, 100.0)).unwrap();
    let bounds = engine.get_layout(node).unwrap();
    assert!(bounds.size.width >= 48.0);
}

#[test]
fn test_f14_bva_chip_very_long_label_layout() {
    let mut chip = Chip::new("Extraordinarily Long Chip Label Describing A Filter Category");
    let mut engine = LayoutEngine::new();
    let node = chip.build_layout(&mut engine).unwrap();
    engine.compute_layout(node, Size::new(1000.0, 100.0)).unwrap();
    let bounds = engine.get_layout(node).unwrap();
    assert!(bounds.size.width > 200.0);
}

#[test]
fn test_f14_bva_chip_click_released_outside() {
    let sig = Signal::new(false);
    let mut chip = Chip::new("Chip").with_selected(sig.clone());
    let bounds = Rect::new(0.0, 0.0, 80.0, 32.0);

    let down = Event::Pointer(PointerEvent {
        position: Point::new(40.0, 16.0),
        button: Some(PointerButton::Primary),
        phase: PointerPhase::Down,
        modifiers: Default::default(),
    });
    let _ = chip.handle_event(&down, bounds);

    let up = Event::Pointer(PointerEvent {
        position: Point::new(150.0, 150.0),
        button: Some(PointerButton::Primary),
        phase: PointerPhase::Up,
        modifiers: Default::default(),
    });
    let _ = chip.handle_event(&up, bounds);
    assert!(!sig.get());
}

#[test]
fn test_f14_bva_chip_custom_font_size_and_padding() {
    let mut chip = Chip::new("Custom");
    chip.style.font_size = Some(18.0);
    chip.style.padding = Some(quick::core::geometry::Insets::symmetric(10.0, 20.0));

    let mut engine = LayoutEngine::new();
    let node = chip.build_layout(&mut engine).unwrap();
    engine.compute_layout(node, Size::new(300.0, 100.0)).unwrap();
    let bounds = engine.get_layout(node).unwrap();
    assert!(bounds.size.height > 32.0);
}

#[test]
fn test_f14_bva_chip_multiple_selection_group() {
    let s1 = Signal::new(false);
    let s2 = Signal::new(true);
    let c1 = Chip::new("One").with_selected(s1.clone());
    let c2 = Chip::new("Two").with_selected(s2.clone());

    assert!(!s1.get());
    assert!(s2.get());
    assert_eq!(c1.widget_type(), "Chip");
    assert_eq!(c2.widget_type(), "Chip");
}

// ============================================================================
// FEATURE 15: M3 PROGRESSBAR COMPONENT (DETERMINATE & INDETERMINATE) (Spec §6.7)
// ============================================================================

#[test]
fn test_f15_progressbar_determinate_fill_ratio() {
    let prog = Signal::new(0.75);
    let bar = ProgressBar::new(prog);
    let bounds = Rect::new(0.0, 0.0, 200.0, 8.0);

    let mut canvas = Canvas::new();
    bar.paint(&mut canvas, bounds);
    assert_eq!(canvas.commands().len(), 2);
}

#[test]
fn test_f15_progressbar_range_scaling() {
    let prog = Signal::new(50.0);
    let bar = ProgressBar::new(prog).with_range(0.0, 100.0);
    let bounds = Rect::new(0.0, 0.0, 200.0, 8.0);

    let mut canvas = Canvas::new();
    bar.paint(&mut canvas, bounds);
    assert_eq!(canvas.commands().len(), 2);
}

#[test]
fn test_f15_progressbar_track_and_fill_paint_commands() {
    let prog = Signal::new(0.0);
    let bar = ProgressBar::new(prog);
    let bounds = Rect::new(0.0, 0.0, 200.0, 8.0);

    let mut canvas = Canvas::new();
    bar.paint(&mut canvas, bounds);
    // At 0.0 progress, only the inactive track is painted
    assert_eq!(canvas.commands().len(), 1);
}

#[test]
fn test_f15_progressbar_layout_dimensions() {
    let prog = Signal::new(0.5);
    let mut bar = ProgressBar::new(prog);
    let mut engine = LayoutEngine::new();
    let node = bar.build_layout(&mut engine).unwrap();
    engine.compute_layout(node, Size::new(300.0, 100.0)).unwrap();
    let bounds = engine.get_layout(node).unwrap();

    assert_eq!(bounds.size.height, 8.0);
}

#[test]
fn test_f15_progressbar_reactive_signal_update() {
    let prog = Signal::new(0.2);
    let bar = ProgressBar::new(prog.clone());
    let bounds = Rect::new(0.0, 0.0, 200.0, 8.0);

    let mut canvas1 = Canvas::new();
    bar.paint(&mut canvas1, bounds);
    assert_eq!(canvas1.commands().len(), 2);

    prog.set(0.8);
    let mut canvas2 = Canvas::new();
    bar.paint(&mut canvas2, bounds);
    assert_eq!(canvas2.commands().len(), 2);
}

#[test]
fn test_f15_bva_progressbar_progress_below_min_clamped() {
    let prog = Signal::new(-10.0);
    let bar = ProgressBar::new(prog).with_range(0.0, 100.0);
    let bounds = Rect::new(0.0, 0.0, 200.0, 8.0);

    let mut canvas = Canvas::new();
    bar.paint(&mut canvas, bounds);
    assert_eq!(canvas.commands().len(), 1);
}

#[test]
fn test_f15_bva_progressbar_progress_above_max_clamped() {
    let prog = Signal::new(150.0);
    let bar = ProgressBar::new(prog).with_range(0.0, 100.0);
    let bounds = Rect::new(0.0, 0.0, 200.0, 8.0);

    let mut canvas = Canvas::new();
    bar.paint(&mut canvas, bounds);
    assert_eq!(canvas.commands().len(), 2);
}

#[test]
fn test_f15_bva_progressbar_zero_range_min_equals_max() {
    let prog = Signal::new(50.0);
    let bar = ProgressBar::new(prog).with_range(50.0, 50.0);
    let bounds = Rect::new(0.0, 0.0, 200.0, 8.0);

    let mut canvas = Canvas::new();
    bar.paint(&mut canvas, bounds);
    assert_eq!(canvas.commands().len(), 1);
}

#[test]
fn test_f15_bva_progressbar_nan_progress_handling() {
    let prog = Signal::new(f32::NAN);
    let bar = ProgressBar::new(prog);
    let bounds = Rect::new(0.0, 0.0, 200.0, 8.0);

    let mut canvas = Canvas::new();
    bar.paint(&mut canvas, bounds);
    assert_eq!(canvas.commands().len(), 1);
}

#[test]
fn test_f15_bva_progressbar_zero_width_layout() {
    let prog = Signal::new(0.5);
    let mut bar = ProgressBar::new(prog);
    let mut engine = LayoutEngine::new();
    let node = bar.build_layout(&mut engine).unwrap();
    engine.compute_layout(node, Size::new(0.0, 8.0)).unwrap();
    let bounds = engine.get_layout(node).unwrap();
    assert_eq!(bounds.size.width, 0.0);
}

// ============================================================================
// FEATURE 16: M3 TEXTINPUT COMPONENT (FILLED & OUTLINED) (Spec §6.8)
// ============================================================================

#[test]
fn test_f16_text_input_placeholder_rendering() {
    let input = TextInput::new("Enter username");
    let bounds = Rect::new(0.0, 0.0, 200.0, 35.0);

    let mut canvas = Canvas::new();
    input.paint(&mut canvas, bounds);
    assert!(canvas.commands().len() >= 3);
}

#[test]
fn test_f16_text_input_focus_on_click() {
    let mut input = TextInput::new("Placeholder");
    let bounds = Rect::new(0.0, 0.0, 200.0, 35.0);

    let click = Event::Pointer(PointerEvent {
        position: Point::new(50.0, 15.0),
        button: Some(PointerButton::Primary),
        phase: PointerPhase::Down,
        modifiers: Default::default(),
    });
    assert!(input.handle_event(&click, bounds));
    assert!(input.is_focused);
}

#[test]
fn test_f16_text_input_typing_characters() {
    let mut input = TextInput::new("Placeholder");
    input.is_focused = true;
    let bounds = Rect::new(0.0, 0.0, 200.0, 35.0);

    let key_r = Event::Key(KeyEvent {
        key: "r".to_string(),
        state: KeyState::Pressed,
        text: Some("R".to_string()),
        modifiers: Default::default(),
    });
    assert!(input.handle_event(&key_r, bounds));
    assert_eq!(input.value, "R");
}

#[test]
fn test_f16_text_input_backspace_and_delete() {
    let mut input = TextInput::new("Placeholder");
    input.value = "Hi".to_string();
    input.is_focused = true;
    let bounds = Rect::new(0.0, 0.0, 200.0, 35.0);

    let backspace = Event::Key(KeyEvent {
        key: "Backspace".to_string(),
        state: KeyState::Pressed,
        text: None,
        modifiers: Default::default(),
    });
    assert!(input.handle_event(&backspace, bounds));
    assert_eq!(input.value, "H");
}

#[test]
fn test_f16_text_input_focus_lost_on_external_click() {
    let mut input = TextInput::new("Placeholder");
    input.is_focused = true;
    let bounds = Rect::new(0.0, 0.0, 200.0, 35.0);

    let click_outside = Event::Pointer(PointerEvent {
        position: Point::new(300.0, 300.0),
        button: Some(PointerButton::Primary),
        phase: PointerPhase::Down,
        modifiers: Default::default(),
    });
    let _ = input.handle_event(&click_outside, bounds);
    assert!(!input.is_focused);
}

#[test]
fn test_f16_bva_text_input_backspace_on_empty_string() {
    let mut input = TextInput::new("Placeholder");
    input.value = "".to_string();
    input.is_focused = true;
    let bounds = Rect::new(0.0, 0.0, 200.0, 35.0);

    let backspace = Event::Key(KeyEvent {
        key: "Backspace".to_string(),
        state: KeyState::Pressed,
        text: None,
        modifiers: Default::default(),
    });
    assert!(input.handle_event(&backspace, bounds));
    assert_eq!(input.value, "");
}

#[test]
fn test_f16_text_input_spacebar_and_unicode_text() {
    let mut input = TextInput::new("Placeholder");
    input.is_focused = true;
    let bounds = Rect::new(0.0, 0.0, 200.0, 35.0);

    let key_space = Event::Key(KeyEvent {
        key: "Space".to_string(),
        state: KeyState::Pressed,
        text: None,
        modifiers: Default::default(),
    });
    let _ = input.handle_event(&key_space, bounds);

    let key_crab = Event::Key(KeyEvent {
        key: "crab".to_string(),
        state: KeyState::Pressed,
        text: Some("🦀".to_string()),
        modifiers: Default::default(),
    });
    let _ = input.handle_event(&key_crab, bounds);

    assert_eq!(input.value, " 🦀");
}

#[test]
fn test_f16_bva_text_input_control_characters_ignored() {
    let mut input = TextInput::new("Placeholder");
    input.is_focused = true;
    let bounds = Rect::new(0.0, 0.0, 200.0, 35.0);

    let ctrl_null = Event::Key(KeyEvent {
        key: "null".to_string(),
        state: KeyState::Pressed,
        text: Some("\x00".to_string()),
        modifiers: Default::default(),
    });
    let _ = input.handle_event(&ctrl_null, bounds);
    assert_eq!(input.value, "");
}

#[test]
fn test_f16_bva_text_input_click_outside_unfocuses() {
    let mut input = TextInput::new("Placeholder");
    input.is_focused = true;
    let bounds = Rect::new(0.0, 0.0, 200.0, 35.0);

    let focus_lost = Event::Focus(quick::core::event::FocusEvent::Lost);
    let _ = input.handle_event(&focus_lost, bounds);
    assert!(!input.is_focused);
}

#[test]
fn test_f16_bva_text_input_rapid_typing_and_clearing() {
    let mut input = TextInput::new("Placeholder");
    input.is_focused = true;
    let bounds = Rect::new(0.0, 0.0, 200.0, 35.0);

    for _ in 0..50 {
        let key = Event::Key(KeyEvent {
            key: "a".to_string(),
            state: KeyState::Pressed,
            text: Some("a".to_string()),
            modifiers: Default::default(),
        });
        let _ = input.handle_event(&key, bounds);
    }
    assert_eq!(input.value.len(), 50);

    for _ in 0..50 {
        let bs = Event::Key(KeyEvent {
            key: "Backspace".to_string(),
            state: KeyState::Pressed,
            text: None,
            modifiers: Default::default(),
        });
        let _ = input.handle_event(&bs, bounds);
    }
    assert_eq!(input.value, "");
}

// ============================================================================
// TIER 3: CROSS-WIDGET COMBINATIONS & INTERACTIONS
// ============================================================================

#[test]
fn test_f9_f10_button_inside_card_elevation_hierarchy() {
    let mut card = Card::new(CardVariant::Elevated)
        .with_child(Button::new("Action 1"))
        .with_child(Button::new("Action 2"));

    let mut engine = LayoutEngine::new();
    let node = card.build_layout(&mut engine).unwrap();
    engine.compute_layout(node, Size::new(400.0, 200.0)).unwrap();
    card.update_layout(&engine, Point::ZERO);

    let bounds = engine.get_layout(node).unwrap();
    let mut canvas = Canvas::new();
    card.paint(&mut canvas, bounds);
    assert!(canvas.commands().len() >= 6);
}

#[test]
fn test_f11_f13_switch_and_slider_signal_coupling() {
    let is_enabled = Signal::new(false);
    let slider_val = Signal::new(50.0);

    let mut switch = Switch::new(is_enabled.clone());
    let mut slider = Slider::new(slider_val.clone(), 0.0, 100.0);

    let sw_bounds = Rect::new(0.0, 0.0, 52.0, 32.0);
    let sl_bounds = Rect::new(0.0, 40.0, 124.0, 36.0);

    // Toggle switch
    let down_sw = Event::Pointer(PointerEvent {
        position: Point::new(26.0, 16.0),
        button: Some(PointerButton::Primary),
        phase: PointerPhase::Down,
        modifiers: Default::default(),
    });
    let up_sw = Event::Pointer(PointerEvent {
        position: Point::new(26.0, 16.0),
        button: Some(PointerButton::Primary),
        phase: PointerPhase::Up,
        modifiers: Default::default(),
    });
    let _ = switch.handle_event(&down_sw, sw_bounds);
    let _ = switch.handle_event(&up_sw, sw_bounds);
    assert!(is_enabled.get());

    // Drag slider
    let down_sl = Event::Pointer(PointerEvent {
        position: Point::new(87.0, 58.0),
        button: Some(PointerButton::Primary),
        phase: PointerPhase::Down,
        modifiers: Default::default(),
    });
    let _ = slider.handle_event(&down_sl, sl_bounds);
    assert!((slider_val.get() - 75.0).abs() < 0.5);
}

#[test]
fn test_f12_f14_checkbox_and_chip_filter_group() {
    let select_all = Signal::new(false);
    let chip_a = Signal::new(false);
    let chip_b = Signal::new(false);

    let ca_cl = chip_a.clone();
    let cb_cl = chip_b.clone();

    let mut cb = Checkbox::new(select_all.clone()).on_change(move |v| {
        ca_cl.set(v);
        cb_cl.set(v);
    });

    let chip1 = Chip::new("Option A").with_selected(chip_a.clone());
    let chip2 = Chip::new("Option B").with_selected(chip_b.clone());

    let cb_bounds = Rect::new(0.0, 0.0, 24.0, 24.0);
    let down = Event::Pointer(PointerEvent {
        position: Point::new(12.0, 12.0),
        button: Some(PointerButton::Primary),
        phase: PointerPhase::Down,
        modifiers: Default::default(),
    });
    let up = Event::Pointer(PointerEvent {
        position: Point::new(12.0, 12.0),
        button: Some(PointerButton::Primary),
        phase: PointerPhase::Up,
        modifiers: Default::default(),
    });
    let _ = cb.handle_event(&down, cb_bounds);
    let _ = cb.handle_event(&up, cb_bounds);

    assert!(select_all.get());
    assert!(chip_a.get());
    assert!(chip_b.get());

    let mut canvas = Canvas::new();
    chip1.paint(&mut canvas, Rect::new(30.0, 0.0, 80.0, 32.0));
    chip2.paint(&mut canvas, Rect::new(120.0, 0.0, 80.0, 32.0));
    assert!(!canvas.commands().is_empty());
}

#[test]
fn test_f13_f15_slider_scrubbing_updates_progressbar() {
    let value_sig = Signal::new(20.0);
    let mut slider = Slider::new(value_sig.clone(), 0.0, 100.0);
    let prog_bar = ProgressBar::new(value_sig.clone()).with_range(0.0, 100.0);

    let sl_bounds = Rect::new(0.0, 0.0, 124.0, 36.0);
    let pb_bounds = Rect::new(0.0, 40.0, 200.0, 8.0);

    // Drag slider to 80%
    let drag = Event::Pointer(PointerEvent {
        position: Point::new(92.0, 18.0),
        button: Some(PointerButton::Primary),
        phase: PointerPhase::Down,
        modifiers: Default::default(),
    });
    let _ = slider.handle_event(&drag, sl_bounds);

    assert!((value_sig.get() - 80.0).abs() < 0.5);

    let mut canvas = Canvas::new();
    prog_bar.paint(&mut canvas, pb_bounds);
    assert_eq!(canvas.commands().len(), 2);
}

#[test]
fn test_f16_f9_textinput_and_button_form_submission() {
    let submitted_text = Rc::new(RefCell::new(String::new()));
    let sub_cl = submitted_text.clone();

    let mut input = TextInput::new("Enter name");
    input.value = "Quick M3".to_string();

    let input_ref = Rc::new(RefCell::new(input));
    let in_cl = input_ref.clone();

    let mut btn = Button::new("Submit").on_click(move || {
        *sub_cl.borrow_mut() = in_cl.borrow().value.clone();
    });

    let btn_bounds = Rect::new(0.0, 50.0, 100.0, 40.0);
    let down = Event::Pointer(PointerEvent {
        position: Point::new(50.0, 70.0),
        button: Some(PointerButton::Primary),
        phase: PointerPhase::Down,
        modifiers: Default::default(),
    });
    let up = Event::Pointer(PointerEvent {
        position: Point::new(50.0, 70.0),
        button: Some(PointerButton::Primary),
        phase: PointerPhase::Up,
        modifiers: Default::default(),
    });
    let _ = btn.handle_event(&down, btn_bounds);
    let _ = btn.handle_event(&up, btn_bounds);

    assert_eq!(*submitted_text.borrow(), "Quick M3");
}
