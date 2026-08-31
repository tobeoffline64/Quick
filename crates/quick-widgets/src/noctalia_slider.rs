//! `NoctaliaSlider` — Interactive Slider with Leading Icon, Hover Knob Enlarge & Track Fill.

use crate::widget::Widget;
use quick_core::event::{Event, PointerButton, PointerEvent, PointerPhase};
use quick_core::geometry::{BorderRadius, Point, Rect};
use quick_layout::engine::LayoutEngine;
use quick_render::canvas::Canvas;
use quick_style::noctalia::NoctaliaPalette;
use quick_style::property::{Dimension, Style};
use taffy::prelude::NodeId;
use taffy::TaffyError;

pub struct NoctaliaSlider {
    pub id: Option<String>,
    pub classes: Vec<String>,
    pub style: Style,
    pub min: f32,
    pub max: f32,
    pub value: f32,
    pub icon_text: Option<String>,
    pub is_muted: bool,
    pub is_dragging: bool,
    pub is_hovered: bool,
    pub on_change: Option<Box<dyn FnMut(f32)>>,
}

impl NoctaliaSlider {
    pub fn new(min: f32, max: f32, value: f32) -> Self {
        Self {
            id: None,
            classes: Vec::new(),
            style: Style::default(),
            min,
            max,
            value: value.clamp(min, max),
            icon_text: None,
            is_muted: false,
            is_dragging: false,
            is_hovered: false,
            on_change: None,
        }
    }

    pub fn icon(mut self, icon: impl Into<String>) -> Self {
        self.icon_text = Some(icon.into());
        self
    }

    pub fn on_change<F: FnMut(f32) + 'static>(mut self, handler: F) -> Self {
        self.on_change = Some(Box::new(handler));
        self
    }
}

impl Widget for NoctaliaSlider {
    fn widget_type(&self) -> &'static str {
        "NoctaliaSlider"
    }

    fn id(&self) -> Option<&str> { self.id.as_deref() }
    fn classes(&self) -> &[String] { &self.classes }
    fn style(&self) -> &Style { &self.style }
    fn style_mut(&mut self) -> &mut Style { &mut self.style }

    fn build_layout(&mut self, engine: &mut LayoutEngine) -> Result<NodeId, TaffyError> {
        let mut computed = self.style.clone();
        if computed.width.is_none() {
            computed.width = Some(Dimension::Px(200.0));
        }
        if computed.height.is_none() {
            computed.height = Some(Dimension::Px(34.0));
        }
        engine.new_leaf(&computed)
    }

    fn paint(&self, canvas: &mut Canvas, bounds: Rect) {
        let pal = NoctaliaPalette::noctalia_dark();
        let track_h = 8.0;
        let mut track_x = bounds.origin.x;
        let mut track_w = bounds.size.width;

        // Draw leading icon if present
        if let Some(ref icon) = self.icon_text {
            let icon_color = if self.is_muted { pal.on_surface_variant } else { pal.primary };
            canvas.draw_text(icon, Point::new(bounds.origin.x, bounds.origin.y + 22.0), icon_color, 14.0, None);
            track_x += 28.0;
            track_w -= 28.0;
        }

        let track_y = bounds.origin.y + (bounds.size.height - track_h) / 2.0;
        let track_rect = Rect::new(track_x, track_y, track_w, track_h);
        let track_radius = BorderRadius::all(4.0);

        // Background Track
        canvas.fill_rounded_rect(track_rect, track_radius, pal.surface_variant);
        canvas.stroke_rounded_rect(track_rect, track_radius, pal.outline, 1.0);

        // Active Fill
        let progress = if (self.max - self.min).abs() > 0.001 {
            ((self.value - self.min) / (self.max - self.min)).clamp(0.0, 1.0)
        } else {
            0.0
        };

        let fill_w = track_w * progress;
        if fill_w > 0.0 {
            let fill_rect = Rect::new(track_x, track_y, fill_w, track_h);
            canvas.fill_rounded_rect(fill_rect, track_radius, pal.primary);
        }

        // Thumb Knob (enlarges on hover/drag from 16px to 20px)
        let knob_size = if self.is_dragging || self.is_hovered { 20.0 } else { 16.0 };
        let knob_x = track_x + fill_w - (knob_size / 2.0);
        let knob_y = bounds.origin.y + (bounds.size.height - knob_size) / 2.0;
        let knob_rect = Rect::new(knob_x, knob_y, knob_size, knob_size);
        let knob_radius = BorderRadius::all(knob_size / 2.0);

        canvas.fill_rounded_rect(knob_rect, knob_radius, pal.primary);
        canvas.stroke_rounded_rect(knob_rect, knob_radius, pal.surface, 2.0);
    }

    fn handle_event(&mut self, event: &Event, bounds: Rect) -> bool {
        if let Event::Pointer(PointerEvent { position, button, phase, .. }) = event {
            let inside = bounds.contains(*position);
            let prev_hover = self.is_hovered;
            self.is_hovered = inside;

            let icon_w = if self.icon_text.is_some() { 28.0 } else { 0.0 };
            let track_x = bounds.origin.x + icon_w;
            let track_w = (bounds.size.width - icon_w).max(1.0);

            match phase {
                PointerPhase::Down if inside && *button == Some(PointerButton::Primary) => {
                    // Check if clicked leading icon
                    if position.x < track_x {
                        self.is_muted = !self.is_muted;
                        return true;
                    }
                    self.is_dragging = true;
                    let rel_x = (position.x - track_x).clamp(0.0, track_w);
                    let ratio = rel_x / track_w;
                    self.value = self.min + ratio * (self.max - self.min);
                    if let Some(ref mut handler) = self.on_change {
                        handler(self.value);
                    }
                    true
                }
                PointerPhase::Moved if self.is_dragging => {
                    let rel_x = (position.x - track_x).clamp(0.0, track_w);
                    let ratio = rel_x / track_w;
                    self.value = self.min + ratio * (self.max - self.min);
                    if let Some(ref mut handler) = self.on_change {
                        handler(self.value);
                    }
                    true
                }
                PointerPhase::Moved => {
                    prev_hover != self.is_hovered
                }
                PointerPhase::Up if self.is_dragging => {
                    self.is_dragging = false;
                    true
                }
                _ => inside,
            }
        } else {
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use quick_render::canvas::Canvas;

    #[test]
    fn test_noctalia_slider_value_clamp() {
        let slider = NoctaliaSlider::new(0.0, 100.0, 150.0);
        assert_eq!(slider.value, 100.0);

        let mut canvas = Canvas::new();
        slider.paint(&mut canvas, Rect::new(0.0, 0.0, 200.0, 34.0));
        assert!(!canvas.commands().is_empty());
    }
}
