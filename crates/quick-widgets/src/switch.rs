use crate::widget::Widget;
use quick_core::event::{Event, PointerButton, PointerEvent, PointerPhase};
use quick_core::geometry::{BorderRadius, Color, Point, Rect, Size};
use quick_core::signals::Signal;
use quick_layout::engine::LayoutEngine;
use quick_render::canvas::Canvas;
use quick_style::property::{Dimension, Style};
use taffy::prelude::NodeId;
use taffy::TaffyError;

pub struct Switch {
    pub id: Option<String>,
    pub classes: Vec<String>,
    pub style: Style,
    pub checked: Signal<bool>,
    pub on_change: Option<Box<dyn FnMut(bool)>>,
    is_hovered: bool,
    is_pressed: bool,
}

impl Switch {
    pub fn new(checked: Signal<bool>) -> Self {
        let mut style = Style::default();
        style.width = Some(Dimension::Px(52.0));
        style.height = Some(Dimension::Px(32.0));

        Self {
            id: None,
            classes: Vec::new(),
            style,
            checked,
            on_change: None,
            is_hovered: false,
            is_pressed: false,
        }
    }

    pub fn on_change<F: FnMut(bool) + 'static>(mut self, handler: F) -> Self {
        self.on_change = Some(Box::new(handler));
        self
    }
}

impl Widget for Switch {
    fn widget_type(&self) -> &'static str {
        "Switch"
    }

    fn id(&self) -> Option<&str> {
        self.id.as_deref()
    }

    fn classes(&self) -> &[String] {
        &self.classes
    }

    fn style(&self) -> &Style {
        &self.style
    }

    fn style_mut(&mut self) -> &mut Style {
        &mut self.style
    }

    fn build_layout(&mut self, engine: &mut LayoutEngine) -> Result<NodeId, TaffyError> {
        let mut computed_style = self.style.clone();
        if computed_style.width.is_none() {
            computed_style.width = Some(Dimension::Px(52.0));
        }
        if computed_style.height.is_none() {
            computed_style.height = Some(Dimension::Px(32.0));
        }
        engine.new_leaf(&computed_style)
    }

    fn paint(&self, canvas: &mut Canvas, bounds: Rect) {
        let is_on = self.checked.get();

        // Colors
        let track_color = if is_on {
            self.style.background_color.unwrap_or(Color::from_hex("#6750A4").unwrap())
        } else {
            Color::from_hex("#36343B").unwrap()
        };

        let thumb_color = if is_on {
            Color::from_hex("#FFFFFF").unwrap()
        } else {
            Color::from_hex("#938F99").unwrap()
        };

        // Draw track pill
        let track_radius = BorderRadius::all(bounds.size.height / 2.0);
        canvas.fill_rounded_rect(bounds, track_radius, track_color);

        if !is_on {
            canvas.stroke_rounded_rect(bounds, track_radius, Color::from_hex("#79747E").unwrap(), 2.0);
        }

        // Draw thumb circle
        let thumb_size = if is_on {
            bounds.size.height - 8.0
        } else {
            bounds.size.height - 14.0
        };

        let thumb_x = if is_on {
            bounds.origin.x + bounds.size.width - thumb_size - 4.0
        } else {
            bounds.origin.x + 7.0
        };

        let thumb_y = bounds.origin.y + (bounds.size.height - thumb_size) / 2.0;
        let thumb_rect = Rect::new(thumb_x, thumb_y, thumb_size, thumb_size);
        canvas.fill_rounded_rect(thumb_rect, BorderRadius::all(thumb_size / 2.0), thumb_color);
    }

    fn handle_event(&mut self, event: &Event, bounds: Rect) -> bool {
        if let Event::Pointer(PointerEvent { position, button, phase, .. }) = event {
            let inside = bounds.contains(*position);
            self.is_hovered = inside;

            match phase {
                PointerPhase::Down if inside && *button == Some(PointerButton::Primary) => {
                    self.is_pressed = true;
                    return true;
                }
                PointerPhase::Up if self.is_pressed => {
                    self.is_pressed = false;
                    if inside {
                        let new_state = !self.checked.get();
                        self.checked.set(new_state);
                        if let Some(ref mut handler) = self.on_change {
                            handler(new_state);
                        }
                        return true;
                    }
                }
                PointerPhase::Cancel => {
                    self.is_pressed = false;
                }
                _ => {}
            }
        }
        false
    }
}
