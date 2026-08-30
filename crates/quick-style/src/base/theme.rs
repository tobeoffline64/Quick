//! `BaseTheme` — Avalonia Fluent-inspired default theme for Quick widgets.
//! Provides light/dark presets and OS-adaptive theme detection.

use super::palette::{AccentColors, NeutralPalette};
use super::radius::RadiusScale;
use super::spacing::SpacingScale;
use super::system::{ColorScheme, SystemColors};
use super::typography::{FontStack, TypeScale};
use quick_core::geometry::Color;

/// Resolved set of semantic colors for a single color scheme (light OR dark).
#[derive(Debug, Clone)]
pub struct BaseColors {
    // Surfaces
    pub bg:               Color, // window/app background
    pub surface:          Color, // card, panel background
    pub surface_raised:   Color, // elevated surface (dropdown, popup)
    pub overlay:          Color, // modal scrim

    // Borders
    pub border:           Color, // default input/card border
    pub border_strong:    Color, // focused / hovered border
    pub divider:          Color, // subtle list dividers

    // Text
    pub text_primary:     Color,
    pub text_secondary:   Color,
    pub text_disabled:    Color,
    pub text_placeholder: Color,
    pub text_on_accent:   Color,

    // Accent (OS-resolved)
    pub accent:           AccentColors,

    // State
    pub hover_overlay:    Color, // hover fill on interactive items
    pub press_overlay:    Color, // pressed fill
    pub focus_ring:       Color, // keyboard focus indicator

    // Semantic
    pub error:            Color,
    pub success:          Color,
    pub warning:          Color,
}

impl BaseColors {
    fn light(accent: AccentColors) -> Self {
        let hover_overlay = Color::from_rgba(0, 0, 0, 13);   // 5% black
        let press_overlay = Color::from_rgba(0, 0, 0, 31);   // 12% black
        Self {
            bg:               NeutralPalette::N0,
            surface:          NeutralPalette::N10,
            surface_raised:   Color::WHITE,
            overlay:          Color::from_rgba(0, 0, 0, 77),
            border:           NeutralPalette::N30,
            border_strong:    NeutralPalette::N50,
            divider:          NeutralPalette::N20,
            text_primary:     NeutralPalette::N90,
            text_secondary:   NeutralPalette::N60,
            text_disabled:    NeutralPalette::N40,
            text_placeholder: NeutralPalette::N50,
            text_on_accent:   accent.on_accent,
            accent,
            hover_overlay,
            press_overlay,
            focus_ring:       NeutralPalette::ACCENT_DEFAULT,
            error:            NeutralPalette::ERROR,
            success:          NeutralPalette::SUCCESS,
            warning:          NeutralPalette::WARNING,
        }
    }

    fn dark(accent: AccentColors) -> Self {
        let hover_overlay = Color::from_rgba(255, 255, 255, 13); // 5% white
        let press_overlay = Color::from_rgba(255, 255, 255, 31); // 12% white
        Self {
            bg:               NeutralPalette::DARK_SURFACE,
            surface:          NeutralPalette::DARK_SURFACE_RAISED,
            surface_raised:   NeutralPalette::DARK_SURFACE_HIGH,
            overlay:          Color::from_rgba(0, 0, 0, 128),
            border:           NeutralPalette::DARK_BORDER,
            border_strong:    NeutralPalette::N50,
            divider:          NeutralPalette::DARK_BORDER,
            text_primary:     NeutralPalette::DARK_TEXT_PRIMARY,
            text_secondary:   NeutralPalette::DARK_TEXT_SECONDARY,
            text_disabled:    NeutralPalette::N60,
            text_placeholder: NeutralPalette::N60,
            text_on_accent:   accent.on_accent,
            accent,
            hover_overlay,
            press_overlay,
            focus_ring:       NeutralPalette::ACCENT_DEFAULT,
            error:            Color::from_rgb(255, 153, 144),
            success:          Color::from_rgb(102, 204, 153),
            warning:          Color::from_rgb(255, 214, 102),
        }
    }
}

