//! Material 3 Design Tokens: Shapes, Elevation Shadows, State Layers, and Motion.

use quick_core::geometry::{BorderRadius, Color};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Shape tokens providing corner radii scales from 0px to 9999px.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ShapeTokens {
    pub corner_none: f32,
    pub corner_extra_small: f32,
    pub corner_small: f32,
    pub corner_medium: f32,
    pub corner_large: f32,
    pub corner_extra_large: f32,
    pub corner_full: f32,
    #[serde(default)]
    custom: Option<HashMap<String, f32>>,
}

impl Default for ShapeTokens {
    fn default() -> Self {
        Self {
            corner_none: 0.0,
            corner_extra_small: 4.0,
            corner_small: 8.0,
            corner_medium: 12.0,
            corner_large: 16.0,
            corner_extra_large: 28.0,
            corner_full: 9999.0,
            custom: None,
        }
    }
}

impl ShapeTokens {
    pub const M3: Self = Self {
        corner_none: 0.0,
        corner_extra_small: 4.0,
        corner_small: 8.0,
        corner_medium: 12.0,
        corner_large: 16.0,
        corner_extra_large: 28.0,
        corner_full: 9999.0,
        custom: None,
    };

    pub fn to_border_radius(radius: f32) -> BorderRadius {
        BorderRadius::all(radius)
    }

    pub fn to_map(&self) -> HashMap<String, f32> {
        let custom_len = self.custom.as_ref().map_or(0, |m| m.len());
        let mut map = HashMap::with_capacity(7 + custom_len);
        map.insert("corner_none".into(), self.corner_none);
        map.insert("corner_extra_small".into(), self.corner_extra_small);
        map.insert("corner_small".into(), self.corner_small);
        map.insert("corner_medium".into(), self.corner_medium);
        map.insert("corner_large".into(), self.corner_large);
        map.insert("corner_extra_large".into(), self.corner_extra_large);
        map.insert("corner_full".into(), self.corner_full);
        if let Some(custom) = &self.custom {
            for (k, v) in custom {
                map.insert(k.clone(), *v);
            }
        }
        map
    }

    pub fn get(&self, name: &str) -> Option<&f32> {
        match name {
            "corner_none" | "none" => Some(&self.corner_none),
            "corner_extra_small" | "extra_small" | "xs" => Some(&self.corner_extra_small),
            "corner_small" | "small" | "sm" => Some(&self.corner_small),
            "corner_medium" | "medium" | "md" => Some(&self.corner_medium),
            "corner_large" | "large" | "lg" => Some(&self.corner_large),
            "corner_extra_large" | "extra_large" | "xl" => Some(&self.corner_extra_large),
            "corner_full" | "full" | "pill" => Some(&self.corner_full),
            other => self.custom.as_ref().and_then(|m| m.get(other)),
        }
    }

    pub fn insert(&mut self, name: impl Into<String>, val: f32) {
        let name_str = name.into();
        match name_str.as_str() {
            "corner_none" | "none" => self.corner_none = val,
            "corner_extra_small" | "extra_small" | "xs" => self.corner_extra_small = val,
            "corner_small" | "small" | "sm" => self.corner_small = val,
            "corner_medium" | "medium" | "md" => self.corner_medium = val,
            "corner_large" | "large" | "lg" => self.corner_large = val,
            "corner_extra_large" | "extra_large" | "xl" => self.corner_extra_large = val,
            "corner_full" | "full" | "pill" => self.corner_full = val,
            _ => {
                self.custom.get_or_insert_with(HashMap::new).insert(name_str, val);
            }
        }
    }

    pub fn is_empty(&self) -> bool {
        false
    }

    pub fn contains_key(&self, name: &str) -> bool {
        self.get(name).is_some()
    }

    pub fn len(&self) -> usize {
        7 + self.custom.as_ref().map_or(0, |m| m.len())
    }
}

/// Elevation shadow layer definition.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Shadow {
    pub offset_x: f32,
    pub offset_y: f32,
    pub blur_radius: f32,
    pub spread_radius: f32,
    pub color: Color,
}

impl Shadow {
    pub const fn new(
        offset_x: f32,
        offset_y: f32,
        blur_radius: f32,
        spread_radius: f32,
        color: Color,
    ) -> Self {
        Self {
            offset_x,
            offset_y,
            blur_radius,
            spread_radius,
            color,
        }
    }

