//! `NoctaliaBar` — Wayland Desktop Shell Status Bar for Noctalia UI.

use crate::widget::Widget;
use quick_core::event::{Event, PointerButton, PointerEvent, PointerPhase};
use quick_core::geometry::{BorderRadius, Color, Point, Rect};
use quick_layout::engine::LayoutEngine;
use quick_render::canvas::Canvas;
use quick_style::noctalia::NoctaliaPalette;
use quick_style::property::{Dimension, Style};
use taffy::prelude::NodeId;
use taffy::TaffyError;

pub struct NoctaliaBar {
    pub id: Option<String>,
    pub classes: Vec<String>,
    pub style: Style,
    pub active_workspace: usize,
    pub workspace_count: usize,
    pub window_title: String,
    pub time_str: String,
    pub battery_percent: u32,
    pub volume_percent: u32,
    pub wifi_percent: u32,
    pub on_workspace_switch: Option<Box<dyn FnMut(usize)>>,
}

impl NoctaliaBar {
    pub fn new() -> Self {
        Self {
            id: None,
            classes: Vec::new(),
            style: Style::default(),
            active_workspace: 1,
            workspace_count: 5,
            window_title: "Terminal — bash".into(),
            time_str: "20:49 PM".into(),
            battery_percent: 92,
            volume_percent: 80,
            wifi_percent: 84,
            on_workspace_switch: None,
        }
    }

    pub fn on_workspace<F: FnMut(usize) + 'static>(mut self, handler: F) -> Self {
        self.on_workspace_switch = Some(Box::new(handler));
        self
    }
}

impl Default for NoctaliaBar {
    fn default() -> Self {
        Self::new()
    }
}

impl Widget for NoctaliaBar {
    fn widget_type(&self) -> &'static str {
        "NoctaliaBar"
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
            computed.height = Some(Dimension::Px(34.0));
        }
        engine.new_leaf(&computed)
    }

    fn paint(&self, canvas: &mut Canvas, bounds: Rect) {
        let pal = NoctaliaPalette::noctalia_dark();

        // Bar background & bottom border
        canvas.fill_rounded_rect(bounds, BorderRadius::ZERO, pal.surface_variant);
        canvas.draw_line(
            Point::new(bounds.origin.x, bounds.origin.y + bounds.size.height),
            Point::new(bounds.origin.x + bounds.size.width, bounds.origin.y + bounds.size.height),
            pal.outline,
            1.0,
        );

        // 1. Left Section: Workspaces (1, 2, 3, 4, 5)
        let mut ws_x = bounds.origin.x + 12.0;
        let ws_w = 26.0;
        let ws_h = 22.0;
        let ws_y = bounds.origin.y + (bounds.size.height - ws_h) / 2.0;

        for i in 1..=self.workspace_count {
            let ws_rect = Rect::new(ws_x, ws_y, ws_w, ws_h);
            if i == self.active_workspace {
                canvas.fill_rounded_rect(ws_rect, BorderRadius::all(6.0), pal.primary);
                canvas.draw_text(&format!("{}", i), Point::new(ws_x + 9.0, ws_y + 15.0), pal.on_primary, 11.0, None);
            } else {
                canvas.fill_rounded_rect(ws_rect, BorderRadius::all(6.0), Color::from_rgba(pal.primary.r, pal.primary.g, pal.primary.b, 20));
                canvas.draw_text(&format!("{}", i), Point::new(ws_x + 9.0, ws_y + 15.0), pal.on_surface_variant, 11.0, None);
            }
            ws_x += ws_w + 6.0;
        }

        // 2. Center Section: Active Window Title
        let font_size = 12.0;
        let title_w = (self.window_title.chars().count() as f32) * font_size * 0.55;
        let title_x = bounds.origin.x + (bounds.size.width - title_w) / 2.0;
        let title_y = bounds.origin.y + (bounds.size.height + font_size * 0.75) / 2.0;
        canvas.draw_text(&self.window_title, Point::new(title_x, title_y), pal.on_surface, font_size, None);

        // 3. Right Section: System Metrics (WiFi, Volume, Battery, Clock)
        let status_str = format!("📶 {}%  🔊 {}%  🔋 {}%  ⏰ {}", self.wifi_percent, self.volume_percent, self.battery_percent, self.time_str);
        let status_w = (status_str.chars().count() as f32) * 11.0 * 0.55;
        let status_x = bounds.origin.x + bounds.size.width - status_w - 16.0;
        let status_y = bounds.origin.y + (bounds.size.height + 11.0 * 0.75) / 2.0;
        canvas.draw_text(&status_str, Point::new(status_x, status_y), pal.on_surface_variant, 11.0, None);
    }

    fn handle_event(&mut self, event: &Event, bounds: Rect) -> bool {
        if let Event::Pointer(PointerEvent { position, button, phase, .. }) = event {
            if *phase == PointerPhase::Down && *button == Some(PointerButton::Primary) && bounds.contains(*position) {
                let ws_w = 26.0;
                let ws_step = ws_w + 6.0;
                let ws_start_x = bounds.origin.x + 12.0;

                if position.x >= ws_start_x && position.x <= ws_start_x + (self.workspace_count as f32) * ws_step {
                    let clicked_ws = (((position.x - ws_start_x) / ws_step) as usize) + 1;
                    if clicked_ws <= self.workspace_count {
                        self.active_workspace = clicked_ws;
                        if let Some(ref mut handler) = self.on_workspace_switch {
                            handler(clicked_ws);
                        }
                        return true;
                    }
                }
            }
            bounds.contains(*position)
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
    fn test_noctalia_bar_workspaces() {
        let bar = NoctaliaBar::new();
        assert_eq!(bar.active_workspace, 1);
        assert_eq!(bar.workspace_count, 5);

        let mut canvas = Canvas::new();
        bar.paint(&mut canvas, Rect::new(0.0, 0.0, 1200.0, 34.0));
        assert!(!canvas.commands().is_empty());
    }
}
