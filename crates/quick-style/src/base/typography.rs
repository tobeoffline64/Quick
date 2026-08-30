//! Typography / type scale matching Avalonia Fluent's font size steps.

pub struct TypeScale;

impl TypeScale {
    pub const CAPTION:      f32 = 10.0; // metadata, timestamps
    pub const BODY:         f32 = 12.0; // standard body text (standard 12px)
    pub const BODY_LARGE:   f32 = 14.0; // prominent body / list items
    pub const TITLE:        f32 = 16.0; // section headers
    pub const TITLE_LARGE:  f32 = 18.0; // dialog titles
    pub const DISPLAY:      f32 = 22.0; // page headlines
    pub const DISPLAY_LARGE:f32 = 26.0; // hero text

    /// Default button label size
    pub const BUTTON: f32  = Self::BODY;
    /// Default input placeholder/value size
    pub const INPUT:  f32  = Self::BODY;
    /// Default chip label size
    pub const CHIP:   f32  = 11.0;

    /// Reactive scale helper for dynamically adjusting typography with UI zoom / scaling.
    pub fn scale(base_size: f32, scale_factor: f32) -> f32 {
        (base_size * scale_factor).max(6.0)
    }
}

/// Named font weight constants (CSS numeric weight values).
pub struct FontWeight;

impl FontWeight {
    pub const REGULAR: u16  = 400;
    pub const MEDIUM:  u16  = 500;
    pub const SEMIBOLD:u16  = 600;
    pub const BOLD:    u16  = 700;
}

// ─── Font stack ─────────────────────────────────────────────────────────────

/// Priority-ordered list of font families for the Quick UI framework.
///
/// Resolution order:
/// 1. `QUICK_FONT_FAMILY` env var (user override, comma-separated)
/// 2. Platform-detected system font (SF Pro on macOS, Segoe UI Variable on Windows)
/// 3. **Inter** (embedded OFL-licensed, visually equivalent to SF Pro)
/// 4. Generic system-ui / sans-serif fallback
///
/// **Note on Apple SF Pro:** SF Pro is Apple-proprietary and cannot be redistributed.
/// On macOS, Quick uses `-apple-system` so the OS automatically serves SF Pro — no
/// font file is ever bundled or redistributed. On non-Apple platforms, Inter is used.
#[derive(Debug, Clone)]
pub struct FontStack {
    /// Ordered list of family names. First available family wins.
    pub families: Vec<String>,
    /// Primary family name for use in single-family contexts (e.g. Skia typeface lookup).
    pub primary: String,
}

impl FontStack {
    /// Build the font stack for the current OS.
    pub fn for_current_platform() -> Self {
        // 1. User override via env var
        if let Ok(env_val) = std::env::var("QUICK_FONT_FAMILY") {
            let families: Vec<String> = env_val
                .split(',')
                .map(|s| s.trim().to_string())
                .filter(|s| !s.is_empty())
                .collect();
            if !families.is_empty() {
                let primary = families[0].clone();
                return Self { families, primary };
            }
        }

        detect_system_font_stack()
    }

    /// CSS `font-family` value (quoted names, comma-separated).
    pub fn to_css_value(&self) -> String {
        self.families
            .iter()
            .map(|f| {
                if f.contains(' ') || f.starts_with('-') {
                    format!("\"{f}\"")
                } else {
                    f.clone()
                }
            })
            .collect::<Vec<_>>()
            .join(", ")
    }
}

impl Default for FontStack {
    fn default() -> Self {
        Self::for_current_platform()
    }
}

/// Detect the best available system font stack for this OS.
/// Returns Inter as the universal baseline — always available via embedded bytes.
fn detect_system_font_stack() -> FontStack {
    #[cfg(target_os = "macos")]
    {
        // On macOS the OS automatically maps -apple-system → SF Pro Text / SF Pro Display.
        // This is purely a CSS/Skia hint — no font file is bundled.
        return FontStack {
            families: vec![
                "-apple-system".into(),
                "BlinkMacSystemFont".into(),
                "SF Pro Text".into(),
                "Inter".into(),
                "system-ui".into(),
                "sans-serif".into(),
            ],
            primary: "-apple-system".into(),
        };
    }

    #[cfg(target_os = "windows")]
    {
        return FontStack {
            families: vec![
                "Segoe UI Variable".into(),
                "Segoe UI".into(),
                "Inter".into(),
                "system-ui".into(),
                "sans-serif".into(),
            ],
            primary: "Segoe UI Variable".into(),
        };
    }

    // Linux + all other platforms: Inter first, then Cantarell (GNOME), Noto Sans
    #[allow(unreachable_code)]
    linux_font_stack()
}

fn linux_font_stack() -> FontStack {
    // Probe for fonts actually installed on this system
    let candidates = [
        ("Inter",       &["/usr/share/fonts/truetype/inter", "/usr/local/share/fonts/inter",
                          "/usr/share/fonts/inter"][..]),
        ("Cantarell",   &["/usr/share/fonts/cantarell", "/usr/share/fonts/truetype/cantarell"][..]),
        ("Ubuntu",      &["/usr/share/fonts/truetype/ubuntu", "/usr/share/fonts/ubuntu"][..]),
        ("Noto Sans",   &["/usr/share/fonts/truetype/noto", "/usr/share/fonts/noto"][..]),
        ("DejaVu Sans", &["/usr/share/fonts/truetype/dejavu"][..]),
    ];

    let mut families: Vec<String> = vec!["Inter".into()]; // always first (embedded)

    for (name, paths) in &candidates {
        if *name == "Inter" { continue; } // already added
        let found = paths.iter().any(|p| std::path::Path::new(p).exists());
        if found {
            families.push(name.to_string());
        }
    }
    families.extend(["system-ui".into(), "sans-serif".into()]);

    FontStack {
        primary: families[0].clone(),
        families,
    }
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

    #[test]
    fn test_font_stack_has_fallback() {
        let stack = FontStack::for_current_platform();
        assert!(!stack.families.is_empty(), "font stack must not be empty");
        assert!(!stack.primary.is_empty(), "primary font must not be empty");
    }

    #[test]
    fn test_font_stack_css_value() {
        let stack = FontStack {
            families: vec!["Inter".into(), "-apple-system".into(), "sans-serif".into()],
            primary: "Inter".into(),
        };
        let css = stack.to_css_value();
        assert!(css.contains("Inter"));
        assert!(css.contains("-apple-system"));
    }

    #[test]
    fn test_font_stack_env_override() {
        std::env::set_var("QUICK_FONT_FAMILY", "Roboto, sans-serif");
        let stack = FontStack::for_current_platform();
        std::env::remove_var("QUICK_FONT_FAMILY");
        assert_eq!(stack.primary, "Roboto");
        assert_eq!(stack.families.len(), 2);
    }

    #[test]
    fn test_typescale_reactive_scaling() {
        assert_eq!(TypeScale::BODY, 12.0);
        let scaled_1_5 = TypeScale::scale(TypeScale::BODY, 1.5);
        assert_eq!(scaled_1_5, 18.0);
        let scaled_2_0 = TypeScale::scale(TypeScale::BODY, 2.0);
        assert_eq!(scaled_2_0, 24.0);
    }
}
