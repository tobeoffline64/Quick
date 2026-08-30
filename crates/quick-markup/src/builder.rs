use crate::schema::{UiDocument, UiNode};
use quick_core::signals::Signal;
use quick_style::parser::{parse_inline_style, parse_stylesheet};
use quick_style::rule::StyleSheet;
use quick_style::theme::ThemePackage;
use quick_widgets::button::Button;
use quick_widgets::card::{Card, CardVariant};
use quick_widgets::checkbox::Checkbox;
use quick_widgets::chip::Chip;
use quick_widgets::container::Container;
use quick_widgets::progress::ProgressBar;
use quick_widgets::slider::Slider;
use quick_widgets::stack::{HStack, VStack};
use quick_widgets::switch::Switch;
use quick_widgets::text::Text;
use quick_widgets::text_input::TextInput;
use quick_widgets::widget::Widget;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

#[derive(Default, Clone)]
pub struct DataContext {
    pub string_signals: HashMap<String, Signal<String>>,
    pub bool_signals: HashMap<String, Signal<bool>>,
    pub f32_signals: HashMap<String, Signal<f32>>,
    pub action_handlers: HashMap<String, Rc<RefCell<dyn FnMut()>>>,
}

impl DataContext {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn bind_signal(&mut self, name: impl Into<String>, signal: Signal<String>) {
        self.string_signals.insert(name.into(), signal);
    }

    pub fn bind_bool_signal(&mut self, name: impl Into<String>, signal: Signal<bool>) {
        self.bool_signals.insert(name.into(), signal);
    }

    pub fn bind_f32_signal(&mut self, name: impl Into<String>, signal: Signal<f32>) {
        self.f32_signals.insert(name.into(), signal);
    }

    pub fn bind_action<F: FnMut() + 'static>(&mut self, name: impl Into<String>, handler: F) {
        self.action_handlers.insert(name.into(), Rc::new(RefCell::new(handler)));
    }
}

pub fn build_ui_tree(
    doc: &UiDocument,
    data_ctx: &mut DataContext,
) -> (Box<dyn Widget>, StyleSheet) {
    let mut stylesheet = if let Some(ref css) = doc.styles {
        parse_stylesheet(css)
    } else {
        StyleSheet::new()
    };

    // Apply Theme Package if specified on root or document
    if let Some(ref theme_name) = doc.root.attributes.get("theme") {
        let theme = match theme_name.as_str() {
            "material-you" | "m3" => ThemePackage::material_you(),
            "noctalia" | "noctalia-dark" => ThemePackage::noctalia(),
            "noctalia-light" => ThemePackage::noctalia_light(),
            "nord" => ThemePackage::nord(),
            _ => ThemePackage::material_you(),
        };
        let theme_css = theme.generate_css();
        let theme_sheet = parse_stylesheet(&theme_css);
        stylesheet.rules.splice(0..0, theme_sheet.rules);
    }

    let root_widget = build_node(&doc.root, data_ctx, &stylesheet);
    (root_widget, stylesheet)
}

