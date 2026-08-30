//! Noctalia brand color palettes and theme definitions.

use quick_core::geometry::Color;

#[derive(Debug, Clone, PartialEq)]
pub struct NoctaliaPalette {
    pub name: String,
    pub is_dark: bool,
    pub primary: Color,
    pub on_primary: Color,
    pub secondary: Color,
    pub on_secondary: Color,
    pub tertiary: Color,
    pub on_tertiary: Color,
    pub error: Color,
    pub on_error: Color,
    pub surface: Color,
    pub on_surface: Color,
    pub surface_variant: Color,
    pub on_surface_variant: Color,
    pub outline: Color,
    pub shadow: Color,
    pub hover: Color,
    pub on_hover: Color,
}

impl NoctaliaPalette {
    /// Native Noctalia Dark Brand Palette (Warm gold primary, midnight navy surface, mint accent).
    pub fn noctalia_dark() -> Self {
        Self {
            name: "Noctalia (Dark)".into(),
            is_dark: true,
            primary: Color::from_hex("#fff59b").unwrap(),          // Noctalia Gold
            on_primary: Color::from_hex("#0e0e43").unwrap(),        // Deep Navy
            secondary: Color::from_hex("#a9aefe").unwrap(),        // Soft Periwinkle
            on_secondary: Color::from_hex("#0e0e43").unwrap(),
            tertiary: Color::from_hex("#9BFECE").unwrap(),         // Mint Green
            on_tertiary: Color::from_hex("#0e0e43").unwrap(),
            error: Color::from_hex("#FD4663").unwrap(),            // Coral Pink/Red
            on_error: Color::from_hex("#0e0e43").unwrap(),
            surface: Color::from_hex("#070722").unwrap(),          // Midnight Background
            on_surface: Color::from_hex("#f3edf7").unwrap(),        // Warm White Text
            surface_variant: Color::from_hex("#11112d").unwrap(),   // Indigo Container
            on_surface_variant: Color::from_hex("#7c80b4").unwrap(), // Muted Text
            outline: Color::from_hex("#21215F").unwrap(),          // Soft Border
            shadow: Color::from_hex("#070722").unwrap(),
            hover: Color::from_hex("#9BFECE").unwrap(),
            on_hover: Color::from_hex("#0e0e43").unwrap(),
        }
    }

    /// Native Noctalia Light Brand Palette.
    pub fn noctalia_light() -> Self {
        Self {
            name: "Noctalia (Light)".into(),
            is_dark: false,
            primary: Color::from_hex("#5d65f5").unwrap(),
            on_primary: Color::from_hex("#dadcff").unwrap(),
            secondary: Color::from_hex("#8E93D8").unwrap(),
            on_secondary: Color::from_hex("#dadcff").unwrap(),
            tertiary: Color::from_hex("#0e0e43").unwrap(),
            on_tertiary: Color::from_hex("#fef29a").unwrap(),
            error: Color::from_hex("#FD4663").unwrap(),
            on_error: Color::from_hex("#0e0e43").unwrap(),
            surface: Color::from_hex("#e6e8fa").unwrap(),
            on_surface: Color::from_hex("#0e0e43").unwrap(),
            surface_variant: Color::from_hex("#eff0ff").unwrap(),
            on_surface_variant: Color::from_hex("#4b55c8").unwrap(),
            outline: Color::from_hex("#8288fc").unwrap(),
            shadow: Color::from_hex("#cccccc").unwrap(),
            hover: Color::from_hex("#5d65f5").unwrap(),
            on_hover: Color::from_hex("#ffffff").unwrap(),
        }
    }

    pub fn catppuccin_mocha() -> Self {
        Self {
            name: "Catppuccin Mocha".into(),
            is_dark: true,
            primary: Color::from_hex("#cba6f7").unwrap(),
            on_primary: Color::from_hex("#1e1e2e").unwrap(),
            secondary: Color::from_hex("#89b4fa").unwrap(),
            on_secondary: Color::from_hex("#1e1e2e").unwrap(),
            tertiary: Color::from_hex("#a6e3a1").unwrap(),
            on_tertiary: Color::from_hex("#1e1e2e").unwrap(),
            error: Color::from_hex("#f38ba8").unwrap(),
            on_error: Color::from_hex("#1e1e2e").unwrap(),
            surface: Color::from_hex("#1e1e2e").unwrap(),
            on_surface: Color::from_hex("#cdd6f4").unwrap(),
            surface_variant: Color::from_hex("#313244").unwrap(),
            on_surface_variant: Color::from_hex("#a6adc8").unwrap(),
            outline: Color::from_hex("#45475a").unwrap(),
            shadow: Color::from_hex("#11111b").unwrap(),
            hover: Color::from_hex("#f5e0dc").unwrap(),
            on_hover: Color::from_hex("#1e1e2e").unwrap(),
        }
    }

