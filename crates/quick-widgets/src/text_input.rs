use quick_style::base::base_theme;
use crate::widget::Widget;
use quick_core::event::{Event, FocusEvent, KeyEvent, KeyState, PointerButton, PointerEvent, PointerPhase};
use quick_core::geometry::{BorderRadius, Color, Insets, Point, Rect};
use quick_layout::engine::LayoutEngine;
use quick_render::canvas::Canvas;
use quick_style::property::{Dimension, Style};
use serde::{Deserialize, Serialize};
use taffy::prelude::NodeId;
use taffy::TaffyError;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum InputVariant {
    #[default]
    Filled,
    Outlined,
}

pub struct TextInput {
    pub id: Option<String>,
    pub classes: Vec<String>,
    pub style: Style,
    pub value: String,
    pub placeholder: String,
    pub variant: InputVariant,
    pub on_change: Option<Box<dyn FnMut(String)>>,
    pub is_disabled: bool,
    pub is_focused: bool,
    pub cursor_pos: usize,
    cursor_initialized: bool,
}

impl TextInput {
    pub fn new(placeholder: impl Into<String>) -> Self {
        let mut style = Style::default();
        style.background_color = Some(Color::from_hex("#1e1e2e").unwrap_or(Color::from_rgb(30, 30, 46)));
        style.text_color = Some(Color::WHITE);
        style.border_color = Some(Color::from_hex("#45475a").unwrap_or(Color::from_rgb(69, 71, 90)));
        style.border_width = Some(1.0);
        style.border_radius = Some(BorderRadius::all(4.0));
        style.padding = Some(Insets::symmetric(6.0, 10.0));
        style.font_size = Some(quick_style::base::TypeScale::INPUT);

        Self {
            id: None,
            classes: Vec::new(),
            style,
            value: String::new(),
            placeholder: placeholder.into(),
            variant: InputVariant::Filled,
            on_change: None,
            is_disabled: false,
            is_focused: false,
            cursor_pos: 0,
            cursor_initialized: false,
        }
    }

    pub fn with_variant(mut self, variant: InputVariant) -> Self {
        self.variant = variant;
        if variant == InputVariant::Outlined {
            self.style.background_color = Some(Color::TRANSPARENT);
            self.style.border_color = Some(Color::from_hex("#79747E").unwrap_or(Color::from_rgb(121, 116, 126)));
        }
        self
    }

    pub fn with_placeholder(mut self, placeholder: impl Into<String>) -> Self {
        self.placeholder = placeholder.into();
        self
    }

    pub fn with_value(mut self, value: impl Into<String>) -> Self {
        self.value = value.into();
        self.cursor_pos = self.value.chars().count();
        self.cursor_initialized = true;
        self
    }

    pub fn with_disabled(mut self, disabled: bool) -> Self {
        self.is_disabled = disabled;
        self
    }

