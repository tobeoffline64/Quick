//! Noctalia Glassmorphism & UI Geometry Tokens

#[derive(Debug, Clone, Copy)]
pub struct NoctaliaGlassTokens {
    pub window_opacity: f32,
    pub card_acrylic_opacity: f32,
    pub blur_radius: f32,
    pub border_width: f32,
    pub shadow_spread: f32,
}

impl Default for NoctaliaGlassTokens {
    fn default() -> Self {
        Self {
            window_opacity: 0.92,
            card_acrylic_opacity: 0.75,
            blur_radius: 20.0,
            border_width: 1.0,
            shadow_spread: 2.0,
        }
    }
}

pub struct NoctaliaRadius;

impl NoctaliaRadius {
    pub const NONE: f32 = 0.0;
    pub const XS: f32   = 4.0;
    pub const SM: f32   = 6.0;
    pub const MD: f32   = 8.0;
    pub const LG: f32   = 10.0;
    pub const XL: f32   = 12.0;
    pub const PILL: f32 = 9999.0;
}
