//! 7 Material Design 3 Scheme Variants and DynamicScheme Resolver.

use crate::color::hct::Hct;
use crate::theme::color_scheme::ColorScheme;
use crate::theme::palette::{CorePalette, TonalPalette};
use quick_core::geometry::Color;
use serde::{Deserialize, Serialize};
use std::fmt;
use std::str::FromStr;

/// The 7 official Google Material You (M3) Scheme Variants.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum SchemeVariant {
    #[default]
    TonalSpot,
    Vibrant,
    Expressive,
    Fidelity,
    Content,
    Monochrome,
    Neutral,
}

impl fmt::Display for SchemeVariant {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::TonalSpot => write!(f, "tonal_spot"),
            Self::Vibrant => write!(f, "vibrant"),
            Self::Expressive => write!(f, "expressive"),
            Self::Fidelity => write!(f, "fidelity"),
            Self::Content => write!(f, "content"),
            Self::Monochrome => write!(f, "monochrome"),
            Self::Neutral => write!(f, "neutral"),
        }
    }
}

impl FromStr for SchemeVariant {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let normalized = s.trim().to_lowercase().replace('-', "_");
        match normalized.as_str() {
            "tonalspot" | "tonal_spot" | "default" => Ok(Self::TonalSpot),
            "vibrant" => Ok(Self::Vibrant),
            "expressive" => Ok(Self::Expressive),
            "fidelity" => Ok(Self::Fidelity),
            "content" => Ok(Self::Content),
            "monochrome" | "mono" | "grayscale" => Ok(Self::Monochrome),
            "neutral" => Ok(Self::Neutral),
            _ => Err(format!("Unknown scheme variant: '{}'", s)),
        }
    }
}

impl SchemeVariant {
    /// Derive the 6 core tonal palettes from a seed color according to this variant's rules.
    pub fn generate_palette(&self, seed: Color) -> CorePalette {
        let hct = Hct::from_color(seed);
        let h = hct.hue;
        let c = hct.chroma;

        // Error palette is constant across all variants: Hue 25.0, Chroma 84.0
        let error = TonalPalette::from_hue_and_chroma(25.0, 84.0);

        let (primary, secondary, tertiary, neutral, neutral_variant) = match self {
            Self::TonalSpot => (
                TonalPalette::from_hue_and_chroma(h, c.max(48.0)),
                TonalPalette::from_hue_and_chroma(h, 16.0),
                TonalPalette::from_hue_and_chroma(h + 60.0, 24.0),
                TonalPalette::from_hue_and_chroma(h, 6.0),
                TonalPalette::from_hue_and_chroma(h, 8.0),
            ),
            Self::Vibrant => (
                TonalPalette::from_hue_and_chroma(h, c.max(74.0)),
                TonalPalette::from_hue_and_chroma(h + 24.0, 24.0),
                TonalPalette::from_hue_and_chroma(h + 48.0, 32.0),
                TonalPalette::from_hue_and_chroma(h, 10.0),
                TonalPalette::from_hue_and_chroma(h, 12.0),
            ),
            Self::Expressive => (
                TonalPalette::from_hue_and_chroma(h + 240.0, 40.0),
                TonalPalette::from_hue_and_chroma(h + 15.0, 24.0),
                TonalPalette::from_hue_and_chroma(h + 120.0, 32.0),
                TonalPalette::from_hue_and_chroma(h + 15.0, 8.0),
                TonalPalette::from_hue_and_chroma(h + 15.0, 12.0),
            ),
            Self::Fidelity => (
                TonalPalette::from_hue_and_chroma(h, c),
                TonalPalette::from_hue_and_chroma(h, (c - 32.0).max(c * 0.5)),
                TonalPalette::from_hue_and_chroma(h + 60.0, (c - 16.0).max(24.0)),
                TonalPalette::from_hue_and_chroma(h, (c / 8.0).min(4.0)),
                TonalPalette::from_hue_and_chroma(h, c / 8.0 + 4.0),
            ),
            Self::Content => (
                TonalPalette::from_hue_and_chroma(h, c),
                TonalPalette::from_hue_and_chroma(h, (c - 32.0).max(c * 0.4)),
                TonalPalette::from_hue_and_chroma(h + 60.0, (c - 16.0).max(24.0)),
                TonalPalette::from_hue_and_chroma(h, (c / 8.0).min(4.0)),
                TonalPalette::from_hue_and_chroma(h, c / 8.0 + 4.0),
            ),
            Self::Monochrome => (
                TonalPalette::from_hue_and_chroma(h, 0.0),
                TonalPalette::from_hue_and_chroma(h, 0.0),
                TonalPalette::from_hue_and_chroma(h, 0.0),
                TonalPalette::from_hue_and_chroma(h, 0.0),
                TonalPalette::from_hue_and_chroma(h, 0.0),
            ),
            Self::Neutral => (
                TonalPalette::from_hue_and_chroma(h, 12.0),
                TonalPalette::from_hue_and_chroma(h, 8.0),
                TonalPalette::from_hue_and_chroma(h, 16.0),
                TonalPalette::from_hue_and_chroma(h, 2.0),
                TonalPalette::from_hue_and_chroma(h, 2.0),
            ),
        };

        CorePalette {
            primary,
            secondary,
            tertiary,
            neutral,
            neutral_variant,
            error,
        }
    }
}

/// Dynamic Scheme bundling seed color, variant, dark mode state, and contrast level.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DynamicScheme {
    pub seed: Color,
    pub variant: SchemeVariant,
    pub is_dark: bool,
    pub contrast: f64,
    pub core_palette: CorePalette,
}

impl DynamicScheme {
    pub fn new(seed: Color, variant: SchemeVariant, is_dark: bool, contrast: f64) -> Self {
        let core_palette = variant.generate_palette(seed);
        Self {
            seed,
            variant,
            is_dark,
            contrast,
            core_palette,
        }
    }

    /// Converts this dynamic scheme directly into a ColorScheme.
    pub fn to_color_scheme(&self) -> ColorScheme {
        ColorScheme::from_core_palette_with_contrast(&self.core_palette, self.is_dark, self.contrast)
    }
}
