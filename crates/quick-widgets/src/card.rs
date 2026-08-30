use crate::container::Container;
use crate::widget::Widget;
use quick_core::event::Event;
use quick_core::geometry::{BorderRadius, Color, Insets, Point, Rect};
use quick_layout::engine::LayoutEngine;
use quick_render::canvas::Canvas;
use quick_style::property::{Dimension, FlexDirection, Style};
use taffy::prelude::NodeId;
use taffy::TaffyError;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CardVariant {
    Elevated,
    Filled,
    Outlined,
}

pub struct Card {
    pub container: Container,
    pub variant: CardVariant,
}

impl Card {
    pub fn new(variant: CardVariant) -> Self {
        let mut container = Container::new();
        container.style.flex_direction = Some(FlexDirection::Column);
        container.style.border_radius = Some(BorderRadius::all(16.0));
        container.style.padding = Some(Insets::all(24.0));
        container.style.gap = Some(16.0);

        match variant {
            CardVariant::Elevated => {
                container.style.background_color = Some(Color::from_hex("#1E1F2B").unwrap());
            }
            CardVariant::Filled => {
                container.style.background_color = Some(Color::from_hex("#252736").unwrap());
            }
            CardVariant::Outlined => {
                container.style.background_color = Some(Color::from_hex("#161B22").unwrap());
                container.style.border_color = Some(Color::from_hex("#30363D").unwrap());
                container.style.border_width = Some(1.0);
            }
        }

        Self { container, variant }
    }

    pub fn with_child(mut self, child: impl Widget + 'static) -> Self {
        self.container.add_child(child);
        self
    }

    pub fn add_child(&mut self, child: impl Widget + 'static) {
        self.container.add_child(child);
    }
}

impl Widget for Card {
    fn widget_type(&self) -> &'static str {
        "Card"
    }

    fn id(&self) -> Option<&str> {
        self.container.id()
    }

    fn classes(&self) -> &[String] {
        self.container.classes()
    }

    fn style(&self) -> &Style {
        self.container.style()
    }

    fn style_mut(&mut self) -> &mut Style {
        self.container.style_mut()
    }

    fn build_layout(&mut self, engine: &mut LayoutEngine) -> Result<NodeId, TaffyError> {
        self.container.build_layout(engine)
    }

    fn update_layout(&mut self, engine: &LayoutEngine, parent_origin: Point) {
        self.container.update_layout(engine, parent_origin);
    }

    fn paint(&self, canvas: &mut Canvas, bounds: Rect) {
        // For elevated variant, draw a soft shadow
        if self.variant == CardVariant::Elevated {
            let shadow_bounds = Rect::new(bounds.origin.x, bounds.origin.y + 3.0, bounds.size.width, bounds.size.height);
            let radius = self.container.style.border_radius.unwrap_or(BorderRadius::all(16.0));
            canvas.fill_rounded_rect(shadow_bounds, radius, Color::from_rgba(0, 0, 0, 80));
        }

        self.container.paint(canvas, bounds);
    }

    fn handle_event(&mut self, event: &Event, bounds: Rect) -> bool {
        self.container.handle_event(event, bounds)
    }
}
