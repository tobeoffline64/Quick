//! Typography / type scale matching Avalonia Fluent's font size steps.

pub struct TypeScale;

impl TypeScale {
    pub const CAPTION:      f32 = 11.0; // metadata, timestamps
    pub const BODY:         f32 = 14.0; // default body text
    pub const BODY_LARGE:   f32 = 16.0; // prominent body / list items
    pub const TITLE:        f32 = 18.0; // section headers
    pub const TITLE_LARGE:  f32 = 22.0; // dialog titles
    pub const DISPLAY:      f32 = 28.0; // page headlines
    pub const DISPLAY_LARGE:f32 = 34.0; // hero text

    /// Default button label size
    pub const BUTTON: f32  = Self::BODY;
    /// Default input placeholder/value size
    pub const INPUT:  f32  = Self::BODY;
    /// Default chip label size
    pub const CHIP:   f32  = 13.0;
}

/// Named font weight constants (CSS numeric weight values).
pub struct FontWeight;

impl FontWeight {
    pub const REGULAR: u16  = 400;
    pub const MEDIUM:  u16  = 500;
    pub const SEMIBOLD:u16  = 600;
    pub const BOLD:    u16  = 700;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_typescale_ascending() {
        assert!(TypeScale::CAPTION < TypeScale::BODY);
        assert!(TypeScale::BODY    < TypeScale::BODY_LARGE);
        assert!(TypeScale::BODY_LARGE < TypeScale::TITLE);
        assert!(TypeScale::TITLE   < TypeScale::DISPLAY);
    }
}
