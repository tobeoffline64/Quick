//! TabControl — Horizontal tab bar with content panel switching.
//! Follows Avalonia Fluent tab visual style: flat tabs, accent underline on active.

use crate::widget::Widget;
use quick_core::event::{Event, PointerPhase};
use quick_core::geometry::{Point, Rect};
use quick_layout::engine::LayoutEngine;
use quick_render::canvas::Canvas;
use quick_style::base::{base_theme, SpacingScale, TypeScale};
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
            tab_height: 40.0,
            bounds: Rect::ZERO,
        }
    }

    pub fn add_tab(mut self, tab: TabItem) -> Self {
        self.tabs.push(tab);
        self
    }

    fn tab_width(&self, bounds: &Rect) -> f32 {
        if self.tabs.is_empty() { return 120.0; }
        (bounds.size.width / self.tabs.len() as f32).max(80.0).min(200.0)
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
        engine.new_leaf(&self.style)
    }

    fn update_layout(&mut self, _engine: &LayoutEngine, origin: Point) {
        self.bounds = Rect::new(origin.x, origin.y, self.bounds.size.width, self.bounds.size.height);
    }

    fn paint(&self, canvas: &mut Canvas, bounds: Rect) {
        let bt = base_theme();
        let tab_w = self.tab_width(&bounds);

        // Tab bar background
        canvas.fill_rect(
            Rect::new(bounds.origin.x, bounds.origin.y, bounds.size.width, self.tab_height),
            bt.colors.surface,
        );

        // Bottom divider of tab bar
        canvas.stroke_rect(
            Rect::new(bounds.origin.x, bounds.origin.y + self.tab_height - 1.0, bounds.size.width, 1.0),
            bt.colors.border, 1.0,
        );

        // Tab items
        for (i, tab) in self.tabs.iter().enumerate() {
            let tab_x = bounds.origin.x + i as f32 * tab_w;
            let is_selected = i == self.selected_index;

            // Tab label
            let text_color = if is_selected { bt.colors.accent.normal } else { bt.colors.text_secondary };
            canvas.draw_text(
                &tab.label,
                Point::new(tab_x + SpacingScale::LG, bounds.origin.y + self.tab_height / 2.0 - TypeScale::BODY / 2.0),
                text_color,
                TypeScale::BODY,
                None,
            );

            // Active underline
            if is_selected {
                canvas.fill_rect(
                    Rect::new(tab_x + 2.0, bounds.origin.y + self.tab_height - 3.0, tab_w - 4.0, 3.0),
                    bt.colors.accent.normal,
                );
            }
        }

        // Content area
        let content_y = bounds.origin.y + self.tab_height + SpacingScale::MD;
        let content_h = bounds.size.height - self.tab_height - SpacingScale::MD;
        canvas.fill_rect(
            Rect::new(bounds.origin.x, content_y, bounds.size.width, content_h),
            bt.colors.bg,
        );
    }

    fn handle_event(&mut self, event: &Event, bounds: Rect) -> bool {
        if let Event::Pointer(p) = event {
            if p.phase == PointerPhase::Up && bounds.contains(p.position) {
                if p.position.y < bounds.origin.y + self.tab_height {
                    let tab_w = self.tab_width(&bounds);
                    let rel_x = p.position.x - bounds.origin.x;
                    let idx = (rel_x / tab_w) as usize;
                    if idx < self.tabs.len() {
                        self.selected_index = idx;
                        return true;
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
