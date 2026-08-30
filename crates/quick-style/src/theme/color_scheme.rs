//! Comprehensive Material 3 Color Scheme containing all 47 M3 Color Roles.

use crate::theme::palette::CorePalette;
use crate::theme::scheme::SchemeVariant;
use quick_core::geometry::Color;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Comprehensive Material 3 Color Scheme containing all 47 M3 Color Roles.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct ColorScheme {
    // Primary Group
    pub primary: Color,
    pub on_primary: Color,
    pub primary_container: Color,
    pub on_primary_container: Color,
    pub inverse_primary: Color,
    pub primary_fixed: Color,
    pub primary_fixed_dim: Color,
    pub on_primary_fixed: Color,
    pub on_primary_fixed_variant: Color,

    // Secondary Group
    pub secondary: Color,
    pub on_secondary: Color,
    pub secondary_container: Color,
    pub on_secondary_container: Color,
    pub secondary_fixed: Color,
    pub secondary_fixed_dim: Color,
    pub on_secondary_fixed: Color,
    pub on_secondary_fixed_variant: Color,

    // Tertiary Group
    pub tertiary: Color,
    pub on_tertiary: Color,
    pub tertiary_container: Color,
    pub on_tertiary_container: Color,
    pub tertiary_fixed: Color,
    pub tertiary_fixed_dim: Color,
    pub on_tertiary_fixed: Color,
    pub on_tertiary_fixed_variant: Color,

    // Error Group
    pub error: Color,
    pub on_error: Color,
    pub error_container: Color,
    pub on_error_container: Color,

    // Surface & Background Group
    pub surface: Color,
    pub on_surface: Color,
    pub surface_dim: Color,
    pub surface_bright: Color,
    pub surface_container_lowest: Color,
    pub surface_container_low: Color,
    pub surface_container: Color,
    pub surface_container_high: Color,
    pub surface_container_highest: Color,
    pub surface_variant: Color,
    pub on_surface_variant: Color,
    pub background: Color,
    pub on_background: Color,

    // Outlines & Tint
    pub outline: Color,
    pub outline_variant: Color,
    pub surface_tint: Color,

    // Inverse & Scrim Group
    pub inverse_surface: Color,
    pub inverse_on_surface: Color,
    pub shadow: Color,
    pub scrim: Color,
}

impl Default for ColorScheme {
    fn default() -> Self {
        // Baseline M3 default seed (#6750A4) in Dark Mode
        let seed = Color::from_rgb(103, 80, 164);
        Self::dark(seed, SchemeVariant::TonalSpot)
    }
}

impl ColorScheme {
    /// Derive a ColorScheme from a CorePalette for Light or Dark mode.
    pub fn from_core_palette(palette: &CorePalette, is_dark: bool) -> Self {
        Self::from_core_palette_with_contrast(palette, is_dark, 0.0)
    }

