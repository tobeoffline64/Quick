use quick_core::geometry::{Point, Rect, Size};
use quick_style::property::{
    AlignItems, Dimension, FlexDirection, JustifyContent, Style,
};
use taffy::prelude::*;

pub struct LayoutEngine {
    tree: TaffyTree<()>,
}

impl Default for LayoutEngine {
    fn default() -> Self {
        Self::new()
    }
}

impl LayoutEngine {
    pub fn new() -> Self {
        Self {
            tree: TaffyTree::new(),
        }
    }

    pub fn new_leaf(&mut self, style: &Style) -> Result<NodeId, TaffyError> {
        let taffy_style = Self::convert_style(style);
        self.tree.new_leaf(taffy_style)
    }

    pub fn new_with_children(&mut self, style: &Style, children: &[NodeId]) -> Result<NodeId, TaffyError> {
        let taffy_style = Self::convert_style(style);
        self.tree.new_with_children(taffy_style, children)
    }

    pub fn set_style(&mut self, node: NodeId, style: &Style) -> Result<(), TaffyError> {
        let taffy_style = Self::convert_style(style);
        self.tree.set_style(node, taffy_style)
    }

    pub fn compute_layout(&mut self, root: NodeId, available_space: Size) -> Result<(), TaffyError> {
        let space = taffy::geometry::Size {
            width: AvailableSpace::Definite(available_space.width),
            height: AvailableSpace::Definite(available_space.height),
        };
        self.tree.compute_layout(root, space)
    }

    pub fn get_layout(&self, node: NodeId) -> Result<Rect, TaffyError> {
        let layout = self.tree.layout(node)?;
        Ok(Rect::new(
            layout.location.x,
            layout.location.y,
            layout.size.width,
            layout.size.height,
        ))
    }

    pub fn reset(&mut self) {
        self.tree.clear();
    }

    pub fn convert_style(style: &Style) -> taffy::style::Style {
        let mut ts = taffy::style::Style::default();

        if let Some(dim) = style.width {
            ts.size.width = match dim {
                Dimension::Auto => LengthPercentageAuto::Auto,
                Dimension::Px(px) => LengthPercentageAuto::Length(px),
                Dimension::Percent(pct) => LengthPercentageAuto::Percent(pct / 100.0),
            };
        }

        if let Some(dim) = style.height {
            ts.size.height = match dim {
                Dimension::Auto => LengthPercentageAuto::Auto,
                Dimension::Px(px) => LengthPercentageAuto::Length(px),
                Dimension::Percent(pct) => LengthPercentageAuto::Percent(pct / 100.0),
            };
        }

        if let Some(pad) = style.padding {
            ts.padding = taffy::geometry::Rect {
                top: LengthPercentage::Length(pad.top),
                right: LengthPercentage::Length(pad.right),
                bottom: LengthPercentage::Length(pad.bottom),
                left: LengthPercentage::Length(pad.left),
            };
        }

        if let Some(mar) = style.margin {
            ts.margin = taffy::geometry::Rect {
                top: LengthPercentageAuto::Length(mar.top),
                right: LengthPercentageAuto::Length(mar.right),
                bottom: LengthPercentageAuto::Length(mar.bottom),
                left: LengthPercentageAuto::Length(mar.left),
            };
        }

        if let Some(dir) = style.flex_direction {
            ts.flex_direction = match dir {
                FlexDirection::Row => taffy::style::FlexDirection::Row,
                FlexDirection::Column => taffy::style::FlexDirection::Column,
                FlexDirection::RowReverse => taffy::style::FlexDirection::RowReverse,
                FlexDirection::ColumnReverse => taffy::style::FlexDirection::ColumnReverse,
            };
        }

        if let Some(justify) = style.justify_content {
            ts.justify_content = match justify {
                JustifyContent::FlexStart => Some(taffy::style::JustifyContent::FlexStart),
                JustifyContent::Center => Some(taffy::style::JustifyContent::Center),
                JustifyContent::FlexEnd => Some(taffy::style::JustifyContent::FlexEnd),
                JustifyContent::SpaceBetween => Some(taffy::style::JustifyContent::SpaceBetween),
                JustifyContent::SpaceAround => Some(taffy::style::JustifyContent::SpaceAround),
                JustifyContent::SpaceEvenly => Some(taffy::style::JustifyContent::SpaceEvenly),
            };
        }

        if let Some(align) = style.align_items {
            ts.align_items = match align {
                AlignItems::FlexStart => Some(taffy::style::AlignItems::FlexStart),
                AlignItems::Center => Some(taffy::style::AlignItems::Center),
                AlignItems::FlexEnd => Some(taffy::style::AlignItems::FlexEnd),
                AlignItems::Stretch => Some(taffy::style::AlignItems::Stretch),
                AlignItems::Baseline => Some(taffy::style::AlignItems::Baseline),
            };
        }

        if let Some(gap) = style.gap {
            ts.gap = taffy::geometry::Size {
                width: LengthPercentage::Length(gap),
                height: LengthPercentage::Length(gap),
            };
        }

        ts
    }
}
