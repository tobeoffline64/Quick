//! ComboBox — Dropdown selector with a value display and collapsed option list.

use crate::widget::Widget;
use quick_core::event::{Event, PointerPhase};
use quick_core::geometry::{BorderRadius, Color, Point, Rect};
use quick_layout::engine::LayoutEngine;
use quick_render::canvas::Canvas;
use quick_style::base::{base_theme, RadiusScale, SpacingScale, TypeScale};
use quick_style::property::Style;
use taffy::prelude::NodeId;
use taffy::TaffyError;

pub struct ComboBox {
    pub id: Option<String>,
    pub classes: Vec<String>,
    pub style: Style,
    pub options: Vec<String>,
    pub selected_index: Option<usize>,
    pub is_open: bool,
    pub placeholder: String,
    pub on_change: Option<Box<dyn FnMut(usize, &str)>>,
    bounds: Rect,
}

impl ComboBox {
    pub fn new() -> Self {
        let bt = base_theme();
        let mut style = Style::default();
        style.background_color = Some(bt.colors.bg);
        style.border_color = Some(bt.colors.border);
        style.border_width = Some(1.0);
        style.border_radius = Some(BorderRadius::all(RadiusScale::SM));
        Self {
            id: None, classes: Vec::new(), style,
            options: Vec::new(), selected_index: None,
            is_open: false, placeholder: "Select…".into(),
            on_change: None, bounds: Rect::ZERO,
        }
    }

    pub fn with_options(mut self, opts: Vec<impl Into<String>>) -> Self {
        self.options = opts.into_iter().map(|o| o.into()).collect(); self
    }

    pub fn with_placeholder(mut self, p: impl Into<String>) -> Self {
        self.placeholder = p.into(); self
    }

    pub fn selected_value(&self) -> Option<&str> {
        self.selected_index.and_then(|i| self.options.get(i)).map(|s| s.as_str())
    }

    const ITEM_H: f32 = 32.0;
    const HEADER_H: f32 = 36.0;
    const ARROW: &'static str = "▾";
}

impl Default for ComboBox {
    fn default() -> Self { Self::new() }
}

impl Widget for ComboBox {
    fn widget_type(&self) -> &'static str {
        "ComboBox"
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
        let header = Rect::new(bounds.origin.x, bounds.origin.y, bounds.size.width, Self::HEADER_H);

        // Header box
        canvas.fill_rounded_rect(header, BorderRadius::all(RadiusScale::SM), bt.colors.bg);
        canvas.stroke_rounded_rect(header, BorderRadius::all(RadiusScale::SM), bt.colors.border, 1.0);

        // Label / placeholder
        let label = self.selected_value().unwrap_or(&self.placeholder);
        let text_color = if self.selected_index.is_some() { bt.colors.text_primary } else { bt.colors.text_placeholder };
        canvas.draw_text(label, Point::new(bounds.origin.x + SpacingScale::MD, bounds.origin.y + 11.0), text_color, TypeScale::BODY, None);

        // Chevron
        canvas.draw_text(Self::ARROW, Point::new(bounds.origin.x + bounds.size.width - 20.0, bounds.origin.y + 11.0), bt.colors.text_secondary, TypeScale::BODY, None);

        // Dropdown list
        if self.is_open {
            let list_y = bounds.origin.y + Self::HEADER_H + 2.0;
            let list_h = self.options.len() as f32 * Self::ITEM_H + 4.0;
            let list_rect = Rect::new(bounds.origin.x, list_y, bounds.size.width, list_h);
            canvas.fill_rounded_rect(list_rect, BorderRadius::all(RadiusScale::SM), bt.colors.surface_raised);
            canvas.stroke_rounded_rect(list_rect, BorderRadius::all(RadiusScale::SM), bt.colors.border, 1.0);

            for (i, opt) in self.options.iter().enumerate() {
                let item_y = list_y + 2.0 + i as f32 * Self::ITEM_H;
                let bg = if Some(i) == self.selected_index { bt.colors.hover_overlay } else { Color::TRANSPARENT };
                if bg != Color::TRANSPARENT {
                    canvas.fill_rect(Rect::new(bounds.origin.x + 2.0, item_y, bounds.size.width - 4.0, Self::ITEM_H), bg);
                }
                canvas.draw_text(opt, Point::new(bounds.origin.x + SpacingScale::MD, item_y + 9.0), bt.colors.text_primary, TypeScale::BODY, None);
            }
        }
    }

    fn handle_event(&mut self, event: &Event, bounds: Rect) -> bool {
        if let Event::Pointer(p) = event {
            let header = Rect::new(bounds.origin.x, bounds.origin.y, bounds.size.width, Self::HEADER_H);
            if p.phase == PointerPhase::Down {
                if header.contains(p.position) {
                    self.is_open = !self.is_open;
                    return true;
                }
                if self.is_open {
                    let list_y = bounds.origin.y + Self::HEADER_H + 2.0;
                    let rel_y = p.position.y - list_y - 2.0;
                    if rel_y >= 0.0 && p.position.x >= bounds.origin.x && p.position.x <= bounds.origin.x + bounds.size.width {
                        let idx = (rel_y / Self::ITEM_H) as usize;
                        if idx < self.options.len() {
                            self.selected_index = Some(idx);
                            self.is_open = false;
                            if let Some(ref mut cb) = self.on_change {
                                cb(idx, &self.options[idx]);
                            }
                            return true;
                        }
                    }
                    self.is_open = false;
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
    fn test_combo_box_options() {
        let cb = ComboBox::new().with_options(vec!["Alpha", "Beta", "Gamma"]);
        assert_eq!(cb.options.len(), 3);
        assert!(cb.selected_value().is_none());
    }

    #[test]
    fn test_combo_box_paint() {
        let mut cb = ComboBox::new().with_options(vec!["Option 1", "Option 2"]);
        cb.selected_index = Some(0);
        let mut canvas = quick_render::canvas::Canvas::new();
        cb.paint(&mut canvas, Rect::new(0.0, 0.0, 200.0, 40.0));
        assert!(!canvas.commands().is_empty());
    }
}