    /// Derive a ColorScheme from a CorePalette with explicit contrast adjustment (-1.0 to 1.0).
    pub fn from_core_palette_with_contrast(
        palette: &CorePalette,
        is_dark: bool,
        contrast: f64,
    ) -> Self {
        let c = contrast.clamp(-1.0, 1.0);

        // Helper closures to adjust tones dynamically based on contrast level
        let fg_tone = |base_light: f64, base_dark: f64| -> f64 {
            if is_dark {
                (base_dark + c * 10.0).clamp(0.0, 100.0)
            } else {
                (base_light - c * 10.0).clamp(0.0, 100.0)
            }
        };

        let bg_tone = |base_light: f64, base_dark: f64| -> f64 {
            if is_dark {
                (base_dark - c * 6.0).clamp(0.0, 100.0)
            } else {
                (base_light + c * 4.0).clamp(0.0, 100.0)
            }
        };

        let primary_tone = fg_tone(40.0, 80.0);
        let on_primary_tone = if is_dark { 20.0 } else { 100.0 };
        let primary_container_tone = bg_tone(90.0, 30.0);
        let on_primary_container_tone = if is_dark { 90.0 } else { 10.0 };

        let secondary_tone = fg_tone(40.0, 80.0);
        let on_secondary_tone = if is_dark { 20.0 } else { 100.0 };
        let secondary_container_tone = bg_tone(90.0, 30.0);
        let on_secondary_container_tone = if is_dark { 90.0 } else { 10.0 };

        let tertiary_tone = fg_tone(40.0, 80.0);
        let on_tertiary_tone = if is_dark { 20.0 } else { 100.0 };
        let tertiary_container_tone = bg_tone(90.0, 30.0);
        let on_tertiary_container_tone = if is_dark { 90.0 } else { 10.0 };

        let error_tone = fg_tone(40.0, 80.0);
        let on_error_tone = if is_dark { 20.0 } else { 100.0 };
        let error_container_tone = bg_tone(90.0, 30.0);
        let on_error_container_tone = if is_dark { 90.0 } else { 10.0 };

        let surface_tone = bg_tone(98.0, 6.0);
        let on_surface_tone = fg_tone(10.0, 90.0);

        let surface_dim_tone = if is_dark { 6.0 } else { 87.0 };
        let surface_bright_tone = if is_dark { 24.0 } else { 98.0 };
        let surface_container_lowest_tone = if is_dark { 4.0 } else { 100.0 };
        let surface_container_low_tone = if is_dark { 10.0 } else { 96.0 };
        let surface_container_tone = if is_dark { 12.0 } else { 94.0 };
        let surface_container_high_tone = if is_dark { 17.0 } else { 92.0 };
        let surface_container_highest_tone = if is_dark { 22.0 } else { 90.0 };

        let surface_variant_tone = if is_dark { 30.0 } else { 90.0 };
        let on_surface_variant_tone = if is_dark { 80.0 } else { 30.0 };

        let outline_tone = if is_dark { 60.0 } else { 50.0 };
        let outline_variant_tone = if is_dark { 30.0 } else { 80.0 };

        let inverse_surface_tone = if is_dark { 90.0 } else { 20.0 };
        let inverse_on_surface_tone = if is_dark { 20.0 } else { 95.0 };
        let inverse_primary_tone = if is_dark { 40.0 } else { 80.0 };

        Self {
            // Primary
            primary: palette.primary.get(primary_tone),
            on_primary: palette.primary.get(on_primary_tone),
            primary_container: palette.primary.get(primary_container_tone),
            on_primary_container: palette.primary.get(on_primary_container_tone),
            inverse_primary: palette.primary.get(inverse_primary_tone),
            primary_fixed: palette.primary.get(90.0),
            primary_fixed_dim: palette.primary.get(80.0),
            on_primary_fixed: palette.primary.get(10.0),
            on_primary_fixed_variant: palette.primary.get(30.0),

            // Secondary
            secondary: palette.secondary.get(secondary_tone),
            on_secondary: palette.secondary.get(on_secondary_tone),
            secondary_container: palette.secondary.get(secondary_container_tone),
            on_secondary_container: palette.secondary.get(on_secondary_container_tone),
            secondary_fixed: palette.secondary.get(90.0),
            secondary_fixed_dim: palette.secondary.get(80.0),
            on_secondary_fixed: palette.secondary.get(10.0),
            on_secondary_fixed_variant: palette.secondary.get(30.0),

            // Tertiary
            tertiary: palette.tertiary.get(tertiary_tone),
            on_tertiary: palette.tertiary.get(on_tertiary_tone),
            tertiary_container: palette.tertiary.get(tertiary_container_tone),
            on_tertiary_container: palette.tertiary.get(on_tertiary_container_tone),
            tertiary_fixed: palette.tertiary.get(90.0),
            tertiary_fixed_dim: palette.tertiary.get(80.0),
            on_tertiary_fixed: palette.tertiary.get(10.0),
            on_tertiary_fixed_variant: palette.tertiary.get(30.0),

            // Error
            error: palette.error.get(error_tone),
            on_error: palette.error.get(on_error_tone),
            error_container: palette.error.get(error_container_tone),
            on_error_container: palette.error.get(on_error_container_tone),

            // Surface & Background
            surface: palette.neutral.get(surface_tone),
            on_surface: palette.neutral.get(on_surface_tone),
            surface_dim: palette.neutral.get(surface_dim_tone),
            surface_bright: palette.neutral.get(surface_bright_tone),
            surface_container_lowest: palette.neutral.get(surface_container_lowest_tone),
            surface_container_low: palette.neutral.get(surface_container_low_tone),
            surface_container: palette.neutral.get(surface_container_tone),
            surface_container_high: palette.neutral.get(surface_container_high_tone),
            surface_container_highest: palette.neutral.get(surface_container_highest_tone),
            surface_variant: palette.neutral_variant.get(surface_variant_tone),
            on_surface_variant: palette.neutral_variant.get(on_surface_variant_tone),
            background: palette.neutral.get(surface_tone),
            on_background: palette.neutral.get(on_surface_tone),

            // Outlines & Tint
            outline: palette.neutral_variant.get(outline_tone),
            outline_variant: palette.neutral_variant.get(outline_variant_tone),
            surface_tint: palette.primary.get(primary_tone),

            // Inverse & Scrim
            inverse_surface: palette.neutral.get(inverse_surface_tone),
            inverse_on_surface: palette.neutral.get(inverse_on_surface_tone),
            shadow: Color::from_rgb(0, 0, 0),
            scrim: Color::from_rgb(0, 0, 0),
        }
    }

