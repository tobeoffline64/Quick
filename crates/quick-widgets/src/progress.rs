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
}

impl ProgressBar {
    pub fn new(progress: Signal<f32>) -> Self {
        let mut style = Style::default();
        style.height = Some(Dimension::Px(8.0));
        style.width = Some(Dimension::Percent(100.0));

        Self {
            id: None,
            classes: Vec::new(),
            style,
            progress,
        }
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
        let pct = self.progress.get().clamp(0.0, 1.0);
        let radius = BorderRadius::all(bounds.size.height / 2.0);

        // Inactive background track
        let track_color = Color::from_hex("#36343B").unwrap();
        canvas.fill_rounded_rect(bounds, radius, track_color);

        // Active fill
        let active_w = bounds.size.width * pct;
        if active_w > 0.0 {
            let active_rect = Rect::new(bounds.origin.x, bounds.origin.y, active_w, bounds.size.height);
            let fill_color = self.style.background_color.unwrap_or(Color::from_hex("#6750A4").unwrap());
            canvas.fill_rounded_rect(active_rect, radius, fill_color);
        }
    }
}
