//! Dynamic Material You ThemePackage and Dynamic CSS Generator.

use crate::theme::color_scheme::ColorScheme;
use crate::theme::scheme::{DynamicScheme, SchemeVariant};
use crate::theme::tokens::{ElevationTokens, ShapeTokens, StateLayerTokens};
use quick_core::geometry::Color;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Unified Theme Package encapsulating dynamic color schemes, tokens, and CSS generators.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThemePackage {
    pub name: String,
    pub color_scheme: ColorScheme,
    pub shapes: ShapeTokens,
    pub elevation: ElevationTokens,
    pub state_layers: StateLayerTokens,
    pub is_dark: bool,
    pub contrast: f64,
    pub colors: HashMap<String, Color>,
    pub shape_map: HashMap<String, f32>,
}

impl Default for ThemePackage {
    fn default() -> Self {
        Self::material_you()
    }
}

impl ThemePackage {
    /// Creates an empty ThemePackage with default shapes and elevation tokens, and empty colors.
    pub fn new(name: impl Into<String>) -> Self {
        let name_str = name.into();
        let color_scheme = ColorScheme::default();
        let shapes = ShapeTokens::default();
        let elevation = ElevationTokens::default();
        let state_layers = StateLayerTokens::default();
        let colors = HashMap::new();
        let shape_map = shapes.to_map();

        Self {
            name: name_str,
            color_scheme,
            shapes,
            elevation,
            state_layers,
            is_dark: true,
            contrast: 0.0,
            colors,
            shape_map,
        }
    }

    /// Dynamic Material You generator from a seed Color.
    pub fn from_seed_color(seed: Color, variant: SchemeVariant, is_dark: bool) -> Self {
        Self::from_seed_color_with_contrast(seed, variant, is_dark, 0.0)
    }

    /// Dynamic Material You generator from a seed hex string.
    pub fn from_seed_hex(hex: &str, variant: SchemeVariant, is_dark: bool) -> Result<Self, String> {
        let color = Color::from_hex(hex)?;
        Ok(Self::from_seed_color(color, variant, is_dark))
    }

    /// Dynamic Material You generator with explicit WCAG contrast adjustment (-1.0 to 1.0).
    pub fn from_seed_color_with_contrast(
        seed: Color,
        variant: SchemeVariant,
        is_dark: bool,
        contrast: f64,
    ) -> Self {
        let dynamic_scheme = DynamicScheme::new(seed, variant, is_dark, contrast);
        let color_scheme = dynamic_scheme.to_color_scheme();
        let shapes = ShapeTokens::default();
        let elevation = ElevationTokens::default();
        let state_layers = StateLayerTokens::default();

        let colors = color_scheme.to_map();
        let shape_map = shapes.to_map();

        Self {
            name: "material-you".to_string(),
            color_scheme,
            shapes,
            elevation,
            state_layers,
            is_dark,
            contrast,
            colors,
            shape_map,
        }
    }

