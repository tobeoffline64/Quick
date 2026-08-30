use crate::widget::Widget;
use quick_core::geometry::{Color, Point, Rect};
use quick_core::signals::Signal;
use quick_layout::engine::LayoutEngine;
use quick_render::canvas::Canvas;
use quick_style::property::{Dimension, Style};
use taffy::prelude::NodeId;
use taffy::TaffyError;

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
        let char_count = content.chars().count() as f32;
        let pad_h = self.style.padding.map(|p| p.left + p.right).unwrap_or(0.0);
        let pad_v = self.style.padding.map(|p| p.top + p.bottom).unwrap_or(0.0);
        let estimated_width = (char_count * font_size * 0.55 + pad_h).max(10.0);
        let estimated_height = font_size * 1.3 + pad_v;

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
        if let Some(bg) = self.style.background_color {
            if let Some(radius) = self.style.border_radius {
                canvas.fill_rounded_rect(bounds, radius, bg);
            } else {
                canvas.fill_rect(bounds, bg);
            }
        }

        if let (Some(border_color), Some(border_width)) =
            (self.style.border_color, self.style.border_width)
        {
            if let Some(radius) = self.style.border_radius {
                canvas.stroke_rounded_rect(bounds, radius, border_color, border_width);
            } else {
                canvas.stroke_rect(bounds, border_color, border_width);
            }
        }

        let color = self.style.text_color.unwrap_or(Color::WHITE);
        let font_size = self.style.font_size.unwrap_or(14.0);
        let pad_left = self.style.padding.map(|p| p.left).unwrap_or(0.0);
        let pad_right = self.style.padding.map(|p| p.right).unwrap_or(0.0);
        let pad_top = self.style.padding.map(|p| p.top).unwrap_or(0.0);
        let content_w = (bounds.size.width - pad_left - pad_right).max(0.0);
        let char_count = self.text().chars().count() as f32;
        let text_w = char_count * font_size * 0.55;

        let offset_x = match self.style.text_align {
            Some(quick_style::property::TextAlignment::Center) => {
                pad_left + ((content_w - text_w) / 2.0).max(0.0)
            }
            Some(quick_style::property::TextAlignment::Right) => {
                pad_left + (content_w - text_w).max(0.0)
            }
            _ => pad_left,
        };

        let origin = Point::new(bounds.origin.x + offset_x, bounds.origin.y + pad_top + font_size);
        canvas.draw_text(
            self.text(),
            origin,
            color,
            font_size,
            self.style.font_family.clone(),
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use quick_core::geometry::{BorderRadius, Insets};
    use quick_render::canvas::DrawCommand;
    use quick_style::property::TextAlignment;

    #[test]
    fn test_text_static_and_dynamic() {
        let sig = Signal::new("Initial".to_string());
        let text_dyn = Text::dynamic(sig.clone());
        assert_eq!(text_dyn.text(), "Initial");

        sig.set("Updated".to_string());
        assert_eq!(text_dyn.text(), "Updated");
    }

    #[test]
    fn test_text_styled_paint() {
        let mut text = Text::new("Badge");
        text.style.background_color = Some(Color::from_rgb(30, 30, 40));
        text.style.border_radius = Some(BorderRadius::all(8.0));
        text.style.padding = Some(Insets::all(6.0));

        let mut canvas = Canvas::new();
        let bounds = Rect::new(10.0, 10.0, 80.0, 30.0);
        text.paint(&mut canvas, bounds);

        // Canvas should record background fill rounded rect + text draw command
        assert_eq!(canvas.commands().len(), 2);
    }

    #[test]
    fn test_text_aligned_paint() {
        let mut text = Text::new("Hi");
        text.style.text_align = Some(TextAlignment::Center);
        text.style.font_size = Some(10.0);

        let mut canvas = Canvas::new();
        let bounds = Rect::new(0.0, 0.0, 100.0, 30.0);
        text.paint(&mut canvas, bounds);

        assert_eq!(canvas.commands().len(), 1);
        if let DrawCommand::DrawText { origin, .. } = &canvas.commands()[0] {
            // Text width for 2 chars at 10px is 2 * 10 * 0.55 = 11.0
            // Offset for centering in 100px width is (100 - 11) / 2 = 44.5
            assert!(origin.x > 40.0 && origin.x < 50.0);
        } else {
            panic!("Expected DrawText command");
        }
    }
}
