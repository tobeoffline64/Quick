//! `NoctaliaButton` — Glassmorphic Noctalia styled button.
//! Supports Primary, Secondary, Ghost, Outline, Danger variants.

use crate::widget::Widget;
use quick_core::event::{Event, PointerButton, PointerEvent, PointerPhase};
use quick_core::geometry::{BorderRadius, Color, Point, Rect};
use quick_layout::engine::LayoutEngine;
use quick_render::canvas::Canvas;
use quick_style::noctalia::NoctaliaPalette;
use quick_style::property::{Dimension, Style};
use taffy::prelude::NodeId;
use taffy::TaffyError;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum NoctaliaButtonVariant {
    #[default]
    Primary,
    Secondary,
    Ghost,
    Outline,
    Danger,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum NoctaliaControlSize {
    Small,
    #[default]
    Medium,
    Large,
}

pub struct NoctaliaButton {
    pub id: Option<String>,
    pub classes: Vec<String>,
    pub style: Style,
    pub text: String,
    pub variant: NoctaliaButtonVariant,
    pub size: NoctaliaControlSize,
    pub is_hovered: bool,
    pub is_pressed: bool,
    pub is_disabled: bool,
    pub on_click: Option<Box<dyn FnMut()>>,
}

impl NoctaliaButton {
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            id: None,
            classes: Vec::new(),
            style: Style::default(),
            text: text.into(),
            variant: NoctaliaButtonVariant::Primary,
            size: NoctaliaControlSize::Medium,
            is_hovered: false,
            is_pressed: false,
            is_disabled: false,
            on_click: None,
        }
    }

    pub fn variant(mut self, variant: NoctaliaButtonVariant) -> Self {
        self.variant = variant;
        self
    }

    pub fn size(mut self, size: NoctaliaControlSize) -> Self {
        self.size = size;
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.is_disabled = disabled;
        self
    }

    pub fn on_click<F: FnMut() + 'static>(mut self, handler: F) -> Self {
        self.on_click = Some(Box::new(handler));
        self
    }
}

impl Widget for NoctaliaButton {
    fn widget_type(&self) -> &'static str {
        "NoctaliaButton"
    }

    fn id(&self) -> Option<&str> { self.id.as_deref() }
    fn classes(&self) -> &[String] { &self.classes }
    fn style(&self) -> &Style { &self.style }
    fn style_mut(&mut self) -> &mut Style { &mut self.style }

    fn build_layout(&mut self, engine: &mut LayoutEngine) -> Result<NodeId, TaffyError> {
        let (height, pad_h, font_size) = match self.size {
            NoctaliaControlSize::Small  => (28.0, 12.0, 11.0),
            NoctaliaControlSize::Medium => (36.0, 18.0, 12.0),
            NoctaliaControlSize::Large  => (44.0, 24.0, 14.0),
        };

        let char_count = self.text.chars().count() as f32;
        let estimated_w = (char_count * font_size * 0.58 + pad_h * 2.0).max(48.0);

        let mut computed = self.style.clone();
        if computed.width.is_none() {
            computed.width = Some(Dimension::Px(estimated_w));
        }
        if computed.height.is_none() {
            computed.height = Some(Dimension::Px(height));
        }
        engine.new_leaf(&computed)
    }

    fn paint(&self, canvas: &mut Canvas, bounds: Rect) {
        let pal = NoctaliaPalette::noctalia_dark();
        let radius = self.style.border_radius.unwrap_or(BorderRadius::all(8.0));

        let (mut bg, mut fg, border_color, border_w) = match self.variant {
            NoctaliaButtonVariant::Primary => (
                self.style.background_color.unwrap_or(pal.primary),
                self.style.text_color.unwrap_or(pal.on_primary),
                None,
                0.0,
            ),
            NoctaliaButtonVariant::Secondary => (
                self.style.background_color.unwrap_or(pal.surface_variant),
                self.style.text_color.unwrap_or(pal.on_surface),
                Some(pal.outline),
                1.0,
            ),
            NoctaliaButtonVariant::Ghost => (
                Color::TRANSPARENT,
                self.style.text_color.unwrap_or(pal.on_surface),
                None,
                0.0,
            ),
            NoctaliaButtonVariant::Outline => (
                Color::TRANSPARENT,
                self.style.text_color.unwrap_or(pal.on_surface),
                Some(self.style.border_color.unwrap_or(pal.outline)),
                1.0,
            ),
            NoctaliaButtonVariant::Danger => (
                self.style.background_color.unwrap_or(pal.error),
                self.style.text_color.unwrap_or(pal.on_error),
                None,
                0.0,
            ),
        };

        if self.is_disabled {
            bg = Color::from_rgba(bg.r, bg.g, bg.b, (bg.a as f32 * 0.38) as u8);
            fg = Color::from_rgba(fg.r, fg.g, fg.b, 90);
        } else if self.is_pressed {
            bg = Color::from_rgba((bg.r as f32 * 0.85) as u8, (bg.g as f32 * 0.85) as u8, (bg.b as f32 * 0.85) as u8, bg.a);
        } else if self.is_hovered {
            if bg == Color::TRANSPARENT {
                bg = Color::from_rgba(pal.primary.r, pal.primary.g, pal.primary.b, 30);
            } else {
                bg = Color::from_rgba((bg.r as f32 * 1.15).min(255.0) as u8, (bg.g as f32 * 1.15).min(255.0) as u8, (bg.b as f32 * 1.15).min(255.0) as u8, bg.a);
            }
        }

        if bg.a > 0 {
            canvas.fill_rounded_rect(bounds, radius, bg);
        }

        if let Some(bc) = border_color {
            if border_w > 0.0 {
                canvas.stroke_rounded_rect(bounds, radius, bc, border_w);
            }
        }

        let font_size = match self.size {
            NoctaliaControlSize::Small  => 11.0,
            NoctaliaControlSize::Medium => 12.0,
            NoctaliaControlSize::Large  => 14.0,
        };

        let char_count = self.text.chars().count() as f32;
        let text_w = char_count * font_size * 0.55;
        let text_x = bounds.origin.x + ((bounds.size.width - text_w) / 2.0).max(0.0);
        let text_y = bounds.origin.y + (bounds.size.height + font_size * 0.75) / 2.0;

        canvas.draw_text(&self.text, Point::new(text_x, text_y), fg, font_size, self.style.font_family.clone());
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
                    true
                }
                PointerPhase::Up if self.is_pressed => {
                    self.is_pressed = false;
                    if inside {
                        if let Some(ref mut handler) = self.on_click {
                            handler();
                        }
                    }
                    true
                }
                PointerPhase::Cancel => {
                    if self.is_pressed {
                        self.is_pressed = false;
                        true
                    } else {
                        false
                    }
                }
                PointerPhase::Moved => {
                    prev_hover != self.is_hovered
                }
                _ => inside,
            }
        } else {
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_noctalia_button_variants() {
        let btn_primary = NoctaliaButton::new("Primary").variant(NoctaliaButtonVariant::Primary);
        assert_eq!(btn_primary.variant, NoctaliaButtonVariant::Primary);

        let btn_ghost = NoctaliaButton::new("Ghost").variant(NoctaliaButtonVariant::Ghost);
        assert_eq!(btn_ghost.variant, NoctaliaButtonVariant::Ghost);
    }
}