    pub fn to_css(&self) -> String {
        format!(
            "{:.0}px {:.0}px {:.0}px {:.0}px rgba({}, {}, {}, {:.2})",
            self.offset_x,
            self.offset_y,
            self.blur_radius,
            self.spread_radius,
            self.color.r,
            self.color.g,
            self.color.b,
            self.color.a as f32 / 255.0
        )
    }
}

/// Single elevation level definition containing dual shadows and surface tint opacity.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ElevationLevel {
    pub level: u8,
    pub elevation_dp: f32,
    pub key_shadow: Option<Shadow>,
    pub ambient_shadow: Option<Shadow>,
    pub surface_tint_opacity: f32,
}

/// Elevation Levels 0 through 5.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ElevationTokens {
    pub level_0: ElevationLevel,
    pub level_1: ElevationLevel,
    pub level_2: ElevationLevel,
    pub level_3: ElevationLevel,
    pub level_4: ElevationLevel,
    pub level_5: ElevationLevel,
}

impl Default for ElevationTokens {
    fn default() -> Self {
        let key_color = Color::from_rgba(0, 0, 0, 77);     // 30% alpha
        let ambient_color = Color::from_rgba(0, 0, 0, 38); // 15% alpha

        Self {
            level_0: ElevationLevel {
                level: 0,
                elevation_dp: 0.0,
                key_shadow: None,
                ambient_shadow: None,
                surface_tint_opacity: 0.0,
            },
            level_1: ElevationLevel {
                level: 1,
                elevation_dp: 1.0,
                key_shadow: Some(Shadow::new(0.0, 1.0, 2.0, 0.0, key_color)),
                ambient_shadow: Some(Shadow::new(0.0, 1.0, 3.0, 1.0, ambient_color)),
                surface_tint_opacity: 0.05,
            },
            level_2: ElevationLevel {
                level: 2,
                elevation_dp: 3.0,
                key_shadow: Some(Shadow::new(0.0, 1.0, 2.0, 0.0, key_color)),
                ambient_shadow: Some(Shadow::new(0.0, 2.0, 6.0, 2.0, ambient_color)),
                surface_tint_opacity: 0.08,
            },
            level_3: ElevationLevel {
                level: 3,
                elevation_dp: 6.0,
                key_shadow: Some(Shadow::new(0.0, 1.0, 3.0, 0.0, key_color)),
                ambient_shadow: Some(Shadow::new(0.0, 4.0, 8.0, 3.0, ambient_color)),
                surface_tint_opacity: 0.11,
            },
            level_4: ElevationLevel {
                level: 4,
                elevation_dp: 8.0,
                key_shadow: Some(Shadow::new(0.0, 2.0, 3.0, 0.0, key_color)),
                ambient_shadow: Some(Shadow::new(0.0, 6.0, 10.0, 4.0, ambient_color)),
                surface_tint_opacity: 0.12,
            },
            level_5: ElevationLevel {
                level: 5,
                elevation_dp: 12.0,
                key_shadow: Some(Shadow::new(0.0, 4.0, 4.0, 0.0, key_color)),
                ambient_shadow: Some(Shadow::new(0.0, 8.0, 12.0, 6.0, ambient_color)),
                surface_tint_opacity: 0.14,
            },
        }
    }
}

impl ElevationTokens {
    pub fn get(&self, level: u8) -> &ElevationLevel {
        match level {
            0 => &self.level_0,
            1 => &self.level_1,
            2 => &self.level_2,
            3 => &self.level_3,
            4 => &self.level_4,
            _ => &self.level_5,
        }
    }

    /// Blends surface tint overlay with base surface color based on elevation level.
    pub fn calculate_surface_tint(
        &self,
        level: u8,
        base_surface: Color,
        surface_tint: Color,
    ) -> Color {
        let opacity = self.get(level).surface_tint_opacity;
        if opacity <= 0.0 {
            return base_surface;
        }

        let r = (base_surface.r as f32 * (1.0 - opacity) + surface_tint.r as f32 * opacity).round()
            as u8;
        let g = (base_surface.g as f32 * (1.0 - opacity) + surface_tint.g as f32 * opacity).round()
            as u8;
        let b = (base_surface.b as f32 * (1.0 - opacity) + surface_tint.b as f32 * opacity).round()
            as u8;

        Color::from_rgba(r, g, b, base_surface.a)
    }

    /// Convert dual-pass shadow to standard CSS box-shadow declaration string.
    pub fn to_css_box_shadow(&self, level: u8) -> String {
        let elev = self.get(level);
        match (&elev.key_shadow, &elev.ambient_shadow) {
            (Some(k), Some(a)) => format!("{}, {}", k.to_css(), a.to_css()),
            (Some(k), None) => k.to_css(),
            (None, Some(a)) => a.to_css(),
            (None, None) => "none".to_string(),
        }
    }
}

