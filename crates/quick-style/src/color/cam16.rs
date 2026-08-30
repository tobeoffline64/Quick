//! CAM16 Color Appearance Model & Viewing Conditions under Standard D65 Environment.

use crate::color::cie::{y_from_lstar, D65_X, D65_Y, D65_Z};
use quick_core::geometry::Color;
use std::sync::OnceLock;

/// Standard viewing conditions for the CAM16 model.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ViewingConditions {
    pub white_point: [f64; 3],
    pub adapting_luminance: f64,
    pub background_lstar: f64,
    pub surround: f64,
    pub discounting_illuminant: bool,
    pub background_y: f64,
    pub n: f64,
    pub aw: f64,
    pub nbb: f64,
    pub ncb: f64,
    pub c: f64,
    pub nc: f64,
    pub dr: f64,
    pub dg: f64,
    pub db: f64,
    pub fl: f64,
    pub fl_root: f64,
    pub z: f64,
}

impl ViewingConditions {
    /// Creates a ViewingConditions struct dynamically from physical parameters.
    pub fn make(
        white_point: [f64; 3],
        adapting_luminance: f64,
        background_lstar: f64,
        surround: f64,
        discounting_illuminant: bool,
    ) -> Self {
        let background_y = y_from_lstar(background_lstar);
        let n = background_y / white_point[1];
        let z = 1.48 + n.sqrt();
        let nbb = 0.725 * n.powf(-0.2);
        let ncb = nbb;

        let c = if surround >= 1.0 {
            0.69
        } else if surround >= 0.0 {
            0.59 + 0.1 * surround
        } else {
            0.525
        };

        let nc = if surround >= 1.0 {
            1.0
        } else if surround >= 0.0 {
            0.9 + 0.1 * surround
        } else {
            0.8
        };

        let f = if surround >= 1.0 {
            0.8
        } else if surround >= 0.0 {
            0.8 + 0.1 * surround
        } else {
            0.9
        };

        let k = 1.0 / (5.0 * adapting_luminance + 1.0);
        let k4 = k * k * k * k;
        let k4_comp = 1.0 - k4;
        let k4_comp2 = k4_comp * k4_comp;
        let fl = 0.2 * k4 * (5.0 * adapting_luminance)
            + 0.1 * k4_comp2 * (5.0 * adapting_luminance).powf(1.0 / 3.0);
        let fl_root = fl.powf(0.25);

        let d = if discounting_illuminant {
            1.0
        } else {
            (f * (1.0 - (1.0 / 3.6) * ((-adapting_luminance - 42.0) / 92.0).exp())).clamp(0.0, 1.0)
        };

        // White point cone responses via CAT16 matrix
        let r_w = 0.401288 * white_point[0] + 0.650173 * white_point[1] - 0.051461 * white_point[2];
        let g_w = -0.250268 * white_point[0] + 1.204414 * white_point[1] + 0.045854 * white_point[2];
        let b_w = -0.002079 * white_point[0] + 0.048952 * white_point[1] + 0.953127 * white_point[2];

        let d_r = d * (white_point[1] / r_w) + 1.0 - d;
        let d_g = d * (white_point[1] / g_w) + 1.0 - d;
        let d_b = d * (white_point[1] / b_w) + 1.0 - d;

        let r_wc = d_r * r_w;
        let g_wc = d_g * g_w;
        let b_wc = d_b * b_w;

        let r_aw = (400.0 * (fl * r_wc / 100.0).powf(0.42)) / ((fl * r_wc / 100.0).powf(0.42) + 27.13);
        let g_aw = (400.0 * (fl * g_wc / 100.0).powf(0.42)) / ((fl * g_wc / 100.0).powf(0.42) + 27.13);
        let b_aw = (400.0 * (fl * b_wc / 100.0).powf(0.42)) / ((fl * b_wc / 100.0).powf(0.42) + 27.13);

        let aw = (2.0 * r_aw + g_aw + 0.05 * b_aw - 0.305) * nbb;

        Self {
            white_point,
            adapting_luminance,
            background_lstar,
            surround,
            discounting_illuminant,
            background_y,
            n,
            aw,
            nbb,
            ncb,
            c,
            nc,
            dr: d_r,
            dg: d_g,
            db: d_b,
            fl,
            fl_root,
            z,
        }
    }