    /// Built-in baseline Material You Dark Theme (Default Seed #6750A4).
    pub fn material_you() -> Self {
        let seed = Color::from_rgb(103, 80, 164); // #6750A4
        let mut theme = Self::from_seed_color(seed, SchemeVariant::TonalSpot, true);
        let primary = Color::from_hex("#D0BCFF").unwrap();
        let on_primary = Color::from_hex("#381E72").unwrap();
        let primary_container = Color::from_hex("#4F378B").unwrap();
        let on_primary_container = Color::from_hex("#EADDFF").unwrap();
        let surface = Color::from_hex("#141218").unwrap();
        let on_surface = Color::from_hex("#E6E0E9").unwrap();
        let surface_container = Color::from_hex("#211F26").unwrap();
        let surface_container_high = Color::from_hex("#2B2930").unwrap();
        let outline = Color::from_hex("#938F99").unwrap();
        let outline_variant = Color::from_hex("#49454F").unwrap();
        let error = Color::from_hex("#F2B8B5").unwrap();

        theme.colors.insert("primary".into(), primary);
        theme.colors.insert("on_primary".into(), on_primary);
        theme.colors.insert("primary_container".into(), primary_container);
        theme.colors.insert("on_primary_container".into(), on_primary_container);
        theme.colors.insert("surface".into(), surface);
        theme.colors.insert("on_surface".into(), on_surface);
        theme.colors.insert("surface_container".into(), surface_container);
        theme.colors.insert("surface_container_high".into(), surface_container_high);
        theme.colors.insert("outline".into(), outline);
        theme.colors.insert("outline_variant".into(), outline_variant);
        theme.colors.insert("error".into(), error);

        theme.color_scheme.primary = primary;
        theme.color_scheme.on_primary = on_primary;
        theme.color_scheme.primary_container = primary_container;
        theme.color_scheme.on_primary_container = on_primary_container;
        theme.color_scheme.surface = surface;
        theme.color_scheme.on_surface = on_surface;
        theme.color_scheme.surface_container = surface_container;
        theme.color_scheme.surface_container_high = surface_container_high;
        theme.color_scheme.outline = outline;
        theme.color_scheme.outline_variant = outline_variant;
        theme.color_scheme.error = error;

        theme
    }

    /// Built-in baseline Material You Light Theme (Default Seed #6750A4).
    pub fn material_you_light() -> Self {
        let seed = Color::from_rgb(103, 80, 164); // #6750A4
        Self::from_seed_color(seed, SchemeVariant::TonalSpot, false)
    }

    /// Built-in Nord Arctic Palette for backward compatibility.
    pub fn nord() -> Self {
        let mut theme = Self::new("nord");
        theme.is_dark = true;

        let primary = Color::from_hex("#88C0D0").unwrap();
        let on_primary = Color::from_hex("#2E3440").unwrap();
        let surface = Color::from_hex("#2E3440").unwrap();
        let on_surface = Color::from_hex("#ECEFF4").unwrap();
        let surface_container = Color::from_hex("#3B4252").unwrap();
        let surface_container_high = Color::from_hex("#434C5E").unwrap();
        let outline = Color::from_hex("#4C566A").unwrap();
        let outline_variant = Color::from_hex("#3B4252").unwrap();
        let error = Color::from_hex("#BF616A").unwrap();

        theme.colors.insert("primary".into(), primary);
        theme.colors.insert("on_primary".into(), on_primary);
        theme.colors.insert("on-primary".into(), on_primary);
        theme.colors.insert("primary_container".into(), surface_container_high);
        theme.colors.insert("primary-container".into(), surface_container_high);
        theme.colors.insert("surface".into(), surface);
        theme.colors.insert("on_surface".into(), on_surface);
        theme.colors.insert("on-surface".into(), on_surface);
        theme.colors.insert("surface_container".into(), surface_container);
        theme.colors.insert("surface-container".into(), surface_container);
        theme.colors.insert("surface_container_high".into(), surface_container_high);
        theme.colors.insert("surface-container-high".into(), surface_container_high);
        theme.colors.insert("outline".into(), outline);
        theme.colors.insert("outline_variant".into(), outline_variant);
        theme.colors.insert("outline-variant".into(), outline_variant);
        theme.colors.insert("error".into(), error);

        theme.shapes.corner_small = 6.0;
        theme.shapes.corner_medium = 12.0;
        theme.shapes.corner_large = 16.0;
        theme.shapes.corner_full = 999.0;
        theme.shape_map = theme.shapes.to_map();

        theme
    }