    /// Convenience builder for Light Mode ColorScheme from seed Color.
    pub fn light(seed: Color, variant: SchemeVariant) -> Self {
        let palette = CorePalette::from_seed_color(seed, variant);
        Self::from_core_palette(&palette, false)
    }

    /// Convenience builder for Dark Mode ColorScheme from seed Color.
    pub fn dark(seed: Color, variant: SchemeVariant) -> Self {
        let palette = CorePalette::from_seed_color(seed, variant);
        Self::from_core_palette(&palette, true)
    }

    /// Convert the ColorScheme to a HashMap with both snake_case and kebab-case keys.
    pub fn to_map(&self) -> HashMap<String, Color> {
        let mut map = HashMap::with_capacity(96);
        for (name, color) in self.iter() {
            map.insert(name.to_string(), color);
            map.insert(name.replace('_', "-"), color);
        }
        map
    }

    /// Get a color role by name (supports both snake_case and kebab-case).
    pub fn get_by_name(&self, name: &str) -> Option<Color> {
        let normalized = name.trim().to_lowercase().replace('-', "_");
        match normalized.as_str() {
            "primary" => Some(self.primary),
            "on_primary" => Some(self.on_primary),
            "primary_container" => Some(self.primary_container),
            "on_primary_container" => Some(self.on_primary_container),
            "inverse_primary" => Some(self.inverse_primary),
            "primary_fixed" => Some(self.primary_fixed),
            "primary_fixed_dim" => Some(self.primary_fixed_dim),
            "on_primary_fixed" => Some(self.on_primary_fixed),
            "on_primary_fixed_variant" => Some(self.on_primary_fixed_variant),

            "secondary" => Some(self.secondary),
            "on_secondary" => Some(self.on_secondary),
            "secondary_container" => Some(self.secondary_container),
            "on_secondary_container" => Some(self.on_secondary_container),
            "secondary_fixed" => Some(self.secondary_fixed),
            "secondary_fixed_dim" => Some(self.secondary_fixed_dim),
            "on_secondary_fixed" => Some(self.on_secondary_fixed),
            "on_secondary_fixed_variant" => Some(self.on_secondary_fixed_variant),

            "tertiary" => Some(self.tertiary),
            "on_tertiary" => Some(self.on_tertiary),
            "tertiary_container" => Some(self.tertiary_container),
            "on_tertiary_container" => Some(self.on_tertiary_container),
            "tertiary_fixed" => Some(self.tertiary_fixed),
            "tertiary_fixed_dim" => Some(self.tertiary_fixed_dim),
            "on_tertiary_fixed" => Some(self.on_tertiary_fixed),
            "on_tertiary_fixed_variant" => Some(self.on_tertiary_fixed_variant),

            "error" => Some(self.error),
            "on_error" => Some(self.on_error),
            "error_container" => Some(self.error_container),
            "on_error_container" => Some(self.on_error_container),

            "surface" => Some(self.surface),
            "on_surface" => Some(self.on_surface),
            "surface_dim" => Some(self.surface_dim),
            "surface_bright" => Some(self.surface_bright),
            "surface_container_lowest" => Some(self.surface_container_lowest),
            "surface_container_low" => Some(self.surface_container_low),
            "surface_container" => Some(self.surface_container),
            "surface_container_high" => Some(self.surface_container_high),
            "surface_container_highest" => Some(self.surface_container_highest),
            "surface_variant" => Some(self.surface_variant),
            "on_surface_variant" => Some(self.on_surface_variant),
            "background" => Some(self.background),
            "on_background" => Some(self.on_background),

            "outline" => Some(self.outline),
            "outline_variant" => Some(self.outline_variant),
            "surface_tint" => Some(self.surface_tint),

            "inverse_surface" => Some(self.inverse_surface),
            "inverse_on_surface" => Some(self.inverse_on_surface),
            "shadow" => Some(self.shadow),
            "scrim" => Some(self.scrim),
            _ => None,
        }
    }

