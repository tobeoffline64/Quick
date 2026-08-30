use crate::container::Container;
use crate::widget::Widget;
use quick_core::event::Event;
use quick_core::geometry::{BorderRadius, Color, Insets, Point, Rect};
use quick_layout::engine::LayoutEngine;
use quick_render::canvas::Canvas;
use quick_style::base::{base_theme, RadiusScale, SpacingScale};
use quick_style::property::{FlexDirection, Style};
use quick_style::theme::tokens::ElevationTokens;
use serde::{Deserialize, Serialize};
use taffy::prelude::NodeId;
use taffy::TaffyError;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum CardVariant {
    #[default]
    Elevated,
    Filled,
    Outlined,
}

pub struct Card {
    pub container: Container,
    pub variant: CardVariant,
    pub elevation: u8,
}

impl Card {
    pub fn new(variant: CardVariant) -> Self {
        let bt = base_theme();
        let mut container = Container::new();
        container.style.flex_direction = Some(FlexDirection::Column);
        container.style.border_radius = Some(BorderRadius::all(RadiusScale::MD));
        container.style.padding = Some(Insets::all(SpacingScale::XXL));
        container.style.gap = Some(SpacingScale::LG);

        let elevation = match variant {
            CardVariant::Elevated => {
                container.style.background_color = Some(bt.colors.surface);
                container.style.border_color = Some(bt.colors.border);
                container.style.border_width = Some(1.0);
                1
            }
            CardVariant::Filled => {
                container.style.background_color = Some(bt.colors.surface_raised);
                0
            }
            CardVariant::Outlined => {
                container.style.background_color = Some(bt.colors.bg);
                container.style.border_color = Some(bt.colors.border);
                container.style.border_width = Some(1.0);
                0
            }
        };

        Self {
            container,
            variant,
            elevation,
        }
    }

    pub fn elevated() -> Self {
        Self::new(CardVariant::Elevated)
    }

    pub fn filled() -> Self {
        Self::new(CardVariant::Filled)
    }

    pub fn outlined() -> Self {
        Self::new(CardVariant::Outlined)
    }

    pub fn with_variant(mut self, variant: CardVariant) -> Self {
        self.variant = variant;
        match variant {
            CardVariant::Elevated => {
                self.container.style.background_color = Some(Color::from_hex("#1D1B20").unwrap_or(Color::from_hex("#1E1F2B").unwrap()));
                self.container.style.border_color = None;
                self.container.style.border_width = None;
                if self.elevation == 0 {
                    self.elevation = 1;
                }
            }
            CardVariant::Filled => {
                self.container.style.background_color = Some(Color::from_hex("#2B2930").unwrap_or(Color::from_hex("#252736").unwrap()));
                self.container.style.border_color = None;
                self.container.style.border_width = None;
                self.elevation = 0;
            }
            CardVariant::Outlined => {
                self.container.style.background_color = Some(Color::from_hex("#141218").unwrap_or(Color::from_hex("#161B22").unwrap()));
                self.container.style.border_color = Some(Color::from_hex("#49454F").unwrap_or(Color::from_hex("#30363D").unwrap()));
                self.container.style.border_width = Some(1.0);
                self.elevation = 0;
            }
        }
        self
    }

    pub fn with_elevation(mut self, elevation: u8) -> Self {
        self.elevation = elevation;
        self
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
        let radius = self.container.style.border_radius.unwrap_or_else(|| BorderRadius::all(16.0));

        // Draw dual-pass elevation shadow for elevated variant
        if self.variant == CardVariant::Elevated && self.elevation > 0 {
            let elev_tokens = ElevationTokens::default();
            canvas.draw_elevation_shadow(bounds, radius, self.elevation, &elev_tokens);
        }

        self.container.paint(canvas, bounds);
    }

    fn handle_event(&mut self, event: &Event, bounds: Rect) -> bool {
        self.container.handle_event(event, bounds)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::button::Button;
    use crate::text::Text;
    use quick_core::geometry::Size;

    #[test]
    fn test_card_variants_and_painting() {
        let mut card_elevated = Card::new(CardVariant::Elevated)
            .with_child(Text::new("Title"))
            .with_child(Button::new("Action"));

        let card_filled = Card::new(CardVariant::Filled)
            .with_child(Text::new("Filled Content"));

        let card_outlined = Card::new(CardVariant::Outlined)
            .with_child(Text::new("Outlined Content"));

        let mut engine = LayoutEngine::new();
        let el_node = card_elevated.build_layout(&mut engine).unwrap();
        engine.compute_layout(el_node, Size::new(400.0, 300.0)).unwrap();
        card_elevated.update_layout(&engine, Point::ZERO);

        let mut canvas = Canvas::new();
        let bounds = engine.get_layout(el_node).unwrap();
        card_elevated.paint(&mut canvas, bounds);

        // Canvas should have 2 shadow commands + card bg + text + button bg + button text (>= 5 commands)
        assert!(canvas.commands().len() >= 5);

        let mut canvas_filled = Canvas::new();
        card_filled.paint(&mut canvas_filled, bounds);
        assert!(!canvas_filled.commands().is_empty());

        let mut canvas_outlined = Canvas::new();
        card_outlined.paint(&mut canvas_outlined, bounds);
        assert!(!canvas_outlined.commands().is_empty());
    }
}
