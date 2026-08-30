use serde::{Deserialize, Serialize};

/// 2D Point in physical or logical coordinates.
#[derive(Debug, Clone, Copy, PartialEq, Default, Serialize, Deserialize)]
pub struct Point {
    pub x: f32,
    pub y: f32,
}

impl Point {
    pub const ZERO: Self = Self { x: 0.0, y: 0.0 };

    pub const fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    pub fn distance_to(self, other: Self) -> f32 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        (dx * dx + dy * dy).sqrt()
    }
}

/// 2D Dimensions (width and height).
#[derive(Debug, Clone, Copy, PartialEq, Default, Serialize, Deserialize)]
pub struct Size {
    pub width: f32,
    pub height: f32,
}

impl Size {
    pub const ZERO: Self = Self { width: 0.0, height: 0.0 };

    pub const fn new(width: f32, height: f32) -> Self {
        Self { width, height }
    }

    pub fn is_empty(self) -> bool {
        self.width <= 0.0 || self.height <= 0.0
    }
}

/// 2D Rectangle.
#[derive(Debug, Clone, Copy, PartialEq, Default, Serialize, Deserialize)]
pub struct Rect {
    pub origin: Point,
    pub size: Size,
}

impl Rect {
    pub const ZERO: Self = Self {
        origin: Point::ZERO,
        size: Size::ZERO,
    };

    pub const fn new(x: f32, y: f32, width: f32, height: f32) -> Self {
        Self {
            origin: Point::new(x, y),
            size: Size::new(width, height),
        }
    }

    pub const fn from_origin_size(origin: Point, size: Size) -> Self {
        Self { origin, size }
    }

    pub fn min_x(&self) -> f32 {
        self.origin.x
    }

    pub fn min_y(&self) -> f32 {
        self.origin.y
    }

    pub fn max_x(&self) -> f32 {
        self.origin.x + self.size.width
    }

    pub fn max_y(&self) -> f32 {
        self.origin.y + self.size.height
    }

    pub fn width(&self) -> f32 {
        self.size.width
    }

    pub fn height(&self) -> f32 {
        self.size.height
    }

    pub fn contains(&self, point: Point) -> bool {
        point.x >= self.min_x()
            && point.x <= self.max_x()
            && point.y >= self.min_y()
            && point.y <= self.max_y()
    }

    pub fn intersects(&self, other: &Self) -> bool {
        self.min_x() < other.max_x()
            && self.max_x() > other.min_x()
            && self.min_y() < other.max_y()
            && self.max_y() > other.min_y()
    }

    pub fn union(&self, other: &Self) -> Self {
        if self.size.is_empty() {
            return *other;
        }
        if other.size.is_empty() {
            return *self;
        }
        let min_x = self.min_x().min(other.min_x());
        let min_y = self.min_y().min(other.min_y());
        let max_x = self.max_x().max(other.max_x());
        let max_y = self.max_y().max(other.max_y());
        Self::new(min_x, min_y, max_x - min_x, max_y - min_y)
    }

    pub fn inset(&self, insets: Insets) -> Self {
        let x = self.origin.x + insets.left;
        let y = self.origin.y + insets.top;
        let width = (self.size.width - insets.left - insets.right).max(0.0);
        let height = (self.size.height - insets.top - insets.bottom).max(0.0);
        Self::new(x, y, width, height)
    }

    pub fn offset(&self, dx: f32, dy: f32) -> Self {
        Self::new(self.origin.x + dx, self.origin.y + dy, self.size.width, self.size.height)
    }
}

/// Insets (padding or margin) for 4 sides.
#[derive(Debug, Clone, Copy, PartialEq, Default, Serialize, Deserialize)]
pub struct Insets {
    pub top: f32,
    pub right: f32,
    pub bottom: f32,
    pub left: f32,
}

impl Insets {
    pub const ZERO: Self = Self {
        top: 0.0,
        right: 0.0,
        bottom: 0.0,
        left: 0.0,
    };

    pub const fn all(val: f32) -> Self {
        Self {
            top: val,
            right: val,
            bottom: val,
            left: val,
        }
    }

