//! ListBox — Scrollable list of selectable items.
//! Avalonia-style: bordered container, highlight on hover/selected.

use crate::widget::Widget;
use quick_core::event::{Event, PointerPhase};
use quick_core::geometry::{Color, Point, Rect};
use quick_layout::engine::LayoutEngine;
use quick_render::canvas::Canvas;
use quick_style::base::{base_theme, RadiusScale, SpacingScale, TypeScale};
use quick_style::property::Style;
use taffy::prelude::NodeId;
use taffy::TaffyError;

pub struct ListItem {
    pub label: String,
    pub value: String,
}

impl ListItem {
    pub fn new(label: impl Into<String>) -> Self {
        let l = label.into();
        Self { value: l.clone(), label: l }
    }
    pub fn with_value(mut self, value: impl Into<String>) -> Self {
        self.value = value.into(); self
    }
}

pub struct ListBox {
    pub id: Option<String>,
    pub classes: Vec<String>,
    pub style: Style,
    pub items: Vec<ListItem>,
    pub selected_index: Option<usize>,
    pub hovered_index: Option<usize>,
    pub item_height: f32,
    pub on_select: Option<Box<dyn FnMut(&str)>>,
    bounds: Rect,
}

impl ListBox {
    pub fn new() -> Self {
        let bt = base_theme();
        let mut style = Style::default();
        style.background_color = Some(bt.colors.bg);
        style.border_color = Some(bt.colors.border);
        style.border_width = Some(1.0);
        style.border_radius = Some(quick_core::geometry::BorderRadius::all(RadiusScale::SM));
        Self {
            id: None, classes: Vec::new(), style,
            items: Vec::new(), selected_index: None,
            hovered_index: None, item_height: 36.0,
            on_select: None, bounds: Rect::ZERO,
        }
    }

    pub fn add_item(mut self, item: ListItem) -> Self {
        self.items.push(item); self
    }

    fn item_at(&self, pos: Point, bounds: &Rect) -> Option<usize> {
        if !bounds.contains(pos) { return None; }
        let rel_y = pos.y - bounds.origin.y;
        let idx = (rel_y / self.item_height) as usize;
        if idx < self.items.len() { Some(idx) } else { None }
    }
}

impl Default for ListBox {
    fn default() -> Self { Self::new() }
}

impl Widget for ListBox {
    fn widget_type(&self) -> &'static str {
        "ListBox"
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
        // Container background + border
        canvas.fill_rounded_rect(bounds, quick_core::geometry::BorderRadius::all(RadiusScale::SM), bt.colors.bg);
        canvas.stroke_rounded_rect(bounds, quick_core::geometry::BorderRadius::all(RadiusScale::SM), bt.colors.border, 1.0);

        // Items
        for (i, item) in self.items.iter().enumerate() {
            let item_y = bounds.origin.y + i as f32 * self.item_height;
            let item_rect = Rect::new(bounds.origin.x + 1.0, item_y, bounds.size.width - 2.0, self.item_height);

            let bg = if Some(i) == self.selected_index {
                bt.colors.accent.normal
            } else if Some(i) == self.hovered_index {
                bt.colors.hover_overlay
            } else {
                Color::TRANSPARENT
            };

            if bg != Color::TRANSPARENT {
                canvas.fill_rect(item_rect, bg);
            }

            let text_color = if Some(i) == self.selected_index {
                bt.colors.accent.on_accent
            } else {
                bt.colors.text_primary
            };

            canvas.draw_text(
                &item.label,
                Point::new(item_rect.origin.x + SpacingScale::MD, item_y + self.item_height / 2.0 - TypeScale::BODY / 2.0),
                text_color,
                TypeScale::BODY,
                None,
            );
        }
    }

    fn handle_event(&mut self, event: &Event, bounds: Rect) -> bool {
        match event {
            Event::Pointer(p) => {
                let idx = self.item_at(p.position, &bounds);
                match p.phase {
                    PointerPhase::Moved => { self.hovered_index = idx; }
                    PointerPhase::Down => {
                        if let Some(i) = idx {
                            self.selected_index = Some(i);
                            if let Some(ref mut cb) = self.on_select {
                                cb(&self.items[i].value);
                            }
                            return true;
                        }
                    }
                    _ => {}
                }
            }
            _ => {}
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_list_box_items_and_selection() {
        let mut lb = ListBox::new()
            .add_item(ListItem::new("Apple"))
            .add_item(ListItem::new("Banana"))
            .add_item(ListItem::new("Cherry"));
        assert_eq!(lb.items.len(), 3);
        lb.selected_index = Some(1);
        assert_eq!(lb.selected_index, Some(1));
    }

    #[test]
    fn test_list_box_paint() {
        let lb = ListBox::new()
            .add_item(ListItem::new("Item 1"))
            .add_item(ListItem::new("Item 2"));
        let mut canvas = quick_render::canvas::Canvas::new();
        lb.paint(&mut canvas, Rect::new(0.0, 0.0, 200.0, 150.0));
        assert!(!canvas.commands().is_empty());
    }
}
