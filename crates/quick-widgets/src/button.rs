use crate::state_layer::{StateLayer, WidgetState};
use crate::widget::Widget;
use quick_core::event::{Event, PointerButton, PointerEvent, PointerPhase};
use quick_core::geometry::{BorderRadius, Color, Insets, Point, Rect};
use quick_layout::engine::LayoutEngine;
use quick_render::canvas::Canvas;
use quick_style::base::{base_theme, RadiusScale, SpacingScale, TypeScale};
use quick_style::property::{Dimension, Style};
use quick_style::theme::tokens::{ElevationTokens, StateLayerTokens};
use serde::{Deserialize, Serialize};
use taffy::prelude::NodeId;
use taffy::TaffyError;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum ButtonVariant {
    #[default]
    Filled,
    Tonal,
    Elevated,
    Outlined,
    Text,
}

pub struct Button {
    pub id: Option<String>,
    pub classes: Vec<String>,
    pub style: Style,
    pub text: String,
    pub variant: ButtonVariant,
    pub icon: Option<String>,
    pub disabled: bool,
    pub on_click: Option<Box<dyn FnMut()>>,
    pub is_hovered: bool,
    pub is_pressed: bool,
    pub is_focused: bool,
}

impl Button {
    pub fn new(text: impl Into<String>) -> Self {
        let bt = base_theme();
        let mut style = Style::default();
        style.background_color = Some(bt.colors.accent.normal);
        style.text_color = Some(bt.colors.accent.on_accent);
        style.border_radius = Some(BorderRadius::all(RadiusScale::PILL));
        style.padding = Some(Insets::symmetric(SpacingScale::SM, SpacingScale::XL));
        style.font_size = Some(TypeScale::BUTTON);

        Self {
            id: None,
            classes: Vec::new(),
            style,
            text: text.into(),
            variant: ButtonVariant::Filled,
            icon: None,
            disabled: false,
            on_click: None,
            is_hovered: false,
            is_pressed: false,
            is_focused: false,
        }
    }

    pub fn with_variant(mut self, variant: ButtonVariant) -> Self {
        self.variant = variant;
        let bt = base_theme();
        let pad = Insets::symmetric(SpacingScale::SM, SpacingScale::XL);
        match variant {
            ButtonVariant::Filled => {
                self.style.background_color = Some(bt.colors.accent.normal);
                self.style.text_color = Some(bt.colors.accent.on_accent);
                self.style.border_color = None;
                self.style.border_width = None;
                self.style.padding = Some(pad);
            }
            ButtonVariant::Tonal => {
                // Tonal: muted accent bg, dark text
                let acc = bt.colors.accent.normal;
                let tonal_bg = Color::from_rgb(
                    ((acc.r as u16 * 3 + 255) / 4) as u8,
                    ((acc.g as u16 * 3 + 255) / 4) as u8,
                    ((acc.b as u16 * 3 + 255) / 4) as u8,
                );
                self.style.background_color = Some(tonal_bg);
                self.style.text_color = Some(bt.colors.text_primary);
                self.style.border_color = None;
                self.style.border_width = None;
                self.style.padding = Some(pad);
            }
            ButtonVariant::Elevated => {
                self.style.background_color = Some(bt.colors.surface_raised);
                self.style.text_color = Some(bt.colors.accent.normal);
                self.style.border_color = None;
                self.style.border_width = None;
                self.style.padding = Some(pad);
            }
            ButtonVariant::Outlined => {
                self.style.background_color = Some(Color::TRANSPARENT);
                self.style.text_color = Some(bt.colors.accent.normal);
                self.style.border_color = Some(bt.colors.border);
                self.style.border_width = Some(1.0);
                self.style.padding = Some(pad);
            }
            ButtonVariant::Text => {
                self.style.background_color = Some(Color::TRANSPARENT);
                self.style.text_color = Some(bt.colors.accent.normal);
                self.style.border_color = None;
                self.style.border_width = None;
                self.style.padding = Some(Insets::symmetric(SpacingScale::SM, SpacingScale::LG));
            }
        }
        self
    }

    pub fn with_icon(mut self, icon: impl Into<String>) -> Self {
        self.icon = Some(icon.into());
        self
    }

    pub fn with_disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn on_click<F: FnMut() + 'static>(mut self, handler: F) -> Self {
        self.on_click = Some(Box::new(handler));
        self
    }
}

impl Widget for Button {
    fn widget_type(&self) -> &'static str {
        "Button"
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
        let font_size = self.style.font_size.unwrap_or(14.0);
        let char_count = self.text.chars().count() as f32;
        let pad_h = self.style.padding.map(|p| p.left + p.right).unwrap_or_else(|| {
            if self.variant == ButtonVariant::Text { 32.0 } else { 48.0 }
        });
        let pad_v = self.style.padding.map(|p| p.top + p.bottom).unwrap_or(20.0);

        let estimated_width = (char_count * font_size * 0.55 + pad_h).max(60.0);
        let estimated_height = font_size * 1.3 + pad_v;

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
        let bt = base_theme();
        let acc = &bt.colors.accent;
        let (default_bg, default_fg, default_border, default_border_w) = match self.variant {
            ButtonVariant::Filled => (
                acc.normal,
                acc.on_accent,
                None,
                None,
            ),
            ButtonVariant::Tonal => {
                let tonal_bg = Color::from_rgb(
                    ((acc.normal.r as u16 * 3 + 255) / 4) as u8,
                    ((acc.normal.g as u16 * 3 + 255) / 4) as u8,
                    ((acc.normal.b as u16 * 3 + 255) / 4) as u8,
                );
                (tonal_bg, bt.colors.text_primary, None, None)
            }
            ButtonVariant::Elevated => (
                bt.colors.surface_raised,
                acc.normal,
                None,
                None,
            ),
            ButtonVariant::Outlined => (
                Color::TRANSPARENT,
                acc.normal,
                Some(bt.colors.border),
                Some(1.0f32),
            ),
            ButtonVariant::Text => (
                Color::TRANSPARENT,
                acc.normal,
                None,
                None,
            ),
        };

