use quick_core::event::{
    FocusEvent, KeyEvent, KeyState, ModifiersState, PointerButton, PointerEvent, PointerPhase,
    ScrollDelta,
};
use quick_core::geometry::Point;
use winit::event::{ElementState, MouseButton, MouseScrollDelta, WindowEvent};
use winit::keyboard::{Key, NamedKey};

pub struct EventBridge {
    cursor_position: Point,
    modifiers: ModifiersState,
}

impl Default for EventBridge {
    fn default() -> Self {
        Self::new()
    }
}

impl EventBridge {
    pub fn new() -> Self {
        Self {
            cursor_position: Point::ZERO,
            modifiers: ModifiersState::default(),
        }
    }

    pub fn translate_event(&mut self, window_event: &WindowEvent) -> Option<quick_core::event::Event> {
        match window_event {
            WindowEvent::CursorMoved { position, .. } => {
                self.cursor_position = Point::new(position.x as f32, position.y as f32);
                Some(quick_core::event::Event::Pointer(PointerEvent {
                    position: self.cursor_position,
                    button: None,
                    phase: PointerPhase::Moved,
                    modifiers: self.modifiers,
                }))
            }
            WindowEvent::MouseInput { state, button, .. } => {
                let btn = match button {
                    MouseButton::Left => PointerButton::Primary,
                    MouseButton::Right => PointerSecondary(),
                    MouseButton::Middle => PointerButton::Middle,
                    MouseButton::Other(code) => PointerButton::Other(*code),
                    _ => PointerButton::Primary,
                };
                let phase = match state {
                    ElementState::Pressed => PointerPhase::Down,
                    ElementState::Released => PointerPhase::Up,
                };
                Some(quick_core::event::Event::Pointer(PointerEvent {
                    position: self.cursor_position,
                    button: Some(btn),
                    phase,
                    modifiers: self.modifiers,
                }))
            }
            WindowEvent::MouseWheel { delta, .. } => {
                let scroll = match delta {
                    MouseScrollDelta::LineDelta(x, y) => ScrollDelta::LineDelta(*x, *y),
                    MouseScrollDelta::PixelDelta(pos) => {
                        ScrollDelta::PixelDelta(pos.x as f32, pos.y as f32)
                    }
                };
                Some(quick_core::event::Event::Scroll(scroll))
            }
            WindowEvent::KeyboardInput { event, .. } => {
                let state = match event.state {
                    ElementState::Pressed => KeyState::Pressed,
                    ElementState::Released => KeyState::Released,
                };
                let key_str = match &event.logical_key {
                    Key::Named(named) => format!("{:?}", named),
                    Key::Character(ch) => ch.to_string(),
                    _ => "Unknown".to_string(),
                };
                let text = event.text.as_ref().map(|s| s.to_string());
                Some(quick_core::event::Event::Key(KeyEvent {
                    key: key_str,
                    state,
                    text,
                    modifiers: self.modifiers,
                }))
            }
            WindowEvent::Focused(focused) => {
                let focus_event = if *focused {
                    FocusEvent::Gained
                } else {
                    FocusEvent::Lost
                };
                Some(quick_core::event::Event::Focus(focus_event))
            }
            _ => None,
        }
    }
}

fn PointerSecondary() -> PointerButton {
    PointerButton::Secondary
}
