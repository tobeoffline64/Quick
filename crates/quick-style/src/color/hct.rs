//! Google Material You HCT (Hue, Chroma, Tone) Color Space.

use crate::color::cam16::Cam16;
use crate::color::cie::{lstar_from_y, rgb_to_xyz};
use crate::color::gamut::solve_gamut;
use quick_core::geometry::Color;
use serde::{Deserialize, Serialize};

/// Color in Google Material Design 3 HCT (Hue, Chroma, Tone) perceptual color space.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Hct {
    pub hue: f64,
    pub chroma: f64,
    pub tone: f64,
    argb: Color,
}

impl Hct {
    /// Creates a new Hct instance with given hue [0..360), chroma (>=0), and tone [0..100].
    pub fn new(hue: f64, chroma: f64, tone: f64) -> Self {
        let h = ((hue % 360.0) + 360.0) % 360.0;
        let c = chroma.max(0.0);
        let t = tone.clamp(0.0, 100.0);
        let argb = solve_gamut(h, c, t);
        Self {
            hue: h,
            chroma: c,
            tone: t,
            argb,
        }
    }

    /// Converts an sRGB Color into Hct.
    pub fn from_color(color: Color) -> Self {
        let [_, y, _] = rgb_to_xyz(color.r, color.g, color.b);
        let tone = lstar_from_y(y);
        let cam = Cam16::from_color(color);
        Self {
            hue: cam.hue,
            chroma: cam.chroma,
            tone,
            argb: color,
        }
    }

    /// Converts an ARGB u32 into Hct.
    pub fn from_argb_u32(argb: u32) -> Self {
        let a = ((argb >> 24) & 0xFF) as u8;
        let r = ((argb >> 16) & 0xFF) as u8;
        let g = ((argb >> 8) & 0xFF) as u8;
        let b = (argb & 0xFF) as u8;
        Self::from_color(Color::from_rgba(r, g, b, a))
    }

    pub fn hue(&self) -> f64 {
        self.hue
    }

    pub fn chroma(&self) -> f64 {
        self.chroma
    }

    pub fn tone(&self) -> f64 {
        self.tone
    }

    /// Returns the solved sRGB Color.
    pub fn to_color(&self) -> Color {
        self.argb
    }

    /// Returns the ARGB u32 integer representation.
    pub fn to_argb_u32(&self) -> u32 {
        self.argb.to_argb_u32()
    }

    pub fn with_hue(&self, hue: f64) -> Self {
        Self::new(hue, self.chroma, self.tone)
    }

    pub fn with_chroma(&self, chroma: f64) -> Self {
        Self::new(self.hue, chroma, self.tone)
    }

    pub fn with_tone(&self, tone: f64) -> Self {
        Self::new(self.hue, self.chroma, tone)
    }

    pub fn set_hue(&mut self, hue: f64) {
        *self = self.with_hue(hue);
    }

    pub fn set_chroma(&mut self, chroma: f64) {
        *self = self.with_chroma(chroma);
    }

    pub fn set_tone(&mut self, tone: f64) {
        *self = self.with_tone(tone);
    }
}

impl From<Color> for Hct {
    fn from(c: Color) -> Self {
        Self::from_color(c)
    }
}

impl From<Hct> for Color {
    fn from(h: Hct) -> Self {
        h.to_color()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hct_creation_and_primary_colors() {
        let red = Color::from_rgb(255, 0, 0);
        let hct_red = Hct::from_color(red);
        assert!((hct_red.tone - 53.2).abs() < 1.0, "Red tone: {}", hct_red.tone);
        assert!((hct_red.hue - 27.4).abs() < 2.0, "Red hue: {}", hct_red.hue);

        let purple = Color::from_hex("#6750A4").unwrap();
        let hct_purple = Hct::from_color(purple);
        assert!((hct_purple.tone - 40.08).abs() < 1.0, "Purple tone: {}", hct_purple.tone);
        assert!((hct_purple.hue - 297.78).abs() < 1.0, "Purple hue: {}", hct_purple.hue);

        let reconstructed = hct_purple.to_color();
        assert!((reconstructed.r as i32 - purple.r as i32).abs() <= 2);
        assert!((reconstructed.g as i32 - purple.g as i32).abs() <= 2);
        assert!((reconstructed.b as i32 - purple.b as i32).abs() <= 2);
    }
}