    pub fn on_change<F: FnMut(String) + 'static>(mut self, handler: F) -> Self {
        self.on_change = Some(Box::new(handler));
        self
    }

    fn clamp_cursor(&mut self) {
        let char_count = self.value.chars().count();
        if self.cursor_pos > char_count {
            self.cursor_pos = char_count;
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
            computed_style.height = Some(Dimension::Px(35.0));
        }
        engine.new_leaf(&computed_style)
    }

    fn paint(&self, canvas: &mut Canvas, bounds: Rect) {
        let radius = self.style.border_radius.unwrap_or_else(|| BorderRadius::all(4.0));

        let bt = base_theme();
        // 1. Container Background
        let bg_color = if self.variant == InputVariant::Outlined {
            self.style.background_color.unwrap_or(Color::TRANSPARENT)
        } else {
            self.style.background_color.unwrap_or(bt.colors.surface)
        };
        canvas.fill_rounded_rect(bounds, radius, bg_color);

        // 2. Border Stroke (1.0px unfocused, 2.0px focused active indicator)
        let (border_color, border_width) = if self.is_focused {
            (bt.colors.accent.normal, 2.0)
        } else {
            (
                self.style.border_color.unwrap_or(bt.colors.border),
                self.style.border_width.unwrap_or(1.0),
            )
        };
        canvas.stroke_rounded_rect(bounds, radius, border_color, border_width);

        // 3. Text & Placeholder Rendering
        let font_size = self.style.font_size.unwrap_or(quick_style::base::TypeScale::INPUT);
        let pad_left = self.style.padding.map(|p| p.left).unwrap_or(8.0);
        let origin_x = bounds.origin.x + pad_left;
        let origin_y = bounds.origin.y + ((bounds.size.height + font_size * 0.8) / 2.0);

        if self.value.is_empty() && !self.placeholder.is_empty() {
            let placeholder_color = Color::from_rgba(150, 150, 150, 180);
            canvas.draw_text(
                &self.placeholder,
                Point::new(origin_x, origin_y),
                placeholder_color,
                font_size,
                self.style.font_family.clone(),
            );
        } else {
            let text_color = self.style.text_color.unwrap_or(Color::WHITE);
            canvas.draw_text(
                &self.value,
                Point::new(origin_x, origin_y),
                text_color,
                font_size,
                self.style.font_family.clone(),
            );
        }

        // 4. Cursor Rendering (Active Focus)
        if self.is_focused {
            let char_count = self.value.chars().take(self.cursor_pos).count() as f32;
            let cursor_x = origin_x + (char_count * font_size * 0.55);
            let cursor_top = bounds.origin.y + (bounds.size.height - font_size * 1.2) / 2.0;
            let cursor_bottom = cursor_top + font_size * 1.2;
            let cursor_color = Color::from_hex("#89b4fa").unwrap_or(Color::WHITE);

            canvas.draw_line(
                Point::new(cursor_x, cursor_top),
                Point::new(cursor_x, cursor_bottom),
                cursor_color,
                1.5,
            );
        }
    }

    fn handle_event(&mut self, event: &Event, bounds: Rect) -> bool {
        if self.is_disabled {
            return false;
        }

        match event {
            Event::Pointer(PointerEvent { position, button, phase, .. }) => {
                if *phase == PointerPhase::Down && *button == Some(PointerButton::Primary) {
                    if bounds.contains(*position) {
                        self.is_focused = true;
                        // Calculate cursor position from click X coordinate
                        let pad_left = self.style.padding.map(|p| p.left).unwrap_or(8.0);
                        let font_size = self.style.font_size.unwrap_or(quick_style::base::TypeScale::INPUT);
                        let char_width = font_size * 0.55;
                        let relative_x = (position.x - (bounds.origin.x + pad_left)).max(0.0);
                        let clicked_idx = (relative_x / char_width).round() as usize;
                        let total_chars = self.value.chars().count();
                        self.cursor_pos = clicked_idx.min(total_chars);
                        self.cursor_initialized = true;
                        return true;
                    } else {
                        self.is_focused = false;
                        return false;
                    }
                }
            }
            Event::Key(KeyEvent { key, state, text, .. }) if self.is_focused && *state == KeyState::Pressed => {
                if !self.cursor_initialized {
                    self.cursor_pos = self.value.chars().count();
                    self.cursor_initialized = true;
                }
                let mut chars: Vec<char> = self.value.chars().collect();
                self.clamp_cursor();

                match key.as_str() {
                    "Left" | "ArrowLeft" => {
                        self.cursor_pos = self.cursor_pos.saturating_sub(1);
                        return true;
                    }
                    "Right" | "ArrowRight" => {
                        self.cursor_pos = (self.cursor_pos + 1).min(chars.len());
                        return true;
                    }
                    "Home" => {
                        self.cursor_pos = 0;
                        return true;
                    }
                    "End" => {
                        self.cursor_pos = chars.len();
                        return true;
                    }
                    "Backspace" => {
                        if self.cursor_pos > 0 && !chars.is_empty() {
                            chars.remove(self.cursor_pos - 1);
                            self.cursor_pos -= 1;
                            self.value = chars.into_iter().collect();
                            if let Some(ref mut handler) = self.on_change {
                                handler(self.value.clone());
                            }
                        }
                        return true;
                    }
                    "Delete" => {
                        if self.cursor_pos < chars.len() {
                            chars.remove(self.cursor_pos);
                            self.value = chars.into_iter().collect();
                            if let Some(ref mut handler) = self.on_change {
                                handler(self.value.clone());
                            }
                        }
                        return true;
                    }
                    "Space" if text.is_none() => {
                        chars.insert(self.cursor_pos, ' ');
                        self.cursor_pos += 1;
                        self.value = chars.into_iter().collect();
                        if let Some(ref mut handler) = self.on_change {
                            handler(self.value.clone());
                        }
                        return true;
                    }
                    _ => {
                        if let Some(ref ch_str) = text {
                            let insert_chars: Vec<char> = ch_str.chars().filter(|c| !c.is_control()).collect();
                            if !insert_chars.is_empty() {
                                let insert_count = insert_chars.len();
                                for (i, c) in insert_chars.into_iter().enumerate() {
                                    chars.insert(self.cursor_pos + i, c);
                                }
                                self.cursor_pos += insert_count;
                                self.value = chars.into_iter().collect();
                                if let Some(ref mut handler) = self.on_change {
                                    handler(self.value.clone());
                                }
                                return true;
                            }
                        }
                    }
                }
            }
            Event::Focus(FocusEvent::Lost) => {
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

        // Move to start, then Delete
        let home = Event::Key(KeyEvent {
            key: "Home".to_string(),
            state: KeyState::Pressed,
            text: None,
            modifiers: Default::default(),
        });
        assert!(input.handle_event(&home, bounds));

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

    #[test]
    fn test_text_input_cursor_navigation_and_insert() {
        let mut input = TextInput::new("Placeholder");
        input.is_focused = true;
        let bounds = Rect::new(0.0, 0.0, 200.0, 35.0);

        for c in ["H", "e", "l", "l", "o"] {
            input.handle_event(&Event::Key(KeyEvent {
                key: c.to_string(),
                state: KeyState::Pressed,
                text: Some(c.to_string()),
                modifiers: Default::default(),
            }), bounds);
        }
        assert_eq!(input.value, "Hello");
        assert_eq!(input.cursor_pos, 5);

        // Move cursor left twice
        for _ in 0..2 {
            input.handle_event(&Event::Key(KeyEvent {
                key: "ArrowLeft".to_string(),
                state: KeyState::Pressed,
                text: None,
                modifiers: Default::default(),
            }), bounds);
        }
        assert_eq!(input.cursor_pos, 3);

        // Insert 'p' -> "Helplo"
        input.handle_event(&Event::Key(KeyEvent {
            key: "p".to_string(),
            state: KeyState::Pressed,
            text: Some("p".to_string()),
            modifiers: Default::default(),
        }), bounds);
        assert_eq!(input.value, "Helplo");
        assert_eq!(input.cursor_pos, 4);
    }
}
