use quick_core::geometry::{BorderRadius, Color};
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct ThemePackage {
    pub name: String,
    pub colors: HashMap<String, Color>,
    pub shapes: HashMap<String, f32>,
}

impl ThemePackage {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            colors: HashMap::new(),
            shapes: HashMap::new(),
        }
    }

    /// Built-in Google Material You (M3) Dark Theme Package
    pub fn material_you() -> Self {
        let mut theme = Self::new("material-you");

        // Material You Dynamic Tonal Palettes
        theme.colors.insert("primary".into(), Color::from_hex("#D0BCFF").unwrap());
        theme.colors.insert("on_primary".into(), Color::from_hex("#381E72").unwrap());
        theme.colors.insert("primary_container".into(), Color::from_hex("#4F378B").unwrap());
        theme.colors.insert("on_primary_container".into(), Color::from_hex("#EADDFF").unwrap());

        theme.colors.insert("surface".into(), Color::from_hex("#141218").unwrap());
        theme.colors.insert("on_surface".into(), Color::from_hex("#E6E0E9").unwrap());
        theme.colors.insert("surface_container".into(), Color::from_hex("#211F26").unwrap());
        theme.colors.insert("surface_container_high".into(), Color::from_hex("#2B2930").unwrap());

        theme.colors.insert("outline".into(), Color::from_hex("#938F99").unwrap());
        theme.colors.insert("outline_variant".into(), Color::from_hex("#49454F").unwrap());
        theme.colors.insert("error".into(), Color::from_hex("#F2B8B5").unwrap());

        // Shapes
        theme.shapes.insert("corner_small".into(), 8.0);
        theme.shapes.insert("corner_medium".into(), 16.0);
        theme.shapes.insert("corner_large".into(), 24.0);
        theme.shapes.insert("corner_full".into(), 999.0);

        theme
    }

    /// Built-in Nord Arctic Palette
    pub fn nord() -> Self {
        let mut theme = Self::new("nord");

        theme.colors.insert("primary".into(), Color::from_hex("#88C0D0").unwrap());
        theme.colors.insert("on_primary".into(), Color::from_hex("#2E3440").unwrap());
        theme.colors.insert("surface".into(), Color::from_hex("#2E3440").unwrap());
        theme.colors.insert("on_surface".into(), Color::from_hex("#ECEFF4").unwrap());
        theme.colors.insert("surface_container".into(), Color::from_hex("#3B4252").unwrap());
        theme.colors.insert("outline".into(), Color::from_hex("#4C566A").unwrap());

        theme.shapes.insert("corner_small".into(), 6.0);
        theme.shapes.insert("corner_medium".into(), 12.0);
        theme.shapes.insert("corner_large".into(), 16.0);
        theme.shapes.insert("corner_full".into(), 999.0);

        theme
    }

    /// Generates CSS stylesheet rules matching this theme package
    pub fn generate_css(&self) -> String {
        let mut css = String::new();

        if let (Some(primary), Some(on_primary)) = (self.colors.get("primary"), self.colors.get("on_primary")) {
            css.push_str(&format!(
                "Button.btn-primary, Button[variant=\"filled\"] {{ background: {}; color: {}; border-radius: 999px; font-weight: bold; }}\n",
                primary.to_hex(), on_primary.to_hex()
            ));
        }

        if let (Some(surf_c), Some(outline)) = (self.colors.get("surface_container"), self.colors.get("outline")) {
            css.push_str(&format!(
                "Card, Card[variant=\"elevated\"] {{ background: {}; border-radius: 16px; border-color: {}; border-width: 1px; }}\n",
                surf_c.to_hex(), outline.to_hex()
            ));
        }

        if let Some(on_surface) = self.colors.get("on_surface") {
            css.push_str(&format!(
                "Text {{ color: {}; }}\n",
                on_surface.to_hex()
            ));
        }

        css
    }
}
