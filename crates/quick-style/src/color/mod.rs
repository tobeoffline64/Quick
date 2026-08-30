//! Pure Rust Material You HCT & CAM16 Color Engine.

pub mod cam16;
pub mod cie;
pub mod contrast;
pub mod gamut;
pub mod hct;

pub use cam16::{Cam16, ViewingConditions};
pub use cie::{
    delinearize, linearize, lstar_from_y, rgb_to_xyz, xyz_to_linear_rgb, y_from_lstar,
    CIE_EPSILON, CIE_KAPPA, D65_X, D65_Y, D65_Z,
};
pub use contrast::{
    contrast_ratio, contrast_ratio_tones, darker_tone, is_accessible, lighter_tone,
    relative_luminance,
};
pub use gamut::{grayscale_from_y, solve_gamut, test_gamut_point};
pub use hct::Hct;
