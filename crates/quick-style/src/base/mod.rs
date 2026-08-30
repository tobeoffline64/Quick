//! Avalonia Fluent-inspired base theme for Quick widgets.
//!
//! Provides the un-themed default visual standard:
//! - 11-step neutral palette + OS-resolved accent color
//! - 4px spacing grid
//! - Corner radius scale (XS=2 → PILL=9999)
//! - Typography scale (Caption=11 → Display=28)
//! - OS color scheme (light/dark) + accent detection
//! - `BaseTheme` — the single API for widget constructors

pub mod palette;
pub mod spacing;
pub mod radius;
pub mod typography;
pub mod system;
pub mod theme;

pub use palette::{AccentColors, NeutralPalette};
pub use spacing::SpacingScale;
pub use radius::RadiusScale;
pub use typography::{FontWeight, TypeScale};
pub use system::{ColorScheme, SystemColors, detect_color_scheme, detect_accent_color};
pub use theme::{BaseColors, BaseTheme, RadiusScaleRef, SpacingScaleRef, TypeScaleRef};

use std::sync::OnceLock;

/// Global singleton — OS theme detected once at startup, reused everywhere.
static GLOBAL_BASE_THEME: OnceLock<BaseTheme> = OnceLock::new();

/// Initialize the global base theme from the OS. Call once at app start.
/// Subsequent calls return the cached value.
pub fn init_base_theme() -> &'static BaseTheme {
    GLOBAL_BASE_THEME.get_or_init(BaseTheme::from_system)
}

/// Access the global base theme (auto-initializes if not yet set).
pub fn base_theme() -> &'static BaseTheme {
    init_base_theme()
}