    /// Standard D65 viewing conditions for sRGB displays.
    pub fn standard() -> &'static ViewingConditions {
        static STANDARD: OnceLock<ViewingConditions> = OnceLock::new();
        STANDARD.get_or_init(|| {
            let white_point = [D65_X, D65_Y, D65_Z];
            let adapting_luminance = (200.0 / std::f64::consts::PI) * (y_from_lstar(50.0) / 100.0);
            Self::make(white_point, adapting_luminance, 50.0, 1.0, false)
        })
    }
}

/// CAM16 Color Representation.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Cam16 {
    pub hue: f64,
    pub chroma: f64,
    pub j: f64,
    pub q: f64,
    pub m: f64,
    pub s: f64,
}

impl Cam16 {
    /// Creates a Cam16 instance directly from J (lightness), C (chroma), and H (hue in degrees).
    pub fn from_jch(j: f64, c: f64, h: f64) -> Self {
        Self::from_jch_in_viewing_conditions(j, c, h, ViewingConditions::standard())
    }

    pub fn from_jch_in_viewing_conditions(j: f64, c: f64, h: f64, vc: &ViewingConditions) -> Self {
        let j_clamped = j.clamp(0.0, 100.0);
        let c_clamped = c.max(0.0);
        let h_norm = ((h % 360.0) + 360.0) % 360.0;

        let q = (4.0 / vc.c) * (j_clamped / 100.0).sqrt() * (vc.aw + 4.0) * vc.fl_root;
        let m = c_clamped * vc.fl_root;
        let s = 100.0 * (m / q.max(1e-9)).sqrt();

        Self {
            hue: h_norm,
            chroma: c_clamped,
            j: j_clamped,
            q,
            m,
            s,
        }
    }

    /// Converts an sRGB Color to CAM16 under standard viewing conditions.
    pub fn from_color(color: Color) -> Self {
        let [x, y, z] = crate::color::cie::rgb_to_xyz(color.r, color.g, color.b);
        Self::from_xyz(x, y, z, ViewingConditions::standard())
    }

    /// Converts CIE XYZ to CAM16 under specified viewing conditions.
    pub fn from_xyz(x: f64, y: f64, z: f64, vc: &ViewingConditions) -> Self {
        // Step 1: CAT16 transform
        let r = 0.401288 * x + 0.650173 * y - 0.051461 * z;
        let g = -0.250268 * x + 1.204414 * y + 0.045854 * z;
        let b = -0.002079 * x + 0.048952 * y + 0.953127 * z;

        // Step 2: Chromatic adaptation
        let r_c = vc.dr * r;
        let g_c = vc.dg * g;
        let b_c = vc.db * b;

        // Step 3: Hunt-Pointer-Estevez non-linear compression
        let r_a = (vc.fl * r_c.abs() / 100.0).powf(0.42);
        let g_a = (vc.fl * g_c.abs() / 100.0).powf(0.42);
        let b_a = (vc.fl * b_c.abs() / 100.0).powf(0.42);

        let r_resp = r_c.signum() * (400.0 * r_a) / (r_a + 27.13);
        let g_resp = g_c.signum() * (400.0 * g_a) / (g_a + 27.13);
        let b_resp = b_c.signum() * (400.0 * b_a) / (b_a + 27.13);

        // Step 4: Opponent responses
        let a = r_resp - (12.0 / 11.0) * g_resp + (1.0 / 11.0) * b_resp;
        let b_opp = (r_resp + g_resp - 2.0 * b_resp) / 9.0;

        // Step 5: Hue angle
        let mut h = b_opp.atan2(a).to_degrees();
        if h < 0.0 {
            h += 360.0;
        }
        let h_rad = h.to_radians();

        // Step 6: Eccentricity factor
        let e_t = 0.25 * ((h_rad + 2.0).cos() + 3.8);

        // Step 7: Achromatic response & Lightness J
        let achromatic = (2.0 * r_resp + g_resp + 0.05 * b_resp - 0.305) * vc.nbb;
        let j = 100.0 * (achromatic / vc.aw).max(0.0).powf(vc.c * vc.z);

        // Step 8: Brightness Q
        let q = (4.0 / vc.c) * (j / 100.0).sqrt() * (vc.aw + 4.0) * vc.fl_root;

        // Step 9: Chroma C
        let t = ((50000.0 / 13.0) * vc.nc * vc.ncb * e_t * (a * a + b_opp * b_opp).sqrt())
            / (r_resp + g_resp + 1.05 * b_resp + 0.305);
        let alpha = t.powf(0.9) * (1.64 - 0.29f64.powf(vc.n)).powf(0.73);
        let c = alpha * (j / 100.0).sqrt();

        // Step 10: Colorfulness M & Saturation s
        let m = c * vc.fl_root;
        let s = 100.0 * (m / q.max(1e-9)).sqrt();

        Self {
            hue: h,
            chroma: c,
            j,
            q,
            m,
            s,
        }
    }

