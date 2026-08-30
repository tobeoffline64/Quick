//! Material 3 State Layer Blending Engine.
//!
//! Provides alpha compositing for interactive widget states (Hover 8%, Focus 12%,
//! Pressed 12%, Dragged 16%, Disabled Container 12%, Disabled Content 38%) according to M3 specifications.

use quick_core::geometry::Color;
use quick_style::theme::tokens::StateLayerTokens;

/// Represents the interactive state of a widget.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct WidgetState {
    pub is_hovered: bool,
    pub is_focused: bool,
    pub is_pressed: bool,
    pub is_dragged: bool,
    pub is_disabled: bool,
}

impl WidgetState {
    pub const NORMAL: Self = Self {
        is_hovered: false,
        is_focused: false,
        is_pressed: false,
        is_dragged: false,
        is_disabled: false,
    };

    pub fn hovered() -> Self {
        Self { is_hovered: true, ..Self::NORMAL }
    }

    pub fn pressed() -> Self {
        Self { is_pressed: true, ..Self::NORMAL }
    }

    pub fn focused() -> Self {
        Self { is_focused: true, ..Self::NORMAL }
    }

    pub fn dragged() -> Self {
        Self { is_dragged: true, ..Self::NORMAL }
    }

    pub fn disabled() -> Self {
        Self { is_disabled: true, ..Self::NORMAL }
    }
}

/// Helper for computing state layer alpha compositing on colors.
pub struct StateLayer;

impl StateLayer {
    /// Blend an overlay color onto a base color using a given alpha factor [0.0, 1.0].
    #[inline]
    pub fn blend(base: Color, overlay: Color, alpha: f32) -> Color {
        let a = if alpha.is_nan() { 0.0 } else { alpha.clamp(0.0, 1.0) };
        let r = (base.r as f32 * (1.0 - a) + overlay.r as f32 * a).round() as u8;
        let g = (base.g as f32 * (1.0 - a) + overlay.g as f32 * a).round() as u8;
        let b = (base.b as f32 * (1.0 - a) + overlay.b as f32 * a).round() as u8;
        Color::from_rgba(r, g, b, base.a)
    }

    /// Computes the effective background color given base color, on-surface/on-primary overlay,
    /// and current widget state based on M3 state priority.
    pub fn apply_state(
        base: Color,
        on_color: Color,
        state: WidgetState,
        tokens: &StateLayerTokens,
    ) -> Color {
        if state.is_disabled {
            return tokens.apply_disabled_container(base);
        }
        if state.is_pressed {
            return tokens.apply_pressed(base, on_color);
        }
        if state.is_dragged {
            return tokens.apply_dragged(base, on_color);
        }
        if state.is_hovered {
            return tokens.apply_hover(base, on_color);
        }
        if state.is_focused {
            return tokens.apply_focus(base, on_color);
        }
        base
    }

    /// Convenience helper using standard M3 state tokens.
    #[inline]
    pub fn apply_m3_state(base: Color, on_color: Color, state: WidgetState) -> Color {
        Self::apply_state(base, on_color, state, &StateLayerTokens::M3)
    }

    /// Apply hover state layer (8% overlay).
    #[inline]
    pub fn apply_hover(base: Color, on_color: Color) -> Color {
        StateLayerTokens::M3.apply_hover(base, on_color)
    }

    /// Apply pressed state layer (12% overlay).
    #[inline]
    pub fn apply_pressed(base: Color, on_color: Color) -> Color {
        StateLayerTokens::M3.apply_pressed(base, on_color)
    }

    /// Apply focus state layer (12% overlay).
    #[inline]
    pub fn apply_focus(base: Color, on_color: Color) -> Color {
        StateLayerTokens::M3.apply_focus(base, on_color)
    }

    /// Apply dragged state layer (16% overlay).
    #[inline]
    pub fn apply_dragged(base: Color, on_color: Color) -> Color {
        StateLayerTokens::M3.apply_dragged(base, on_color)
    }

    /// Apply disabled styling to container background (12% opacity) or content (38% opacity).
    #[inline]
    pub fn apply_disabled(color: Color, is_container: bool) -> Color {
        if is_container {
            StateLayerTokens::M3.apply_disabled_container(color)
        } else {
            StateLayerTokens::M3.apply_disabled_content(color)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_state_layer_alpha_blending_matrix() {
        let base = Color::from_rgb(100, 100, 100);
        let overlay = Color::WHITE;

        let hover = StateLayer::apply_hover(base, overlay);
        assert_eq!(hover.r, 112); // 100 * 0.92 + 255 * 0.08 = 112.4 -> 112

        let pressed = StateLayer::apply_pressed(base, overlay);
        assert_eq!(pressed.r, 119); // 100 * 0.88 + 255 * 0.12 = 118.6 -> 119

        let dragged = StateLayer::apply_dragged(base, overlay);
        assert_eq!(dragged.r, 125); // 100 * 0.84 + 255 * 0.16 = 124.8 -> 125

        let disabled_container = StateLayer::apply_disabled(base, true);
        assert_eq!(disabled_container.a, 31); // 255 * 0.12 = 30.6 -> 31

        let disabled_content = StateLayer::apply_disabled(base, false);
        assert_eq!(disabled_content.a, 97); // 255 * 0.38 = 96.9 -> 97
    }

    #[test]
    fn test_state_layer_state_priority_and_nan() {
        let base = Color::BLACK;
        let overlay = Color::WHITE;

        let mut state = WidgetState::NORMAL;
        assert_eq!(StateLayer::apply_m3_state(base, overlay, state), base);

        state.is_hovered = true;
        assert_eq!(StateLayer::apply_m3_state(base, overlay, state), StateLayer::apply_hover(base, overlay));

        state.is_pressed = true;
        assert_eq!(StateLayer::apply_m3_state(base, overlay, state), StateLayer::apply_pressed(base, overlay));

        state.is_disabled = true;
        assert_eq!(StateLayer::apply_m3_state(base, overlay, state), StateLayer::apply_disabled(base, true));

        // NaN safety
        let blended_nan = StateLayer::blend(base, overlay, f32::NAN);
        assert_eq!(blended_nan, base);
    }
}
