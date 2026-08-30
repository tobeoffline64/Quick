use crate::geometry::Point;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PointerButton {
    Primary,   // Left
    Secondary, // Right
    Middle,
    Other(u16),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PointerPhase {
    Moved,
    Down,
    Up,
    Cancel,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct ModifiersState {
    pub shift: bool,
    pub ctrl: bool,
    pub alt: bool,
    pub meta: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PointerEvent {
    pub position: Point,
    pub button: Option<PointerButton>,
    pub phase: PointerPhase,
    pub modifiers: ModifiersState,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum KeyState {
    Pressed,
    Released,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct KeyEvent {
    pub key: String,
    pub state: KeyState,
    pub text: Option<String>,
    pub modifiers: ModifiersState,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum ScrollDelta {
    LineDelta(f32, f32),
    PixelDelta(f32, f32),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum FocusEvent {
    Gained,
    Lost,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Event {
    Pointer(PointerEvent),
    Key(KeyEvent),
    Scroll(ScrollDelta),
    Focus(FocusEvent),
    Custom { name: String, payload: serde_json::Value },
}
