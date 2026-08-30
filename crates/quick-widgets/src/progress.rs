use crate::widget::Widget;
use quick_core::geometry::{BorderRadius, Color, Rect};
use quick_core::signals::Signal;
use quick_layout::engine::LayoutEngine;
use quick_render::canvas::Canvas;
use quick_style::property::{Dimension, Style};
use taffy::prelude::NodeId;
use taffy::TaffyError;

pub struct ProgressBar {
    pub id: Option<String>,
    pub classes: Vec<String>,
    pub style: Style,
    pub progress: Signal<f32>,
    pub min: f32,
    pub max: f32,
    pub is_indeterminate: bool,
    pub animation_phase: f32,
}

impl ProgressBar {
    pub fn new(progress: Signal<f32>) -> Self {
        let mut style = Style::default();
        style.height = Some(Dimension::Px(8.0));

        Self {
            id: None,
            classes: Vec::new(),
            style,
            progress,
            min: 0.0,
            max: 1.0,
            is_indeterminate: false,
            animation_phase: 0.0,
        }
    }

    pub fn with_range(mut self, min: f32, max: f32) -> Self {
        self.min = min;
        self.max = max;
        self
    }

    pub fn with_indeterminate(mut self, indeterminate: bool) -> Self {
        self.is_indeterminate = indeterminate;
        self
    }

    pub fn indeterminate(self, indeterminate: bool) -> Self {
        self.with_indeterminate(indeterminate)
    }

    pub fn with_phase(mut self, phase: f32) -> Self {
        self.animation_phase = if phase.is_nan() { 0.0 } else { phase.fract().abs() };
        self
    }
}

impl Widget for ProgressBar {
    fn widget_type(&self) -> &'static str {
        "ProgressBar"
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
            computed_style.height = Some(Dimension::Px(8.0));
        }
        engine.new_leaf(&computed_style)
    }

    fn paint(&self, canvas: &mut Canvas, bounds: Rect) {
        if bounds.size.width <= 0.0 || bounds.size.height <= 0.0 {
            return;
        }

        let radius = self
            .style
            .border_radius
            .unwrap_or_else(|| BorderRadius::all(bounds.size.height / 2.0));

        // 1. Inactive Background Track
        let track_color = self
            .style
            .border_color
            .unwrap_or_else(|| Color::from_hex("#36343B").unwrap_or(Color::from_rgb(54, 52, 59)));
        canvas.fill_rounded_rect(bounds, radius, track_color);

        // 2. Active Indicator
        let fill_color = self
            .style
            .background_color
            .or(self.style.text_color)
            .unwrap_or_else(|| Color::from_hex("#6750A4").unwrap_or(Color::from_rgb(103, 80, 164)));

        if self.is_indeterminate {
            // Indeterminate animated pulse mode
            let pulse_w = (bounds.size.width * 0.35).max(12.0);
            let total_travel = bounds.size.width + pulse_w;
            let current_x = bounds.origin.x - pulse_w + total_travel * self.animation_phase;

            let visible_x = current_x.max(bounds.origin.x);
            let visible_right = (current_x + pulse_w).min(bounds.origin.x + bounds.size.width);
            let visible_w = (visible_right - visible_x).max(0.0);

            if visible_w > 0.0 {
                let active_rect = Rect::new(visible_x, bounds.origin.y, visible_w, bounds.size.height);
                canvas.fill_rounded_rect(active_rect, radius, fill_color);
            }
        } else {
            // Determinate Fill Ratio mode
            let (min_val, max_val) = if self.min <= self.max {
                (self.min, self.max)
            } else {
                (self.max, self.min)
            };

            let raw_val = self.progress.get();
            let val = if raw_val.is_nan() { min_val } else { raw_val.clamp(min_val, max_val) };

            let pct = if (max_val - min_val).abs() > 0.0001 {
                ((val - min_val) / (max_val - min_val)).clamp(0.0, 1.0)
            } else {
                0.0
            };

            let active_w = bounds.size.width * pct;
            if active_w > 0.0 {
                let active_rect = Rect::new(bounds.origin.x, bounds.origin.y, active_w, bounds.size.height);
                canvas.fill_rounded_rect(active_rect, radius, fill_color);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_progress_bar_paint() {
        let prog_sig = Signal::new(0.65);
        let bar = ProgressBar::new(prog_sig.clone());
        let bounds = Rect::new(0.0, 0.0, 200.0, 8.0);

        let mut canvas = Canvas::new();
        bar.paint(&mut canvas, bounds);
        assert_eq!(canvas.commands().len(), 2);

        // Test with 0..100 range
        let scale_sig = Signal::new(75.0);
        let bar_100 = ProgressBar::new(scale_sig).with_range(0.0, 100.0);
        let mut canvas2 = Canvas::new();
        bar_100.paint(&mut canvas2, bounds);
        assert_eq!(canvas2.commands().len(), 2);
    }

    #[test]
    fn test_progress_bar_indeterminate() {
        let prog_sig = Signal::new(0.5);
        let bar = ProgressBar::new(prog_sig).with_indeterminate(true).with_phase(0.25);
        let bounds = Rect::new(0.0, 0.0, 200.0, 8.0);

        let mut canvas = Canvas::new();
        bar.paint(&mut canvas, bounds);
        assert_eq!(canvas.commands().len(), 2);
    }
}
