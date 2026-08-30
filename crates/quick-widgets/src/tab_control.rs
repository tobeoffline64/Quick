//! TabControl — Horizontal tab bar with content panel switching.
//! Follows Avalonia Fluent tab visual style: flat tabs, accent underline on active.

use crate::widget::Widget;
use quick_core::event::{Event, PointerPhase};
use quick_core::geometry::{Point, Rect};
use quick_layout::engine::LayoutEngine;
use quick_render::canvas::Canvas;
use quick_style::base::{base_theme, SpacingScale};
use quick_style::property::Style;
use taffy::prelude::NodeId;
use taffy::TaffyError;

/// A single tab item.
#[derive(Debug, Clone)]
pub struct TabItem {
    pub label: String,
    pub content: Vec<String>, // placeholder — real impl holds child widgets
}

impl TabItem {
    pub fn new(label: impl Into<String>) -> Self {
        Self { label: label.into(), content: Vec::new() }
    }
}

/// Horizontal tab control with Fluent-style underline indicator.
pub struct TabControl {
    pub id: Option<String>,
    pub classes: Vec<String>,
    pub style: Style,
    pub tabs: Vec<TabItem>,
    pub selected_index: usize,
    pub tab_height: f32,
    pub children: Vec<Box<dyn Widget>>,
    pub on_tab_change: Option<Box<dyn FnMut(usize)>>,
    child_nodes: Vec<NodeId>,
    bounds: Rect,
}

impl TabControl {
    pub fn new() -> Self {
        let bt = base_theme();
        let mut style = Style::default();
        style.background_color = Some(bt.colors.bg);
        Self {
            id: None,
            classes: Vec::new(),
            style,
            tabs: Vec::new(),
            selected_index: 0,
            tab_height: 44.0,
            children: Vec::new(),
            on_tab_change: None,
            child_nodes: Vec::new(),
            bounds: Rect::ZERO,
        }
    }

    pub fn add_tab(mut self, tab: TabItem) -> Self {
        self.tabs.push(tab);
        self
    }

    pub fn add_page(mut self, tab: TabItem, child: Box<dyn Widget>) -> Self {
        self.tabs.push(tab);
        self.children.push(child);
        self
    }

    pub fn on_change<F: FnMut(usize) + 'static>(mut self, handler: F) -> Self {
        self.on_tab_change = Some(Box::new(handler));
        self
    }

    fn tab_width(&self, bounds: &Rect) -> f32 {
        if self.tabs.is_empty() { return 140.0; }
        (bounds.size.width / self.tabs.len() as f32).max(120.0).min(280.0)
    }
}

impl Default for TabControl {
    fn default() -> Self { Self::new() }
}

