//! `NoctaliaColorPicker` — Interactive Color Picker for Noctalia UI.

use crate::widget::Widget;
use quick_core::event::{Event, PointerButton, PointerEvent, PointerPhase};
use quick_core::geometry::{BorderRadius, Color, Point, Rect};
use quick_layout::engine::LayoutEngine;
use quick_render::canvas::Canvas;
use quick_style::noctalia::NoctaliaPalette;
use quick_style::property::{Dimension, Style};
use taffy::prelude::NodeId;
use taffy::TaffyError;

pub struct NoctaliaColorPicker {
    pub id: Option<String>,
    pub classes: Vec<String>,
    pub style: Style,
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub active_slider: Option<usize>, // 0=R, 1=G, 2=B
    pub on_color_change: Option<Box<dyn FnMut(Color)>>,
}

impl NoctaliaColorPicker {
    pub fn new(color: Color) -> Self {
        Self {
            id: None,
            classes: Vec::new(),
            style: Style::default(),
            r: color.r,
            g: color.g,
            b: color.b,
            active_slider: None,
            on_color_change: None,
        }
    }

    pub fn on_change<F: FnMut(Color) + 'static>(mut self, handler: F) -> Self {
        self.on_color_change = Some(Box::new(handler));
        self
    }
}

impl Widget for NoctaliaColorPicker {
    fn widget_type(&self) -> &'static str {
        "NoctaliaColorPicker"
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
            computed.height = Some(Dimension::Px(150.0));
        }
        engine.new_leaf(&computed)
    }

    fn paint(&self, canvas: &mut Canvas, bounds: Rect) {
        let pal = NoctaliaPalette::noctalia_dark();
        let radius = BorderRadius::all(10.0);

        // Container
        canvas.fill_rounded_rect(bounds, radius, pal.surface_variant);
        canvas.stroke_rounded_rect(bounds, radius, pal.outline, 1.0);

        // Color Preview Swatch
        let current_c = Color::from_rgb(self.r, self.g, self.b);
        let swatch_rect = Rect::new(bounds.origin.x + 12.0, bounds.origin.y + 12.0, 36.0, 36.0);
        canvas.fill_rounded_rect(swatch_rect, BorderRadius::all(6.0), current_c);
        canvas.stroke_rounded_rect(swatch_rect, BorderRadius::all(6.0), pal.outline, 1.0);

        // Hex Readout
        let hex_str = current_c.to_hex();
        canvas.draw_text(&hex_str, Point::new(bounds.origin.x + 56.0, bounds.origin.y + 34.0), pal.on_surface, 13.0, None);

        // R, G, B Sliders
        let slider_x = bounds.origin.x + 12.0;
        let slider_w = bounds.size.width - 24.0;
        let slider_h = 8.0;

        let channels = [(self.r, "R", Color::from_rgb(240, 70, 70)),
                        (self.g, "G", Color::from_rgb(70, 200, 100)),
                        (self.b, "B", Color::from_rgb(70, 140, 240))];

        for (i, (val, _lbl, col)) in channels.iter().enumerate() {
            let sy = bounds.origin.y + 60.0 + (i as f32) * 26.0;
            let tr = Rect::new(slider_x, sy, slider_w, slider_h);
            canvas.fill_rounded_rect(tr, BorderRadius::all(4.0), pal.surface);

            let fill_w = slider_w * (*val as f32 / 255.0);
            if fill_w > 0.0 {
                let fr = Rect::new(slider_x, sy, fill_w, slider_h);
                canvas.fill_rounded_rect(fr, BorderRadius::all(4.0), *col);
            }
        }
    }

    fn handle_event(&mut self, event: &Event, bounds: Rect) -> bool {
        if let Event::Pointer(PointerEvent { position, button, phase, .. }) = event {
            let slider_x = bounds.origin.x + 12.0;
            let slider_w = (bounds.size.width - 24.0).max(1.0);

            match phase {
                PointerPhase::Down if bounds.contains(*position) && *button == Some(PointerButton::Primary) => {
                    for i in 0..3 {
                        let sy = bounds.origin.y + 54.0 + (i as f32) * 26.0;
                        if position.y >= sy && position.y <= sy + 20.0 {
                            self.active_slider = Some(i);
                            let ratio = ((position.x - slider_x) / slider_w).clamp(0.0, 1.0);
                            let val = (ratio * 255.0).round() as u8;
                            match i {
                                0 => self.r = val,
                                1 => self.g = val,
                                2 => self.b = val,
                                _ => {}
                            }
                            if let Some(ref mut handler) = self.on_color_change {
                                handler(Color::from_rgb(self.r, self.g, self.b));
                            }
                            return true;
                        }
                    }
                    true
                }
                PointerPhase::Moved if self.active_slider.is_some() => {
                    if let Some(idx) = self.active_slider {
                        let ratio = ((position.x - slider_x) / slider_w).clamp(0.0, 1.0);
                        let val = (ratio * 255.0).round() as u8;
                        match idx {
                            0 => self.r = val,
                            1 => self.g = val,
                            2 => self.b = val,
                            _ => {}
                        }
                        if let Some(ref mut handler) = self.on_color_change {
                            handler(Color::from_rgb(self.r, self.g, self.b));
                        }
                    }
                    true
                }
                PointerPhase::Up if self.active_slider.is_some() => {
                    self.active_slider = None;
                    true
                }
                _ => bounds.contains(*position),
            }
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
    fn test_noctalia_color_picker_rgb() {
        let picker = NoctaliaColorPicker::new(quick_core::geometry::Color::from_rgb(255, 200, 100));
        assert_eq!(picker.r, 255);
        assert_eq!(picker.g, 200);
        assert_eq!(picker.b, 100);

        let mut canvas = Canvas::new();
        picker.paint(&mut canvas, Rect::new(0.0, 0.0, 240.0, 150.0));
        assert!(!canvas.commands().is_empty());
    }
}