    /// Native Noctalia Dark Theme Preset (Warm gold primary, midnight navy surface, mint hover).
    pub fn noctalia() -> Self {
        let mut theme = Self::new("noctalia");
        theme.is_dark = true;

        let pal = crate::noctalia::palette::NoctaliaPalette::noctalia_dark();
        theme.colors.insert("primary".into(), pal.primary);
        theme.colors.insert("on_primary".into(), pal.on_primary);
        theme.colors.insert("on-primary".into(), pal.on_primary);
        theme.colors.insert("secondary".into(), pal.secondary);
        theme.colors.insert("on_secondary".into(), pal.on_secondary);
        theme.colors.insert("tertiary".into(), pal.tertiary);
        theme.colors.insert("surface".into(), pal.surface);
        theme.colors.insert("on_surface".into(), pal.on_surface);
        theme.colors.insert("on-surface".into(), pal.on_surface);
        theme.colors.insert("surface_variant".into(), pal.surface_variant);
        theme.colors.insert("surface-variant".into(), pal.surface_variant);
        theme.colors.insert("surface_container".into(), pal.surface_variant);
        theme.colors.insert("surface-container".into(), pal.surface_variant);
        theme.colors.insert("outline".into(), pal.outline);
        theme.colors.insert("error".into(), pal.error);
        theme.colors.insert("hover".into(), pal.hover);

        theme.shapes.corner_small = 4.0;
        theme.shapes.corner_medium = 8.0;
        theme.shapes.corner_large = 12.0;
        theme.shapes.corner_full = 9999.0;
        theme.shape_map = theme.shapes.to_map();

        theme
    }

    /// Native Noctalia Light Theme Preset.
    pub fn noctalia_light() -> Self {
        let mut theme = Self::new("noctalia-light");
        theme.is_dark = false;

        let pal = crate::noctalia::palette::NoctaliaPalette::noctalia_light();
        theme.colors.insert("primary".into(), pal.primary);
        theme.colors.insert("on_primary".into(), pal.on_primary);
        theme.colors.insert("on-primary".into(), pal.on_primary);
        theme.colors.insert("secondary".into(), pal.secondary);
        theme.colors.insert("on_secondary".into(), pal.on_secondary);
        theme.colors.insert("tertiary".into(), pal.tertiary);
        theme.colors.insert("surface".into(), pal.surface);
        theme.colors.insert("on_surface".into(), pal.on_surface);
        theme.colors.insert("on-surface".into(), pal.on_surface);
        theme.colors.insert("surface_variant".into(), pal.surface_variant);
        theme.colors.insert("surface-variant".into(), pal.surface_variant);
        theme.colors.insert("surface_container".into(), pal.surface_variant);
        theme.colors.insert("surface-container".into(), pal.surface_variant);
        theme.colors.insert("outline".into(), pal.outline);
        theme.colors.insert("error".into(), pal.error);
        theme.colors.insert("hover".into(), pal.hover);

        theme.shapes.corner_small = 4.0;
        theme.shapes.corner_medium = 8.0;
        theme.shapes.corner_large = 12.0;
        theme.shapes.corner_full = 9999.0;
        theme.shape_map = theme.shapes.to_map();

        theme
    }

    /// Get color by role name.
    pub fn get_color(&self, role: &str) -> Option<Color> {
        self.colors.get(role).copied().or_else(|| self.color_scheme.get_by_name(role))
    }

    /// Get shape radius by token name.
    pub fn get_shape(&self, name: &str) -> Option<f32> {
        self.shapes.get(name).copied()
    }

