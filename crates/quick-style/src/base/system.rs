//! OS-level theme (light/dark) and accent color detection.
//!
//! Uses environment variables and platform-specific heuristics to determine:
//! - `ColorScheme::Light` or `ColorScheme::Dark` from the OS preference
//! - The OS accent color (falls back to Fluent default #0078D4)
//!
//! Platform detection strategy:
//! - **Linux / freedesktop**: reads `gsettings get org.gnome.desktop.interface gtk-theme`
//!   and `color-scheme` for dark mode; accent from `QUICK_ACCENT_COLOR` env var
//!   or hardcoded Fluent default (freedesktop accent APIs are not yet standardized).
//! - **macOS** (future): `NSUserDefaults` via `defaults read -g AppleInterfaceStyle`
//! - **Windows** (future): registry `HKCU\Software\Microsoft\Windows\CurrentVersion\Themes\Personalize`
//! - **Env override**: `QUICK_DARK_MODE=1` forces dark; `QUICK_ACCENT_COLOR=#RRGGBB` forces accent.

use quick_core::geometry::Color;
use super::palette::{AccentColors, NeutralPalette};

/// Whether the OS is in light or dark mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ColorScheme {
    #[default]
    Light,
    Dark,
}

/// Detects the OS color scheme (light vs dark).
pub fn detect_color_scheme() -> ColorScheme {
    // 1. Explicit env override
    if std::env::var("QUICK_DARK_MODE").as_deref() == Ok("1") {
        return ColorScheme::Dark;
    }
    if std::env::var("QUICK_LIGHT_MODE").as_deref() == Ok("1") {
        return ColorScheme::Light;
    }

    // 2. XDG / freedesktop color-scheme (GNOME 42+, KDE Plasma 5.26+)
    if let Ok(scheme) = std::env::var("XDG_COLOR_SCHEME") {
        if scheme.contains("dark") {
            return ColorScheme::Dark;
        }
    }

    // 3. GTK_THEME env (common in Linux desktops)
    if let Ok(theme) = std::env::var("GTK_THEME") {
        if theme.to_lowercase().contains("dark") {
            return ColorScheme::Dark;
        }
    }

    // 4. Try gsettings (Linux GNOME) — non-blocking, ignore errors
    #[cfg(target_os = "linux")]
    {
        if let Ok(output) = std::process::Command::new("gsettings")
            .args(["get", "org.gnome.desktop.interface", "color-scheme"])
            .output()
        {
            let s = String::from_utf8_lossy(&output.stdout);
            if s.contains("dark") {
                return ColorScheme::Dark;
            }
        }
        // Fallback: check gtk-theme name
        if let Ok(output) = std::process::Command::new("gsettings")
            .args(["get", "org.gnome.desktop.interface", "gtk-theme"])
            .output()
        {
            let s = String::from_utf8_lossy(&output.stdout);
            if s.to_lowercase().contains("dark") {
                return ColorScheme::Dark;
            }
        }
    }

    // Default: light
    ColorScheme::Light
}

/// Detects the OS accent color.
/// Falls back to Avalonia Fluent default `#0078D4` if not detectable.
pub fn detect_accent_color() -> Color {
    // 1. Explicit env override: QUICK_ACCENT_COLOR=#RRGGBB
    if let Ok(hex) = std::env::var("QUICK_ACCENT_COLOR") {
        if let Ok(color) = Color::from_hex(&hex) {
            return color;
        }
    }

    // 2. KDE Plasma: kdeglobals accent color
    #[cfg(target_os = "linux")]
    {
        if let Some(color) = try_kde_accent() {
            return color;
        }
        if let Some(color) = try_gnome_accent() {
            return color;
        }
    }

    // 3. Default: Fluent blue
    NeutralPalette::ACCENT_DEFAULT
}

#[cfg(target_os = "linux")]
fn try_kde_accent() -> Option<Color> {
    // KDE stores accent in kdeglobals — try common paths
    let home = std::env::var("HOME").ok()?;
    let paths = [
        format!("{home}/.config/kdeglobals"),
        "/etc/xdg/kdeglobals".to_string(),
    ];
    for path in &paths {
        if let Ok(contents) = std::fs::read_to_string(path) {
            for line in contents.lines() {
                if line.starts_with("AccentColor=") {
                    let hex = line.trim_start_matches("AccentColor=").trim();
                    if let Ok(color) = Color::from_hex(hex) {
                        return Some(color);
                    }
                    // KDE also uses R,G,B format
                    let parts: Vec<&str> = hex.split(',').collect();
                    if parts.len() == 3 {
                        let r = parts[0].trim().parse::<u8>().ok()?;
                        let g = parts[1].trim().parse::<u8>().ok()?;
                        let b = parts[2].trim().parse::<u8>().ok()?;
                        return Some(Color::from_rgb(r, g, b));
                    }
                }
            }
        }
    }
    None
}

#[cfg(target_os = "linux")]
fn try_gnome_accent() -> Option<Color> {
    // GNOME 47+ supports accent-color via gsettings
    let output = std::process::Command::new("gsettings")
        .args(["get", "org.gnome.desktop.interface", "accent-color"])
        .output()
        .ok()?;
    let s = String::from_utf8_lossy(&output.stdout);
    let s = s.trim().trim_matches('\'');
    // GNOME accent names map to colors
    let color = match s {
        "blue"   => Color::from_rgb(  0, 120, 212),
        "teal"   => Color::from_rgb(  0, 160, 164),
        "green"  => Color::from_rgb( 35, 171, 100),
        "yellow" => Color::from_rgb(246, 198,   0),
        "orange" => Color::from_rgb(230, 100,   0),
        "red"    => Color::from_rgb(196,  43,  28),
        "pink"   => Color::from_rgb(220,  68, 128),
        "purple" => Color::from_rgb(119,  86, 178),
        "slate"  => Color::from_rgb(105, 119, 138),
        _        => return None,
    };
    Some(color)
}

/// Full system colors resolved from OS at app startup.
#[derive(Debug, Clone)]
pub struct SystemColors {
    pub scheme: ColorScheme,
    pub accent: AccentColors,
}

impl SystemColors {
    /// Detect from the running OS. Call once at app startup.
    pub fn detect() -> Self {
        let scheme = detect_color_scheme();
        let accent_base = detect_accent_color();
        Self {
            scheme,
            accent: AccentColors::from_color(accent_base),
        }
    }
}

impl Default for SystemColors {
    fn default() -> Self {
        Self {
            scheme: ColorScheme::Light,
            accent: AccentColors::default(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detect_color_scheme_returns_valid_variant() {
        let s = detect_color_scheme();
        assert!(matches!(s, ColorScheme::Light | ColorScheme::Dark));
    }

    #[test]
    fn test_detect_accent_returns_valid_color() {
        let c = detect_accent_color();
        // R+G+B should be nonzero (it's a real color, not transparent black)
        assert!(c.r as u32 + c.g as u32 + c.b as u32 > 0);
    }

    #[test]
    fn test_env_override_dark_mode() {
        std::env::set_var("QUICK_DARK_MODE", "1");
        assert_eq!(detect_color_scheme(), ColorScheme::Dark);
        std::env::remove_var("QUICK_DARK_MODE");
    }

    #[test]
    fn test_env_override_accent_color() {
        std::env::set_var("QUICK_ACCENT_COLOR", "#FF5500");
        let c = detect_accent_color();
        assert_eq!(c.r, 255);
        assert_eq!(c.g, 85);
        assert_eq!(c.b, 0);
        std::env::remove_var("QUICK_ACCENT_COLOR");
    }
}
