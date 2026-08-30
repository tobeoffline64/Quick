//! Wayland `wlr-layer-shell-unstable-v1` Protocol Integration for Quick UI.
//!
//! Enables desktop status panels, docks, notification overlays, and launchers.

use quick_core::geometry::Insets;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Layer {
    Background,
    Bottom,
    Top,
    Overlay,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Anchor {
    pub top: bool,
    pub bottom: bool,
    pub left: bool,
    pub right: bool,
}

impl Anchor {
    pub const TOP: Self = Self { top: true, bottom: false, left: false, right: false };
    pub const BOTTOM: Self = Self { top: false, bottom: true, left: false, right: false };
    pub const LEFT: Self = Self { top: false, bottom: false, left: true, right: false };
    pub const RIGHT: Self = Self { top: false, bottom: false, left: false, right: true };
    pub const TOP_BAR: Self = Self { top: true, bottom: false, left: true, right: true };
    pub const BOTTOM_DOCK: Self = Self { top: false, bottom: true, left: true, right: true };
    pub const FULLSCREEN: Self = Self { top: true, bottom: true, left: true, right: true };
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KeyboardInteractivity {
    None,
    Exclusive,
    OnDemand,
}

#[derive(Debug, Clone)]
pub struct LayerShellOptions {
    pub layer: Layer,
    pub anchor: Anchor,
    pub exclusive_zone: i32,
    pub margin: Insets,
    pub namespace: String,
    pub output_name: Option<String>,
    pub keyboard_interactivity: KeyboardInteractivity,
}

impl Default for LayerShellOptions {
    fn default() -> Self {
        Self {
            layer: Layer::Top,
            anchor: Anchor::TOP_BAR,
            exclusive_zone: 34,
            margin: Insets::ZERO,
            namespace: "noctalia-panel".into(),
            output_name: None,
            keyboard_interactivity: KeyboardInteractivity::None,
        }
    }
}

impl LayerShellOptions {
    pub fn top_bar(exclusive_height: i32) -> Self {
        Self {
            layer: Layer::Top,
            anchor: Anchor::TOP_BAR,
            exclusive_zone: exclusive_height,
            namespace: "noctalia-bar".into(),
            ..Self::default()
        }
    }

    pub fn bottom_dock(exclusive_height: i32) -> Self {
        Self {
            layer: Layer::Top,
            anchor: Anchor::BOTTOM_DOCK,
            exclusive_zone: exclusive_height,
            namespace: "noctalia-dock".into(),
            ..Self::default()
        }
    }

    pub fn overlay() -> Self {
        Self {
            layer: Layer::Overlay,
            anchor: Anchor::FULLSCREEN,
            exclusive_zone: 0,
            namespace: "noctalia-overlay".into(),
            keyboard_interactivity: KeyboardInteractivity::OnDemand,
            ..Self::default()
        }
    }
}
