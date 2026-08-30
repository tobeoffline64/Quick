//! `NoctaliaMenuBar` — Desktop Application Menu Bar for Noctalia UI.

use crate::widget::Widget;
use quick_core::event::{Event, PointerButton, PointerEvent, PointerPhase};
use quick_core::geometry::{BorderRadius, Color, Point, Rect};
use quick_layout::engine::LayoutEngine;
use quick_render::canvas::Canvas;
use quick_style::noctalia::NoctaliaPalette;
use quick_style::property::{Dimension, Style};
use taffy::prelude::NodeId;
use taffy::TaffyError;

#[derive(Debug, Clone)]
pub struct MenuItem {
    pub label: String,
    pub shortcut: Option<String>,
}

#[derive(Debug, Clone)]
pub struct MenuSection {
    pub title: String,
    pub items: Vec<MenuItem>,
}

pub struct NoctaliaMenuBar {
    pub id: Option<String>,
    pub classes: Vec<String>,
    pub style: Style,
    pub menus: Vec<MenuSection>,
    pub open_menu_idx: Option<usize>,
    pub hovered_menu_idx: Option<usize>,
    pub on_item_click: Option<Box<dyn FnMut(usize, usize)>>,
}

impl NoctaliaMenuBar {
    pub fn new(menus: Vec<MenuSection>) -> Self {
        Self {
            id: None,
            classes: Vec::new(),
            style: Style::default(),
            menus,
            open_menu_idx: None,
            hovered_menu_idx: None,
            on_item_click: None,
        }
    }

    pub fn on_select<F: FnMut(usize, usize) + 'static>(mut self, handler: F) -> Self {
        self.on_item_click = Some(Box::new(handler));
        self
    }
}

impl Widget for NoctaliaMenuBar {
    fn widget_type(&self) -> &'static str {
        "NoctaliaMenuBar"
    }

    fn id(&self) -> Option<&str> { self.id.as_deref() }
    fn classes(&self) -> &[String] { &self.classes }
    fn style(&self) -> &Style { &self.style }
    fn style_mut(&mut self) -> &mut Style { &mut self.style }

    fn build_layout(&mut self, engine: &mut LayoutEngine) -> Result<NodeId, TaffyError> {
        let mut computed = self.style.clone();
        if computed.width.is_none() {
            computed.width = Some(Dimension::Percent(100.0));
        }
        if computed.height.is_none() {
            computed.height = Some(Dimension::Px(30.0));
        }
        engine.new_leaf(&computed)
    }

    fn paint(&self, canvas: &mut Canvas, bounds: Rect) {
        let pal = NoctaliaPalette::noctalia_dark();

        // Bar background
        canvas.fill_rounded_rect(bounds, BorderRadius::ZERO, pal.surface_variant);
        canvas.draw_line(
            Point::new(bounds.origin.x, bounds.origin.y + bounds.size.height),
            Point::new(bounds.origin.x + bounds.size.width, bounds.origin.y + bounds.size.height),
            pal.outline,
            1.0,
        );

        let mut curr_x = bounds.origin.x + 8.0;
        for (i, menu) in self.menus.iter().enumerate() {
            let font_size = 12.0;
            let tw = (menu.title.chars().count() as f32) * font_size * 0.55;
            let item_w = tw + 16.0;

            let item_rect = Rect::new(curr_x, bounds.origin.y + 2.0, item_w, bounds.size.height - 4.0);

            if self.hovered_menu_idx == Some(i) || self.open_menu_idx == Some(i) {
                canvas.fill_rounded_rect(item_rect, BorderRadius::all(4.0), Color::from_rgba(pal.primary.r, pal.primary.g, pal.primary.b, 40));
            }

            canvas.draw_text(&menu.title, Point::new(curr_x + 8.0, bounds.origin.y + 19.0), pal.on_surface, font_size, None);
            curr_x += item_w + 4.0;
        }
    }

    fn handle_event(&mut self, event: &Event, bounds: Rect) -> bool {
        if let Event::Pointer(PointerEvent { position, button, phase, .. }) = event {
            let inside = bounds.contains(*position);
            if inside {
                let mut curr_x = bounds.origin.x + 8.0;
                let mut found_idx = None;

                for (i, menu) in self.menus.iter().enumerate() {
                    let tw = (menu.title.chars().count() as f32) * 12.0 * 0.55;
                    let item_w = tw + 16.0;
                    if position.x >= curr_x && position.x <= curr_x + item_w {
                        found_idx = Some(i);
                        break;
                    }
                    curr_x += item_w + 4.0;
                }

                self.hovered_menu_idx = found_idx;

                if *phase == PointerPhase::Down && *button == Some(PointerButton::Primary) {
                    self.open_menu_idx = found_idx;
                    return true;
                }
            } else {
                self.hovered_menu_idx = None;
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
    fn test_noctalia_menu_bar_paint() {
        let menu = NoctaliaMenuBar::new(vec![
            MenuSection { title: "File".into(), items: vec![] },
            MenuSection { title: "Edit".into(), items: vec![] },
        ]);
        let mut canvas = Canvas::new();
        menu.paint(&mut canvas, Rect::new(0.0, 0.0, 800.0, 30.0));
        assert!(!canvas.commands().is_empty());
    }
}
