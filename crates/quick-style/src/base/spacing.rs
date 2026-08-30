//! 4px-grid spacing scale, matching Avalonia Fluent's spacing rhythm.

/// Spacing constants on a 4px base grid.
pub struct SpacingScale;

impl SpacingScale {
    pub const XS: f32    =  4.0;
    pub const SM: f32    =  8.0;
    pub const MD: f32    = 12.0;
    pub const LG: f32    = 16.0;
    pub const XL: f32    = 20.0;
    pub const XXL: f32   = 24.0;
    pub const XXXL: f32  = 32.0;
    pub const XXXXL: f32 = 48.0;

    /// Returns the closest grid-aligned value for any input.
    pub fn snap(value: f32) -> f32 {
        (value / 4.0).round() * 4.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_spacing_is_4px_grid() {
        for s in [SpacingScale::XS, SpacingScale::SM, SpacingScale::MD,
                  SpacingScale::LG, SpacingScale::XL, SpacingScale::XXL, SpacingScale::XXXL] {
            assert_eq!(s % 4.0, 0.0, "{s} is not 4px-aligned");
        }
    }

    #[test]
    fn test_snap() {
        assert_eq!(SpacingScale::snap(5.0), 4.0);
        assert_eq!(SpacingScale::snap(6.5), 8.0);
        assert_eq!(SpacingScale::snap(16.0), 16.0);
    }
}