    pub const fn symmetric(vertical: f32, horizontal: f32) -> Self {
        Self {
            top: vertical,
            right: horizontal,
            bottom: vertical,
            left: horizontal,
        }
    }

    pub const fn new(top: f32, right: f32, bottom: f32, left: f32) -> Self {
        Self {
            top,
            right,
            bottom,
            left,
        }
    }
}

/// Border radius for rectangle corners.
#[derive(Debug, Clone, Copy, PartialEq, Default, Serialize, Deserialize)]
pub struct BorderRadius {
    pub top_left: f32,
    pub top_right: f32,
    pub bottom_right: f32,
    pub bottom_left: f32,
}

impl BorderRadius {
    pub const ZERO: Self = Self {
        top_left: 0.0,
        top_right: 0.0,
        bottom_right: 0.0,
        bottom_left: 0.0,
    };

    pub const fn all(radius: f32) -> Self {
        Self {
            top_left: radius,
            top_right: radius,
            bottom_right: radius,
            bottom_left: radius,
        }
    }

    pub const fn new(top_left: f32, top_right: f32, bottom_right: f32, bottom_left: f32) -> Self {
        Self {
            top_left,
            top_right,
            bottom_right,
            bottom_left,
        }
    }
}

/// RGBA 8-bit color.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Default for Color {
    fn default() -> Self {
        Self::BLACK
    }
}

impl Color {
    pub const TRANSPARENT: Self = Self::from_rgba(0, 0, 0, 0);
    pub const BLACK: Self = Self::from_rgb(0, 0, 0);
    pub const WHITE: Self = Self::from_rgb(255, 255, 255);
    pub const RED: Self = Self::from_rgb(255, 0, 0);
    pub const GREEN: Self = Self::from_rgb(0, 255, 0);
    pub const BLUE: Self = Self::from_rgb(0, 0, 255);

    pub const fn from_rgba(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { r, g, b, a }
    }