/// The Avalonia Fluent-inspired base theme for Quick.
/// Created once at app startup from OS detection; passed to widget constructors.
#[derive(Debug, Clone)]
pub struct BaseTheme {
    pub scheme: ColorScheme,
    pub colors: BaseColors,
    pub radius: RadiusScaleRef,
    pub spacing: SpacingScaleRef,
    pub type_scale: TypeScaleRef,
    /// Resolved font stack for this platform (Inter on Linux/Win, -apple-system on macOS).
    pub font_stack: FontStack,
}

/// Snapshot of the radius scale (for ergonomic access).
#[derive(Debug, Clone, Copy)]
pub struct RadiusScaleRef {
    pub none: f32, pub xs: f32, pub sm: f32, pub md: f32,
    pub lg: f32,  pub xl: f32, pub pill: f32,
}

impl Default for RadiusScaleRef {
    fn default() -> Self {
        Self {
            none: RadiusScale::NONE, xs: RadiusScale::XS, sm: RadiusScale::SM,
            md: RadiusScale::MD,    lg: RadiusScale::LG,  xl: RadiusScale::XL,
            pill: RadiusScale::PILL,
        }
    }
}

/// Snapshot of the spacing scale.
#[derive(Debug, Clone, Copy)]
pub struct SpacingScaleRef {
    pub xs: f32, pub sm: f32, pub md: f32, pub lg: f32,
    pub xl: f32, pub xxl: f32, pub xxxl: f32,
}

impl Default for SpacingScaleRef {
    fn default() -> Self {
        Self {
            xs: SpacingScale::XS, sm: SpacingScale::SM, md: SpacingScale::MD,
            lg: SpacingScale::LG, xl: SpacingScale::XL, xxl: SpacingScale::XXL,
            xxxl: SpacingScale::XXXL,
        }
    }
}

/// Snapshot of the type scale.
#[derive(Debug, Clone, Copy)]
pub struct TypeScaleRef {
    pub caption: f32, pub body: f32, pub body_large: f32,
    pub title: f32,   pub title_large: f32, pub display: f32,
    pub button: f32,  pub input: f32, pub chip: f32,
}

impl Default for TypeScaleRef {
    fn default() -> Self {
        Self {
            caption: TypeScale::CAPTION, body: TypeScale::BODY,
            body_large: TypeScale::BODY_LARGE, title: TypeScale::TITLE,
            title_large: TypeScale::TITLE_LARGE, display: TypeScale::DISPLAY,
            button: TypeScale::BUTTON, input: TypeScale::INPUT, chip: TypeScale::CHIP,
        }
    }
}

impl BaseTheme {
    /// Detect from OS at startup. Light by default, dark if OS says so.
    pub fn from_system() -> Self {
        let sys = SystemColors::detect();
        Self::from_system_colors(sys)
    }

    pub fn from_system_colors(sys: SystemColors) -> Self {
        let colors = match sys.scheme {
            ColorScheme::Light => BaseColors::light(sys.accent),
            ColorScheme::Dark  => BaseColors::dark(sys.accent),
        };
        Self {
            scheme: sys.scheme,
            colors,
            radius:     RadiusScaleRef::default(),
            spacing:    SpacingScaleRef::default(),
            type_scale: TypeScaleRef::default(),
            font_stack: FontStack::for_current_platform(),
        }
    }

    /// Explicit light preset with Fluent default blue accent.
    pub fn fluent() -> Self {
        Self::from_system_colors(SystemColors::default())
    }

    /// Explicit dark preset with Fluent default blue accent.
    pub fn fluent_dark() -> Self {
        let sys = SystemColors { scheme: ColorScheme::Dark, accent: AccentColors::default() };
        Self::from_system_colors(sys)
    }

    pub fn is_dark(&self) -> bool {
        self.scheme == ColorScheme::Dark
    }