    /// Iterator yielding all 47 color role names and their colors.
    pub fn iter(&self) -> impl Iterator<Item = (&'static str, Color)> {
        [
            ("primary", self.primary),
            ("on_primary", self.on_primary),
            ("primary_container", self.primary_container),
            ("on_primary_container", self.on_primary_container),
            ("inverse_primary", self.inverse_primary),
            ("primary_fixed", self.primary_fixed),
            ("primary_fixed_dim", self.primary_fixed_dim),
            ("on_primary_fixed", self.on_primary_fixed),
            ("on_primary_fixed_variant", self.on_primary_fixed_variant),

            ("secondary", self.secondary),
            ("on_secondary", self.on_secondary),
            ("secondary_container", self.secondary_container),
            ("on_secondary_container", self.on_secondary_container),
            ("secondary_fixed", self.secondary_fixed),
            ("secondary_fixed_dim", self.secondary_fixed_dim),
            ("on_secondary_fixed", self.on_secondary_fixed),
            ("on_secondary_fixed_variant", self.on_secondary_fixed_variant),

            ("tertiary", self.tertiary),
            ("on_tertiary", self.on_tertiary),
            ("tertiary_container", self.tertiary_container),
            ("on_tertiary_container", self.on_tertiary_container),
            ("tertiary_fixed", self.tertiary_fixed),
            ("tertiary_fixed_dim", self.tertiary_fixed_dim),
            ("on_tertiary_fixed", self.on_tertiary_fixed),
            ("on_tertiary_fixed_variant", self.on_tertiary_fixed_variant),

            ("error", self.error),
            ("on_error", self.on_error),
            ("error_container", self.error_container),
            ("on_error_container", self.on_error_container),

            ("surface", self.surface),
            ("on_surface", self.on_surface),
            ("surface_dim", self.surface_dim),
            ("surface_bright", self.surface_bright),
            ("surface_container_lowest", self.surface_container_lowest),
            ("surface_container_low", self.surface_container_low),
            ("surface_container", self.surface_container),
            ("surface_container_high", self.surface_container_high),
            ("surface_container_highest", self.surface_container_highest),
            ("surface_variant", self.surface_variant),
            ("on_surface_variant", self.on_surface_variant),
            ("background", self.background),
            ("on_background", self.on_background),

            ("outline", self.outline),
            ("outline_variant", self.outline_variant),
            ("surface_tint", self.surface_tint),

            ("inverse_surface", self.inverse_surface),
            ("inverse_on_surface", self.inverse_on_surface),
            ("shadow", self.shadow),
            ("scrim", self.scrim),
        ].into_iter()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_color_scheme_generation_and_roles() {
        let seed = Color::from_hex("#6750A4").unwrap();
        let light = ColorScheme::light(seed, SchemeVariant::TonalSpot);
        let dark = ColorScheme::dark(seed, SchemeVariant::TonalSpot);

        assert_eq!(light.iter().count(), 49);
        assert_eq!(dark.iter().count(), 49);

        let map_light = light.to_map();
        assert!(map_light.contains_key("primary"));
        assert!(map_light.contains_key("surface_container_highest"));
        assert!(map_light.contains_key("surface-container-highest"));

        assert_ne!(light.primary, light.on_primary);
        assert_ne!(dark.primary, dark.on_primary);
    }

    #[test]
    fn test_dynamic_contrast_direction_monotonicity() {
        use crate::color::contrast_ratio;

        let seed = Color::from_hex("#6750A4").unwrap();
        let palette = CorePalette::from_seed_color(seed, SchemeVariant::TonalSpot);

        // Light mode: high contrast must have higher contrast ratio than low contrast
        let light_low = ColorScheme::from_core_palette_with_contrast(&palette, false, -1.0);
        let light_normal = ColorScheme::from_core_palette_with_contrast(&palette, false, 0.0);
        let light_high = ColorScheme::from_core_palette_with_contrast(&palette, false, 1.0);

        let cr_light_low = contrast_ratio(light_low.primary, light_low.on_primary);
        let cr_light_normal = contrast_ratio(light_normal.primary, light_normal.on_primary);
        let cr_light_high = contrast_ratio(light_high.primary, light_high.on_primary);

        assert!(cr_light_high > cr_light_normal);
        assert!(cr_light_normal > cr_light_low);
        assert!(cr_light_high >= 7.0);

        // Dark mode: high contrast must have higher contrast ratio than low contrast
        let dark_low = ColorScheme::from_core_palette_with_contrast(&palette, true, -1.0);
        let dark_normal = ColorScheme::from_core_palette_with_contrast(&palette, true, 0.0);
        let dark_high = ColorScheme::from_core_palette_with_contrast(&palette, true, 1.0);

        let cr_dark_low = contrast_ratio(dark_low.primary, dark_low.on_primary);
        let cr_dark_normal = contrast_ratio(dark_normal.primary, dark_normal.on_primary);
        let cr_dark_high = contrast_ratio(dark_high.primary, dark_high.on_primary);

        assert!(cr_dark_high > cr_dark_normal);
        assert!(cr_dark_normal > cr_dark_low);
        assert!(cr_dark_high >= 7.0);
    }
}
