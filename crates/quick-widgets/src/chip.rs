use crate::widget::Widget;
use quick_core::event::{Event, PointerButton, PointerEvent, PointerPhase};
use quick_core::geometry::{BorderRadius, Color, Insets, Point, Rect};
use quick_core::signals::Signal;
use quick_layout::engine::LayoutEngine;
use quick_render::canvas::Canvas;
use quick_style::property::{Dimension, Style};
use serde::{Deserialize, Serialize};
use taffy::prelude::NodeId;
use taffy::TaffyError;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum ChipVariant {
    #[default]
    Filter,
    Assist,
    Input,
    Suggestion,
}

pub struct Chip {
    pub id: Option<String>,
    pub classes: Vec<String>,
    pub style: Style,
    pub text: String,
    pub variant: ChipVariant,
    pub selected: Option<Signal<bool>>,
    pub on_click: Option<Box<dyn FnMut()>>,
    pub is_disabled: bool,
    pub is_hovered: bool,
    pub is_pressed: bool,
}

impl Chip {
    pub fn new(text: impl Into<String>) -> Self {
        let mut style = Style::default();
        style.border_radius = Some(BorderRadius::all(999.0));
        style.padding = Some(Insets::symmetric(6.0, 14.0));
        style.font_size = Some(13.0);

        Self {
            id: None,
            classes: Vec::new(),
            style,
            text: text.into(),
            variant: ChipVariant::Filter,
            selected: None,
            on_click: None,
            is_disabled: false,
            is_hovered: false,
            is_pressed: false,
        }
    }

    pub fn with_variant(mut self, variant: ChipVariant) -> Self {
        self.variant = variant;
        self
    }

    pub fn with_selected(mut self, selected: Signal<bool>) -> Self {
        self.selected = Some(selected);
        self
    }

    pub fn with_disabled(mut self, disabled: bool) -> Self {
        self.is_disabled = disabled;
        self
    }

    pub fn on_click<F: FnMut() + 'static>(mut self, handler: F) -> Self {
        self.on_click = Some(Box::new(handler));
        self
    }
}

impl Widget for Chip {
    fn widget_type(&self) -> &'static str {
        "Chip"
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
        let font_size = self.style.font_size.unwrap_or(quick_style::base::TypeScale::CHIP);
        let char_count = self.text.chars().count() as f32;
        let pad_h = self.style.padding.map(|p| p.left + p.right).unwrap_or(28.0);
        let pad_v = self.style.padding.map(|p| p.top + p.bottom).unwrap_or(12.0);

        let estimated_width = (char_count * font_size * 0.60 + pad_h + 10.0).max(48.0);
        let estimated_height = font_size * 1.4 + pad_v;

        let mut computed_style = self.style.clone();
        if computed_style.width.is_none() {
            computed_style.width = Some(Dimension::Px(estimated_width));
        }
        if computed_style.height.is_none() {
            computed_style.height = Some(Dimension::Px(estimated_height));
        }