    pub const fn from_rgb(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b, a: 255 }
    }

    pub fn from_hex(hex: &str) -> Result<Self, String> {
        let trimmed = hex.trim().to_lowercase();
        match trimmed.as_str() {
            "transparent" => return Ok(Self::TRANSPARENT),
            "black" => return Ok(Self::BLACK),
            "white" => return Ok(Self::WHITE),
            "red" => return Ok(Self::RED),
            "green" => return Ok(Self::GREEN),
            "blue" => return Ok(Self::BLUE),
            "gray" | "grey" => return Ok(Self::from_rgb(128, 128, 128)),
            "lightgray" | "lightgrey" => return Ok(Self::from_rgb(211, 211, 211)),
            "darkgray" | "darkgrey" => return Ok(Self::from_rgb(169, 169, 169)),
            "yellow" => return Ok(Self::from_rgb(255, 255, 0)),
            "cyan" => return Ok(Self::from_rgb(0, 255, 255)),
            "magenta" => return Ok(Self::from_rgb(255, 0, 255)),
            "orange" => return Ok(Self::from_rgb(255, 165, 0)),
            "purple" => return Ok(Self::from_rgb(128, 0, 128)),
            "pink" => return Ok(Self::from_rgb(255, 192, 203)),
            "brown" => return Ok(Self::from_rgb(165, 42, 42)),
            "navy" => return Ok(Self::from_rgb(0, 0, 128)),
            "teal" => return Ok(Self::from_rgb(0, 128, 128)),
            "olive" => return Ok(Self::from_rgb(128, 128, 0)),
            "maroon" => return Ok(Self::from_rgb(128, 0, 0)),
            "silver" => return Ok(Self::from_rgb(192, 192, 192)),
            "gold" => return Ok(Self::from_rgb(255, 215, 0)),
            "lime" => return Ok(Self::from_rgb(0, 255, 0)),
            "indigo" => return Ok(Self::from_rgb(75, 0, 130)),
            "violet" => return Ok(Self::from_rgb(238, 130, 238)),
            _ => {}
        }

        // Handle rgb(r, g, b) and rgba(r, g, b, a)
        if let Some(rgb_args) = trimmed.strip_prefix("rgb(").and_then(|s| s.strip_suffix(')')) {
            let parts: Vec<&str> = rgb_args.split(',').map(|s| s.trim()).collect();
            if parts.len() == 3 {
                let r = parts[0].parse::<u8>().map_err(|e| e.to_string())?;
                let g = parts[1].parse::<u8>().map_err(|e| e.to_string())?;
                let b = parts[2].parse::<u8>().map_err(|e| e.to_string())?;
                return Ok(Self::from_rgb(r, g, b));
            }
        } else if let Some(rgba_args) = trimmed.strip_prefix("rgba(").and_then(|s| s.strip_suffix(')')) {
            let parts: Vec<&str> = rgba_args.split(',').map(|s| s.trim()).collect();
            if parts.len() == 4 {
                let r = parts[0].parse::<u8>().map_err(|e| e.to_string())?;
                let g = parts[1].parse::<u8>().map_err(|e| e.to_string())?;
                let b = parts[2].parse::<u8>().map_err(|e| e.to_string())?;
                let a = if let Ok(f) = parts[3].parse::<f32>() {
                    if f <= 1.0 {
                        (f * 255.0).round() as u8
                    } else {
                        f.min(255.0) as u8
                    }
                } else {
                    parts[3].parse::<u8>().map_err(|e| e.to_string())?
                };
                return Ok(Self::from_rgba(r, g, b, a));
            }
        }

        let hex = trimmed.trim_start_matches('#');
        match hex.len() {
            3 => {
                let r = u8::from_str_radix(&hex[0..1].repeat(2), 16).map_err(|e| e.to_string())?;
                let g = u8::from_str_radix(&hex[1..2].repeat(2), 16).map_err(|e| e.to_string())?;
                let b = u8::from_str_radix(&hex[2..3].repeat(2), 16).map_err(|e| e.to_string())?;
                Ok(Self::from_rgb(r, g, b))
            }
            4 => {
                let r = u8::from_str_radix(&hex[0..1].repeat(2), 16).map_err(|e| e.to_string())?;
                let g = u8::from_str_radix(&hex[1..2].repeat(2), 16).map_err(|e| e.to_string())?;
                let b = u8::from_str_radix(&hex[2..3].repeat(2), 16).map_err(|e| e.to_string())?;
                let a = u8::from_str_radix(&hex[3..4].repeat(2), 16).map_err(|e| e.to_string())?;
                Ok(Self::from_rgba(r, g, b, a))
            }
            6 => {
                let r = u8::from_str_radix(&hex[0..2], 16).map_err(|e| e.to_string())?;
                let g = u8::from_str_radix(&hex[2..4], 16).map_err(|e| e.to_string())?;
                let b = u8::from_str_radix(&hex[4..6], 16).map_err(|e| e.to_string())?;
                Ok(Self::from_rgb(r, g, b))
            }
            8 => {
                let r = u8::from_str_radix(&hex[0..2], 16).map_err(|e| e.to_string())?;
                let g = u8::from_str_radix(&hex[2..4], 16).map_err(|e| e.to_string())?;
                let b = u8::from_str_radix(&hex[4..6], 16).map_err(|e| e.to_string())?;
                let a = u8::from_str_radix(&hex[6..8], 16).map_err(|e| e.to_string())?;
                Ok(Self::from_rgba(r, g, b, a))
            }
            _ => Err(format!("Invalid hex color: #{hex}")),
        }
    }

    pub fn to_hex(&self) -> String {
        if self.a == 255 {
            format!("#{:02X}{:02X}{:02X}", self.r, self.g, self.b)
        } else {
            format!("#{:02X}{:02X}{:02X}{:02X}", self.r, self.g, self.b, self.a)
        }
    }

    pub const fn from_argb_u32(argb: u32) -> Self {
        let a = ((argb >> 24) & 0xFF) as u8;
        let r = ((argb >> 16) & 0xFF) as u8;
        let g = ((argb >> 8) & 0xFF) as u8;
        let b = (argb & 0xFF) as u8;
        Self { r, g, b, a }
    }

    pub fn to_argb_u32(self) -> u32 {
        ((self.a as u32) << 24)
            | ((self.r as u32) << 16)
            | ((self.g as u32) << 8)
            | (self.b as u32)
    }

    pub fn to_rgba_f32(self) -> [f32; 4] {
        [
            self.r as f32 / 255.0,
            self.g as f32 / 255.0,
            self.b as f32 / 255.0,
            self.a as f32 / 255.0,
        ]
    }
}

