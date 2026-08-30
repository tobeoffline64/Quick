use crate::widget::Widget;
use quick_core::event::{Event, PointerButton, PointerEvent, PointerPhase};
use quick_core::geometry::{BorderRadius, Color, Insets, Point, Rect};
use quick_layout::engine::LayoutEngine;
use quick_render::canvas::Canvas;
use quick_style::property::{Dimension, Style};
use taffy::prelude::NodeId;
use taffy::TaffyError;

pub struct Button {
    pub id: Option<String>,
    pub classes: Vec<String>,
    pub style: Style,
    pub text: String,
    pub on_click: Option<Box<dyn FnMut()>>,
    is_hovered: bool,
    is_pressed: bool,
}

impl Button {
    pub fn new(text: impl Into<String>) -> Self {
        let mut style = Style::default();
        style.background_color = Some(Color::from_hex("#3b82f6").unwrap());
        style.text_color = Some(Color::WHITE);
        style.border_radius = Some(BorderRadius::all(6.0));
        style.padding = Some(Insets::symmetric(8.0, 16.0));
        style.font_size = Some(14.0);

        Self {
            id: None,
            classes: Vec::new(),
            style,
            text: text.into(),
            on_click: None,
            is_hovered: false,
            is_pressed: false,
        }
    }

    pub fn on_click<F: FnMut() + 'static>(mut self, handler: F) -> Self {
        self.on_click = Some(Box::new(handler));
        self
    }
}

impl Widget for Button {
    fn widget_type(&self) -> &'static str {
        "Button"
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
        let font_size = self.style.font_size.unwrap_or(14.0);
        let char_count = self.text.chars().count() as f32;
        let pad_h = self.style.padding.map(|p| p.left + p.right).unwrap_or(32.0);
        let pad_v = self.style.padding.map(|p| p.top + p.bottom).unwrap_or(16.0);

        let estimated_width = (char_count * font_size * 0.55 + pad_h).max(60.0);
        let estimated_height = font_size * 1.3 + pad_v;

        let mut computed_style = self.style.clone();
        if computed_style.width.is_none() {
            computed_style.width = Some(Dimension::Px(estimated_width));
        }
        if computed_style.height.is_none() {
            computed_style.height = Some(Dimension::Px(estimated_height));
        }

        engine.new_leaf(&computed_style)
    }

    fn paint(&self, canvas: &mut Canvas, bounds: Rect) {
        let mut bg_color = self.style.background_color.unwrap_or(Color::BLUE);
        if self.is_pressed {
            bg_color = Color::from_rgba(
                (bg_color.r as f32 * 0.7) as u8,
                (bg_color.g as f32 * 0.7) as u8,
                (bg_color.b as f32 * 0.7) as u8,
                bg_color.a,
            );
        } else if self.is_hovered {
            bg_color = Color::from_rgba(
                (bg_color.r as f32 * 1.15).min(255.0) as u8,
                (bg_color.g as f32 * 1.15).min(255.0) as u8,
                (bg_color.b as f32 * 1.15).min(255.0) as u8,
                bg_color.a,
            );
        }

        if let Some(radius) = self.style.border_radius {
            canvas.fill_rounded_rect(bounds, radius, bg_color);
        } else {
            canvas.fill_rect(bounds, bg_color);
        }

        if let (Some(border_color), Some(border_width)) =
            (self.style.border_color, self.style.border_width)
        {
            if let Some(radius) = self.style.border_radius {
                canvas.stroke_rounded_rect(bounds, radius, border_color, border_width);
            } else {
                canvas.stroke_rect(bounds, border_color, border_width);
            }
        }

        let font_size = self.style.font_size.unwrap_or(14.0);
        let text_color = self.style.text_color.unwrap_or(Color::WHITE);
        let char_count = self.text.chars().count() as f32;
        let text_w = char_count * font_size * 0.55;
        let origin_x = bounds.origin.x + ((bounds.size.width - text_w) / 2.0).max(0.0);
        let origin_y = bounds.origin.y + ((bounds.size.height + font_size * 0.8) / 2.0);

        canvas.draw_text(
            &self.text,
            Point::new(origin_x, origin_y),
            text_color,
            font_size,
            self.style.font_family.clone(),
        );
    }

    fn handle_event(&mut self, event: &Event, bounds: Rect) -> bool {
        if let Event::Pointer(PointerEvent { position, button, phase, .. }) = event {
            let inside = bounds.contains(*position);
            self.is_hovered = inside;

            match phase {
                PointerPhase::Down if inside && *button == Some(PointerButton::Primary) => {
                    self.is_pressed = true;
                    return true;
                }
                PointerPhase::Up if self.is_pressed => {
                    self.is_pressed = false;
                    if inside {
                        if let Some(ref mut handler) = self.on_click {
                            handler();
                        }
                        return true;
                    }
                }
                PointerPhase::Cancel => {
                    self.is_pressed = false;
                }
                _ => {}
            }
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    #[test]
    fn test_button_click_event() {
        let clicked = Rc::new(RefCell::new(false));
        let clicked_cl = clicked.clone();

        let mut btn = Button::new("Click").on_click(move || {
            *clicked_cl.borrow_mut() = true;
        });

        let bounds = Rect::new(0.0, 0.0, 100.0, 40.0);

        let down_event = Event::Pointer(PointerEvent {
            position: Point::new(50.0, 20.0),
            button: Some(PointerButton::Primary),
            phase: PointerPhase::Down,
            modifiers: Default::default(),
        });
        assert!(btn.handle_event(&down_event, bounds));

        let up_event = Event::Pointer(PointerEvent {
            position: Point::new(50.0, 20.0),
            button: Some(PointerButton::Primary),
            phase: PointerPhase::Up,
            modifiers: Default::default(),
        });
        assert!(btn.handle_event(&up_event, bounds));
        assert!(*clicked.borrow());
    }
}
