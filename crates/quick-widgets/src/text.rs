use crate::widget::Widget;
use quick_core::geometry::{Color, Point, Rect};
use quick_core::signals::Signal;
use quick_layout::engine::LayoutEngine;
use quick_render::canvas::Canvas;
use quick_style::property::{Dimension, Style};
use taffy::prelude::{NodeId, TaffyError};

enum TextSource {
    Static(String),
    Dynamic(Signal<String>),
}

pub struct Text {
    pub id: Option<String>,
    pub classes: Vec<String>,
    pub style: Style,
    source: TextSource,
}

impl Text {
    pub fn new(text: impl Into<String>) -> Self {
        let mut style = Style::default();
        style.text_color = Some(Color::WHITE);
        style.font_size = Some(14.0);

        Self {
            id: None,
            classes: Vec::new(),
            style,
            source: TextSource::Static(text.into()),
        }
    }

    pub fn dynamic(signal: Signal<String>) -> Self {
        let mut style = Style::default();
        style.text_color = Some(Color::WHITE);
        style.font_size = Some(14.0);

        Self {
            id: None,
            classes: Vec::new(),
            style,
            source: TextSource::Dynamic(signal),
        }
    }

    pub fn text(&self) -> String {
        match &self.source {
            TextSource::Static(s) => s.clone(),
            TextSource::Dynamic(sig) => sig.get(),
        }
    }

    pub fn set_text(&mut self, text: impl Into<String>) {
        self.source = TextSource::Static(text.into());
    }
}

impl Widget for Text {
    fn widget_type(&self) -> &'static str {
        "Text"
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
        let content = self.text();
        let font_size = self.style.font_size.unwrap_or(14.0);
        let char_count = content.len() as f32;
        let estimated_width = (char_count * font_size * 0.55).max(10.0);
        let estimated_height = font_size * 1.3;

        let mut computed_style = self.style.clone();
        if computed_style.width.is_none() {
            computed_style.width = Some(Dimension::Px(estimated_width));
        }
        if computed_style.height.is_none() {
            computed_style.height = Some(Dimension::Px(estimated_height));
        }

        engine.new_leaf(&computed_style)
    }

    fn paint(&self, canvas: &mut Canvas, bounds: Rect) {
        let color = self.style.text_color.unwrap_or(Color::WHITE);
        let font_size = self.style.font_size.unwrap_or(14.0);
        let origin = Point::new(bounds.origin.x, bounds.origin.y + font_size);
        canvas.draw_text(
            self.text(),
            origin,
            color,
            font_size,
            self.style.font_family.clone(),
        );
    }
}
