//! `FramelessTitleBar` — 44px Draggable Header with Window Buttons for Frameless Wayland.

use crate::widget::Widget;
use quick_core::event::{Event, PointerButton, PointerEvent, PointerPhase};
use quick_core::geometry::{BorderRadius, Color, Point, Rect};
use quick_layout::engine::LayoutEngine;
use quick_render::canvas::Canvas;
use quick_style::noctalia::NoctaliaPalette;
use quick_style::property::{Dimension, Style};
use taffy::prelude::NodeId;
use taffy::TaffyError;

pub struct FramelessTitleBar {
    pub id: Option<String>,
    pub classes: Vec<String>,
    pub style: Style,
    pub title: String,
    pub subtitle: Option<String>,
    pub is_close_hovered: bool,
    pub is_min_hovered: bool,
    pub is_max_hovered: bool,
    pub on_close: Option<Box<dyn FnMut()>>,
    pub on_minimize: Option<Box<dyn FnMut()>>,
    pub on_maximize: Option<Box<dyn FnMut()>>,
}

impl FramelessTitleBar {
    pub fn new(title: impl Into<String>) -> Self {
        Self {
            id: None,
            classes: Vec::new(),
            style: Style::default(),
            title: title.into(),
            subtitle: None,
            is_close_hovered: false,
            is_min_hovered: false,
            is_max_hovered: false,
            on_close: None,
            on_minimize: None,
            on_maximize: None,
        }
    }

    pub fn subtitle(mut self, sub: impl Into<String>) -> Self {
        self.subtitle = Some(sub.into());
        self
    }
}

impl Widget for FramelessTitleBar {
    fn widget_type(&self) -> &'static str {
        "FramelessTitleBar"
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
            computed.height = Some(Dimension::Px(44.0));
        }
        engine.new_leaf(&computed)
    }

    fn paint(&self, canvas: &mut Canvas, bounds: Rect) {
        let pal = NoctaliaPalette::noctalia_dark();

        // Draggable Titlebar background
        canvas.fill_rounded_rect(bounds, BorderRadius::ZERO, pal.surface);
        canvas.draw_line(
            Point::new(bounds.origin.x, bounds.origin.y + bounds.size.height),
            Point::new(bounds.origin.x + bounds.size.width, bounds.origin.y + bounds.size.height),
            pal.outline,
            1.0,
        );

        // Title and optional Subtitle
        let title_x = bounds.origin.x + 16.0;
        let title_y = bounds.origin.y + 24.0;
        canvas.draw_text(&self.title, Point::new(title_x, title_y), pal.on_surface, 13.0, None);

        if let Some(ref sub) = self.subtitle {
            let tw = (self.title.chars().count() as f32) * 13.0 * 0.55;
            canvas.draw_text(sub, Point::new(title_x + tw + 12.0, title_y), pal.on_surface_variant, 11.0, None);
        }

        // Window Control Buttons (Minimize, Maximize, Close on right side)
        let btn_w = 40.0;
        let btn_h = bounds.size.height;
        let right_x = bounds.origin.x + bounds.size.width;

        // Close button
        let close_rect = Rect::new(right_x - btn_w, bounds.origin.y, btn_w, btn_h);
        if self.is_close_hovered {
            canvas.fill_rounded_rect(close_rect, BorderRadius::ZERO, pal.error);
        }
        canvas.draw_text("✕", Point::new(close_rect.origin.x + 15.0, bounds.origin.y + 26.0), if self.is_close_hovered { pal.on_error } else { pal.on_surface }, 12.0, None);

        // Maximize button
        let max_rect = Rect::new(right_x - btn_w * 2.0, bounds.origin.y, btn_w, btn_h);
        if self.is_max_hovered {
            canvas.fill_rounded_rect(max_rect, BorderRadius::ZERO, Color::from_rgba(pal.primary.r, pal.primary.g, pal.primary.b, 40));
        }
        canvas.draw_text("◻", Point::new(max_rect.origin.x + 15.0, bounds.origin.y + 26.0), pal.on_surface, 12.0, None);

        // Minimize button
        let min_rect = Rect::new(right_x - btn_w * 3.0, bounds.origin.y, btn_w, btn_h);
        if self.is_min_hovered {
            canvas.fill_rounded_rect(min_rect, BorderRadius::ZERO, Color::from_rgba(pal.primary.r, pal.primary.g, pal.primary.b, 40));
        }
        canvas.draw_text("—", Point::new(min_rect.origin.x + 15.0, bounds.origin.y + 26.0), pal.on_surface, 12.0, None);
    }

    fn handle_event(&mut self, event: &Event, bounds: Rect) -> bool {
        if let Event::Pointer(PointerEvent { position, button, phase, .. }) = event {
            let right_x = bounds.origin.x + bounds.size.width;
            let btn_w = 40.0;
            let close_rect = Rect::new(right_x - btn_w, bounds.origin.y, btn_w, bounds.size.height);
            let max_rect = Rect::new(right_x - btn_w * 2.0, bounds.origin.y, btn_w, bounds.size.height);
            let min_rect = Rect::new(right_x - btn_w * 3.0, bounds.origin.y, btn_w, bounds.size.height);

            self.is_close_hovered = close_rect.contains(*position);
            self.is_max_hovered = max_rect.contains(*position);
            self.is_min_hovered = min_rect.contains(*position);

            if *phase == PointerPhase::Down && *button == Some(PointerButton::Primary) {
                if self.is_close_hovered {
                    if let Some(ref mut h) = self.on_close { h(); }
                    return true;
                }
                if self.is_max_hovered {
                    if let Some(ref mut h) = self.on_maximize { h(); }
                    return true;
                }
                if self.is_min_hovered {
                    if let Some(ref mut h) = self.on_minimize { h(); }
                    return true;
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
    fn test_frameless_titlebar_paint() {
        let tb = FramelessTitleBar::new("Noctalia App").subtitle("v1.0");
        let mut canvas = Canvas::new();
        tb.paint(&mut canvas, Rect::new(0.0, 0.0, 800.0, 44.0));
        assert!(!canvas.commands().is_empty());
    }
}