        let base_bg = self.style.background_color.unwrap_or(default_bg);
        let text_color = self.style.text_color.unwrap_or(default_fg);
        let border_color = self.style.border_color.or(default_border);
        let border_width = self.style.border_width.or(default_border_w);
        let radius = self.style.border_radius.unwrap_or_else(|| BorderRadius::all(bounds.size.height / 2.0));

        // 1. Draw Elevation Shadows for Elevated Variant
        if self.variant == ButtonVariant::Elevated && base_bg != Color::TRANSPARENT {
            let level = if self.is_pressed || self.is_hovered { 2 } else { 1 };
            let elev_tokens = ElevationTokens::default();
            canvas.draw_elevation_shadow(bounds, radius, level, &elev_tokens);
        }

        // 2. Compute State Layer Overlay on Background
        let mut final_bg = base_bg;
        let mut final_fg = text_color;

        if self.disabled {
            final_bg = StateLayer::apply_disabled(base_bg, true);
            final_fg = StateLayer::apply_disabled(text_color, false);
        } else if base_bg != Color::TRANSPARENT {
            let state = WidgetState {
                is_hovered: self.is_hovered,
                is_focused: self.is_focused,
                is_pressed: self.is_pressed,
                is_dragged: false,
                is_disabled: false,
            };
            final_bg = StateLayer::apply_state(base_bg, text_color, state, &StateLayerTokens::M3);
        } else if self.is_pressed {
            final_bg = StateLayer::blend(Color::TRANSPARENT, text_color, 0.12);
        } else if self.is_hovered {
            final_bg = StateLayer::blend(Color::TRANSPARENT, text_color, 0.08);
        } else if self.is_focused {
            final_bg = StateLayer::blend(Color::TRANSPARENT, text_color, 0.12);
        }

        // 3. Paint Background
        if final_bg.a > 0 {
            canvas.fill_rounded_rect(bounds, radius, final_bg);
        }

        // 4. Paint Border
        if let (Some(b_color), Some(b_width)) = (border_color, border_width) {
            if b_width > 0.0 {
                canvas.stroke_rounded_rect(bounds, radius, b_color, b_width);
            }
        }

        // 5. Paint Text Label & Icon
        let font_size = self.style.font_size.unwrap_or(14.0);
        let char_count = self.text.chars().count() as f32;
        let text_w = char_count * font_size * 0.55;
        let origin_x = bounds.origin.x + ((bounds.size.width - text_w) / 2.0).max(0.0);
        let origin_y = bounds.origin.y + ((bounds.size.height + font_size * 0.8) / 2.0);

        canvas.draw_text(
            &self.text,
            Point::new(origin_x, origin_y),
            final_fg,
            font_size,
            self.style.font_family.clone(),
        );
    }

    fn handle_event(&mut self, event: &Event, bounds: Rect) -> bool {
        if self.disabled {
            return false;
        }

        if let Event::Pointer(PointerEvent { position, button, phase, .. }) = event {
            let inside = bounds.contains(*position);
            self.is_hovered = inside;

            match phase {
                PointerPhase::Down if inside && *button == Some(PointerButton::Primary) => {
                    self.is_pressed = true;
                    return true;
                }
                PointerPhase::Up if self.is_pressed => {
                    self.is_pressed = false;
                    if inside && *button == Some(PointerButton::Primary) {
                        if let Some(ref mut handler) = self.on_click {
                            handler();
                        }
                        return true;
                    }
                }
                PointerPhase::Cancel => {
                    self.is_pressed = false;
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
    fn test_button_click_event() {
        let clicked = Rc::new(RefCell::new(false));
        let clicked_cl = clicked.clone();

        let mut btn = Button::new("Click").on_click(move || {
            *clicked_cl.borrow_mut() = true;
        });

        let bounds = Rect::new(0.0, 0.0, 100.0, 40.0);

        let down_event = Event::Pointer(PointerEvent {
            position: Point::new(50.0, 20.0),
            button: Some(PointerButton::Primary),
            phase: PointerPhase::Down,
            modifiers: Default::default(),
        });
        assert!(btn.handle_event(&down_event, bounds));

        let up_event = Event::Pointer(PointerEvent {
            position: Point::new(50.0, 20.0),
            button: Some(PointerButton::Primary),
            phase: PointerPhase::Up,
            modifiers: Default::default(),
        });
        assert!(btn.handle_event(&up_event, bounds));
        assert!(*clicked.borrow());
    }

    #[test]
    fn test_button_variants_and_builder() {
        let btn_filled = Button::new("Filled").with_variant(ButtonVariant::Filled);
        assert_eq!(btn_filled.variant, ButtonVariant::Filled);

        let btn_tonal = Button::new("Tonal").with_variant(ButtonVariant::Tonal);
        assert_eq!(btn_tonal.variant, ButtonVariant::Tonal);

        let btn_elev = Button::new("Elevated").with_variant(ButtonVariant::Elevated);
        assert_eq!(btn_elev.variant, ButtonVariant::Elevated);

        let btn_outlined = Button::new("Outlined").with_variant(ButtonVariant::Outlined);
        assert_eq!(btn_outlined.variant, ButtonVariant::Outlined);

        let btn_text = Button::new("Text").with_variant(ButtonVariant::Text);
        assert_eq!(btn_text.variant, ButtonVariant::Text);
    }
}
