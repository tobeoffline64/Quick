use quick_core::geometry::{BorderRadius, Color, Insets};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum Dimension {
    Auto,
    Px(f32),
    Percent(f32),
}

impl Dimension {
    pub fn to_px(self, parent_size: f32) -> Option<f32> {
        match self {
            Dimension::Auto => None,
            Dimension::Px(val) => Some(val),
            Dimension::Percent(pct) => Some(parent_size * (pct / 100.0)),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum FlexDirection {
    Row,
    Column,
    RowReverse,
    ColumnReverse,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum JustifyContent {
    FlexStart,
    Center,
    FlexEnd,
    SpaceBetween,
    SpaceAround,
    SpaceEvenly,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AlignItems {
    FlexStart,
    Center,
    FlexEnd,
    Stretch,
    Baseline,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TextAlignment {
    Left,
    Center,
    Right,
    Justify,
}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct Style {
    pub width: Option<Dimension>,
    pub height: Option<Dimension>,
    pub min_width: Option<Dimension>,
    pub min_height: Option<Dimension>,
    pub max_width: Option<Dimension>,
    pub max_height: Option<Dimension>,

    pub padding: Option<Insets>,
    pub margin: Option<Insets>,

    pub flex_direction: Option<FlexDirection>,
    pub justify_content: Option<JustifyContent>,
    pub align_items: Option<AlignItems>,
    pub gap: Option<f32>,

    pub background_color: Option<Color>,
    pub text_color: Option<Color>,
    pub border_color: Option<Color>,
    pub border_width: Option<f32>,
    pub border_radius: Option<BorderRadius>,
    pub opacity: Option<f32>,

    pub font_family: Option<String>,
    pub font_size: Option<f32>,
    pub font_weight: Option<u16>,
    pub text_align: Option<TextAlignment>,
}

impl Style {
    pub fn merge_with(&mut self, other: &Self) {
        if other.width.is_some() { self.width = other.width; }
        if other.height.is_some() { self.height = other.height; }
        if other.min_width.is_some() { self.min_width = other.min_width; }
        if other.min_height.is_some() { self.min_height = other.min_height; }
        if other.max_width.is_some() { self.max_width = other.max_width; }
        if other.max_height.is_some() { self.max_height = other.max_height; }

        if other.padding.is_some() { self.padding = other.padding; }
        if other.margin.is_some() { self.margin = other.margin; }

        if other.flex_direction.is_some() { self.flex_direction = other.flex_direction; }
        if other.justify_content.is_some() { self.justify_content = other.justify_content; }
        if other.align_items.is_some() { self.align_items = other.align_items; }
        if other.gap.is_some() { self.gap = other.gap; }

        if other.background_color.is_some() { self.background_color = other.background_color; }
        if other.text_color.is_some() { self.text_color = other.text_color; }
        if other.border_color.is_some() { self.border_color = other.border_color; }
        if other.border_width.is_some() { self.border_width = other.border_width; }
        if other.border_radius.is_some() { self.border_radius = other.border_radius; }
        if other.opacity.is_some() { self.opacity = other.opacity; }

        if other.font_family.is_some() { self.font_family = other.font_family.clone(); }
        if other.font_size.is_some() { self.font_size = other.font_size; }
        if other.font_weight.is_some() { self.font_weight = other.font_weight; }
        if other.text_align.is_some() { self.text_align = other.text_align; }
    }
}