    pub fn tokyo_night() -> Self {
        Self {
            name: "Tokyo Night".into(),
            is_dark: true,
            primary: Color::from_hex("#7aa2f7").unwrap(),
            on_primary: Color::from_hex("#1a1b26").unwrap(),
            secondary: Color::from_hex("#bb9af7").unwrap(),
            on_secondary: Color::from_hex("#1a1b26").unwrap(),
            tertiary: Color::from_hex("#7dcfff").unwrap(),
            on_tertiary: Color::from_hex("#1a1b26").unwrap(),
            error: Color::from_hex("#f7768e").unwrap(),
            on_error: Color::from_hex("#1a1b26").unwrap(),
            surface: Color::from_hex("#1a1b26").unwrap(),
            on_surface: Color::from_hex("#c0caf5").unwrap(),
            surface_variant: Color::from_hex("#24283b").unwrap(),
            on_surface_variant: Color::from_hex("#a9b1d6").unwrap(),
            outline: Color::from_hex("#414868").unwrap(),
            shadow: Color::from_hex("#10101a").unwrap(),
            hover: Color::from_hex("#b4f9f8").unwrap(),
            on_hover: Color::from_hex("#1a1b26").unwrap(),
        }
    }

    pub fn generate_css(&self) -> String {
        format!(
            ":root {{\n\
            --noctalia-primary: {primary};\n\
            --noctalia-on-primary: {on_primary};\n\
            --noctalia-secondary: {secondary};\n\
            --noctalia-on-secondary: {on_secondary};\n\
            --noctalia-tertiary: {tertiary};\n\
            --noctalia-on-tertiary: {on_tertiary};\n\
            --noctalia-error: {error};\n\
            --noctalia-on-error: {on_error};\n\
            --noctalia-surface: {surface};\n\
            --noctalia-on-surface: {on_surface};\n\
            --noctalia-surface-variant: {surface_variant};\n\
            --noctalia-on-surface-variant: {on_surface_variant};\n\
            --noctalia-outline: {outline};\n\
            --noctalia-shadow: {shadow};\n\
            --noctalia-hover: {hover};\n\
            --noctalia-on-hover: {on_hover};\n\
            --noctalia-radius-xs: 4px;\n\
            --noctalia-radius-md: 8px;\n\
            --noctalia-radius-xl: 12px;\n\
            --noctalia-radius-pill: 9999px;\n\
            }}",
            primary = self.primary.to_hex(),
            on_primary = self.on_primary.to_hex(),
            secondary = self.secondary.to_hex(),
            on_secondary = self.on_secondary.to_hex(),
            tertiary = self.tertiary.to_hex(),
            on_tertiary = self.on_tertiary.to_hex(),
            error = self.error.to_hex(),
            on_error = self.on_error.to_hex(),
            surface = self.surface.to_hex(),
            on_surface = self.on_surface.to_hex(),
            surface_variant = self.surface_variant.to_hex(),
            on_surface_variant = self.on_surface_variant.to_hex(),
            outline = self.outline.to_hex(),
            shadow = self.shadow.to_hex(),
            hover = self.hover.to_hex(),
            on_hover = self.on_hover.to_hex(),
        )
    }
}

impl Default for NoctaliaPalette {
    fn default() -> Self {
        Self::noctalia_dark()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_noctalia_palette_generation() {
        let dark = NoctaliaPalette::noctalia_dark();
        assert!(dark.is_dark);
        assert_eq!(dark.name, "Noctalia (Dark)");

        let light = NoctaliaPalette::noctalia_light();
        assert!(!light.is_dark);

        let css = dark.generate_css();
        assert!(css.contains("--noctalia-primary"));
        assert!(css.contains("--noctalia-surface"));
    }
}
