//! Frameless Wayland / Desktop Window Helpers for Quick UI & Noctalia.

use quick_core::geometry::{Point, Rect};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WindowEdge {
    None,
    North,
    South,
    West,
    East,
    NorthWest,
    NorthEast,
    SouthWest,
    SouthEast,
}

#[derive(Debug, Clone)]
pub struct FramelessWindowConfig {
    pub titlebar_height: f32,
    pub resize_margin: f32,
    pub corner_radius: f32,
    pub acrylic_blur: bool,
}

impl Default for FramelessWindowConfig {
    fn default() -> Self {
        Self {
            titlebar_height: 44.0,
            resize_margin: 8.0,
            corner_radius: 12.0,
            acrylic_blur: true,
        }
    }
}

impl FramelessWindowConfig {
    /// Detect which window edge or titlebar region a cursor point falls in.
    pub fn hit_test_edge(&self, pos: Point, window_size: quick_core::geometry::Size) -> WindowEdge {
        let m = self.resize_margin;
        let w = window_size.width;
        let h = window_size.height;

        let left = pos.x < m;
        let right = pos.x > w - m;
        let top = pos.y < m;
        let bottom = pos.y > h - m;

        match (top, bottom, left, right) {
            (true, _, true, _) => WindowEdge::NorthWest,
            (true, _, _, true) => WindowEdge::NorthEast,
            (_, true, true, _) => WindowEdge::SouthWest,
            (_, true, _, true) => WindowEdge::SouthEast,
            (true, _, _, _) => WindowEdge::North,
            (_, true, _, _) => WindowEdge::South,
            (_, _, true, _) => WindowEdge::West,
            (_, _, _, true) => WindowEdge::East,
            _ => WindowEdge::None,
        }
    }

    /// Check if cursor position is inside the draggable 44px titlebar region (excluding resize margins).
    pub fn is_in_titlebar(&self, pos: Point, window_size: quick_core::geometry::Size) -> bool {
        let m = self.resize_margin;
        pos.x >= m && pos.x <= (window_size.width - m) && pos.y >= m && pos.y <= self.titlebar_height
    }

    /// Titlebar bounds rect.
    pub fn titlebar_rect(&self, window_size: quick_core::geometry::Size) -> Rect {
        Rect::new(0.0, 0.0, window_size.width, self.titlebar_height)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use quick_core::geometry::Size;

    #[test]
    fn test_frameless_edge_detection() {
        let cfg = FramelessWindowConfig::default();
        let size = Size::new(800.0, 600.0);

        assert_eq!(cfg.hit_test_edge(Point::new(2.0, 2.0), size), WindowEdge::NorthWest);
        assert_eq!(cfg.hit_test_edge(Point::new(798.0, 2.0), size), WindowEdge::NorthEast);
        assert_eq!(cfg.hit_test_edge(Point::new(400.0, 2.0), size), WindowEdge::North);
        assert_eq!(cfg.hit_test_edge(Point::new(2.0, 300.0), size), WindowEdge::West);
        assert_eq!(cfg.hit_test_edge(Point::new(400.0, 300.0), size), WindowEdge::None);
    }

    #[test]
    fn test_frameless_titlebar_drag_hit() {
        let cfg = FramelessWindowConfig::default();
        let size = Size::new(800.0, 600.0);

        assert!(cfg.is_in_titlebar(Point::new(400.0, 20.0), size));
        assert!(!cfg.is_in_titlebar(Point::new(400.0, 100.0), size));
    }
}