impl Widget for TabControl {
    fn widget_type(&self) -> &'static str {
        "TabControl"
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
        engine.new_leaf(&self.style)
    }

    fn update_layout(&mut self, engine: &LayoutEngine, origin: Point) {
        self.bounds = Rect::new(origin.x, origin.y, self.bounds.size.width, self.bounds.size.height);
        let content_origin = Point::new(origin.x, origin.y + self.tab_height);
        for child in &mut self.children {
            child.update_layout(engine, content_origin);
        }
    }

    fn paint(&self, canvas: &mut Canvas, bounds: Rect) {
        let bt = base_theme();
        let tab_w = self.tab_width(&bounds);

        // Tab bar container background (GNOME HIG header style)
        canvas.fill_rect(
            Rect::new(bounds.origin.x, bounds.origin.y, bounds.size.width, self.tab_height),
            bt.colors.surface,
        );

        // Bottom divider
        canvas.stroke_rect(
            Rect::new(bounds.origin.x, bounds.origin.y + self.tab_height - 1.0, bounds.size.width, 1.0),
            bt.colors.border, 1.0,
        );

        // Tab items
        for (i, tab) in self.tabs.iter().enumerate() {
            let tab_x = bounds.origin.x + i as f32 * tab_w;
            let is_selected = i == self.selected_index;

            // Tab button pill background for active tab
            if is_selected {
                let pill_rect = Rect::new(
                    tab_x + 6.0,
                    bounds.origin.y + 6.0,
                    tab_w - 12.0,
                    self.tab_height - 12.0,
                );
                canvas.fill_rounded_rect(
                    pill_rect,
                    quick_core::geometry::BorderRadius::all(8.0),
                    bt.colors.bg,
                );
                canvas.stroke_rounded_rect(
                    pill_rect,
                    quick_core::geometry::BorderRadius::all(8.0),
                    bt.colors.border,
                    1.0,
                );
            }

            // Tab label text
            let text_color = if is_selected { bt.colors.accent.normal } else { bt.colors.text_secondary };
            let font_size = if is_selected { 13.0 } else { 12.0 };
            let tw = (tab.label.chars().count() as f32) * font_size * 0.55;
            let tx = tab_x + ((tab_w - tw) / 2.0).max(SpacingScale::MD);
            let ty = bounds.origin.y + (self.tab_height + font_size * 0.75) / 2.0;

            canvas.draw_text(
                &tab.label,
                Point::new(tx, ty),
                text_color,
                font_size,
                None,
            );

            // Active underline indicator
            if is_selected {
                canvas.fill_rect(
                    Rect::new(tab_x + 12.0, bounds.origin.y + self.tab_height - 3.0, tab_w - 24.0, 3.0),
                    bt.colors.accent.normal,
                );
            }
        }

        // Content area
        let content_y = bounds.origin.y + self.tab_height;
        let content_h = (bounds.size.height - self.tab_height).max(0.0);
        let content_rect = Rect::new(bounds.origin.x, content_y, bounds.size.width, content_h);

        canvas.fill_rect(content_rect, bt.colors.bg);

        // Paint selected tab child
        if let Some(child) = self.children.get(self.selected_index) {
            child.paint(canvas, content_rect);
        }
    }

    fn handle_event(&mut self, event: &Event, bounds: Rect) -> bool {
        if let Event::Pointer(p) = event {
            if bounds.contains(p.position) {
                if p.position.y < bounds.origin.y + self.tab_height {
                    if p.phase == PointerPhase::Down || p.phase == PointerPhase::Up {
                        let tab_w = self.tab_width(&bounds);
                        let rel_x = p.position.x - bounds.origin.x;
                        let idx = (rel_x / tab_w) as usize;
                        if idx < self.tabs.len() {
                            if self.selected_index != idx {
                                self.selected_index = idx;
                                if let Some(ref mut handler) = self.on_tab_change {
                                    handler(idx);
                                }
                            }
                            return true;
                        }
                    }
                    return true;
                } else {
                    let content_y = bounds.origin.y + self.tab_height;
                    let content_h = (bounds.size.height - self.tab_height).max(0.0);
                    let content_rect = Rect::new(bounds.origin.x, content_y, bounds.size.width, content_h);

                    if let Some(child) = self.children.get_mut(self.selected_index) {
                        return child.handle_event(event, content_rect);
                    }
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
    fn test_tab_control_creation_and_selection() {
        let mut tc = TabControl::new()
            .add_tab(TabItem::new("Overview"))
            .add_tab(TabItem::new("Details"))
            .add_tab(TabItem::new("Settings"));
        assert_eq!(tc.tabs.len(), 3);
        assert_eq!(tc.selected_index, 0);
        tc.selected_index = 2;
        assert_eq!(tc.selected_index, 2);
    }

    #[test]
    fn test_tab_control_paint() {
        let tc = TabControl::new()
            .add_tab(TabItem::new("Tab A"))
            .add_tab(TabItem::new("Tab B"));
        let mut canvas = quick_render::canvas::Canvas::new();
        tc.paint(&mut canvas, Rect::new(0.0, 0.0, 400.0, 300.0));
        assert!(!canvas.commands().is_empty());
    }
}
