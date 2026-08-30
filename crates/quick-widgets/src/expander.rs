//! Expander — Collapsible section with a header toggle and animated content reveal.

use crate::widget::Widget;
use quick_core::event::{Event, PointerPhase};
use quick_core::geometry::{BorderRadius, Point, Rect};
use quick_layout::engine::LayoutEngine;
use quick_render::canvas::Canvas;
use quick_style::base::{base_theme, RadiusScale, SpacingScale, TypeScale};
use quick_style::property::Style;
use taffy::prelude::NodeId;
use taffy::TaffyError;

pub struct Expander {
    pub id: Option<String>,
    pub classes: Vec<String>,
    pub style: Style,
    pub header: String,
    pub is_expanded: bool,
    pub content_height: f32,
    pub children: Vec<Box<dyn Widget>>,
    pub on_toggle: Option<Box<dyn FnMut(bool)>>,
    bounds: Rect,
}

impl Expander {
    pub fn new(header: impl Into<String>) -> Self {
        let bt = base_theme();
        let mut style = Style::default();
        style.background_color = Some(bt.colors.bg);
        style.border_color = Some(bt.colors.border);
        style.border_width = Some(1.0);
        style.border_radius = Some(BorderRadius::all(RadiusScale::SM));
        Self {
            id: None, classes: Vec::new(), style,
            header: header.into(), is_expanded: false,
            content_height: 120.0, children: Vec::new(),
            on_toggle: None, bounds: Rect::ZERO,
        }
    }

    pub fn expanded(mut self) -> Self { self.is_expanded = true; self }
    pub fn with_content_height(mut self, h: f32) -> Self { self.content_height = h; self }
    pub fn add_child(mut self, w: Box<dyn Widget>) -> Self { self.children.push(w); self }

    const HEADER_H: f32 = 44.0;
}

impl Widget for Expander {
    fn widget_type(&self) -> &'static str {
        "Expander"
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
        let header_rect = Rect::new(bounds.origin.x, bounds.origin.y, bounds.size.width, Self::HEADER_H);

        // Header background
        let br_header = if self.is_expanded {
            BorderRadius { top_left: RadiusScale::SM, top_right: RadiusScale::SM, bottom_left: 0.0, bottom_right: 0.0 }
        } else {
            BorderRadius::all(RadiusScale::SM)
        };
        canvas.fill_rounded_rect(header_rect, br_header, bt.colors.surface);
        canvas.stroke_rounded_rect(header_rect, br_header, bt.colors.border, 1.0);

        // Chevron: ▶ collapsed / ▼ expanded
        let chevron = if self.is_expanded { "▼" } else { "▶" };
        canvas.draw_text(chevron, Point::new(bounds.origin.x + SpacingScale::MD, bounds.origin.y + 14.0), bt.colors.text_secondary, TypeScale::BODY, None);

        // Header text
        canvas.draw_text(&self.header, Point::new(bounds.origin.x + SpacingScale::XXXL, bounds.origin.y + 14.0), bt.colors.text_primary, TypeScale::BODY, None);

        // Content panel
        if self.is_expanded {
            let content_rect = Rect::new(
                bounds.origin.x, bounds.origin.y + Self::HEADER_H,
                bounds.size.width, self.content_height,
            );
            canvas.fill_rect(content_rect, bt.colors.bg);
            canvas.stroke_rect(
                Rect::new(bounds.origin.x, bounds.origin.y + Self::HEADER_H, bounds.size.width, self.content_height),
                bt.colors.border, 1.0,
            );
            // Children placeholder text
            if self.children.is_empty() {
                canvas.draw_text(
                    "(content)",
                    Point::new(bounds.origin.x + SpacingScale::LG, bounds.origin.y + Self::HEADER_H + SpacingScale::LG),
                    bt.colors.text_placeholder,
                    TypeScale::BODY,
                    None,
                );
            }
        }
    }

    fn handle_event(&mut self, event: &Event, bounds: Rect) -> bool {
        if let Event::Pointer(p) = event {
            if p.phase == PointerPhase::Down {
                let header = Rect::new(bounds.origin.x, bounds.origin.y, bounds.size.width, Self::HEADER_H);
                if header.contains(p.position) {
                    self.is_expanded = !self.is_expanded;
                    if let Some(ref mut cb) = self.on_toggle { cb(self.is_expanded); }
                    return true;
                }
            }
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_expander_toggle() {
        let mut exp = Expander::new("Advanced Settings");
        assert!(!exp.is_expanded);
        exp.is_expanded = true;
        assert!(exp.is_expanded);
    }

    #[test]
    fn test_expander_paint_collapsed_and_expanded() {
        let exp = Expander::new("Section");
        let mut canvas = quick_render::canvas::Canvas::new();
        exp.paint(&mut canvas, Rect::new(0.0, 0.0, 300.0, 200.0));
        let collapsed_cmds = canvas.commands().len();
        assert!(collapsed_cmds > 0);

        let mut exp2 = Expander::new("Section").expanded();
        let mut canvas2 = quick_render::canvas::Canvas::new();
        exp2.paint(&mut canvas2, Rect::new(0.0, 0.0, 300.0, 200.0));
        assert!(canvas2.commands().len() > collapsed_cmds);
    }
}
