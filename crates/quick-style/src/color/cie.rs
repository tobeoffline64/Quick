//! CIE 1931 XYZ, CIELAB L*, and sRGB Color Conversions

pub const D65_X: f64 = 95.047;
pub const D65_Y: f64 = 100.0;
pub const D65_Z: f64 = 108.883;

pub const CIE_EPSILON: f64 = 216.0 / 24389.0;
pub const CIE_KAPPA: f64 = 24389.0 / 27.0;

/// Linearizes an sRGB component in range [0.0, 1.0].
#[inline]
pub fn linearize(c: f64) -> f64 {
    if c <= 0.04045 {
        c / 12.92
    } else {
        ((c + 0.055) / 1.055).powf(2.4)
    }
}

/// Delinearizes a linear sRGB component in range [0.0, 1.0].
#[inline]
pub fn delinearize(c_lin: f64) -> f64 {
    if c_lin <= 0.0031308 {
        12.92 * c_lin
    } else {
        1.055 * c_lin.powf(1.0 / 2.4) - 0.055
    }
}

/// Converts relative luminance Y in range [0.0, 100.0] to CIELAB L* (Tone) in range [0.0, 100.0].
#[inline]
pub fn lstar_from_y(y: f64) -> f64 {
    let y_norm = y / 100.0;
    if y_norm > CIE_EPSILON {
        116.0 * y_norm.cbrt() - 16.0
    } else {
        CIE_KAPPA * y_norm
    }
}

/// Converts CIELAB L* (Tone) in range [0.0, 100.0] to relative luminance Y in range [0.0, 100.0].
#[inline]
pub fn y_from_lstar(lstar: f64) -> f64 {
    if lstar > 8.0 {
        let p = (lstar + 16.0) / 116.0;
        100.0 * (p * p * p)
    } else {
        100.0 * (lstar / CIE_KAPPA)
    }
}

/// Converts sRGB 8-bit components [0..255] to CIE 1931 XYZ (D65, Y=100).
#[inline]
pub fn rgb_to_xyz(r: u8, g: u8, b: u8) -> [f64; 3] {
    let r_lin = linearize(r as f64 / 255.0);
    let g_lin = linearize(g as f64 / 255.0);
    let b_lin = linearize(b as f64 / 255.0);

    let x = (0.4124564 * r_lin + 0.3575761 * g_lin + 0.1804375 * b_lin) * 100.0;
    let y = (0.2126729 * r_lin + 0.7151522 * g_lin + 0.0721750 * b_lin) * 100.0;
    let z = (0.0193339 * r_lin + 0.1191920 * g_lin + 0.9503041 * b_lin) * 100.0;

    [x, y, z]
}

/// Converts CIE 1931 XYZ (D65, Y=100) to linear sRGB components [0.0, 1.0].
#[inline]
pub fn xyz_to_linear_rgb(x: f64, y: f64, z: f64) -> [f64; 3] {
    let r = (3.2404542 * x - 1.5371385 * y - 0.4985314 * z) / 100.0;
    let g = (-0.9692660 * x + 1.8760108 * y + 0.0415560 * z) / 100.0;
    let b = (0.0556434 * x - 0.2040259 * y + 1.0572252 * z) / 100.0;
    [r, g, b]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cie_constants_and_roundtrips() {
        assert!((linearize(0.0) - 0.0).abs() < 1e-6);
        assert!((linearize(1.0) - 1.0).abs() < 1e-6);
        assert!((delinearize(0.0) - 0.0).abs() < 1e-6);
        assert!((delinearize(1.0) - 1.0).abs() < 1e-6);

        // L* / Y roundtrip
        for tone in [0.0, 10.0, 25.0, 50.0, 75.0, 90.0, 100.0] {
            let y = y_from_lstar(tone);
            let reconstructed_tone = lstar_from_y(y);
            assert!((tone - reconstructed_tone).abs() < 1e-5, "Failed for tone {}", tone);
        }

        // D65 White point
        let [x, y, z] = rgb_to_xyz(255, 255, 255);
        assert!((x - D65_X).abs() < 0.01, "x = {}", x);
        assert!((y - D65_Y).abs() < 0.01, "y = {}", y);
        assert!((z - D65_Z).abs() < 0.01, "z = {}", z);
    }
}
