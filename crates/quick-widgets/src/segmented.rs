//! `Segmented` — Segmented Pill Selector for Noctalia UI.

use crate::widget::Widget;
use quick_core::event::{Event, PointerButton, PointerEvent, PointerPhase};
use quick_core::geometry::{BorderRadius, Point, Rect};
use quick_layout::engine::LayoutEngine;
use quick_render::canvas::Canvas;
use quick_style::noctalia::NoctaliaPalette;
use quick_style::property::{Dimension, Style};
use taffy::prelude::NodeId;
use taffy::TaffyError;

pub struct Segmented {
    pub id: Option<String>,
    pub classes: Vec<String>,
    pub style: Style,
    pub items: Vec<String>,
    pub selected_index: usize,
    pub hovered_index: Option<usize>,
    pub on_select: Option<Box<dyn FnMut(usize)>>,
}

impl Segmented {
    pub fn new(items: Vec<String>, selected_index: usize) -> Self {
        Self {
            id: None,
            classes: Vec::new(),
            style: Style::default(),
            items,
            selected_index,
            hovered_index: None,
            on_select: None,
        }
    }

    pub fn on_select<F: FnMut(usize) + 'static>(mut self, handler: F) -> Self {
        self.on_select = Some(Box::new(handler));
        self
    }
}

impl Widget for Segmented {
    fn widget_type(&self) -> &'static str {
        "Segmented"
    }

    fn id(&self) -> Option<&str> { self.id.as_deref() }
    fn classes(&self) -> &[String] { &self.classes }
    fn style(&self) -> &Style { &self.style }
    fn style_mut(&mut self) -> &mut Style { &mut self.style }

    fn build_layout(&mut self, engine: &mut LayoutEngine) -> Result<NodeId, TaffyError> {
        let count = self.items.len().max(1) as f32;
        let estimated_w = (count * 90.0).max(180.0);

        let mut computed = self.style.clone();
        if computed.width.is_none() {
            computed.width = Some(Dimension::Px(estimated_w));
        }
        if computed.height.is_none() {
            computed.height = Some(Dimension::Px(34.0));
        }
        engine.new_leaf(&computed)
    }

    fn paint(&self, canvas: &mut Canvas, bounds: Rect) {
        let pal = NoctaliaPalette::noctalia_dark();
        let radius = BorderRadius::all(8.0);

        // Container Track
        canvas.fill_rounded_rect(bounds, radius, pal.surface_variant);
        canvas.stroke_rounded_rect(bounds, radius, pal.outline, 1.0);

        let count = self.items.len();
        if count == 0 {
            return;
        }

        let pad = 2.0;
        let seg_w = (bounds.size.width - pad * 2.0) / count as f32;
        let seg_h = bounds.size.height - pad * 2.0;

        // Active Segment Highlight
        if self.selected_index < count {
            let active_x = bounds.origin.x + pad + (self.selected_index as f32) * seg_w;
            let active_y = bounds.origin.y + pad;
            let active_rect = Rect::new(active_x, active_y, seg_w, seg_h);
            canvas.fill_rounded_rect(active_rect, BorderRadius::all(6.0), pal.primary);
        }

        // Segment Labels
        for (i, item) in self.items.iter().enumerate() {
            let seg_x = bounds.origin.x + pad + (i as f32) * seg_w;
            let is_selected = i == self.selected_index;
            let fg = if is_selected { pal.on_primary } else { pal.on_surface_variant };

            let font_size = 12.0;
            let tw = (item.chars().count() as f32) * font_size * 0.55;
            let tx = seg_x + (seg_w - tw) / 2.0;
            let ty = bounds.origin.y + (bounds.size.height + font_size * 0.75) / 2.0;

            canvas.draw_text(item, Point::new(tx, ty), fg, font_size, None);
        }
    }

    fn handle_event(&mut self, event: &Event, bounds: Rect) -> bool {
        if let Event::Pointer(PointerEvent { position, button, phase, .. }) = event {
            let inside = bounds.contains(*position);
            if inside {
                let pad = 2.0;
                let count = self.items.len().max(1);
                let seg_w = (bounds.size.width - pad * 2.0) / count as f32;
                let rel_x = position.x - (bounds.origin.x + pad);
                let clicked_idx = ((rel_x / seg_w) as usize).min(count - 1);

                self.hovered_index = Some(clicked_idx);

                if *phase == PointerPhase::Down && *button == Some(PointerButton::Primary) {
                    if self.selected_index != clicked_idx {
                        self.selected_index = clicked_idx;
                        if let Some(ref mut handler) = self.on_select {
                            handler(clicked_idx);
                        }
                    }
                    return true;
                }
            } else {
                self.hovered_index = None;
            }
            inside
        } else {
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use quick_render::canvas::Canvas;

    #[test]
    fn test_segmented_selection() {
        let seg = Segmented::new(vec!["Tab 1".into(), "Tab 2".into()], 0);
        assert_eq!(seg.selected_index, 0);

        let mut canvas = Canvas::new();
        seg.paint(&mut canvas, Rect::new(0.0, 0.0, 200.0, 34.0));
        assert!(!canvas.commands().is_empty());
    }
}
