use quick_core::event::{
    FocusEvent, KeyEvent, KeyState, ModifiersState, PointerButton, PointerEvent, PointerPhase,
    ScrollDelta,
};
use quick_core::geometry::Point;
use winit::event::{ElementState, MouseButton, MouseScrollDelta, WindowEvent};
use winit::keyboard::Key;

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
        self.translate_event_scaled(window_event, 1.0)
    }

    pub fn translate_event_scaled(&mut self, window_event: &WindowEvent, scale_factor: f32) -> Option<quick_core::event::Event> {
        let sf = if scale_factor > 0.0 { scale_factor } else { 1.0 };
        match window_event {
            WindowEvent::CursorMoved { position, .. } => {
                self.cursor_position = Point::new((position.x as f32) / sf, (position.y as f32) / sf);
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
                    MouseButton::Right => PointerButton::Secondary,
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
                        ScrollDelta::PixelDelta((pos.x as f32) / sf, (pos.y as f32) / sf)
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
            WindowEvent::ModifiersChanged(modifiers) => {
                let state = modifiers.state();
                self.modifiers = ModifiersState {
                    shift: state.shift_key(),
                    ctrl: state.control_key(),
                    alt: state.alt_key(),
                    meta: state.super_key(),
                };
                None
            }
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use winit::dpi::PhysicalPosition;

    #[test]
    fn test_event_bridge_cursor_and_mouse() {
        let mut bridge = EventBridge::new();

        // Cursor moved
        let move_ev = WindowEvent::CursorMoved {
            device_id: unsafe { std::mem::zeroed() },
            position: PhysicalPosition::new(150.0, 250.0),
        };
        let translated = bridge.translate_event(&move_ev).unwrap();
        if let quick_core::event::Event::Pointer(p) = translated {
            assert_eq!(p.position, Point::new(150.0, 250.0));
            assert_eq!(p.phase, PointerPhase::Moved);
            assert_eq!(p.button, None);
        } else {
            panic!("Expected pointer event");
        }

        // Mouse click
        let click_ev = WindowEvent::MouseInput {
            device_id: unsafe { std::mem::zeroed() },
            state: ElementState::Pressed,
            button: MouseButton::Left,
        };
        let translated_click = bridge.translate_event(&click_ev).unwrap();
        if let quick_core::event::Event::Pointer(p) = translated_click {
            assert_eq!(p.position, Point::new(150.0, 250.0));
            assert_eq!(p.phase, PointerPhase::Down);
            assert_eq!(p.button, Some(PointerButton::Primary));
        } else {
            panic!("Expected pointer event");
        }
    }

    #[test]
    fn test_event_bridge_focus() {
        let mut bridge = EventBridge::new();
        let focus_ev = WindowEvent::Focused(true);
        let translated = bridge.translate_event(&focus_ev).unwrap();
        assert_eq!(translated, quick_core::event::Event::Focus(FocusEvent::Gained));

        let blur_ev = WindowEvent::Focused(false);
        let translated_blur = bridge.translate_event(&blur_ev).unwrap();
        assert_eq!(translated_blur, quick_core::event::Event::Focus(FocusEvent::Lost));
    }
}