        engine.new_leaf(&computed_style)
    }

    fn paint(&self, canvas: &mut Canvas, bounds: Rect) {
        let is_sel = self.selected.as_ref().map(|s| s.get()).unwrap_or(false);
        let radius = self.style.border_radius.unwrap_or_else(|| BorderRadius::all(bounds.size.height / 2.0));

        let bg_color = if is_sel {
            self.style.background_color.unwrap_or_else(|| Color::from_hex("#4A4458").unwrap_or(Color::from_rgb(74, 68, 88)))
        } else if self.is_hovered {
            Color::from_hex("#2B2930").unwrap_or(Color::from_rgb(43, 41, 48))
        } else {
            self.style.background_color.unwrap_or_else(|| Color::from_hex("#1D1B20").unwrap_or(Color::from_rgb(29, 27, 32)))
        };

        canvas.fill_rounded_rect(bounds, radius, bg_color);

        let border_color = if is_sel {
            Color::from_hex("#CCC2DC").unwrap_or(Color::from_rgb(204, 194, 220))
        } else {
            self.style.border_color.unwrap_or_else(|| Color::from_hex("#49454F").unwrap_or(Color::from_rgb(73, 69, 79)))
        };
        canvas.stroke_rounded_rect(bounds, radius, border_color, 1.0);

        let text_color = if is_sel {
            Color::from_hex("#E8DEF8").unwrap_or(Color::from_rgb(232, 222, 248))
        } else {
            self.style.text_color.unwrap_or_else(|| Color::from_hex("#CAC4D0").unwrap_or(Color::from_rgb(202, 196, 208)))
        };

        let font_size = self.style.font_size.unwrap_or(quick_style::base::TypeScale::CHIP);
        let char_count = self.text.chars().count() as f32;
        let text_w = char_count * font_size * 0.60;
        let origin_x = bounds.origin.x + ((bounds.size.width - text_w) / 2.0).max(0.0);
        let origin_y = bounds.origin.y + ((bounds.size.height + font_size * 0.8) / 2.0);

        canvas.draw_text(&self.text, Point::new(origin_x, origin_y), text_color, font_size, self.style.font_family.clone());
    }

    fn handle_event(&mut self, event: &Event, bounds: Rect) -> bool {
        if self.is_disabled {
            return false;
        }

        if let Event::Pointer(PointerEvent { position, button, phase, .. }) = event {
            let inside = bounds.contains(*position);
            let prev_hover = self.is_hovered;
            self.is_hovered = inside;

            match phase {
                PointerPhase::Down if inside && *button == Some(PointerButton::Primary) => {
                    self.is_pressed = true;
                    return true;
                }
                PointerPhase::Up if self.is_pressed => {
                    self.is_pressed = false;
                    if inside && *button == Some(PointerButton::Primary) {
                        if let Some(ref sel) = self.selected {
                            sel.set(!sel.get());
                        }
                        if let Some(ref mut handler) = self.on_click {
                            handler();
                        }
                        return true;
                    }
                    return false;
                }
                PointerPhase::Cancel => {
                    self.is_pressed = false;
                    return false;
                }
                PointerPhase::Moved => {
                    if prev_hover != self.is_hovered {
                        return true;
                    }
                }
                _ => {}
            }
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    #[test]
    fn test_chip_click_and_toggle() {
        let sel_sig = Signal::new(false);
        let clicked = Rc::new(RefCell::new(false));
        let clicked_cl = clicked.clone();

        let mut chip = Chip::new("Pure Rust")
            .with_selected(sel_sig.clone())
            .on_click(move || *clicked_cl.borrow_mut() = true);

        let bounds = Rect::new(0.0, 0.0, 80.0, 32.0);

        let down = Event::Pointer(PointerEvent {
            position: Point::new(40.0, 16.0),
            button: Some(PointerButton::Primary),
            phase: PointerPhase::Down,
            modifiers: Default::default(),
        });
        assert!(chip.handle_event(&down, bounds));

        let up = Event::Pointer(PointerEvent {
            position: Point::new(40.0, 16.0),
            button: Some(PointerButton::Primary),
            phase: PointerPhase::Up,
            modifiers: Default::default(),
        });
        assert!(chip.handle_event(&up, bounds));

        assert!(sel_sig.get());
        assert!(*clicked.borrow());

        let mut canvas = Canvas::new();
        chip.paint(&mut canvas, bounds);
        assert!(!canvas.commands().is_empty());
    }

    #[test]
    fn test_chip_variants() {
        let c_filter = Chip::new("Filter").with_variant(ChipVariant::Filter);
        assert_eq!(c_filter.variant, ChipVariant::Filter);

        let c_assist = Chip::new("Assist").with_variant(ChipVariant::Assist);
        assert_eq!(c_assist.variant, ChipVariant::Assist);

        let c_input = Chip::new("Input").with_variant(ChipVariant::Input);
        assert_eq!(c_input.variant, ChipVariant::Input);

        let c_sugg = Chip::new("Sugg").with_variant(ChipVariant::Suggestion);
        assert_eq!(c_sugg.variant, ChipVariant::Suggestion);
    }
}