    /// Generates dynamic CSS rules implementing the full Material Design 3 specification.
    pub fn generate_css(&self) -> String {
        if self.colors.is_empty() {
            return String::new();
        }

        let mut css = String::with_capacity(4096);
        let cs = &self.color_scheme;
        let shapes = &self.shapes;
        let states = &self.state_layers;

        // 1. Root & Base Containers
        css.push_str(&format!(
            "VStack#app-root, HStack#app-root, Container#app-root {{ background: {}; }}\n",
            cs.surface.to_hex()
        ));

        // 2. Typography
        css.push_str(&format!(
            "Text {{ color: {}; font-size: 14px; }}\n\
             Text.title {{ color: {}; font-size: 24px; font-weight: bold; }}\n\
             Text.subtitle {{ color: {}; font-size: 14px; }}\n\
             Text.body {{ color: {}; font-size: 14px; }}\n\
             Text.label {{ color: {}; font-size: 12px; }}\n\
             Text.greeting {{ color: {}; font-size: 16px; font-weight: bold; }}\n\
             Text.description {{ color: {}; font-size: 13px; }}\n\
             Text.pill-badge {{ background: {}; color: {}; border-radius: {:.0}px; font-size: 11px; font-weight: bold; padding: 6px 14px; text-align: center; }}\n",
            cs.on_surface.to_hex(),
            cs.on_surface.to_hex(),
            cs.on_surface_variant.to_hex(),
            cs.on_surface.to_hex(),
            cs.on_surface_variant.to_hex(),
            cs.primary.to_hex(),
            cs.on_surface_variant.to_hex(),
            cs.primary_container.to_hex(),
            cs.on_primary_container.to_hex(),
            shapes.corner_full
        ));

        // 3. Buttons
        // A. Filled Button (Default)
        let filled_hover = states.apply_hover(cs.primary, cs.on_primary);
        let filled_pressed = states.apply_pressed(cs.primary, cs.on_primary);
        css.push_str(&format!(
            "Button, Button[variant=\"filled\"], Button.btn-primary, Button.filled {{\n\
                 background: {};\n\
                 color: {};\n\
                 border-radius: {:.0}px;\n\
                 padding: 10px 24px;\n\
                 font-size: 14px;\n\
                 font-weight: 500;\n\
             }}\n\
             Button:hover, Button[variant=\"filled\"]:hover, Button.btn-primary:hover, Button.filled:hover {{\n\
                 background: {};\n\
             }}\n\
             Button:active, Button:pressed, Button[variant=\"filled\"]:active, Button[variant=\"filled\"]:pressed, Button.btn-primary:active, Button.btn-primary:pressed {{\n\
                 background: {};\n\
             }}\n",
            cs.primary.to_hex(),
            cs.on_primary.to_hex(),
            shapes.corner_full,
            filled_hover.to_hex(),
            filled_pressed.to_hex()
        ));

        // B. Tonal Button
        let tonal_hover = states.apply_hover(cs.secondary_container, cs.on_secondary_container);
        let tonal_pressed = states.apply_pressed(cs.secondary_container, cs.on_secondary_container);
        css.push_str(&format!(
            "Button[variant=\"tonal\"], Button.btn-tonal, Button.tonal {{\n\
                 background: {};\n\
                 color: {};\n\
                 border-radius: {:.0}px;\n\
                 padding: 10px 24px;\n\
                 font-size: 14px;\n\
                 font-weight: 500;\n\
             }}\n\
             Button[variant=\"tonal\"]:hover, Button.btn-tonal:hover, Button.tonal:hover {{\n\
                 background: {};\n\
             }}\n\
             Button[variant=\"tonal\"]:active, Button[variant=\"tonal\"]:pressed, Button.btn-tonal:active {{\n\
                 background: {};\n\
             }}\n",
            cs.secondary_container.to_hex(),
            cs.on_secondary_container.to_hex(),
            shapes.corner_full,
            tonal_hover.to_hex(),
            tonal_pressed.to_hex()
        ));

        // C. Elevated Button
        let elevated_hover = states.apply_hover(cs.surface_container_low, cs.primary);
        let elevated_pressed = states.apply_pressed(cs.surface_container_low, cs.primary);
        css.push_str(&format!(
            "Button[variant=\"elevated\"], Button.btn-elevated, Button.elevated {{\n\
                 background: {};\n\
                 color: {};\n\
                 border-radius: {:.0}px;\n\
                 padding: 10px 24px;\n\
                 font-size: 14px;\n\
                 font-weight: 500;\n\
             }}\n\
             Button[variant=\"elevated\"]:hover, Button.btn-elevated:hover, Button.elevated:hover {{\n\
                 background: {};\n\
             }}\n\
             Button[variant=\"elevated\"]:active, Button[variant=\"elevated\"]:pressed, Button.btn-elevated:active {{\n\
                 background: {};\n\
             }}\n",
            cs.surface_container_low.to_hex(),
            cs.primary.to_hex(),
            shapes.corner_full,
            elevated_hover.to_hex(),
            elevated_pressed.to_hex()
        ));

        // D. Outlined Button
        let outlined_hover = states.apply_hover(cs.surface, cs.primary);
        css.push_str(&format!(
            "Button[variant=\"outlined\"], Button.btn-outlined, Button.outlined {{\n\
                 background: transparent;\n\
                 color: {};\n\
                 border-color: {};\n\
                 border-width: 1px;\n\
                 border-radius: {:.0}px;\n\
                 padding: 10px 24px;\n\
                 font-size: 14px;\n\
                 font-weight: 500;\n\
             }}\n\
             Button[variant=\"outlined\"]:hover, Button.btn-outlined:hover, Button.outlined:hover {{\n\
                 background: {};\n\
             }}\n",
            cs.primary.to_hex(),
            cs.outline.to_hex(),
            shapes.corner_full,
            outlined_hover.to_hex()
        ));

        // E. Text Button
        let text_hover = states.apply_hover(cs.surface, cs.primary);
        css.push_str(&format!(
            "Button[variant=\"text\"], Button.btn-text, Button.text {{\n\
                 background: transparent;\n\
                 color: {};\n\
                 border-radius: {:.0}px;\n\
                 padding: 10px 16px;\n\
                 font-size: 14px;\n\
                 font-weight: 500;\n\
             }}\n\
             Button[variant=\"text\"]:hover, Button.btn-text:hover, Button.text:hover {{\n\
                 background: {};\n\
             }}\n",
            cs.primary.to_hex(),
            shapes.corner_full,
            text_hover.to_hex()
        ));

        // F. Secondary / Accent Buttons
        let sec_hover = states.apply_hover(cs.surface_variant, cs.on_surface_variant);
        css.push_str(&format!(
            "Button.btn-secondary {{\n\
                 background: {};\n\
                 color: {};\n\
                 padding: 10px 20px;\n\
                 border-radius: {:.0}px;\n\
                 font-size: 14px;\n\
             }}\n\
             Button.btn-secondary:hover {{\n\
                 background: {};\n\
             }}\n",
            cs.surface_variant.to_hex(),
            cs.on_surface_variant.to_hex(),
            shapes.corner_full,
            sec_hover.to_hex()
        ));

        // 4. Cards
        css.push_str(&format!(
            "Card, Card[variant=\"elevated\"], Card.elevated, Card.main-card {{\n\
                 background: {};\n\
                 border-radius: {:.0}px;\n\
                 border-color: {};\n\
                 border-width: 1px;\n\
                 padding: 24px;\n\
                 gap: 16px;\n\
             }}\n\
             Card[variant=\"filled\"], Card.filled {{\n\
                 background: {};\n\
                 border-radius: {:.0}px;\n\
                 border-width: 0px;\n\
                 padding: 24px;\n\
                 gap: 16px;\n\
             }}\n\
             Card[variant=\"outlined\"], Card.outlined {{\n\
                 background: {};\n\
                 border-color: {};\n\
                 border-width: 1px;\n\
                 border-radius: {:.0}px;\n\
                 padding: 24px;\n\
                 gap: 16px;\n\
             }}\n",
            cs.surface_container_low.to_hex(),
            shapes.corner_large,
            cs.outline_variant.to_hex(),
            cs.surface_container_highest.to_hex(),
            shapes.corner_large,
            cs.surface.to_hex(),
            cs.outline_variant.to_hex(),
            shapes.corner_large
        ));

        // 5. Selection Controls
        // Switch
        css.push_str(&format!(
            "Switch {{\n\
                 background: {};\n\
                 border-color: {};\n\
                 border-width: 2px;\n\
                 border-radius: {:.0}px;\n\
             }}\n\
             Switch[checked=\"true\"], Switch.checked {{\n\
                 background: {};\n\
                 border-color: {};\n\
             }}\n",
            cs.surface_container_highest.to_hex(),
            cs.outline.to_hex(),
            shapes.corner_full,
            cs.primary.to_hex(),
            cs.primary.to_hex()
        ));

        // Checkbox
        css.push_str(&format!(
            "Checkbox {{\n\
                 background: transparent;\n\
                 border-color: {};\n\
                 border-width: 2px;\n\
                 border-radius: {:.0}px;\n\
             }}\n\
             Checkbox[checked=\"true\"], Checkbox.checked {{\n\
                 background: {};\n\
                 border-color: {};\n\
             }}\n",
            cs.outline.to_hex(),
            shapes.corner_extra_small,
            cs.primary.to_hex(),
            cs.primary.to_hex()
        ));

        // Slider
        css.push_str(&format!(
            "Slider {{\n\
                 background: {};\n\
                 color: {};\n\
                 border-radius: {:.0}px;\n\
             }}\n",
            cs.surface_container_highest.to_hex(),
            cs.primary.to_hex(),
            shapes.corner_full
        ));

        // Chip
        let chip_hover = states.apply_hover(cs.surface_container_low, cs.on_surface_variant);
        css.push_str(&format!(
            "Chip {{\n\
                 background: {};\n\
                 border-color: {};\n\
                 border-width: 1px;\n\
                 border-radius: {:.0}px;\n\
                 color: {};\n\
                 padding: 6px 14px;\n\
             }}\n\
             Chip:hover {{\n\
                 background: {};\n\
             }}\n\
             Chip[selected=\"true\"], Chip.selected {{\n\
                 background: {};\n\
                 color: {};\n\
                 border-color: {};\n\
             }}\n",
            cs.surface_container_low.to_hex(),
            cs.outline_variant.to_hex(),
            shapes.corner_small,
            cs.on_surface_variant.to_hex(),
            chip_hover.to_hex(),
            cs.secondary_container.to_hex(),
            cs.on_secondary_container.to_hex(),
            cs.secondary_container.to_hex()
        ));

        // 6. Progress Bar
        css.push_str(&format!(
            "ProgressBar {{\n\
                 background: {};\n\
                 color: {};\n\
                 border-radius: {:.0}px;\n\
             }}\n",
            cs.surface_container_highest.to_hex(),
            cs.primary.to_hex(),
            shapes.corner_full
        ));

        // 7. Text Input
        css.push_str(&format!(
            "TextInput, TextInput[variant=\"filled\"] {{\n\
                 background: {};\n\
                 color: {};\n\
                 border-radius: {:.0}px;\n\
                 border-width: 1px;\n\
                 border-color: transparent;\n\
                 padding: 8px 12px;\n\
             }}\n\
             TextInput[variant=\"outlined\"] {{\n\
                 background: transparent;\n\
                 color: {};\n\
                 border-color: {};\n\
                 border-width: 1px;\n\
                 border-radius: {:.0}px;\n\
                 padding: 8px 12px;\n\
             }}\n\
             TextInput:focus, TextInput:focused {{\n\
                 border-color: {};\n\
                 border-width: 2px;\n\
             }}\n",
            cs.surface_container_highest.to_hex(),
            cs.on_surface.to_hex(),
            shapes.corner_extra_small,
            cs.on_surface.to_hex(),
            cs.outline.to_hex(),
            shapes.corner_extra_small,
            cs.primary.to_hex()
        ));

        css
    }

    /// Converts generated CSS directly into a parsed StyleSheet instance.
    pub fn to_stylesheet(&self) -> crate::rule::StyleSheet {
        crate::parser::parse_stylesheet(&self.generate_css())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_theme_package_generation_and_css() {
        let theme = ThemePackage::material_you();
        assert_eq!(theme.name, "material-you");
        assert!(theme.is_dark);
        assert!(theme.get_color("primary").is_some());

        let css = theme.generate_css();
        assert!(css.contains("Button"));
        assert!(css.contains("Card"));
        assert!(css.contains("Switch"));
        assert!(css.contains("TextInput"));

        let stylesheet = theme.to_stylesheet();
        assert!(!stylesheet.rules.is_empty());
    }

    #[test]
    fn test_nord_theme() {
        let nord = ThemePackage::nord();
        assert_eq!(nord.name, "nord");
        assert!(nord.get_color("primary").is_some());
        let css = nord.generate_css();
        assert!(!css.is_empty());
    }
}
