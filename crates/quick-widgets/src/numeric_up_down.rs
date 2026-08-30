//! NumericUpDown — Numeric stepper with + / - buttons and optional bounds clamping.

use crate::widget::Widget;
use quick_core::event::{Event, PointerPhase};
use quick_core::geometry::{BorderRadius, Point, Rect};
use quick_layout::engine::LayoutEngine;
use quick_render::canvas::Canvas;
use quick_style::base::{base_theme, RadiusScale, SpacingScale, TypeScale};
use quick_style::property::Style;
use quick_core::signals::Signal;
use taffy::prelude::NodeId;
use taffy::TaffyError;

pub struct NumericUpDown {
    pub id: Option<String>,
    pub classes: Vec<String>,
    pub style: Style,
    pub value: Signal<f64>,
    pub min: f64,
    pub max: f64,
    pub step: f64,
    pub decimal_places: usize,
    pub on_change: Option<Box<dyn FnMut(f64)>>,
    bounds: Rect,
}

impl NumericUpDown {
    pub fn new(value: Signal<f64>) -> Self {
        let bt = base_theme();
        let mut style = Style::default();
        style.background_color = Some(bt.colors.bg);
        style.border_color = Some(bt.colors.border);
        style.border_width = Some(1.0);
        style.border_radius = Some(BorderRadius::all(RadiusScale::SM));
        Self {
            id: None, classes: Vec::new(), style,
            value, min: f64::NEG_INFINITY, max: f64::INFINITY,
            step: 1.0, decimal_places: 0, on_change: None, bounds: Rect::ZERO,
        }
    }

    pub fn with_range(mut self, min: f64, max: f64) -> Self {
        self.min = min; self.max = max; self
    }

    pub fn with_step(mut self, step: f64) -> Self { self.step = step; self }
    pub fn with_decimals(mut self, d: usize) -> Self { self.decimal_places = d; self }

    fn increment(&mut self, delta: f64) {
        let v = (self.value.get_untracked() + delta).clamp(self.min, self.max);
        self.value.set(v);
        if let Some(ref mut cb) = self.on_change { cb(v); }
    }

    const BTN_W: f32 = 28.0;
    const HEIGHT: f32 = 36.0;
}

impl Widget for NumericUpDown {
    fn widget_type(&self) -> &'static str {
        "NumericUpDown"
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
        let h = Self::HEIGHT.min(bounds.size.height);

        // Container
        canvas.fill_rounded_rect(
            Rect::new(bounds.origin.x, bounds.origin.y, bounds.size.width, h),
            BorderRadius::all(RadiusScale::SM),
            bt.colors.bg,
        );
        canvas.stroke_rounded_rect(
            Rect::new(bounds.origin.x, bounds.origin.y, bounds.size.width, h),
            BorderRadius::all(RadiusScale::SM),
            bt.colors.border, 1.0,
        );

        // Minus button
        let minus_rect = Rect::new(bounds.origin.x, bounds.origin.y, Self::BTN_W, h);
        canvas.fill_rect(minus_rect, bt.colors.surface);
        canvas.draw_text("−", Point::new(bounds.origin.x + 8.0, bounds.origin.y + 10.0), bt.colors.text_primary, TypeScale::BODY, None);

        // Plus button
        let plus_x = bounds.origin.x + bounds.size.width - Self::BTN_W;
        let plus_rect = Rect::new(plus_x, bounds.origin.y, Self::BTN_W, h);
        canvas.fill_rect(plus_rect, bt.colors.surface);
        canvas.draw_text("+", Point::new(plus_x + 8.0, bounds.origin.y + 10.0), bt.colors.text_primary, TypeScale::BODY, None);

        // Value display
        let v = self.value.get_untracked();
        let text = if self.decimal_places == 0 { format!("{v:.0}") } else { format!("{v:.prec$}", prec = self.decimal_places) };
        canvas.draw_text(
            &text,
            Point::new(bounds.origin.x + Self::BTN_W + SpacingScale::SM, bounds.origin.y + 10.0),
            bt.colors.text_primary,
            TypeScale::BODY,
            None,
        );
    }

    fn handle_event(&mut self, event: &Event, bounds: Rect) -> bool {
        if let Event::Pointer(p) = event {
            if p.phase == PointerPhase::Down {
                let h = Self::HEIGHT.min(bounds.size.height);
                let minus_rect = Rect::new(bounds.origin.x, bounds.origin.y, Self::BTN_W, h);
                let plus_rect = Rect::new(bounds.origin.x + bounds.size.width - Self::BTN_W, bounds.origin.y, Self::BTN_W, h);
                if minus_rect.contains(p.position) { self.increment(-self.step); return true; }
                if plus_rect.contains(p.position)  { self.increment( self.step); return true; }
            }
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_numeric_up_down_increment() {
        let sig = Signal::new(5.0f64);
        let mut nud = NumericUpDown::new(sig.clone()).with_range(0.0, 10.0).with_step(1.0);
        nud.increment(1.0);
        assert_eq!(sig.get_untracked(), 6.0);
        nud.increment(-3.0);
        assert_eq!(sig.get_untracked(), 3.0);
    }

    #[test]
    fn test_numeric_up_down_clamps() {
        let sig = Signal::new(9.5f64);
        let mut nud = NumericUpDown::new(sig.clone()).with_range(0.0, 10.0);
        nud.increment(5.0);
        assert_eq!(sig.get_untracked(), 10.0);
    }
}
