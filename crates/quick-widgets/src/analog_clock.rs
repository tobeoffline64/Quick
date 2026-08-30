//! `AnalogClock` — GPU Rasterized Vector Analog Clock Dial for Noctalia UI.

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

pub struct AnalogClock {
    pub id: Option<String>,
    pub classes: Vec<String>,
    pub style: Style,
    pub hours: u32,
    pub minutes: u32,
    pub seconds: u32,
    pub size: f32,
    pub timezone_label: Option<String>,
}

impl AnalogClock {
    pub fn new(hours: u32, minutes: u32, seconds: u32) -> Self {
        Self {
            id: None,
            classes: Vec::new(),
            style: Style::default(),
            hours,
            minutes,
            seconds,
            size: 160.0,
            timezone_label: None,
        }
    }

    pub fn size(mut self, size: f32) -> Self {
        self.size = size;
        self
    }

    pub fn timezone(mut self, label: impl Into<String>) -> Self {
        self.timezone_label = Some(label.into());
        self
    }
}

impl Widget for AnalogClock {
    fn widget_type(&self) -> &'static str {
        "AnalogClock"
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
        let radius = bounds.size.width.min(bounds.size.height) / 2.0 - 4.0;

        // Dial face
        let face_rect = Rect::new(center_x - radius, center_y - radius, radius * 2.0, radius * 2.0);
        canvas.fill_rounded_rect(face_rect, BorderRadius::all(radius), pal.surface_variant);
        canvas.stroke_rounded_rect(face_rect, BorderRadius::all(radius), pal.outline, 1.5);

        // 12 Hour Ticks
        for i in 0..12 {
            let angle = (i as f32) * (PI / 6.0) - (PI / 2.0);
            let tick_len = if i % 3 == 0 { 10.0 } else { 5.0 };
            let tick_w = if i % 3 == 0 { 2.5 } else { 1.0 };
            let r1 = radius - tick_len - 4.0;
            let r2 = radius - 4.0;

            let p1 = Point::new(center_x + r1 * angle.cos(), center_y + r1 * angle.sin());
            let p2 = Point::new(center_x + r2 * angle.cos(), center_y + r2 * angle.sin());
            canvas.draw_line(p1, p2, pal.on_surface_variant, tick_w);
        }

        // Hour Hand
        let hour_frac = ((self.hours % 12) as f32 + (self.minutes as f32 / 60.0)) / 12.0;
        let hour_angle = hour_frac * 2.0 * PI - (PI / 2.0);
        let hour_len = radius * 0.5;
        let hp = Point::new(center_x + hour_len * hour_angle.cos(), center_y + hour_len * hour_angle.sin());
        canvas.draw_line(Point::new(center_x, center_y), hp, pal.on_surface, 3.5);

        // Minute Hand
        let min_frac = (self.minutes as f32 + (self.seconds as f32 / 60.0)) / 60.0;
        let min_angle = min_frac * 2.0 * PI - (PI / 2.0);
        let min_len = radius * 0.72;
        let mp = Point::new(center_x + min_len * min_angle.cos(), center_y + min_len * min_angle.sin());
        canvas.draw_line(Point::new(center_x, center_y), mp, pal.secondary, 2.5);

        // Second Hand (accent colored)
        let sec_frac = self.seconds as f32 / 60.0;
        let sec_angle = sec_frac * 2.0 * PI - (PI / 2.0);
        let sec_len = radius * 0.85;
        let sp = Point::new(center_x + sec_len * sec_angle.cos(), center_y + sec_len * sec_angle.sin());
        canvas.draw_line(Point::new(center_x, center_y), sp, pal.primary, 1.5);

        // Center Pin
        let pin_rect = Rect::new(center_x - 3.5, center_y - 3.5, 7.0, 7.0);
        canvas.fill_rounded_rect(pin_rect, BorderRadius::all(3.5), pal.primary);

        // Optional Timezone Label below dial
        if let Some(ref tz) = self.timezone_label {
            let font_size = 11.0;
            let tw = (tz.chars().count() as f32) * font_size * 0.55;
            canvas.draw_text(tz, Point::new(center_x - tw / 2.0, center_y + radius * 0.45), pal.on_surface_variant, font_size, None);
        }
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
    fn test_analog_clock_paint() {
        let clock = AnalogClock::new(10, 10, 30).size(160.0).timezone("UTC+0");
        let mut canvas = Canvas::new();
        clock.paint(&mut canvas, Rect::new(0.0, 0.0, 160.0, 160.0));
        assert!(!canvas.commands().is_empty());
    }
}
