use crate::widget::Widget;
use quick_core::event::{Event, PointerButton, PointerEvent, PointerPhase};
use quick_core::geometry::{BorderRadius, Color, Rect};
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
    pub on_change: Option<Box<dyn FnMut(f32)>>,
    is_dragging: bool,
}

impl Slider {
    pub fn new(value: Signal<f32>, min: f32, max: f32) -> Self {
        let mut style = Style::default();
        style.height = Some(Dimension::Px(36.0));
        style.width = Some(Dimension::Percent(100.0));

        Self {
            id: None,
            classes: Vec::new(),
            style,
            value,
            min,
            max,
            on_change: None,
            is_dragging: false,
        }
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
        let new_val = self.min + ratio * (self.max - self.min);
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
        let val = self.value.get().clamp(self.min, self.max);
        let ratio = if (self.max - self.min).abs() > 0.001 {
            (val - self.min) / (self.max - self.min)
        } else {
            0.0
        };

        let pad = 12.0;
        let track_h = 8.0;
        let track_y = bounds.origin.y + (bounds.size.height - track_h) / 2.0;
        let track_w = bounds.size.width - pad * 2.0;
        let track_radius = BorderRadius::all(track_h / 2.0);

        // Inactive track (right side)
        let inactive_rect = Rect::new(bounds.origin.x + pad, track_y, track_w, track_h);
        canvas.fill_rounded_rect(inactive_rect, track_radius, Color::from_hex("#36343B").unwrap());

        // Active track (left side)
        let active_w = track_w * ratio;
        if active_w > 0.0 {
            let active_rect = Rect::new(bounds.origin.x + pad, track_y, active_w, track_h);
            let active_color = self.style.background_color.unwrap_or(Color::from_hex("#6750A4").unwrap());
            canvas.fill_rounded_rect(active_rect, track_radius, active_color);
        }

        // Thumb
        let thumb_r = 10.0;
        let thumb_x = bounds.origin.x + pad + active_w;
        let thumb_y = bounds.origin.y + bounds.size.height / 2.0;
        let thumb_rect = Rect::new(thumb_x - thumb_r, thumb_y - thumb_r, thumb_r * 2.0, thumb_r * 2.0);
        let thumb_color = Color::from_hex("#D0BCFF").unwrap();
        canvas.fill_rounded_rect(thumb_rect, BorderRadius::all(thumb_r), thumb_color);
    }

    fn handle_event(&mut self, event: &Event, bounds: Rect) -> bool {
        match event {
            Event::Pointer(PointerEvent { position, button, phase, .. }) => {
                match phase {
                    PointerPhase::Down if bounds.contains(*position) && *button == Some(PointerButton::Primary) => {
                        self.is_dragging = true;
                        self.update_from_pos(position.x, bounds);
                        true
                    }
                    PointerPhase::Moved if self.is_dragging => {
                        self.update_from_pos(position.x, bounds);
                        true
                    }
                    PointerPhase::Up if self.is_dragging => {
                        self.is_dragging = false;
                        self.update_from_pos(position.x, bounds);
                        true
                    }
                    PointerPhase::Cancel => {
                        self.is_dragging = false;
                        false
                    }
                    _ => false,
                }
            }
            _ => false,
        }
    }
}
