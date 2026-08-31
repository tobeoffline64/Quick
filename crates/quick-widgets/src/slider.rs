use quick_style::base::base_theme;
use crate::widget::Widget;
use quick_core::event::{Event, PointerButton, PointerEvent, PointerPhase};
use quick_core::geometry::{BorderRadius, Rect};
use quick_core::signals::Signal;
use quick_layout::engine::LayoutEngine;
use quick_render::canvas::Canvas;
use quick_style::property::{Dimension, Style};
use taffy::prelude::NodeId;
use taffy::TaffyError;

pub struct Slider {
    pub id: Option<String>,
    pub classes: Vec<String>,
    pub style: Style,
    pub value: Signal<f32>,
    pub min: f32,
    pub max: f32,
    pub steps: Option<u32>,
    pub on_change: Option<Box<dyn FnMut(f32)>>,
    pub is_disabled: bool,
    pub is_dragging: bool,
    pub is_hovered: bool,
}

impl Slider {
    pub fn new(value: Signal<f32>, min: f32, max: f32) -> Self {
        let mut style = Style::default();
        style.height = Some(Dimension::Px(36.0));

        Self {
            id: None,
            classes: Vec::new(),
            style,
            value,
            min,
            max,
            steps: None,
            on_change: None,
            is_disabled: false,
            is_dragging: false,
            is_hovered: false,
        }
    }

    pub fn with_steps(mut self, steps: Option<u32>) -> Self {
        self.steps = steps;
        self
    }

    pub fn with_disabled(mut self, disabled: bool) -> Self {
        self.is_disabled = disabled;
        self
    }

    pub fn on_change<F: FnMut(f32) + 'static>(mut self, handler: F) -> Self {
        self.on_change = Some(Box::new(handler));
        self
    }

    fn update_from_pos(&mut self, pos_x: f32, bounds: Rect) {
        let pad = 12.0;
        let track_left = bounds.origin.x + pad;
        let track_width = (bounds.size.width - pad * 2.0).max(1.0);
        let ratio = ((pos_x - track_left) / track_width).clamp(0.0, 1.0);
        let (min_val, max_val) = if self.min <= self.max {
            (self.min, self.max)
        } else {
            (self.max, self.min)
        };
        let mut new_val = min_val + ratio * (max_val - min_val);

        // Discrete step quantization
        if let Some(steps) = self.steps {
            if steps > 0 {
                let step_size = (max_val - min_val) / steps as f32;
                if step_size > 0.0 {
                    new_val = min_val + ((new_val - min_val) / step_size).round() * step_size;
                }
            }
        }

        new_val = new_val.clamp(min_val, max_val);
        self.value.set(new_val);
        if let Some(ref mut handler) = self.on_change {
            handler(new_val);
        }
    }
}

