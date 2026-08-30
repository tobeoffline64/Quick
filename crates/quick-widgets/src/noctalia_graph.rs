//! `NoctaliaGraph` — Real-Time Vector Metric Line/Area Chart for Noctalia UI.

use crate::widget::Widget;
use quick_core::event::Event;
use quick_core::geometry::{BorderRadius, Color, Point, Rect};
use quick_layout::engine::LayoutEngine;
use quick_render::canvas::Canvas;
use quick_style::noctalia::NoctaliaPalette;
use quick_style::property::{Dimension, Style};
use taffy::prelude::NodeId;
use taffy::TaffyError;

pub struct NoctaliaGraph {
    pub id: Option<String>,
    pub classes: Vec<String>,
    pub style: Style,
    pub values: Vec<f32>,
    pub max_value: f32,
    pub label: Option<String>,
    pub stroke_color: Option<Color>,
}

impl NoctaliaGraph {
    pub fn new(values: Vec<f32>) -> Self {
        Self {
            id: None,
            classes: Vec::new(),
            style: Style::default(),
            values,
            max_value: 100.0,
            label: None,
            stroke_color: None,
        }
    }

    pub fn max(mut self, max: f32) -> Self {
        self.max_value = max.max(1.0);
        self
    }

    pub fn label(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
        self
    }

    pub fn stroke_color(mut self, color: Color) -> Self {
        self.stroke_color = Some(color);
        self
    }
}

impl Widget for NoctaliaGraph {
    fn widget_type(&self) -> &'static str {
        "NoctaliaGraph"
    }

    fn id(&self) -> Option<&str> { self.id.as_deref() }
    fn classes(&self) -> &[String] { &self.classes }
    fn style(&self) -> &Style { &self.style }
    fn style_mut(&mut self) -> &mut Style { &mut self.style }

    fn build_layout(&mut self, engine: &mut LayoutEngine) -> Result<NodeId, TaffyError> {
        let mut computed = self.style.clone();
        if computed.width.is_none() {
            computed.width = Some(Dimension::Px(240.0));
        }
        if computed.height.is_none() {
            computed.height = Some(Dimension::Px(80.0));
        }
        engine.new_leaf(&computed)
    }

    fn paint(&self, canvas: &mut Canvas, bounds: Rect) {
        let pal = NoctaliaPalette::noctalia_dark();
        let radius = BorderRadius::all(8.0);

        // Chart Background
        canvas.fill_rounded_rect(bounds, radius, pal.surface_variant);
        canvas.stroke_rounded_rect(bounds, radius, pal.outline, 1.0);

        // Header Label & Current Value Readout
        if let Some(ref l) = self.label {
            let latest = self.values.last().copied().unwrap_or(0.0);
            let title = format!("{}: {:.1}%", l, latest);
            canvas.draw_text(&title, Point::new(bounds.origin.x + 8.0, bounds.origin.y + 16.0), pal.on_surface_variant, 10.0, None);
        }

        let count = self.values.len();
        if count < 2 {
            return;
        }

        let pad_h = 8.0;
        let pad_top = 22.0;
        let pad_bot = 8.0;

        let graph_w = bounds.size.width - pad_h * 2.0;
        let graph_h = bounds.size.height - pad_top - pad_bot;
        let dx = graph_w / (count - 1) as f32;

        let stroke_c = self.stroke_color.unwrap_or(pal.tertiary);

        // Render line segments
        for i in 0..(count - 1) {
            let v1 = (self.values[i] / self.max_value).clamp(0.0, 1.0);
            let v2 = (self.values[i + 1] / self.max_value).clamp(0.0, 1.0);

            let x1 = bounds.origin.x + pad_h + (i as f32) * dx;
            let y1 = bounds.origin.y + bounds.size.height - pad_bot - (v1 * graph_h);

            let x2 = bounds.origin.x + pad_h + ((i + 1) as f32) * dx;
            let y2 = bounds.origin.y + bounds.size.height - pad_bot - (v2 * graph_h);

            canvas.draw_line(Point::new(x1, y1), Point::new(x2, y2), stroke_c, 2.0);
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
    fn test_noctalia_graph_paint() {
        let graph = NoctaliaGraph::new(vec![10.0, 20.0, 50.0, 80.0]).label("CPU");
        let mut canvas = Canvas::new();
        graph.paint(&mut canvas, Rect::new(0.0, 0.0, 200.0, 80.0));
        assert!(!canvas.commands().is_empty());
    }
}
