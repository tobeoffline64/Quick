use quick_core::geometry::{Rect, Size};
use quick_style::property::{
    AlignItems, Dimension as QuickDimension, FlexDirection, JustifyContent, Style,
};
use taffy::prelude::*;
use taffy::TaffyError;

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
                QuickDimension::Auto => Dimension::Auto,
                QuickDimension::Px(px) => Dimension::Length(px),
                QuickDimension::Percent(pct) => Dimension::Percent(pct / 100.0),
            };
        }

        if let Some(dim) = style.height {
            ts.size.height = match dim {
                QuickDimension::Auto => Dimension::Auto,
                QuickDimension::Px(px) => Dimension::Length(px),
                QuickDimension::Percent(pct) => Dimension::Percent(pct / 100.0),
            };
        }

        if let Some(dim) = style.min_width {
            ts.min_size.width = match dim {
                QuickDimension::Auto => Dimension::Auto,
                QuickDimension::Px(px) => Dimension::Length(px),
                QuickDimension::Percent(pct) => Dimension::Percent(pct / 100.0),
            };
        }

        if let Some(dim) = style.min_height {
            ts.min_size.height = match dim {
                QuickDimension::Auto => Dimension::Auto,
                QuickDimension::Px(px) => Dimension::Length(px),
                QuickDimension::Percent(pct) => Dimension::Percent(pct / 100.0),
            };
        }

        if let Some(dim) = style.max_width {
            ts.max_size.width = match dim {
                QuickDimension::Auto => Dimension::Auto,
                QuickDimension::Px(px) => Dimension::Length(px),
                QuickDimension::Percent(pct) => Dimension::Percent(pct / 100.0),
            };
        }

        if let Some(dim) = style.max_height {
            ts.max_size.height = match dim {
                QuickDimension::Auto => Dimension::Auto,
                QuickDimension::Px(px) => Dimension::Length(px),
                QuickDimension::Percent(pct) => Dimension::Percent(pct / 100.0),
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_layout_computation() {
        let mut engine = LayoutEngine::new();

        let mut child1_style = Style::default();
        child1_style.width = Some(QuickDimension::Px(100.0));
        child1_style.height = Some(QuickDimension::Px(50.0));
        let c1 = engine.new_leaf(&child1_style).unwrap();

        let mut child2_style = Style::default();
        child2_style.width = Some(QuickDimension::Px(100.0));
        child2_style.height = Some(QuickDimension::Px(50.0));
        let c2 = engine.new_leaf(&child2_style).unwrap();

        let mut root_style = Style::default();
        root_style.flex_direction = Some(FlexDirection::Row);
        root_style.width = Some(QuickDimension::Px(300.0));
        root_style.height = Some(QuickDimension::Px(100.0));

        let root = engine.new_with_children(&root_style, &[c1, c2]).unwrap();
        engine.compute_layout(root, Size::new(300.0, 100.0)).unwrap();

        let r1 = engine.get_layout(c1).unwrap();
        let r2 = engine.get_layout(c2).unwrap();

        assert_eq!(r1.size.width, 100.0);
        assert_eq!(r1.size.height, 50.0);
        assert_eq!(r2.origin.x, 100.0);
    }

    #[test]
    fn test_nested_percent_layout() {
        let mut engine = LayoutEngine::new();

        let mut child_style = Style::default();
        child_style.width = Some(QuickDimension::Percent(50.0));
        child_style.height = Some(QuickDimension::Percent(100.0));
        let c1 = engine.new_leaf(&child_style).unwrap();
        let c2 = engine.new_leaf(&child_style).unwrap();

        let mut parent_style = Style::default();
        parent_style.flex_direction = Some(FlexDirection::Row);
        parent_style.width = Some(QuickDimension::Px(400.0));
        parent_style.height = Some(QuickDimension::Px(200.0));

        let root = engine.new_with_children(&parent_style, &[c1, c2]).unwrap();
        engine.compute_layout(root, Size::new(400.0, 200.0)).unwrap();

        let r1 = engine.get_layout(c1).unwrap();
        let r2 = engine.get_layout(c2).unwrap();

        assert_eq!(r1.size.width, 200.0);
        assert_eq!(r1.size.height, 200.0);
        assert_eq!(r2.origin.x, 200.0);
        assert_eq!(r2.size.width, 200.0);
    }

    #[test]
    fn test_min_max_size_constraints() {
        let mut engine = LayoutEngine::new();

        let mut child_style = Style::default();
        child_style.width = Some(QuickDimension::Px(50.0));
        child_style.min_width = Some(QuickDimension::Px(120.0));
        child_style.max_width = Some(QuickDimension::Px(250.0));
        child_style.height = Some(QuickDimension::Px(30.0));
        child_style.min_height = Some(QuickDimension::Px(60.0));

        let c1 = engine.new_leaf(&child_style).unwrap();

        let mut root_style = Style::default();
        root_style.width = Some(QuickDimension::Px(400.0));
        root_style.height = Some(QuickDimension::Px(200.0));

        let root = engine.new_with_children(&root_style, &[c1]).unwrap();
        engine.compute_layout(root, Size::new(400.0, 200.0)).unwrap();

        let r1 = engine.get_layout(c1).unwrap();
        assert_eq!(r1.size.width, 120.0); // min-width 120 overrides width 50
        assert_eq!(r1.size.height, 60.0); // min-height 60 overrides height 30
    }

    #[test]
    fn test_layout_boundary_and_zero_sizes() {
        let mut engine = LayoutEngine::new();

        let zero_style = Style::default();
        let c1 = engine.new_leaf(&zero_style).unwrap();

        let mut root_style = Style::default();
        root_style.width = Some(QuickDimension::Px(0.0));
        root_style.height = Some(QuickDimension::Px(0.0));

        let root = engine.new_with_children(&root_style, &[c1]).unwrap();
        assert!(engine.compute_layout(root, Size::new(0.0, 0.0)).is_ok());

        let r = engine.get_layout(root).unwrap();
        assert_eq!(r.size.width, 0.0);
        assert_eq!(r.size.height, 0.0);
    }

    #[test]
    fn test_deep_layout_nesting_stress() {
        let mut engine = LayoutEngine::new();
        let mut current = engine.new_leaf(&Style::default()).unwrap();

        for _ in 0..50 {
            let mut container_style = Style::default();
            container_style.padding = Some(quick_core::geometry::Insets::all(2.0));
            current = engine.new_with_children(&container_style, &[current]).unwrap();
        }

        assert!(engine.compute_layout(current, Size::new(500.0, 500.0)).is_ok());
        let layout = engine.get_layout(current).unwrap();
        assert!(layout.size.width <= 500.0);
    }

    #[test]
    fn test_wide_layout_siblings_stress() {
        let mut engine = LayoutEngine::new();
        let mut child_style = Style::default();
        child_style.width = Some(QuickDimension::Px(2.0));
        child_style.height = Some(QuickDimension::Px(10.0));

        let mut children = Vec::with_capacity(500);
        for _ in 0..500 {
            children.push(engine.new_leaf(&child_style).unwrap());
        }

        let mut root_style = Style::default();
        root_style.flex_direction = Some(FlexDirection::Row);
        root_style.width = Some(QuickDimension::Px(1000.0));
        root_style.height = Some(QuickDimension::Px(20.0));

        let root = engine.new_with_children(&root_style, &children).unwrap();
        assert!(engine.compute_layout(root, Size::new(1000.0, 20.0)).is_ok());

        let last_child_layout = engine.get_layout(children[499]).unwrap();
        assert_eq!(last_child_layout.size.width, 2.0);
        assert_eq!(last_child_layout.origin.x, 499.0 * 2.0);
    }
}
