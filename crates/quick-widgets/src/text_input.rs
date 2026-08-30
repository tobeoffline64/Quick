use crate::widget::Widget;
use quick_core::event::{Event, KeyState, PointerButton, PointerEvent, PointerPhase};
use quick_core::geometry::{BorderRadius, Color, Insets, Point, Rect};
use quick_layout::engine::LayoutEngine;
use quick_render::canvas::Canvas;
use quick_style::property::{Dimension, Style};
use taffy::prelude::NodeId;
use taffy::TaffyError;

pub struct TextInput {
    pub id: Option<String>,
    pub classes: Vec<String>,
    pub style: Style,
    pub value: String,
    pub placeholder: String,
    pub on_change: Option<Box<dyn FnMut(String)>>,
    pub is_focused: bool,
    pub cursor_pos: usize,
}

impl TextInput {
    pub fn new(placeholder: impl Into<String>) -> Self {
        let mut style = Style::default();
        style.background_color = Some(Color::from_hex("#1e1e2e").unwrap());
        style.text_color = Some(Color::WHITE);
        style.border_color = Some(Color::from_hex("#45475a").unwrap());
        style.border_width = Some(1.0);
        style.border_radius = Some(BorderRadius::all(4.0));
        style.padding = Some(Insets::symmetric(6.0, 10.0));
        style.font_size = Some(14.0);

        Self {
            id: None,
            classes: Vec::new(),
            style,
            value: String::new(),
            placeholder: placeholder.into(),
            on_change: None,
            is_focused: false,
            cursor_pos: 0,
        }
    }
}

impl Widget for TextInput {
    fn widget_type(&self) -> &'static str {
        "TextInput"
    }

    fn id(&self) -> Option<&str> {
        self.id.as_deref()
    }

    fn classes(&self) -> &[String] {
        &self.classes
    }

    fn style(&self) -> &Style {
        &self.style
    }

    fn style_mut(&mut self) -> &mut Style {
        &mut self.style
    }

    fn build_layout(&mut self, engine: &mut LayoutEngine) -> Result<NodeId, TaffyError> {
        let mut computed_style = self.style.clone();
        if computed_style.width.is_none() {
            computed_style.width = Some(Dimension::Px(180.0));
        }
        if computed_style.height.is_none() {
            computed_style.height = Some(Dimension::Px(34.0));
        }
        engine.new_leaf(&computed_style)
    }

    fn paint(&self, canvas: &mut Canvas, bounds: Rect) {
        let bg = self.style.background_color.unwrap_or(Color::BLACK);
        if let Some(radius) = self.style.border_radius {
            canvas.fill_rounded_rect(bounds, radius, bg);
        } else {
            canvas.fill_rect(bounds, bg);
        }

        let border_color = if self.is_focused {
            Color::from_hex("#89b4fa").unwrap_or(Color::BLUE)
        } else {
            self.style.border_color.unwrap_or(Color::BLACK)
        };

        if let Some(radius) = self.style.border_radius {
            canvas.stroke_rounded_rect(bounds, radius, border_color, 1.5);
        } else {
            canvas.stroke_rect(bounds, border_color, 1.5);
        }

        let font_size = self.style.font_size.unwrap_or(14.0);
        let origin = Point::new(
            bounds.origin.x + 8.0,
            bounds.origin.y + ((bounds.size.height + font_size * 0.8) / 2.0),
        );

        if self.value.is_empty() && !self.placeholder.is_empty() {
            let placeholder_color = Color::from_rgba(150, 150, 150, 180);
            canvas.draw_text(
                &self.placeholder,
                origin,
                placeholder_color,
                font_size,
                self.style.font_family.clone(),
            );
        } else {
            let text_color = self.style.text_color.unwrap_or(Color::WHITE);
            canvas.draw_text(
                &self.value,
                origin,
                text_color,
                font_size,
                self.style.font_family.clone(),
            );
        }
    }

    fn handle_event(&mut self, event: &Event, bounds: Rect) -> bool {
        match event {
            Event::Pointer(PointerEvent { position, button, phase, .. }) => {
                if *phase == PointerPhase::Down && *button == Some(PointerButton::Primary) {
                    self.is_focused = bounds.contains(*position);
                    return self.is_focused;
                }
            }
            Event::Key(key_event) if self.is_focused && key_event.state == KeyState::Pressed => {
                if key_event.key == "Backspace" || key_event.key == "Delete" {
                    if !self.value.is_empty() {
                        self.value.pop();
                        if let Some(ref mut handler) = self.on_change {
                            handler(self.value.clone());
                        }
                    }
                    return true;
                } else if key_event.key == "Space" && key_event.text.is_none() {
                    self.value.push(' ');
                    if let Some(ref mut handler) = self.on_change {
                        handler(self.value.clone());
                    }
                    return true;
                } else if let Some(ref ch) = key_event.text {
                    if !ch.chars().all(|c| c.is_control()) {
                        self.value.push_str(ch);
                        if let Some(ref mut handler) = self.on_change {
                            handler(self.value.clone());
                        }
                    }
                    return true;
                }
            }
            Event::Focus(quick_core::event::FocusEvent::Lost) => {
                self.is_focused = false;
            }
            _ => {}
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use quick_core::event::KeyEvent;
    use std::cell::RefCell;
    use std::rc::Rc;

    #[test]
    fn test_text_input_typing_and_backspace() {
        let changed_val = Rc::new(RefCell::new(String::new()));
        let changed_cl = changed_val.clone();

        let mut input = TextInput::new("Placeholder");
        input.on_change = Some(Box::new(move |val| {
            *changed_cl.borrow_mut() = val;
        }));

        let bounds = Rect::new(0.0, 0.0, 200.0, 35.0);

        // Click to focus
        let click = Event::Pointer(PointerEvent {
            position: Point::new(10.0, 10.0),
            button: Some(PointerButton::Primary),
            phase: PointerPhase::Down,
            modifiers: Default::default(),
        });
        assert!(input.handle_event(&click, bounds));
        assert!(input.is_focused);

        // Type 'H'
        let key_h = Event::Key(KeyEvent {
            key: "h".to_string(),
            state: KeyState::Pressed,
            text: Some("H".to_string()),
            modifiers: Default::default(),
        });
        assert!(input.handle_event(&key_h, bounds));
        assert_eq!(input.value, "H");
        assert_eq!(*changed_val.borrow(), "H");

        // Type 'i'
        let key_i = Event::Key(KeyEvent {
            key: "i".to_string(),
            state: KeyState::Pressed,
            text: Some("i".to_string()),
            modifiers: Default::default(),
        });
        assert!(input.handle_event(&key_i, bounds));
        assert_eq!(input.value, "Hi");

        // Backspace
        let backspace = Event::Key(KeyEvent {
            key: "Backspace".to_string(),
            state: KeyState::Pressed,
            text: None,
            modifiers: Default::default(),
        });
        assert!(input.handle_event(&backspace, bounds));
        assert_eq!(input.value, "H");
        assert_eq!(*changed_val.borrow(), "H");

        // Delete
        let delete = Event::Key(KeyEvent {
            key: "Delete".to_string(),
            state: KeyState::Pressed,
            text: None,
            modifiers: Default::default(),
        });
        assert!(input.handle_event(&delete, bounds));
        assert_eq!(input.value, "");
        assert_eq!(*changed_val.borrow(), "");
    }
}