fn build_node(
    node: &UiNode,
    data_ctx: &mut DataContext,
    stylesheet: &StyleSheet,
) -> Box<dyn Widget> {
    let classes: Vec<String> = node
        .class
        .as_deref()
        .map(|c| c.split_whitespace().map(|s| s.to_string()).collect())
        .unwrap_or_default();
    let class_refs: Vec<&str> = classes.iter().map(|s| s.as_str()).collect();

    let mut computed_style = stylesheet.resolve_with_attrs(
        &node.element,
        &class_refs,
        node.id.as_deref(),
        None,
        Some(&node.attributes),
    );

    if let Some(ref inline_css) = node.style {
        let inline_style = parse_inline_style(inline_css);
        computed_style.merge_with(&inline_style);
    }

    match node.element.as_str() {
        "Text" => {
            let text_val = node.text.as_deref().unwrap_or("");
            let mut text_widget = if let Some(binding_key) = text_val.strip_prefix('$') {
                if let Some(sig) = data_ctx.string_signals.get(binding_key) {
                    Text::dynamic(sig.clone())
                } else {
                    Text::new(text_val)
                }
            } else {
                Text::new(text_val)
            };

            text_widget.id = node.id.clone();
            text_widget.classes = classes;
            text_widget.style.merge_with(&computed_style);
            Box::new(text_widget)
        }
        "Button" => {
            let text_val = node.text.as_deref().unwrap_or("Button");
            let mut btn = Button::new(text_val);
            btn.id = node.id.clone();
            btn.classes = classes;
            btn.style.merge_with(&computed_style);

            let action_opt = node.on_click.as_ref()
                .or_else(|| node.attributes.get("onclick"))
                .or_else(|| node.attributes.get("on_click"));

            if let Some(action_name) = action_opt {
                let clean_name = action_name.trim_end_matches("()").trim();
                if let Some(handler) = data_ctx.action_handlers.get(clean_name) {
                    let handler_cl = handler.clone();
                    btn.on_click = Some(Box::new(move || {
                        (handler_cl.borrow_mut())();
                    }));
                }
            }
            Box::new(btn)
        }
        "Switch" => {
            let checked_sig = node.attributes.get("checked")
                .and_then(|v| v.strip_prefix('$'))
                .and_then(|key| data_ctx.bool_signals.get(key).cloned())
                .unwrap_or_else(|| Signal::new(false));

            let mut switch = Switch::new(checked_sig);
            switch.id = node.id.clone();
            switch.classes = classes;
            switch.style.merge_with(&computed_style);

            let action_opt = node.on_change.as_ref()
                .or_else(|| node.attributes.get("onchange"))
                .or_else(|| node.attributes.get("on_change"));

            if let Some(action_name) = action_opt {
                let clean_name = action_name.trim_end_matches("()").trim();
                if let Some(handler) = data_ctx.action_handlers.get(clean_name) {
                    let handler_cl = handler.clone();
                    switch.on_change = Some(Box::new(move |_| {
                        (handler_cl.borrow_mut())();
                    }));
                }
            }
            Box::new(switch)
        }
        "Checkbox" => {
            let checked_sig = node.attributes.get("checked")
                .and_then(|v| v.strip_prefix('$'))
                .and_then(|key| data_ctx.bool_signals.get(key).cloned())
                .unwrap_or_else(|| Signal::new(false));

            let mut cb = Checkbox::new(checked_sig);
            cb.id = node.id.clone();
            cb.classes = classes;
            cb.style.merge_with(&computed_style);

            let action_opt = node.on_change.as_ref()
                .or_else(|| node.attributes.get("onchange"))
                .or_else(|| node.attributes.get("on_change"));

            if let Some(action_name) = action_opt {
                let clean_name = action_name.trim_end_matches("()").trim();
                if let Some(handler) = data_ctx.action_handlers.get(clean_name) {
                    let handler_cl = handler.clone();
                    cb.on_change = Some(Box::new(move |_| {
                        (handler_cl.borrow_mut())();
                    }));
                }
            }
            Box::new(cb)
        }
        "Slider" => {
            let val_sig = node.attributes.get("value")
                .and_then(|v| v.strip_prefix('$'))
                .and_then(|key| data_ctx.f32_signals.get(key).cloned())
                .unwrap_or_else(|| Signal::new(0.0));

            let min_val = node.attributes.get("min").and_then(|v| v.parse().ok()).unwrap_or(0.0);
            let max_val = node.attributes.get("max").and_then(|v| v.parse().ok()).unwrap_or(100.0);

            let mut slider = Slider::new(val_sig, min_val, max_val);
            slider.id = node.id.clone();
            slider.classes = classes;
            slider.style.merge_with(&computed_style);

            let action_opt = node.on_change.as_ref()
                .or_else(|| node.attributes.get("onchange"))
                .or_else(|| node.attributes.get("on_change"));

            if let Some(action_name) = action_opt {
                let clean_name = action_name.trim_end_matches("()").trim();
                if let Some(handler) = data_ctx.action_handlers.get(clean_name) {
                    let handler_cl = handler.clone();
                    slider.on_change = Some(Box::new(move |_| {
                        (handler_cl.borrow_mut())();
                    }));
                }
            }
            Box::new(slider)
        }
        "Chip" => {
            let text_val = node.text.as_deref().unwrap_or("Chip");
            let mut chip = Chip::new(text_val);

            if let Some(sel_key) = node.attributes.get("selected").and_then(|v| v.strip_prefix('$')) {
                if let Some(sig) = data_ctx.bool_signals.get(sel_key) {
                    chip.selected = Some(sig.clone());
                }
            }

            let action_opt = node.on_click.as_ref()
                .or_else(|| node.attributes.get("onclick"))
                .or_else(|| node.attributes.get("on_click"));

            if let Some(action_name) = action_opt {
                let clean_name = action_name.trim_end_matches("()").trim();
                if let Some(handler) = data_ctx.action_handlers.get(clean_name) {
                    let handler_cl = handler.clone();
                    chip.on_click = Some(Box::new(move || {
                        (handler_cl.borrow_mut())();
                    }));
                }
            }

            chip.id = node.id.clone();
            chip.classes = classes;
            chip.style.merge_with(&computed_style);
            Box::new(chip)
        }
        "ProgressBar" => {
            let prog_sig = node.attributes.get("progress")
                .and_then(|v| v.strip_prefix('$'))
                .and_then(|key| data_ctx.f32_signals.get(key).cloned())
                .unwrap_or_else(|| Signal::new(0.0));

            let min_val = node.attributes.get("min").and_then(|v| v.parse().ok()).unwrap_or(0.0);
            let max_val = node.attributes.get("max").and_then(|v| v.parse().ok()).unwrap_or_else(|| {
                if prog_sig.get_untracked() > 1.0 { 100.0 } else { 1.0 }
            });

            let mut prog = ProgressBar::new(prog_sig);
            prog.min = min_val;
            prog.max = max_val;
            prog.id = node.id.clone();
            prog.classes = classes;
            prog.style.merge_with(&computed_style);
            Box::new(prog)
        }
        "Card" => {
            let variant = match node.attributes.get("variant").map(|s| s.to_lowercase()).as_deref() {
                Some("filled") => CardVariant::Filled,
                Some("outlined") => CardVariant::Outlined,
                _ => CardVariant::Elevated,
            };

            let mut card = Card::new(variant);
            card.container.id = node.id.clone();
            card.container.classes = classes;
            card.container.style.merge_with(&computed_style);

            for child_node in &node.children {
                let child_widget = build_node(child_node, data_ctx, stylesheet);
                card.add_child(child_widget);
            }
            Box::new(card)
        }
        "TextInput" => {
            let placeholder = node.placeholder.as_deref().unwrap_or("");
            let mut input = TextInput::new(placeholder);
            let text_val = node.text.as_deref().unwrap_or("");
            if let Some(binding_key) = text_val.strip_prefix('$') {
                if let Some(sig) = data_ctx.string_signals.get(binding_key) {
                    input.value = sig.get();
                    let sig_cl = sig.clone();
                    input.on_change = Some(Box::new(move |val| {
                        sig_cl.set(val);
                    }));
                }
            } else if !text_val.is_empty() {
                input.value = text_val.to_string();
            }

            let action_opt = node.on_change.as_ref()
                .or_else(|| node.attributes.get("onchange"))
                .or_else(|| node.attributes.get("on_change"));

            if let Some(action_name) = action_opt {
                let clean_name = action_name.trim_end_matches("()").trim();
                if let Some(handler) = data_ctx.action_handlers.get(clean_name) {
                    let handler_cl = handler.clone();
                    let mut prev_on_change = input.on_change.take();
                    input.on_change = Some(Box::new(move |val| {
                        if let Some(ref mut prev) = prev_on_change {
                            prev(val);
                        }
                        (handler_cl.borrow_mut())();
                    }));
                }
            }

            input.id = node.id.clone();
            input.classes = classes;
            input.style.merge_with(&computed_style);
            Box::new(input)
        }
        "HStack" => {
            let mut hstack = HStack::new();
            hstack.id = node.id.clone();
            hstack.classes = classes;
            hstack.style.merge_with(&computed_style);

            for child_node in &node.children {
                let child_widget = build_node(child_node, data_ctx, stylesheet);
                hstack.add_child(child_widget);
            }
            Box::new(hstack)
        }
        "VStack" => {
            let mut vstack = VStack::new();
            vstack.id = node.id.clone();
            vstack.classes = classes;
            vstack.style.merge_with(&computed_style);

            for child_node in &node.children {
                let child_widget = build_node(child_node, data_ctx, stylesheet);
                vstack.add_child(child_widget);
            }
            Box::new(vstack)
        }
        "TabControl" => {
            let tab_labels = node.attributes.get("tabs")
                .cloned()
                .unwrap_or_else(|| "Base,Material You,Noctalia".into());
            let labels: Vec<String> = tab_labels.split(',').map(|s| s.trim().to_string()).collect();
            let sel = node.attributes.get("selected").and_then(|v| v.parse().ok()).unwrap_or(0);

            let mut tc = quick_widgets::tab_control::TabControl::new();
            tc.selected_index = sel;
            for label in labels {
                tc.tabs.push(quick_widgets::tab_control::TabItem::new(label));
            }

            for child_node in &node.children {
                let child_widget = build_node(child_node, data_ctx, stylesheet);
                tc.children.push(child_widget);
            }

            tc.id = node.id.clone();
            tc.classes = classes;
            tc.style.merge_with(&computed_style);
            Box::new(tc)
        }
        "ScrollViewer" => {
            let mut sv = quick_widgets::scroll_viewer::ScrollViewer::new();
            let ch = node.attributes.get("content_height")
                .and_then(|v| v.parse().ok())
                .unwrap_or(1200.0);
            sv.content_height = ch;

            for child_node in &node.children {
                let child_widget = build_node(child_node, data_ctx, stylesheet);
                sv.children.push(child_widget);
            }

            sv.id = node.id.clone();
            sv.classes = classes;
            sv.style.merge_with(&computed_style);
            Box::new(sv)
        }
        "NoctaliaButton" => {
            let text_val = node.text.as_deref().unwrap_or("Button");
            let mut btn = quick_widgets::noctalia_button::NoctaliaButton::new(text_val);
            let variant = match node.attributes.get("variant").map(|s| s.to_lowercase()).as_deref() {
                Some("secondary") => quick_widgets::noctalia_button::NoctaliaButtonVariant::Secondary,
                Some("ghost") => quick_widgets::noctalia_button::NoctaliaButtonVariant::Ghost,
                Some("outline") => quick_widgets::noctalia_button::NoctaliaButtonVariant::Outline,
                Some("danger") => quick_widgets::noctalia_button::NoctaliaButtonVariant::Danger,
                _ => quick_widgets::noctalia_button::NoctaliaButtonVariant::Primary,
            };
            btn.variant = variant;
            btn.id = node.id.clone();
            btn.classes = classes;
            btn.style.merge_with(&computed_style);

            let action_opt = node.on_click.as_ref()
                .or_else(|| node.attributes.get("onclick"))
                .or_else(|| node.attributes.get("on_click"));

            if let Some(action_name) = action_opt {
                let clean_name = action_name.trim_end_matches("()").trim();
                if let Some(handler) = data_ctx.action_handlers.get(clean_name) {
                    let handler_cl = handler.clone();
                    btn.on_click = Some(Box::new(move || {
                        (handler_cl.borrow_mut())();
                    }));
                }
            }
            Box::new(btn)
        }
        "NoctaliaCard" => {
            let mut card = quick_widgets::noctalia_card::NoctaliaCard::new();
            card.id = node.id.clone();
            card.classes = classes;
            card.style.merge_with(&computed_style);

            for child_node in &node.children {
                let child_widget = build_node(child_node, data_ctx, stylesheet);
                card.children.push(child_widget);
            }
            Box::new(card)
        }
        "NoctaliaSlider" => {
            let val = node.attributes.get("value")
                .and_then(|v| v.parse().ok())
                .unwrap_or(50.0);
            let min = node.attributes.get("min").and_then(|v| v.parse().ok()).unwrap_or(0.0);
            let max = node.attributes.get("max").and_then(|v| v.parse().ok()).unwrap_or(100.0);

            let mut slider = quick_widgets::noctalia_slider::NoctaliaSlider::new(min, max, val);
            if let Some(icon) = node.attributes.get("icon") {
                slider.icon_text = Some(icon.clone());
            }
            slider.id = node.id.clone();
            slider.classes = classes;
            slider.style.merge_with(&computed_style);
            Box::new(slider)
        }
        "CountdownRing" => {
            let prog = node.attributes.get("progress")
                .and_then(|v| v.parse().ok())
                .unwrap_or(0.65);
            let mut ring = quick_widgets::countdown_ring::CountdownRing::new(prog);
            if let Some(size) = node.attributes.get("size").and_then(|v| v.parse().ok()) {
                ring.size = size;
            }
            if let Some(txt) = node.text.as_deref() {
                ring.center_text = Some(txt.to_string());
            }
            ring.id = node.id.clone();
            ring.classes = classes;
            ring.style.merge_with(&computed_style);
            Box::new(ring)
        }
        "AnalogClock" => {
            let h = node.attributes.get("hours").and_then(|v| v.parse().ok()).unwrap_or(10);
            let m = node.attributes.get("minutes").and_then(|v| v.parse().ok()).unwrap_or(10);
            let s = node.attributes.get("seconds").and_then(|v| v.parse().ok()).unwrap_or(30);
            let mut clock = quick_widgets::analog_clock::AnalogClock::new(h, m, s);
            if let Some(tz) = node.attributes.get("timezone") {
                clock.timezone_label = Some(tz.clone());
            }
            clock.id = node.id.clone();
            clock.classes = classes;
            clock.style.merge_with(&computed_style);
            Box::new(clock)
        }
        "Segmented" => {
            let items_str = node.attributes.get("items").cloned().unwrap_or_else(|| "Option 1,Option 2,Option 3".into());
            let items: Vec<String> = items_str.split(',').map(|s| s.trim().to_string()).collect();
            let sel = node.attributes.get("selected").and_then(|v| v.parse().ok()).unwrap_or(0);
            let mut seg = quick_widgets::segmented::Segmented::new(items, sel);
            seg.id = node.id.clone();
            seg.classes = classes;
            seg.style.merge_with(&computed_style);
            Box::new(seg)
        }
        "NoctaliaGraph" => {
            let values = vec![20.0, 35.0, 50.0, 45.0, 70.0, 65.0, 85.0, 90.0];
            let mut graph = quick_widgets::noctalia_graph::NoctaliaGraph::new(values);
            if let Some(lbl) = node.attributes.get("label") {
                graph.label = Some(lbl.clone());
            }
            graph.id = node.id.clone();
            graph.classes = classes;
            graph.style.merge_with(&computed_style);
            Box::new(graph)
        }
        "NoctaliaCalendar" => {
            let mut cal = quick_widgets::noctalia_calendar::NoctaliaCalendar::new("August", 2026, 30);
            cal.id = node.id.clone();
            cal.classes = classes;
            cal.style.merge_with(&computed_style);
            Box::new(cal)
        }
        "NoctaliaColorPicker" => {
            let c = quick_core::geometry::Color::from_rgb(255, 245, 155);
            let mut picker = quick_widgets::noctalia_color_picker::NoctaliaColorPicker::new(c);
            picker.id = node.id.clone();
            picker.classes = classes;
            picker.style.merge_with(&computed_style);
            Box::new(picker)
        }
        "NoctaliaBar" => {
            let mut bar = quick_widgets::noctalia_bar::NoctaliaBar::new();
            bar.id = node.id.clone();
            bar.classes = classes;
            bar.style.merge_with(&computed_style);
            Box::new(bar)
        }
        "FramelessTitleBar" => {
            let title = node.attributes.get("title").cloned().unwrap_or_else(|| "Noctalia Window".into());
            let mut tb = quick_widgets::frameless_titlebar::FramelessTitleBar::new(title);
            if let Some(sub) = node.attributes.get("subtitle") {
                tb.subtitle = Some(sub.clone());
            }
            tb.id = node.id.clone();
            tb.classes = classes;
            tb.style.merge_with(&computed_style);
            Box::new(tb)
        }
        _ => {
            let mut container = Container::new();
            container.id = node.id.clone();
            container.classes = classes;
            container.style.merge_with(&computed_style);

            for child_node in &node.children {
                let child_widget = build_node(child_node, data_ctx, stylesheet);
                container.add_child(child_widget);
            }
            Box::new(container)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::quick_parser::parse_quick;
    use quick_core::event::{PointerButton, PointerEvent, PointerPhase};
    use quick_core::geometry::Point;

    #[test]
    fn test_builder_switch_and_slider() {
        let is_checked = Signal::new(false);
        let slider_val = Signal::new(50.0);
        let switch_toggled = Rc::new(RefCell::new(false));
        let slider_adjusted = Rc::new(RefCell::new(false));
        let chip_clicked = Rc::new(RefCell::new(false));
        let btn_clicked = Rc::new(RefCell::new(false));

        let sw_cl = switch_toggled.clone();
        let sl_cl = slider_adjusted.clone();
        let ch_cl = chip_clicked.clone();
        let bt_cl = btn_clicked.clone();

        let mut ctx = DataContext::new();
        ctx.bind_bool_signal("is_active", is_checked.clone());
        ctx.bind_f32_signal("brightness", slider_val.clone());
        ctx.bind_action("on_toggle", move || *sw_cl.borrow_mut() = true);
        ctx.bind_action("on_slide", move || *sl_cl.borrow_mut() = true);
        ctx.bind_action("on_chip", move || *ch_cl.borrow_mut() = true);
        ctx.bind_action("on_btn", move || *bt_cl.borrow_mut() = true);

        let doc = parse_quick(r#"
            <VStack theme="material-you">
                <Card variant="elevated">
                    <Switch id="sw" checked="$is_active" onchange="on_toggle" />
                    <Slider id="sl" min="0" max="100" value="$brightness" onchange="on_slide" />
                    <Chip id="ch" text="WiFi" onclick="on_chip" />
                    <ProgressBar progress="$brightness" />
                    <Button id="btn" text="Submit" onclick="on_btn" />
                </Card>
            </VStack>
        "#).unwrap();

        let (mut root, _) = build_ui_tree(&doc, &mut ctx);

        let mut engine = quick_layout::engine::LayoutEngine::new();
        let root_node = root.build_layout(&mut engine).unwrap();
        engine.compute_layout(root_node, quick_core::geometry::Size::new(600.0, 400.0)).unwrap();
        root.update_layout(&engine, Point::ZERO);

        let mut canvas = quick_render::canvas::Canvas::new();
        root.paint(&mut canvas, quick_core::geometry::Rect::new(0.0, 0.0, 600.0, 400.0));

        assert!(canvas.commands().len() >= 6);

        // Dispatch click to button
        let bounds = engine.get_layout(root_node).unwrap();
        let click_btn = quick_core::event::Event::Pointer(PointerEvent {
            position: Point::new(100.0, 150.0),
            button: Some(PointerButton::Primary),
            phase: PointerPhase::Down,
            modifiers: Default::default(),
        });
        let _ = root.handle_event(&click_btn, bounds);
    }
}

