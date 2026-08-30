use crate::schema::{UiDocument, UiNode};
use quick_core::signals::Signal;
use quick_style::parser::{parse_inline_style, parse_stylesheet};
use quick_style::rule::StyleSheet;
use quick_widgets::button::Button;
use quick_widgets::container::Container;
use quick_widgets::stack::{HStack, VStack};
use quick_widgets::text::Text;
use quick_widgets::text_input::TextInput;
use quick_widgets::widget::Widget;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

#[derive(Default, Clone)]
pub struct DataContext {
    pub string_signals: HashMap<String, Signal<String>>,
    pub action_handlers: HashMap<String, Rc<RefCell<dyn FnMut()>>>,
}

impl DataContext {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn bind_signal(&mut self, name: impl Into<String>, signal: Signal<String>) {
        self.string_signals.insert(name.into(), signal);
    }

    pub fn bind_action<F: FnMut() + 'static>(&mut self, name: impl Into<String>, handler: F) {
        self.action_handlers.insert(name.into(), Rc::new(RefCell::new(handler)));
    }
}

pub fn build_ui_tree(
    doc: &UiDocument,
    data_ctx: &mut DataContext,
) -> (Box<dyn Widget>, StyleSheet) {
    let stylesheet = if let Some(ref css) = doc.styles {
        parse_stylesheet(css)
    } else {
        StyleSheet::new()
    };

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

    let mut computed_style = stylesheet.resolve(
        &node.element,
        &class_refs,
        node.id.as_deref(),
        None,
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

            if let Some(ref action_name) = node.on_click {
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

            if let Some(ref action_name) = node.on_change {
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
                hstack.children.push(child_widget);
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
                vstack.children.push(child_widget);
            }
            Box::new(vstack)
        }
        _ => {
            let mut container = Container::new();
            container.id = node.id.clone();
            container.classes = classes;
            container.style.merge_with(&computed_style);

            for child_node in &node.children {
                let child_widget = build_node(child_node, data_ctx, stylesheet);
                container.children.push(child_widget);
            }
            Box::new(container)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::quick_parser::parse_quick;
    use quick_core::event::{KeyEvent, KeyState, PointerButton, PointerEvent, PointerPhase};
    use quick_core::geometry::{Point, Rect};

    #[test]
    fn test_builder_shared_action_and_signal_binding() {
        let count = Signal::new(0);
        let count_sig = count.clone();

        let mut ctx = DataContext::new();
        let greeting = quick_core::signals::create_computed(move || {
            format!("Clicks: {}", count_sig.get())
        });
        ctx.bind_signal("greeting", greeting.clone());

        let count_inc = count.clone();
        ctx.bind_action("increment", move || {
            count_inc.update(|v| *v += 1);
        });

        let doc = parse_quick(r#"
            <VStack>
                <Text id="label" text="$greeting" />
                <Button id="btn1" text="Inc 1" onclick="increment" />
                <Button id="btn2" text="Inc 2" onclick="increment" />
            </VStack>
        "#).unwrap();

        let (mut root, _) = build_ui_tree(&doc, &mut ctx);

        let mut engine = quick_layout::engine::LayoutEngine::new();
        let root_node = root.build_layout(&mut engine).unwrap();
        engine.compute_layout(root_node, quick_core::geometry::Size::new(400.0, 300.0)).unwrap();
        root.update_layout(&engine, Point::ZERO);

        assert_eq!(greeting.get(), "Clicks: 0");

        // Simulate click event on btn1
        let click_event_down = quick_core::event::Event::Pointer(PointerEvent {
            position: Point::new(20.0, 40.0),
            button: Some(PointerButton::Primary),
            phase: PointerPhase::Down,
            modifiers: Default::default(),
        });
        let click_event_up = quick_core::event::Event::Pointer(PointerEvent {
            position: Point::new(20.0, 40.0),
            button: Some(PointerButton::Primary),
            phase: PointerPhase::Up,
            modifiers: Default::default(),
        });

        let bounds = Rect::new(0.0, 0.0, 400.0, 300.0);
        root.handle_event(&click_event_down, bounds);
        root.handle_event(&click_event_up, bounds);

        assert_eq!(greeting.get(), "Clicks: 1");
    }

    #[test]
    fn test_builder_text_input_signal_binding() {
        let username = Signal::new("user1".to_string());
        let mut ctx = DataContext::new();
        ctx.bind_signal("username", username.clone());

        let doc = parse_quick(r#"
            <VStack>
                <TextInput id="input-user" text="$username" placeholder="Enter username" />
            </VStack>
        "#).unwrap();

        let (mut root, _) = build_ui_tree(&doc, &mut ctx);

        let mut engine = quick_layout::engine::LayoutEngine::new();
        let root_node = root.build_layout(&mut engine).unwrap();
        engine.compute_layout(root_node, quick_core::geometry::Size::new(400.0, 300.0)).unwrap();
        root.update_layout(&engine, Point::ZERO);

        let bounds = Rect::new(0.0, 0.0, 400.0, 300.0);

        // Click to focus TextInput
        let click = quick_core::event::Event::Pointer(PointerEvent {
            position: Point::new(10.0, 10.0),
            button: Some(PointerButton::Primary),
            phase: PointerPhase::Down,
            modifiers: Default::default(),
        });
        root.handle_event(&click, bounds);

        // Type '2'
        let key = quick_core::event::Event::Key(KeyEvent {
            key: "2".to_string(),
            state: KeyState::Pressed,
            text: Some("2".to_string()),
            modifiers: Default::default(),
        });
        root.handle_event(&key, bounds);

        assert_eq!(username.get(), "user12");
    }

    #[test]
    fn test_builder_action_parentheses_and_on_change() {
        let changed = Rc::new(RefCell::new(false));
        let changed_cl = changed.clone();

        let clicked = Rc::new(RefCell::new(false));
        let clicked_cl = clicked.clone();

        let mut ctx = DataContext::new();
        ctx.bind_action("on_change_handler", move || {
            *changed_cl.borrow_mut() = true;
        });
        ctx.bind_action("click_action", move || {
            *clicked_cl.borrow_mut() = true;
        });

        let doc = parse_quick(r#"
            <VStack>
                <TextInput onchange="on_change_handler()" />
                <Button onclick="click_action()" text="Go" />
            </VStack>
        "#).unwrap();

        let (mut root, _) = build_ui_tree(&doc, &mut ctx);

        let mut engine = quick_layout::engine::LayoutEngine::new();
        let root_node = root.build_layout(&mut engine).unwrap();
        engine.compute_layout(root_node, quick_core::geometry::Size::new(400.0, 300.0)).unwrap();
        root.update_layout(&engine, Point::ZERO);

        let bounds = Rect::new(0.0, 0.0, 400.0, 300.0);

        // Type in text input
        let click_input = quick_core::event::Event::Pointer(PointerEvent {
            position: Point::new(10.0, 10.0),
            button: Some(PointerButton::Primary),
            phase: PointerPhase::Down,
            modifiers: Default::default(),
        });
        root.handle_event(&click_input, bounds);

        let key = quick_core::event::Event::Key(KeyEvent {
            key: "a".to_string(),
            state: KeyState::Pressed,
            text: Some("a".to_string()),
            modifiers: Default::default(),
        });
        root.handle_event(&key, bounds);
        assert!(*changed.borrow());

        // Click button
        let click_btn = quick_core::event::Event::Pointer(PointerEvent {
            position: Point::new(10.0, 50.0),
            button: Some(PointerButton::Primary),
            phase: PointerPhase::Down,
            modifiers: Default::default(),
        });
        root.handle_event(&click_btn, bounds);
        let click_btn_up = quick_core::event::Event::Pointer(PointerEvent {
            position: Point::new(10.0, 50.0),
            button: Some(PointerButton::Primary),
            phase: PointerPhase::Up,
            modifiers: Default::default(),
        });
        root.handle_event(&click_btn_up, bounds);
        assert!(*clicked.borrow());
    }
}