/// State layer interaction opacities.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct StateLayerTokens {
    pub hover: f32,
    pub focus: f32,
    pub pressed: f32,
    pub dragged: f32,
    pub disabled_container: f32,
    pub disabled_content: f32,
}

impl Default for StateLayerTokens {
    fn default() -> Self {
        Self {
            hover: 0.08,
            focus: 0.12,
            pressed: 0.12,
            dragged: 0.16,
            disabled_container: 0.12,
            disabled_content: 0.38,
        }
    }
}

impl StateLayerTokens {
    pub const M3: Self = Self {
        hover: 0.08,
        focus: 0.12,
        pressed: 0.12,
        dragged: 0.16,
        disabled_container: 0.12,
        disabled_content: 0.38,
    };

    /// Mathematical alpha blending of an overlay color onto a base color.
    pub fn blend(&self, base: Color, overlay: Color, alpha: f32) -> Color {
        let alpha_clamped = if alpha.is_nan() { 0.0 } else { alpha.clamp(0.0, 1.0) };
        let r = (base.r as f32 * (1.0 - alpha_clamped) + overlay.r as f32 * alpha_clamped).round()
            as u8;
        let g = (base.g as f32 * (1.0 - alpha_clamped) + overlay.g as f32 * alpha_clamped).round()
            as u8;
        let b = (base.b as f32 * (1.0 - alpha_clamped) + overlay.b as f32 * alpha_clamped).round()
            as u8;
        Color::from_rgba(r, g, b, base.a)
    }

    pub fn apply_hover(&self, base: Color, on_color: Color) -> Color {
        self.blend(base, on_color, self.hover)
    }

    pub fn apply_pressed(&self, base: Color, on_color: Color) -> Color {
        self.blend(base, on_color, self.pressed)
    }

    pub fn apply_focus(&self, base: Color, on_color: Color) -> Color {
        self.blend(base, on_color, self.focus)
    }

    pub fn apply_dragged(&self, base: Color, on_color: Color) -> Color {
        self.blend(base, on_color, self.dragged)
    }

    pub fn apply_disabled_container(&self, base: Color) -> Color {
        Color::from_rgba(
            base.r,
            base.g,
            base.b,
            (base.a as f32 * self.disabled_container).round() as u8,
        )
    }

    pub fn apply_disabled_content(&self, content: Color) -> Color {
        Color::from_rgba(
            content.r,
            content.g,
            content.b,
            (content.a as f32 * self.disabled_content).round() as u8,
        )
    }
}

/// Motion transition durations and easing tokens.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct MotionTokens {
    pub duration_short_1: u32,
    pub duration_short_2: u32,
    pub duration_short_3: u32,
    pub duration_short_4: u32,
    pub duration_medium_1: u32,
    pub duration_medium_2: u32,
    pub duration_medium_3: u32,
    pub duration_medium_4: u32,
    pub duration_long_1: u32,
    pub duration_long_2: u32,
}

impl Default for MotionTokens {
    fn default() -> Self {
        Self {
            duration_short_1: 50,
            duration_short_2: 100,
            duration_short_3: 150,
            duration_short_4: 200,
            duration_medium_1: 250,
            duration_medium_2: 300,
            duration_medium_3: 350,
            duration_medium_4: 400,
            duration_long_1: 450,
            duration_long_2: 500,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shape_and_state_tokens() {
        let shapes = ShapeTokens::default();
        assert_eq!(shapes.corner_none, 0.0);
        assert_eq!(shapes.corner_small, 8.0);
        assert_eq!(shapes.corner_medium, 12.0);
        assert_eq!(shapes.corner_large, 16.0);
        assert_eq!(shapes.corner_full, 9999.0);
        assert_eq!(shapes.get("corner_large"), Some(&16.0));

        let states = StateLayerTokens::default();
        let black = Color::BLACK;
        let white = Color::WHITE;
        let hovered = states.apply_hover(black, white);
        assert_eq!(hovered, Color::from_rgb(20, 20, 20));

        let elev = ElevationTokens::default();
        assert!(elev.level_0.key_shadow.is_none());
        assert!(elev.level_1.key_shadow.is_some());
        assert!((elev.level_1.surface_tint_opacity - 0.05).abs() < 1e-4);
    }
}
