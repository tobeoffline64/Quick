//! `NoctaliaCalendar` — Month View Calendar Grid for Noctalia UI.

use crate::widget::Widget;
use quick_core::event::{Event, PointerButton, PointerEvent, PointerPhase};
use quick_core::geometry::{BorderRadius, Point, Rect};
use quick_layout::engine::LayoutEngine;
use quick_render::canvas::Canvas;
use quick_style::noctalia::NoctaliaPalette;
use quick_style::property::{Dimension, Style};
use taffy::prelude::NodeId;
use taffy::TaffyError;

pub struct NoctaliaCalendar {
    pub id: Option<String>,
    pub classes: Vec<String>,
    pub style: Style,
    pub month_name: String,
    pub year: u32,
    pub selected_day: u32,
    pub days_in_month: u32,
    pub start_day_offset: u32, // 0 = Sun, 1 = Mon...
    pub on_select_day: Option<Box<dyn FnMut(u32)>>,
}

impl NoctaliaCalendar {
    pub fn new(month_name: impl Into<String>, year: u32, selected_day: u32) -> Self {
        Self {
            id: None,
            classes: Vec::new(),
            style: Style::default(),
            month_name: month_name.into(),
            year,
            selected_day,
            days_in_month: 31,
            start_day_offset: 2, // Tuesday default
            on_select_day: None,
        }
    }

    pub fn on_select<F: FnMut(u32) + 'static>(mut self, handler: F) -> Self {
        self.on_select_day = Some(Box::new(handler));
        self
    }
}

impl Widget for NoctaliaCalendar {
    fn widget_type(&self) -> &'static str {
        "NoctaliaCalendar"
    }

    fn id(&self) -> Option<&str> { self.id.as_deref() }
    fn classes(&self) -> &[String] { &self.classes }
    fn style(&self) -> &Style { &self.style }
    fn style_mut(&mut self) -> &mut Style { &mut self.style }

    fn build_layout(&mut self, engine: &mut LayoutEngine) -> Result<NodeId, TaffyError> {
        let mut computed = self.style.clone();
        if computed.width.is_none() {
            computed.width = Some(Dimension::Px(240.0));
        }
        if computed.height.is_none() {
            computed.height = Some(Dimension::Px(220.0));
        }
        engine.new_leaf(&computed)
    }

    fn paint(&self, canvas: &mut Canvas, bounds: Rect) {
        let pal = NoctaliaPalette::noctalia_dark();
        let radius = BorderRadius::all(10.0);

        // Background
        canvas.fill_rounded_rect(bounds, radius, pal.surface_variant);
        canvas.stroke_rounded_rect(bounds, radius, pal.outline, 1.0);

        // Header (Month Year)
        let header_text = format!("{} {}", self.month_name, self.year);
        canvas.draw_text(&header_text, Point::new(bounds.origin.x + 12.0, bounds.origin.y + 20.0), pal.on_surface, 13.0, None);

        // Day of Week Headers (S, M, T, W, T, F, S)
        let days_abbr = ["Su", "Mo", "Tu", "We", "Th", "Fr", "Sa"];
        let cell_w = (bounds.size.width - 16.0) / 7.0;
        let cell_h = 24.0;
        let start_y = bounds.origin.y + 36.0;

        for (i, abbr) in days_abbr.iter().enumerate() {
            let x = bounds.origin.x + 8.0 + (i as f32) * cell_w;
            canvas.draw_text(*abbr, Point::new(x + 4.0, start_y + 12.0), pal.on_surface_variant, 10.0, None);
        }

        // Days Grid
        let grid_y = start_y + 20.0;
        for day in 1..=self.days_in_month {
            let slot = (day - 1 + self.start_day_offset) as usize;
            let row = slot / 7;
            let col = slot % 7;

            let cx = bounds.origin.x + 8.0 + (col as f32) * cell_w;
            let cy = grid_y + (row as f32) * cell_h;

            let cell_rect = Rect::new(cx + 2.0, cy, cell_w - 4.0, cell_h - 2.0);

            if day == self.selected_day {
                canvas.fill_rounded_rect(cell_rect, BorderRadius::all(4.0), pal.primary);
                let day_str = format!("{}", day);
                canvas.draw_text(&day_str, Point::new(cx + 8.0, cy + 15.0), pal.on_primary, 11.0, None);
            } else {
                let day_str = format!("{}", day);
                canvas.draw_text(&day_str, Point::new(cx + 8.0, cy + 15.0), pal.on_surface, 11.0, None);
            }
        }
    }

    fn handle_event(&mut self, event: &Event, bounds: Rect) -> bool {
        if let Event::Pointer(PointerEvent { position, button, phase, .. }) = event {
            if *phase == PointerPhase::Down && *button == Some(PointerButton::Primary) {
                let grid_y = bounds.origin.y + 56.0;
                let cell_w = (bounds.size.width - 16.0) / 7.0;
                let cell_h = 24.0;

                if position.y >= grid_y {
                    let col = (((position.x - bounds.origin.x - 8.0) / cell_w) as i32).clamp(0, 6) as usize;
                    let row = (((position.y - grid_y) / cell_h) as i32).clamp(0, 5) as usize;
                    let slot = row * 7 + col;

                    if slot >= self.start_day_offset as usize {
                        let day = (slot - self.start_day_offset as usize + 1) as u32;
                        if day <= self.days_in_month {
                            self.selected_day = day;
                            if let Some(ref mut handler) = self.on_select_day {
                                handler(day);
                            }
                            return true;
                        }
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
    fn test_noctalia_calendar_selection() {
        let cal = NoctaliaCalendar::new("August", 2026, 30);
        assert_eq!(cal.selected_day, 30);

        let mut canvas = Canvas::new();
        cal.paint(&mut canvas, Rect::new(0.0, 0.0, 240.0, 220.0));
        assert!(!canvas.commands().is_empty());
    }
}