    /// Generate CSS custom properties for injection into a stylesheet.
    pub fn generate_css(&self) -> String {
        let c = &self.colors;
        let r = &self.radius;
        let s = &self.spacing;
        let t = &self.type_scale;
        let font_css = self.font_stack.to_css_value();
        format!(
            ":root {{\n\
            /* Base Theme: {scheme} */\n\
            --q-font-family: {font};\n\
            --q-bg: {bg};\n\
            --q-surface: {surf};\n\
            --q-border: {border};\n\
            --q-text: {text};\n\
            --q-text-secondary: {text2};\n\
            --q-text-disabled: {textd};\n\
            --q-accent: {acc};\n\
            --q-accent-hover: {acc_h};\n\
            --q-accent-pressed: {acc_p};\n\
            --q-error: {err};\n\
            --q-radius-none: {r_none}px;\n\
            --q-radius-xs: {r_xs}px;\n\
            --q-radius-sm: {r_sm}px;\n\
            --q-radius-md: {r_md}px;\n\
            --q-radius-lg: {r_lg}px;\n\
            --q-radius-pill: {r_pill}px;\n\
            --q-space-xs: {sp_xs}px;\n\
            --q-space-sm: {sp_sm}px;\n\
            --q-space-md: {sp_md}px;\n\
            --q-space-lg: {sp_lg}px;\n\
            --q-space-xl: {sp_xl}px;\n\
            --q-space-xxl: {sp_xxl}px;\n\
            --q-font-caption: {f_cap}px;\n\
            --q-font-body: {f_body}px;\n\
            --q-font-title: {f_title}px;\n\
            --q-font-display: {f_disp}px;\n\
            }}",
            scheme = if self.is_dark() { "dark" } else { "light" },
            font   = font_css,
            bg     = c.bg.to_hex(),
            surf   = c.surface.to_hex(),
            border = c.border.to_hex(),
            text   = c.text_primary.to_hex(),
            text2  = c.text_secondary.to_hex(),
            textd  = c.text_disabled.to_hex(),
            acc    = c.accent.normal.to_hex(),
            acc_h  = c.accent.hover.to_hex(),
            acc_p  = c.accent.pressed.to_hex(),
            err    = c.error.to_hex(),
            r_none = r.none, r_xs = r.xs, r_sm = r.sm,
            r_md   = r.md,   r_lg = r.lg, r_pill = r.pill,
            sp_xs  = s.xs,  sp_sm = s.sm, sp_md = s.md,
            sp_lg  = s.lg,  sp_xl = s.xl, sp_xxl = s.xxl,
            f_cap  = t.caption, f_body = t.body,
            f_title = t.title,  f_disp = t.display,
        )
    }
}

impl Default for BaseTheme {
    fn default() -> Self { Self::fluent() }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fluent_light_is_light() {
        let t = BaseTheme::fluent();
        assert_eq!(t.scheme, ColorScheme::Light);
        // bg should be nearly white
        assert!(t.colors.bg.r > 200 && t.colors.bg.g > 200 && t.colors.bg.b > 200);
    }

    #[test]
    fn test_fluent_dark_is_dark() {
        let t = BaseTheme::fluent_dark();
        assert_eq!(t.scheme, ColorScheme::Dark);
        // bg should be dark
        assert!(t.colors.bg.r < 60 && t.colors.bg.g < 60 && t.colors.bg.b < 60);
    }

    #[test]
    fn test_generate_css_contains_key_vars() {
        let css = BaseTheme::fluent().generate_css();
        assert!(css.contains("--q-bg:"));
        assert!(css.contains("--q-accent:"));
        assert!(css.contains("--q-radius-md:"));
        assert!(css.contains("--q-space-lg:"));
        assert!(css.contains("--q-font-body:"));
    }

    #[test]
    fn test_dark_theme_generate_css() {
        let css = BaseTheme::fluent_dark().generate_css();
        assert!(css.contains("/* Base Theme: dark */"));
    }

    #[test]
    fn test_from_system_returns_valid_theme() {
        let t = BaseTheme::from_system();
        assert!(matches!(t.scheme, ColorScheme::Light | ColorScheme::Dark));
    }
}
