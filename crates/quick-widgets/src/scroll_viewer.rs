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
    pub is_thumb_hovered: bool,
    child_nodes: Vec<NodeId>,
    child_bounds: Vec<Rect>,
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
            show_scrollbar: true, children: Vec::new(),
            is_thumb_hovered: false,
            child_nodes: Vec::new(),
            child_bounds: Vec::new(),
            bounds: Rect::ZERO,
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
    const SCROLLBAR_HOVER_W: f32 = 14.0;
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
        self.child_nodes.clear();
        for child in &mut self.children {
            let child_node = child.build_layout(engine)?;
            self.child_nodes.push(child_node);
        }
        engine.new_with_children(&self.style, &self.child_nodes)
    }

    fn update_layout(&mut self, engine: &LayoutEngine, origin: Point) {
        self.bounds = Rect::new(origin.x, origin.y, self.bounds.size.width, self.bounds.size.height);
        self.child_bounds.clear();
        for (i, child) in self.children.iter_mut().enumerate() {
            if let Some(&node_id) = self.child_nodes.get(i) {
                if let Ok(rel_layout) = engine.get_layout(node_id) {
                    let abs_bounds = Rect::new(
                        origin.x + rel_layout.origin.x,
                        origin.y + rel_layout.origin.y,
                        rel_layout.size.width,
                        rel_layout.size.height,
                    );
                    self.child_bounds.push(abs_bounds);
                    child.update_layout(engine, abs_bounds.origin);
                }
            }
        }
    }

    fn paint(&self, canvas: &mut Canvas, bounds: Rect) {
        let bt = base_theme();

        // Content area clip
        canvas.push_clip(bounds);
        canvas.fill_rect(bounds, bt.colors.bg);

        // Paint children (offset by scroll)
        for (i, child) in self.children.iter().enumerate() {
            let child_b = self.child_bounds.get(i).copied().unwrap_or(bounds);
            let offset_bounds = Rect::new(
                child_b.origin.x,
                child_b.origin.y - self.scroll_offset,
                child_b.size.width,
                child_b.size.height,
            );
            child.paint(canvas, offset_bounds);
        }

        canvas.pop_clip();

        // Scrollbar (Enlarges from 8px to 14px on hover)
        if self.show_scrollbar && self.content_height > bounds.size.height {
            let sb_w = if self.is_thumb_hovered { Self::SCROLLBAR_HOVER_W } else { Self::SCROLLBAR_W };
            let sb_x = bounds.origin.x + bounds.size.width - sb_w - 2.0;
            let track = Rect::new(sb_x, bounds.origin.y + 2.0, sb_w, bounds.size.height - 4.0);
            canvas.fill_rounded_rect(track, quick_core::geometry::BorderRadius::all(RadiusScale::PILL), bt.colors.surface);

            let ratio = bounds.size.height / self.content_height;
            let thumb_h = (track.size.height * ratio).max(24.0);
            let max_scroll = self.max_scroll(&bounds);
            let thumb_y = if max_scroll > 0.0 {
                track.origin.y + (self.scroll_offset / max_scroll) * (track.size.height - thumb_h)
            } else { track.origin.y };

            let thumb_color = if self.is_thumb_hovered { bt.colors.accent.normal } else { bt.colors.border_strong };

            canvas.fill_rounded_rect(
                Rect::new(sb_x, thumb_y, sb_w, thumb_h),
                quick_core::geometry::BorderRadius::all(RadiusScale::PILL),
                thumb_color,
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
            Event::Pointer(quick_core::event::PointerEvent { position, .. }) => {
                let sb_x = bounds.origin.x + bounds.size.width - Self::SCROLLBAR_HOVER_W - 4.0;
                let prev_sb_hover = self.is_thumb_hovered;
                self.is_thumb_hovered = position.x >= sb_x && bounds.contains(*position);

                let mut handled = false;
                for (i, child) in self.children.iter_mut().enumerate() {
                    let child_b = self.child_bounds.get(i).copied().unwrap_or(bounds);
                    let offset_bounds = Rect::new(
                        child_b.origin.x,
                        child_b.origin.y - self.scroll_offset,
                        child_b.size.width,
                        child_b.size.height,
                    );
                    if child.handle_event(event, offset_bounds) {
                        handled = true;
                    }
                }
                handled || (prev_sb_hover != self.is_thumb_hovered)
            }
            _ => {
                for (i, child) in self.children.iter_mut().enumerate() {
                    let child_b = self.child_bounds.get(i).copied().unwrap_or(bounds);
                    let offset_bounds = Rect::new(
                        child_b.origin.x,
                        child_b.origin.y - self.scroll_offset,
                        child_b.size.width,
                        child_b.size.height,
                    );
                    if child.handle_event(event, offset_bounds) {
                        return true;
                    }
                }
                false
            }
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
