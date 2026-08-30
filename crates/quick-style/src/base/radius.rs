//! Corner radius scale matching Avalonia Fluent's CornerRadius tokens.

use quick_core::geometry::BorderRadius;

pub struct RadiusScale;

impl RadiusScale {
    pub const NONE: f32 =    0.0;
    pub const XS:   f32 =    2.0; // tooltip, badge
    pub const SM:   f32 =    4.0; // text inputs, snackbars (Avalonia TextBox)
    pub const MD:   f32 =    8.0; // cards, dialogs
    pub const LG:   f32 =   12.0; // large cards
    pub const XL:   f32 =   16.0; // bottom sheets, large dialogs
    pub const PILL: f32 = 9999.0; // buttons, chips, badges

    pub fn border(radius: f32) -> BorderRadius {
        BorderRadius::all(radius)
    }

    pub fn top(radius: f32) -> BorderRadius {
        BorderRadius { top_left: radius, top_right: radius, bottom_left: 0.0, bottom_right: 0.0 }
    }

    pub fn bottom(radius: f32) -> BorderRadius {
        BorderRadius { top_left: 0.0, top_right: 0.0, bottom_left: radius, bottom_right: radius }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_radii_ascending() {
        assert!(RadiusScale::NONE < RadiusScale::XS);
        assert!(RadiusScale::XS  < RadiusScale::SM);
        assert!(RadiusScale::SM  < RadiusScale::MD);
        assert!(RadiusScale::MD  < RadiusScale::LG);
        assert!(RadiusScale::LG  < RadiusScale::XL);
        assert!(RadiusScale::XL  < RadiusScale::PILL);
    }

    #[test]
    fn test_border_radius_all() {
        let br = RadiusScale::border(RadiusScale::MD);
        assert_eq!(br.top_left, 8.0);
        assert_eq!(br.bottom_right, 8.0);
    }
}
