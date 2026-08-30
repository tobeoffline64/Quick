//! `CountdownRing` — GPU Rasterized Radial Circular Progress Ring for Noctalia UI.

use crate::widget::Widget;
use quick_core::event::Event;
use quick_core::geometry::{BorderRadius, Point, Rect};
use quick_layout::engine::LayoutEngine;
use quick_render::canvas::Canvas;
use quick_style::noctalia::NoctaliaPalette;
use quick_style::property::{Dimension, Style};
use taffy::prelude::NodeId;
use taffy::TaffyError;
use std::f32::consts::PI;

pub struct CountdownRing {
    pub id: Option<String>,
    pub classes: Vec<String>,
    pub style: Style,
    pub progress: f32, // 0.0 to 1.0
    pub size: f32,
    pub stroke_width: f32,
    pub center_text: Option<String>,
}

impl CountdownRing {
    pub fn new(progress: f32) -> Self {
        Self {
            id: None,
            classes: Vec::new(),
            style: Style::default(),
            progress: progress.clamp(0.0, 1.0),
            size: 140.0,
            stroke_width: 8.0,
            center_text: None,
        }
    }

    pub fn size(mut self, size: f32) -> Self {
        self.size = size;
        self
    }

    pub fn stroke_width(mut self, width: f32) -> Self {
        self.stroke_width = width;
        self
    }

    pub fn text(mut self, text: impl Into<String>) -> Self {
        self.center_text = Some(text.into());
        self
    }
}

impl Widget for CountdownRing {
    fn widget_type(&self) -> &'static str {
        "CountdownRing"
    }

    fn id(&self) -> Option<&str> { self.id.as_deref() }
    fn classes(&self) -> &[String] { &self.classes }
    fn style(&self) -> &Style { &self.style }
    fn style_mut(&mut self) -> &mut Style { &mut self.style }

    fn build_layout(&mut self, engine: &mut LayoutEngine) -> Result<NodeId, TaffyError> {
        let mut computed = self.style.clone();
        if computed.width.is_none() {
            computed.width = Some(Dimension::Px(self.size));
        }
        if computed.height.is_none() {
            computed.height = Some(Dimension::Px(self.size));
        }
        engine.new_leaf(&computed)
    }

    fn paint(&self, canvas: &mut Canvas, bounds: Rect) {
        let pal = NoctaliaPalette::noctalia_dark();
        let center_x = bounds.origin.x + bounds.size.width / 2.0;
        let center_y = bounds.origin.y + bounds.size.height / 2.0;
        let radius = (bounds.size.width.min(bounds.size.height) / 2.0) - self.stroke_width;

        // Background track circle
        let track_rect = Rect::new(center_x - radius, center_y - radius, radius * 2.0, radius * 2.0);
        canvas.stroke_rounded_rect(track_rect, BorderRadius::all(radius), pal.surface_variant, self.stroke_width);

        // Active radial sweep arc (approximated via anti-aliased chord line segments around circle perimeter)
        let sweep_angle = self.progress * 2.0 * PI;
        let step_count = ((sweep_angle / (2.0 * PI)) * 64.0).ceil() as usize;

        if step_count > 1 {
            let start_angle = -PI / 2.0; // Start at 12 o'clock
            let angle_step = sweep_angle / step_count as f32;

            for i in 0..step_count {
                let a1 = start_angle + (i as f32) * angle_step;
                let a2 = start_angle + ((i + 1) as f32) * angle_step;

                let p1 = Point::new(center_x + radius * a1.cos(), center_y + radius * a1.sin());
                let p2 = Point::new(center_x + radius * a2.cos(), center_y + radius * a2.sin());

                canvas.draw_line(p1, p2, pal.primary, self.stroke_width);
            }
        }

        // Center readout text
        let display_text = self.center_text.clone().unwrap_or_else(|| {
            format!("{:.0}%", self.progress * 100.0)
        });

        let font_size = self.size * 0.16;
        let text_w = (display_text.chars().count() as f32) * font_size * 0.55;
        let text_x = center_x - text_w / 2.0;
        let text_y = center_y + font_size * 0.35;

        canvas.draw_text(&display_text, Point::new(text_x, text_y), pal.on_surface, font_size, None);
    }

    fn handle_event(&mut self, _event: &Event, _bounds: Rect) -> bool {
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use quick_render::canvas::Canvas;

    #[test]
    fn test_countdown_ring_progress() {
        let ring = CountdownRing::new(0.75).size(150.0).text("45s");
        assert_eq!(ring.progress, 0.75);

        let mut canvas = Canvas::new();
        ring.paint(&mut canvas, Rect::new(0.0, 0.0, 150.0, 150.0));
        assert!(!canvas.commands().is_empty());
    }
}
