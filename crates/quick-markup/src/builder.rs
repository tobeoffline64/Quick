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
use std::collections::HashMap;

#[derive(Default)]
pub struct DataContext {
    pub string_signals: HashMap<String, Signal<String>>,
    pub action_handlers: HashMap<String, Box<dyn FnMut()>>,
}

impl DataContext {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn bind_signal(&mut self, name: impl Into<String>, signal: Signal<String>) {
        self.string_signals.insert(name.into(), signal);
    }

    pub fn bind_action<F: FnMut() + 'static>(&mut self, name: impl Into<String>, handler: F) {
        self.action_handlers.insert(name.into(), Box::new(handler));
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
                if let Some(handler) = data_ctx.action_handlers.remove(action_name) {
                    btn.on_click = Some(handler);
                }
            }
            Box::new(btn)
        }
        "TextInput" => {
            let placeholder = node.placeholder.as_deref().unwrap_or("");
            let mut input = TextInput::new(placeholder);
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
