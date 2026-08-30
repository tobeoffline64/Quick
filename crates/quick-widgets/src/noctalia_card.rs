//! `NoctaliaCard` — Acrylic Glassmorphic Container for Noctalia UI.

use crate::widget::Widget;
use quick_core::event::Event;
use quick_core::geometry::{BorderRadius, Color, Rect};
use quick_layout::engine::LayoutEngine;
use quick_render::canvas::Canvas;
use quick_style::noctalia::{NoctaliaGlassTokens, NoctaliaPalette};
use quick_style::property::Style;
use quick_style::theme::tokens::Shadow;
use taffy::prelude::NodeId;
use taffy::TaffyError;

pub struct NoctaliaCard {
    pub id: Option<String>,
    pub classes: Vec<String>,
    pub style: Style,
    pub children: Vec<Box<dyn Widget>>,
    pub is_acrylic: bool,
    pub elevation: f32,
}

impl NoctaliaCard {
    pub fn new() -> Self {
        Self {
            id: None,
            classes: Vec::new(),
            style: Style::default(),
            children: Vec::new(),
            is_acrylic: true,
            elevation: 1.0,
        }
    }

    pub fn acrylic(mut self, acrylic: bool) -> Self {
        self.is_acrylic = acrylic;
        self
    }

    pub fn elevation(mut self, elevation: f32) -> Self {
        self.elevation = elevation;
        self
    }

    pub fn child(mut self, child: impl Widget + 'static) -> Self {
        self.children.push(Box::new(child));
        self
    }
}

impl Default for NoctaliaCard {
    fn default() -> Self {
        Self::new()
    }
}

impl Widget for NoctaliaCard {
    fn widget_type(&self) -> &'static str {
        "NoctaliaCard"
    }

    fn id(&self) -> Option<&str> { self.id.as_deref() }
    fn classes(&self) -> &[String] { &self.classes }
    fn style(&self) -> &Style { &self.style }
    fn style_mut(&mut self) -> &mut Style { &mut self.style }

    fn build_layout(&mut self, engine: &mut LayoutEngine) -> Result<NodeId, TaffyError> {
        let mut child_nodes = Vec::with_capacity(self.children.len());
        for child in &mut self.children {
            child_nodes.push(child.build_layout(engine)?);
        }

        let mut computed = self.style.clone();
        if computed.padding.is_none() {
            computed.padding = Some(quick_core::geometry::Insets::all(16.0));
        }
        if computed.gap.is_none() {
            computed.gap = Some(12.0);
        }

        engine.new_with_children(&computed, &child_nodes)
    }

    fn paint(&self, canvas: &mut Canvas, bounds: Rect) {
        let pal = NoctaliaPalette::noctalia_dark();
        let tokens = NoctaliaGlassTokens::default();
        let radius = self.style.border_radius.unwrap_or(BorderRadius::all(12.0));

        // Soft elevation shadow
        if self.elevation > 0.0 {
            canvas.draw_shadow(bounds, radius, Shadow {
                offset_x: 0.0,
                offset_y: self.elevation * 2.0,
                blur_radius: self.elevation * 6.0,
                spread_radius: 0.0,
                color: Color::from_rgba(0, 0, 15, 80),
            });
        }

        // Acrylic glassmorphic background
        let bg = if self.is_acrylic {
            Color::from_rgba(pal.surface_variant.r, pal.surface_variant.g, pal.surface_variant.b, (255.0 * tokens.card_acrylic_opacity) as u8)
        } else {
            pal.surface_variant
        };

        canvas.fill_rounded_rect(bounds, radius, bg);

        // Subtle 1px glass border
        let border_color = self.style.border_color.unwrap_or(pal.outline);
        canvas.stroke_rounded_rect(bounds, radius, border_color, 1.0);

        for child in &self.children {
            let child_bounds = Rect::new(bounds.origin.x + 16.0, bounds.origin.y + 16.0, bounds.size.width - 32.0, bounds.size.height - 32.0);
            child.paint(canvas, child_bounds);
        }
    }

    fn handle_event(&mut self, event: &Event, bounds: Rect) -> bool {
        let mut handled = false;
        for child in &mut self.children {
            let child_bounds = Rect::new(bounds.origin.x + 16.0, bounds.origin.y + 16.0, bounds.size.width - 32.0, bounds.size.height - 32.0);
            if child.handle_event(event, child_bounds) {
                handled = true;
            }
        }
        handled
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use quick_render::canvas::Canvas;

    #[test]
    fn test_noctalia_card_paint() {
        let card = NoctaliaCard::new().acrylic(true).elevation(2.0);
        let mut canvas = Canvas::new();
        card.paint(&mut canvas, Rect::new(0.0, 0.0, 300.0, 200.0));
        assert!(!canvas.commands().is_empty());
    }
}
