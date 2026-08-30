use crate::widget::Widget;
use quick_core::event::{Event, PointerButton, PointerEvent, PointerPhase};
use quick_core::geometry::{BorderRadius, Color, Point, Rect};
use quick_core::signals::Signal;
use quick_layout::engine::LayoutEngine;
use quick_render::canvas::Canvas;
use quick_style::property::{Dimension, Style};
use taffy::prelude::NodeId;
use taffy::TaffyError;

pub struct Checkbox {
    pub id: Option<String>,
    pub classes: Vec<String>,
    pub style: Style,
    pub checked: Signal<bool>,
    pub indeterminate: Option<Signal<bool>>,
    pub on_change: Option<Box<dyn FnMut(bool)>>,
    pub is_disabled: bool,
    pub is_hovered: bool,
    pub is_pressed: bool,
    pub is_focused: bool,
}

impl Checkbox {
    pub fn new(checked: Signal<bool>) -> Self {
        let mut style = Style::default();
        style.width = Some(Dimension::Px(24.0));
        style.height = Some(Dimension::Px(24.0));

        Self {
            id: None,
            classes: Vec::new(),
            style,
            checked,
            indeterminate: None,
            on_change: None,
            is_disabled: false,
            is_hovered: false,
            is_pressed: false,
            is_focused: false,
        }
    }

    pub fn with_indeterminate(mut self, indeterminate: Signal<bool>) -> Self {
        self.indeterminate = Some(indeterminate);
        self
    }

    pub fn with_disabled(mut self, disabled: bool) -> Self {
        self.is_disabled = disabled;
        self
    }

    pub fn on_change<F: FnMut(bool) + 'static>(mut self, handler: F) -> Self {
        self.on_change = Some(Box::new(handler));
        self
    }
}

impl Widget for Checkbox {
    fn widget_type(&self) -> &'static str {
        "Checkbox"
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
            computed_style.width = Some(Dimension::Px(24.0));
        }
        if computed_style.height.is_none() {
            computed_style.height = Some(Dimension::Px(24.0));
        }
        engine.new_leaf(&computed_style)
    }

    fn paint(&self, canvas: &mut Canvas, bounds: Rect) {
        let is_on = self.checked.get();
        let is_indet = self.indeterminate.as_ref().map(|s| s.get()).unwrap_or(false);

        let box_size = 20.0;
        let box_x = bounds.origin.x + (bounds.size.width - box_size) / 2.0;
        let box_y = bounds.origin.y + (bounds.size.height - box_size) / 2.0;
        let box_rect = Rect::new(box_x, box_y, box_size, box_size);
        let radius = self.style.border_radius.unwrap_or_else(|| BorderRadius::all(4.0));

        if is_indet {
            let fill_color = self.style.background_color.unwrap_or_else(|| Color::from_hex("#6750A4").unwrap_or(Color::from_rgb(103, 80, 164)));
            canvas.fill_rounded_rect(box_rect, radius, fill_color);

            let dash_y = box_y + box_size / 2.0;
            let p_start = Point::new(box_x + 4.0, dash_y);
            let p_end = Point::new(box_x + box_size - 4.0, dash_y);
            canvas.draw_line(p_start, p_end, Color::WHITE, 2.0);
        } else if is_on {
            let fill_color = self.style.background_color.unwrap_or_else(|| Color::from_hex("#6750A4").unwrap_or(Color::from_rgb(103, 80, 164)));
            canvas.fill_rounded_rect(box_rect, radius, fill_color);

            // Draw checkmark stroke
            let p1 = Point::new(box_x + 4.5, box_y + 10.0);
            let p2 = Point::new(box_x + 8.5, box_y + 14.5);
            let p3 = Point::new(box_x + 15.5, box_y + 5.5);
            canvas.draw_line(p1, p2, Color::WHITE, 2.0);
            canvas.draw_line(p2, p3, Color::WHITE, 2.0);
        } else {
            let border_color = self.style.border_color.unwrap_or_else(|| Color::from_hex("#79747E").unwrap_or(Color::from_rgb(121, 116, 126)));
            canvas.stroke_rounded_rect(box_rect, radius, border_color, 2.0);
        }
    }

    fn handle_event(&mut self, event: &Event, bounds: Rect) -> bool {
        if self.is_disabled {
            return false;
        }

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
                    if inside && *button == Some(PointerButton::Primary) {
                        let new_state = !self.checked.get();
                        self.checked.set(new_state);
                        if let Some(ref mut handler) = self.on_change {
                            handler(new_state);
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
    fn test_checkbox_toggle_and_event() {
        let is_checked = Signal::new(false);
        let changed = Rc::new(RefCell::new(false));
        let changed_cl = changed.clone();

        let mut cb = Checkbox::new(is_checked.clone())
            .on_change(move |v| *changed_cl.borrow_mut() = v);

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

        assert!(is_checked.get());
        assert!(*changed.borrow());

        let mut canvas = Canvas::new();
        cb.paint(&mut canvas, bounds);
        assert!(!canvas.commands().is_empty());
    }

    #[test]
    fn test_checkbox_indeterminate() {
        let is_checked = Signal::new(false);
        let is_indet = Signal::new(true);
        let cb = Checkbox::new(is_checked).with_indeterminate(is_indet);

        let bounds = Rect::new(0.0, 0.0, 24.0, 24.0);
        let mut canvas = Canvas::new();
        cb.paint(&mut canvas, bounds);
        assert_eq!(canvas.commands().len(), 2); // fill + dash line
    }
}
