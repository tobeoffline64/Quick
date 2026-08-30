//! ScrollViewer — Scrollable content container with optional scrollbar.

use crate::widget::Widget;
use quick_core::event::Event;
use quick_core::geometry::{Point, Rect};
use quick_layout::engine::LayoutEngine;
use quick_render::canvas::Canvas;
use quick_style::base::{base_theme, RadiusScale};
use quick_style::property::Style;
use taffy::prelude::NodeId;
use taffy::TaffyError;

pub struct ScrollViewer {
    pub id: Option<String>,
    pub classes: Vec<String>,
    pub style: Style,
    pub scroll_offset: f32,
    pub content_height: f32,
    pub show_scrollbar: bool,
    pub children: Vec<Box<dyn Widget>>,
    bounds: Rect,
}

impl ScrollViewer {
    pub fn new() -> Self {
        let bt = base_theme();
        let mut style = Style::default();
        style.background_color = Some(bt.colors.bg);
        Self {
            id: None, classes: Vec::new(), style,
            scroll_offset: 0.0, content_height: 0.0,
            show_scrollbar: true, children: Vec::new(), bounds: Rect::ZERO,
        }
    }

    pub fn with_content_height(mut self, h: f32) -> Self { self.content_height = h; self }
    pub fn add_child(mut self, w: Box<dyn Widget>) -> Self {
        self.children.push(w); self
    }

    fn max_scroll(&self, bounds: &Rect) -> f32 {
        (self.content_height - bounds.size.height).max(0.0)
    }

    const SCROLLBAR_W: f32 = 8.0;
}

impl Default for ScrollViewer {
    fn default() -> Self { Self::new() }
}

impl Widget for ScrollViewer {
    fn widget_type(&self) -> &'static str {
        "ScrollViewer"
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
        engine.new_leaf(&self.style)
    }

    fn update_layout(&mut self, _engine: &LayoutEngine, origin: Point) {
        self.bounds = Rect::new(origin.x, origin.y, self.bounds.size.width, self.bounds.size.height);
    }

    fn paint(&self, canvas: &mut Canvas, bounds: Rect) {
        let bt = base_theme();

        // Content area clip
        canvas.push_clip(bounds);
        canvas.fill_rect(bounds, bt.colors.bg);

        // Paint children (offset by scroll)
        for child in &self.children {
            let child_bounds = Rect::new(
                bounds.origin.x, bounds.origin.y - self.scroll_offset,
                bounds.size.width - if self.show_scrollbar { Self::SCROLLBAR_W + 2.0 } else { 0.0 },
                self.content_height,
            );
            child.paint(canvas, child_bounds);
        }

        canvas.pop_clip();

        // Scrollbar
        if self.show_scrollbar && self.content_height > bounds.size.height {
            let sb_x = bounds.origin.x + bounds.size.width - Self::SCROLLBAR_W - 2.0;
            let track = Rect::new(sb_x, bounds.origin.y + 2.0, Self::SCROLLBAR_W, bounds.size.height - 4.0);
            canvas.fill_rounded_rect(track, quick_core::geometry::BorderRadius::all(RadiusScale::PILL), bt.colors.surface);

            let ratio = bounds.size.height / self.content_height;
            let thumb_h = (track.size.height * ratio).max(20.0);
            let max_scroll = self.max_scroll(&bounds);
            let thumb_y = if max_scroll > 0.0 {
                track.origin.y + (self.scroll_offset / max_scroll) * (track.size.height - thumb_h)
            } else { track.origin.y };

            canvas.fill_rounded_rect(
                Rect::new(sb_x, thumb_y, Self::SCROLLBAR_W, thumb_h),
                quick_core::geometry::BorderRadius::all(RadiusScale::PILL),
                bt.colors.border_strong,
            );
        }
    }

    fn handle_event(&mut self, event: &Event, bounds: Rect) -> bool {
        match event {
            Event::Scroll(delta) => {
                let dy = match delta {
                    quick_core::event::ScrollDelta::LineDelta(_, y)  => *y,
                    quick_core::event::ScrollDelta::PixelDelta(_, y) => *y,
                };
                let max = self.max_scroll(&bounds);
                self.scroll_offset = (self.scroll_offset - dy * 40.0).clamp(0.0, max);
                true
            }
            _ => false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scroll_viewer_creation() {
        let sv = ScrollViewer::new().with_content_height(500.0);
        assert_eq!(sv.content_height, 500.0);
        assert_eq!(sv.scroll_offset, 0.0);
    }

    #[test]
    fn test_scroll_viewer_paint() {
        let sv = ScrollViewer::new().with_content_height(400.0);
        let mut canvas = quick_render::canvas::Canvas::new();
        sv.paint(&mut canvas, Rect::new(0.0, 0.0, 200.0, 150.0));
        assert!(!canvas.commands().is_empty());
    }
}