/// 2D Affine transformation.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Transform {
    pub a: f32, // scale x
    pub b: f32, // shear y
    pub c: f32, // shear x
    pub d: f32, // scale y
    pub tx: f32, // translate x
    pub ty: f32, // translate y
}

impl Default for Transform {
    fn default() -> Self {
        Self::IDENTITY
    }
}

impl Transform {
    pub const IDENTITY: Self = Self {
        a: 1.0,
        b: 0.0,
        c: 0.0,
        d: 1.0,
        tx: 0.0,
        ty: 0.0,
    };

    pub fn translation(tx: f32, ty: f32) -> Self {
        Self {
            a: 1.0,
            b: 0.0,
            c: 0.0,
            d: 1.0,
            tx,
            ty,
        }
    }

    pub fn scale(sx: f32, sy: f32) -> Self {
        Self {
            a: sx,
            b: 0.0,
            c: 0.0,
            d: sy,
            tx: 0.0,
            ty: 0.0,
        }
    }

    pub fn transform_point(&self, point: Point) -> Point {
        Point::new(
            self.a * point.x + self.c * point.y + self.tx,
            self.b * point.x + self.d * point.y + self.ty,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_point_and_rect() {
        let p1 = Point::new(10.0, 20.0);
        let p2 = Point::new(10.0, 25.0);
        assert_eq!(p1.distance_to(p2), 5.0);

        let r1 = Rect::new(0.0, 0.0, 100.0, 100.0);
        assert!(r1.contains(Point::new(50.0, 50.0)));
        assert!(!r1.contains(Point::new(150.0, 50.0)));

        let r2 = Rect::new(50.0, 50.0, 100.0, 100.0);
        assert!(r1.intersects(&r2));

        let union = r1.union(&r2);
        assert_eq!(union, Rect::new(0.0, 0.0, 150.0, 150.0));
    }

    #[test]
    fn test_color_hex_parsing() {
        let red = Color::from_hex("#ff0000").unwrap();
        assert_eq!(red, Color::from_rgb(255, 0, 0));

        let blue_short = Color::from_hex("#00f").unwrap();
        assert_eq!(blue_short, Color::from_rgb(0, 0, 255));

        let rgba_short = Color::from_hex("#f008").unwrap();
        assert_eq!(rgba_short, Color::from_rgba(255, 0, 0, 136));

        let semi_green = Color::from_hex("#00ff0080").unwrap();
        assert_eq!(semi_green, Color::from_rgba(0, 255, 0, 128));

        let white = Color::from_hex("white").unwrap();
        assert_eq!(white, Color::WHITE);

        let transparent = Color::from_hex("transparent").unwrap();
        assert_eq!(transparent, Color::TRANSPARENT);

        let rgb_red = Color::from_hex("rgb(255, 0, 0)").unwrap();
        assert_eq!(rgb_red, Color::from_rgb(255, 0, 0));

        let rgba_green = Color::from_hex("rgba(0, 255, 0, 0.5)").unwrap();
        assert_eq!(rgba_green, Color::from_rgba(0, 255, 0, 128));

        let orange = Color::from_hex("orange").unwrap();
        assert_eq!(orange, Color::from_rgb(255, 165, 0));

        assert!(Color::from_hex("invalid_color_123").is_err());
    }

    #[test]
    fn test_transform() {
        let t = Transform::translation(10.0, 20.0);
        let p = t.transform_point(Point::new(5.0, 5.0));
        assert_eq!(p, Point::new(15.0, 25.0));
    }
}
