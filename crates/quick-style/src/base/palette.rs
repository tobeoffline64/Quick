//! 11-step neutral gray palette + semantic colors for the Fluent base theme.
//! Colors are sourced from Avalonia's FluentTheme neutral scale.

use quick_core::geometry::Color;

/// 11-step neutral gray scale (light to dark).
pub struct NeutralPalette;

impl NeutralPalette {
    // Light surface / bg
    pub const N0:   Color = Color::from_rgb(250, 250, 250); // #FAFAFA
    pub const N10:  Color = Color::from_rgb(245, 245, 245); // #F5F5F5
    pub const N20:  Color = Color::from_rgb(238, 238, 238); // #EEEEEE
    pub const N30:  Color = Color::from_rgb(224, 224, 224); // #E0E0E0 — default border
    pub const N40:  Color = Color::from_rgb(189, 189, 189); // #BDBDBD
    pub const N50:  Color = Color::from_rgb(158, 158, 158); // #9E9E9E — placeholder text
    pub const N60:  Color = Color::from_rgb(117, 117, 117); // #757575 — secondary text
    pub const N70:  Color = Color::from_rgb(97,  97,  97 ); // #616161
    pub const N80:  Color = Color::from_rgb(66,  66,  66 ); // #424242 — body text light
    pub const N90:  Color = Color::from_rgb(33,  33,  33 ); // #212121 — heading text
    pub const N100: Color = Color::from_rgb(26,  26,  26 ); // #1A1A1A — near-black

    // Dark mode equivalents (inverted surface/text roles)
    pub const DARK_SURFACE:       Color = Color::from_rgb(28,  28,  28 ); // #1C1C1C
    pub const DARK_SURFACE_RAISED:Color = Color::from_rgb(38,  38,  38 ); // #262626
    pub const DARK_SURFACE_HIGH:  Color = Color::from_rgb(48,  48,  48 ); // #303030
    pub const DARK_BORDER:        Color = Color::from_rgb(64,  64,  64 ); // #404040
    pub const DARK_TEXT_PRIMARY:  Color = Color::from_rgb(230, 230, 230); // #E6E6E6
    pub const DARK_TEXT_SECONDARY:Color = Color::from_rgb(160, 160, 160); // #A0A0A0

    // Fluent default accent (Windows blue) — overridden by OS accent at runtime
    pub const ACCENT_DEFAULT:     Color = Color::from_rgb(0,   120, 212); // #0078D4
    pub const ACCENT_HOVER:       Color = Color::from_rgb(16,  137, 227); // #1089E3
    pub const ACCENT_PRESSED:     Color = Color::from_rgb(0,   102, 180); // #0066B4
    pub const ACCENT_DISABLED:    Color = Color::from_rgb(189, 189, 189); // #BDBDBD

    // Semantic
    pub const ERROR:   Color = Color::from_rgb(196,  43, 28 ); // #C42B1C
    pub const SUCCESS: Color = Color::from_rgb( 15, 157, 88 ); // #0F9D58
    pub const WARNING: Color = Color::from_rgb(255, 185,  0 ); // #FFB900
}

/// Runtime-resolved accent color — starts with Fluent default,
/// overridden by `SystemColorDetector::accent_color()` at app startup.
#[derive(Debug, Clone, Copy)]
pub struct AccentColors {
    pub normal:   Color,
    pub hover:    Color,
    pub pressed:  Color,
    pub disabled: Color,
    pub on_accent: Color,
}

impl Default for AccentColors {
    fn default() -> Self {
        Self {
            normal:    NeutralPalette::ACCENT_DEFAULT,
            hover:     NeutralPalette::ACCENT_HOVER,
            pressed:   NeutralPalette::ACCENT_PRESSED,
            disabled:  NeutralPalette::ACCENT_DISABLED,
            on_accent: Color::WHITE,
        }
    }
}

impl AccentColors {
    /// Derive hover/pressed/on_accent variants from any seed accent color.
    pub fn from_color(accent: Color) -> Self {
        let (r, g, b) = (accent.r as f32, accent.g as f32, accent.b as f32);
        // Hover: +10% luminance
        let hover = Color::from_rgb(
            (r * 1.1).min(255.0) as u8,
            (g * 1.1).min(255.0) as u8,
            (b * 1.1).min(255.0) as u8,
        );
        // Pressed: -15% luminance
        let pressed = Color::from_rgb(
            (r * 0.85) as u8,
            (g * 0.85) as u8,
            (b * 0.85) as u8,
        );
        // on_accent: white if dark enough, dark if light
        let luminance = 0.299 * r + 0.587 * g + 0.114 * b;
        let on_accent = if luminance < 128.0 { Color::WHITE } else { NeutralPalette::N90 };

        Self { normal: accent, hover, pressed, disabled: NeutralPalette::ACCENT_DISABLED, on_accent }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_accent_colors_from_fluent_blue() {
        let ac = AccentColors::from_color(NeutralPalette::ACCENT_DEFAULT);
        assert_eq!(ac.on_accent, Color::WHITE);
        assert!(ac.hover.r > ac.normal.r || ac.hover.g > ac.normal.g || ac.hover.b > ac.normal.b);
        assert!(ac.pressed.b < ac.normal.b);
    }

    #[test]
    fn test_neutral_palette_ordering() {
        // N0 should be lighter (higher values) than N100
        assert!(NeutralPalette::N0.r > NeutralPalette::N100.r);
    }
}