    /// Converts this CAM16 instance back to CIE XYZ [0..100].
    pub fn to_xyz(&self, vc: &ViewingConditions) -> [f64; 3] {
        if self.j <= 1e-9 {
            return [0.0, 0.0, 0.0];
        }

        let h_rad = self.hue.to_radians();
        let e_t = 0.25 * ((h_rad + 2.0).cos() + 3.8);
        let a_resp = vc.aw * (self.j / 100.0).powf(1.0 / (vc.c * vc.z));
        let p2 = a_resp / vc.nbb + 0.305;

        let (r_resp, g_resp, b_resp) = if self.chroma <= 1e-6 {
            let val = (460.0 / 1403.0) * p2;
            (val, val, val)
        } else {
            let alpha_inv = (self.chroma
                / ((self.j / 100.0).sqrt() * (1.64 - 0.29f64.powf(vc.n)).powf(0.73)))
            .powf(1.0 / 0.9);
            let p1 = (50000.0 / 13.0) * vc.nc * vc.ncb * e_t / alpha_inv;
            let r = (p2 + 0.305)
                / (p1 + (671.0 / 1403.0) * h_rad.cos() + (6588.0 / 1403.0) * h_rad.sin());
            let a = r * h_rad.cos();
            let b = r * h_rad.sin();

            let r_a = (460.0 / 1403.0) * p2 + (451.0 / 1403.0) * a + (288.0 / 1403.0) * b;
            let g_a = (460.0 / 1403.0) * p2 - (891.0 / 1403.0) * a - (261.0 / 1403.0) * b;
            let b_a = (460.0 / 1403.0) * p2 - (220.0 / 1403.0) * a - (6300.0 / 1403.0) * b;
            (r_a, g_a, b_a)
        };

        let invert_c = |ca: f64| -> f64 {
            let abs_ca = ca.abs().min(399.999);
            let temp = (27.13 * abs_ca) / (400.0 - abs_ca);
            ca.signum() * (100.0 / vc.fl) * temp.powf(1.0 / 0.42)
        };

        let r_c = invert_c(r_resp);
        let g_c = invert_c(g_resp);
        let b_c = invert_c(b_resp);

        let r = r_c / vc.dr;
        let g = g_c / vc.dg;
        let b = b_c / vc.db;

        // Inverse CAT16 matrix
        let x = 1.86206786 * r - 1.01125463 * g + 0.14918677 * b;
        let y = 0.38752654 * r + 0.62144744 * g - 0.00897398 * b;
        let z = -0.01584120 * r - 0.03412294 * g + 1.04996414 * b;

        [x, y, z]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cam16_forward_and_inverse() {
        let red = Color::from_rgb(255, 0, 0);
        let cam_red = Cam16::from_color(red);
        assert!((cam_red.hue - 27.4).abs() < 2.0, "Red hue: {}", cam_red.hue);
        assert!(cam_red.chroma > 100.0, "Red chroma: {}", cam_red.chroma);

        let [x, y, z] = cam_red.to_xyz(ViewingConditions::standard());
        let [r_lin, g_lin, b_lin] = crate::color::cie::xyz_to_linear_rgb(x, y, z);
        let r = crate::color::cie::delinearize(r_lin);
        let g = crate::color::cie::delinearize(g_lin);
        let b = crate::color::cie::delinearize(b_lin);

        assert!((r * 255.0 - 255.0).abs() < 2.0, "r = {}", r * 255.0);
        assert!((g * 255.0 - 0.0).abs() < 2.0, "g = {}", g * 255.0);
        assert!((b * 255.0 - 0.0).abs() < 2.0, "b = {}", b * 255.0);
    }
}
