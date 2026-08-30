use quick_style::base::base_theme;
use crate::widget::Widget;
use quick_core::event::{Event, PointerButton, PointerEvent, PointerPhase};
use quick_core::geometry::{BorderRadius, Color, Rect};
use quick_core::signals::Signal;
use quick_layout::engine::LayoutEngine;
use quick_render::canvas::Canvas;
use quick_style::property::{Dimension, Style};
use taffy::prelude::NodeId;
use taffy::TaffyError;

pub struct Switch {
    pub id: Option<String>,
    pub classes: Vec<String>,
    pub style: Style,
    pub checked: Signal<bool>,
    pub on_change: Option<Box<dyn FnMut(bool)>>,
    pub is_disabled: bool,
    pub is_hovered: bool,
    pub is_pressed: bool,
    pub is_focused: bool,
}

impl Switch {
    pub fn new(checked: Signal<bool>) -> Self {
        let mut style = Style::default();
        style.width = Some(Dimension::Px(52.0));
        style.height = Some(Dimension::Px(32.0));

        Self {
            id: None,
            classes: Vec::new(),
            style,
            checked,
            on_change: None,
            is_disabled: false,
            is_hovered: false,
            is_pressed: false,
            is_focused: false,
        }
    }

    pub fn on_change<F: FnMut(bool) + 'static>(mut self, handler: F) -> Self {
        self.on_change = Some(Box::new(handler));
        self
    }

    pub fn with_disabled(mut self, disabled: bool) -> Self {
        self.is_disabled = disabled;
        self
    }
}

impl Widget for Switch {
    fn widget_type(&self) -> &'static str {
        "Switch"
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
            computed_style.width = Some(Dimension::Px(52.0));
        }
        if computed_style.height.is_none() {
            computed_style.height = Some(Dimension::Px(32.0));
        }
        engine.new_leaf(&computed_style)
    }

    fn paint(&self, canvas: &mut Canvas, bounds: Rect) {
        let is_on = self.checked.get();
        let bt = base_theme();

        // 1. Resolve Colors
        let track_color = if is_on {
            self.style.background_color.unwrap_or(bt.colors.accent.normal)
        } else {
            bt.colors.surface_raised
        };

        let thumb_color = if is_on {
            bt.colors.accent.on_accent
        } else {
            bt.colors.text_secondary
        };

        // 2. Draw Track Pill
        let track_radius = BorderRadius::all(bounds.size.height / 2.0);
        canvas.fill_rounded_rect(bounds, track_radius, track_color);

        if !is_on {
            let outline_color = self.style.border_color.unwrap_or_else(|| Color::from_hex("#79747E").unwrap_or(Color::from_rgb(121, 116, 126)));
            canvas.stroke_rounded_rect(bounds, track_radius, outline_color, 2.0);
        }

        // 3. Compute Thumb Proportions
        let thumb_size = if self.is_pressed {
            28.0
        } else if is_on {
            24.0
        } else {
            16.0
        };

        let thumb_x = if is_on {
            bounds.origin.x + bounds.size.width - thumb_size - 4.0
        } else {
            bounds.origin.x + 8.0 - (thumb_size - 16.0) / 2.0
        };
        let thumb_y = bounds.origin.y + (bounds.size.height - thumb_size) / 2.0;
        let thumb_rect = Rect::new(thumb_x, thumb_y, thumb_size, thumb_size);

        // 4. Render State Layer Halo (Hover / Focus / Pressed)
        if (self.is_hovered || self.is_pressed || self.is_focused) && !self.is_disabled {
            let halo_size = 40.0;
            let halo_x = thumb_x + (thumb_size - halo_size) / 2.0;
            let halo_y = thumb_y + (thumb_size - halo_size) / 2.0;
            let halo_rect = Rect::new(halo_x, halo_y, halo_size, halo_size);
            let halo_alpha = if self.is_pressed || self.is_focused { 0.12 } else { 0.08 };
            let halo_color = if is_on {
                Color::from_rgba(255, 255, 255, (halo_alpha * 255.0) as u8)
            } else {
                Color::from_rgba(103, 80, 164, (halo_alpha * 255.0) as u8)
            };
            canvas.fill_rounded_rect(halo_rect, BorderRadius::all(halo_size / 2.0), halo_color);
        }

        // 5. Draw Thumb
        canvas.fill_rounded_rect(thumb_rect, BorderRadius::all(thumb_size / 2.0), thumb_color);
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
    use quick_core::geometry::Point;
    use std::cell::RefCell;
    use std::rc::Rc;

    #[test]
    fn test_switch_toggle_and_paint() {
        let is_on = Signal::new(false);
        let changed = Rc::new(RefCell::new(false));
        let changed_cl = changed.clone();

        let mut switch = Switch::new(is_on.clone())
            .on_change(move |v| *changed_cl.borrow_mut() = v);

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

        assert!(is_on.get());
        assert!(*changed.borrow());

        let mut canvas = Canvas::new();
        switch.paint(&mut canvas, bounds);
        assert!(canvas.commands().len() >= 2);
    }
}
