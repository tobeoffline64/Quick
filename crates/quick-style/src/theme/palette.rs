//! 6 Material You Tonal Palettes and Core Palette Definition.

use crate::color::hct::Hct;
use crate::theme::scheme::SchemeVariant;
use quick_core::geometry::Color;
use serde::{Deserialize, Serialize};

/// A 1-dimensional Tonal Palette in HCT color space sharing fixed Hue and Chroma across Tone 0..100.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct TonalPalette {
    pub hue: f64,
    pub chroma: f64,
}

impl TonalPalette {
    /// Creates a tonal palette from Hue [0..360) and Chroma (>= 0).
    pub fn from_hue_and_chroma(hue: f64, chroma: f64) -> Self {
        let normalized_hue = ((hue % 360.0) + 360.0) % 360.0;
        let clamped_chroma = chroma.max(0.0);
        Self {
            hue: normalized_hue,
            chroma: clamped_chroma,
        }
    }

    /// Creates a tonal palette from an existing HCT color.
    pub fn from_hct(hct: &Hct) -> Self {
        Self::from_hue_and_chroma(hct.hue, hct.chroma)
    }

    /// Creates a tonal palette from a standard sRGB Color.
    pub fn from_color(color: Color) -> Self {
        let hct = Hct::from_color(color);
        Self::from_hct(&hct)
    }

    /// Hue angle of this tonal palette in CAM16 degrees [0..360).
    pub fn hue(&self) -> f64 {
        self.hue
    }

    /// Chroma of this tonal palette in CAM16 colorfulness.
    pub fn chroma(&self) -> f64 {
        self.chroma
    }

    /// Sample an sRGB Color at the specified Tone (0.0 to 100.0).
    pub fn get(&self, tone: f64) -> Color {
        let clamped_tone = tone.clamp(0.0, 100.0);
        Hct::new(self.hue, self.chroma, clamped_tone).to_color()
    }

    /// Alias for `get`.
    pub fn get_tone(&self, tone: f64) -> Color {
        self.get(tone)
    }

    /// Sample an Hct color at the specified Tone (0.0 to 100.0).
    pub fn get_hct(&self, tone: f64) -> Hct {
        let clamped_tone = tone.clamp(0.0, 100.0);
        Hct::new(self.hue, self.chroma, clamped_tone)
    }
}

/// The 6 core Tonal Palettes defining a complete Material 3 dynamic theme.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CorePalette {
    pub primary: TonalPalette,
    pub secondary: TonalPalette,
    pub tertiary: TonalPalette,
    pub neutral: TonalPalette,
    pub neutral_variant: TonalPalette,
    pub error: TonalPalette,
}

impl CorePalette {
    pub fn of(
        primary: TonalPalette,
        secondary: TonalPalette,
        tertiary: TonalPalette,
        neutral: TonalPalette,
        neutral_variant: TonalPalette,
        error: TonalPalette,
    ) -> Self {
        Self {
            primary,
            secondary,
            tertiary,
            neutral,
            neutral_variant,
            error,
        }
    }

    /// Generate all 6 tonal palettes from a seed color and scheme variant.
    pub fn from_seed_color(seed: Color, variant: SchemeVariant) -> Self {
        variant.generate_palette(seed)
    }

    /// Generate all 6 tonal palettes from a seed hex string and scheme variant.
    pub fn from_seed_hex(hex: &str, variant: SchemeVariant) -> Result<Self, String> {
        let color = Color::from_hex(hex)?;
        Ok(Self::from_seed_color(color, variant))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tonal_palette_tone_sampling() {
        let palette = TonalPalette::from_hue_and_chroma(280.0, 48.0);
        let tone_0 = palette.get(0.0);
        assert_eq!(tone_0, Color::from_rgb(0, 0, 0));

        let tone_100 = palette.get(100.0);
        assert_eq!(tone_100, Color::from_rgb(255, 255, 255));

        let tone_50 = palette.get(50.0);
        let hct_50 = Hct::from_color(tone_50);
        assert!((hct_50.tone - 50.0).abs() < 2.0);
    }
}