impl Widget for Slider {
    fn widget_type(&self) -> &'static str {
        "Slider"
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
        if computed_style.height.is_none() {
            computed_style.height = Some(Dimension::Px(36.0));
        }
        engine.new_leaf(&computed_style)
    }

    fn paint(&self, canvas: &mut Canvas, bounds: Rect) {
        if bounds.size.width <= 0.0 || bounds.size.height <= 0.0 {
            return;
        }

        let (min_val, max_val) = if self.min <= self.max {
            (self.min, self.max)
        } else {
            (self.max, self.min)
        };
        let raw_val = self.value.get();
        let val = if raw_val.is_nan() { min_val } else { raw_val.clamp(min_val, max_val) };
        let ratio = if (max_val - min_val).abs() > 0.001 {
            ((val - min_val) / (max_val - min_val)).clamp(0.0, 1.0)
        } else {
            0.0
        };

        let pad = 12.0;
        let track_h = 8.0;
        let track_y = bounds.origin.y + (bounds.size.height - track_h) / 2.0;
        let track_w = (bounds.size.width - pad * 2.0).max(0.0);
        let track_radius = BorderRadius::all(track_h / 2.0);

        let bt = base_theme();
        // 1. Inactive track (full width background)
        let inactive_rect = Rect::new(bounds.origin.x + pad, track_y, track_w, track_h);
        canvas.fill_rounded_rect(inactive_rect, track_radius, bt.colors.surface_raised);

        // 2. Active track (left side up to thumb)
        let active_w = track_w * ratio;
        if active_w > 0.0 {
            let active_rect = Rect::new(bounds.origin.x + pad, track_y, active_w, track_h);
            let active_color = self.style.background_color.unwrap_or(bt.colors.accent.normal);
            canvas.fill_rounded_rect(active_rect, track_radius, active_color);
        }

        // 3. Thumb position
        let thumb_r = 10.0;
        let thumb_x = bounds.origin.x + pad + active_w;
        let thumb_y = bounds.origin.y + bounds.size.height / 2.0;
        let thumb_rect = Rect::new(thumb_x - thumb_r, thumb_y - thumb_r, thumb_r * 2.0, thumb_r * 2.0);

        // 4. State Layer Halo (Hover / Dragged)
        if (self.is_dragging || self.is_hovered) && !self.is_disabled {
            let halo_size = 32.0;
            let halo_rect = Rect::new(thumb_x - halo_size / 2.0, thumb_y - halo_size / 2.0, halo_size, halo_size);
            let halo_color = bt.colors.hover_overlay;
            canvas.fill_rounded_rect(halo_rect, BorderRadius::all(halo_size / 2.0), halo_color);
        }

        // 5. Thumb Circle
        let thumb_color = bt.colors.accent.normal;
        canvas.fill_rounded_rect(thumb_rect, BorderRadius::all(thumb_r), thumb_color);
    }

    fn handle_event(&mut self, event: &Event, bounds: Rect) -> bool {
        if self.is_disabled {
            return false;
        }

        match event {
            Event::Pointer(PointerEvent { position, button, phase, .. }) => {
                let inside = bounds.contains(*position);
                let prev_hover = self.is_hovered;
                self.is_hovered = inside;

                match phase {
                    PointerPhase::Down if inside && *button == Some(PointerButton::Primary) => {
                        self.is_dragging = true;
                        self.update_from_pos(position.x, bounds);
                        true
                    }
                    PointerPhase::Moved if self.is_dragging => {
                        self.update_from_pos(position.x, bounds);
                        true
                    }
                    PointerPhase::Moved => {
                        prev_hover != self.is_hovered
                    }
                    PointerPhase::Up if self.is_dragging => {
                        self.is_dragging = false;
                        self.update_from_pos(position.x, bounds);
                        true
                    }
                    PointerPhase::Cancel => {
                        if self.is_dragging {
                            self.is_dragging = false;
                            true
                        } else {
                            false
                        }
                    }
                    _ => false,
                }
            }
            _ => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use quick_core::geometry::Point;
    use std::cell::RefCell;
    use std::rc::Rc;

    #[test]
    fn test_slider_drag_and_value_change() {
        let val_sig = Signal::new(0.0);
        let callback_val = Rc::new(RefCell::new(0.0));
        let cb_cl = callback_val.clone();

        let mut slider = Slider::new(val_sig.clone(), 0.0, 100.0)
            .on_change(move |v| *cb_cl.borrow_mut() = v);

        let bounds = Rect::new(0.0, 0.0, 124.0, 36.0);

        // Down at 50% (x = 62.0)
        let down = Event::Pointer(PointerEvent {
            position: Point::new(62.0, 18.0),
            button: Some(PointerButton::Primary),
            phase: PointerPhase::Down,
            modifiers: Default::default(),
        });
        assert!(slider.handle_event(&down, bounds));
        assert!((val_sig.get() - 50.0).abs() < 0.1);
        assert!((*callback_val.borrow() - 50.0).abs() < 0.1);

        // Move to 75% (x = 87.0)
        let drag = Event::Pointer(PointerEvent {
            position: Point::new(87.0, 18.0),
            button: None,
            phase: PointerPhase::Moved,
            modifiers: Default::default(),
        });
        assert!(slider.handle_event(&drag, bounds));
        assert!((val_sig.get() - 75.0).abs() < 0.1);

        // Up at 100% (x = 112.0)
        let up = Event::Pointer(PointerEvent {
            position: Point::new(112.0, 18.0),
            button: Some(PointerButton::Primary),
            phase: PointerPhase::Up,
            modifiers: Default::default(),
        });
        assert!(slider.handle_event(&up, bounds));
        assert!((val_sig.get() - 100.0).abs() < 0.1);

        let mut canvas = Canvas::new();
        slider.paint(&mut canvas, bounds);
        assert!(canvas.commands().len() >= 3);
    }

    #[test]
    fn test_slider_discrete_steps() {
        let val_sig = Signal::new(0.0);
        let mut slider = Slider::new(val_sig.clone(), 0.0, 100.0).with_steps(Some(4));
        let bounds = Rect::new(0.0, 0.0, 124.0, 36.0);

        // Drag to ~30% -> snaps to 25% (step 1 of 4)
        let down = Event::Pointer(PointerEvent {
            position: Point::new(42.0, 18.0),
            button: Some(PointerButton::Primary),
            phase: PointerPhase::Down,
            modifiers: Default::default(),
        });
        assert!(slider.handle_event(&down, bounds));
        assert_eq!(val_sig.get(), 25.0);
    }
}
