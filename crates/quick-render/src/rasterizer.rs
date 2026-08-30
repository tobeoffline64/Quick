use crate::canvas::{Canvas, DrawCommand};
use quick_core::geometry::{BorderRadius, Color, Insets, Point, Rect};

pub struct SoftwareRasterizer;

impl SoftwareRasterizer {
    /// Rasterize all Canvas draw commands into a 32-bit ARGB/XRGB pixel buffer.
    pub fn render_to_buffer(canvas: &Canvas, width: u32, height: u32, buffer: &mut [u32]) {
        if width == 0 || height == 0 || buffer.len() < (width * height) as usize {
            return;
        }

        let mut clip_stack: Vec<Rect> = vec![Rect::new(0.0, 0.0, width as f32, height as f32)];
        let mut transform_stack: Vec<(f32, f32, f32, f32)> = vec![(0.0, 0.0, 1.0, 1.0)];

        for cmd in canvas.commands() {
            let (tx, ty, sx, sy) = *transform_stack.last().unwrap_or(&(0.0, 0.0, 1.0, 1.0));
            let clip = *clip_stack.last().unwrap_or(&Rect::new(0.0, 0.0, width as f32, height as f32));

            let map_rect = |r: &Rect| -> Rect {
                Rect::new(
                    r.origin.x * sx + tx,
                    r.origin.y * sy + ty,
                    r.size.width * sx,
                    r.size.height * sy,
                )
            };

            match cmd {
                DrawCommand::Clear(c) => {
                    let pixel = c.to_argb_u32();
                    buffer.fill(pixel);
                }
                DrawCommand::FillRect(rect, color) => {
                    Self::fill_rect(buffer, width, height, map_rect(rect), *color, clip);
                }
                DrawCommand::StrokeRect(rect, color, stroke_w) => {
                    Self::stroke_rect(buffer, width, height, map_rect(rect), *color, *stroke_w * sx.max(sy), clip);
                }
                DrawCommand::FillRoundedRect(rect, radius, color) => {
                    let scaled_rad = BorderRadius::new(
                        radius.top_left * sx.max(sy),
                        radius.top_right * sx.max(sy),
                        radius.bottom_right * sx.max(sy),
                        radius.bottom_left * sx.max(sy),
                    );
                    Self::fill_rounded_rect(buffer, width, height, map_rect(rect), scaled_rad, *color, clip);
                }
                DrawCommand::StrokeRoundedRect(rect, radius, color, stroke_w) => {
                    let scaled_rad = BorderRadius::new(
                        radius.top_left * sx.max(sy),
                        radius.top_right * sx.max(sy),
                        radius.bottom_right * sx.max(sy),
                        radius.bottom_left * sx.max(sy),
                    );
                    Self::stroke_rounded_rect(buffer, width, height, map_rect(rect), scaled_rad, *color, *stroke_w * sx.max(sy), clip);
                }
                DrawCommand::DrawText { text, origin, color, font_size, .. } => {
                    let offset_origin = Point::new(origin.x * sx + tx, origin.y * sy + ty);
                    Self::draw_text(buffer, width, height, text, offset_origin, *color, *font_size * sy, clip);
                }
                DrawCommand::DrawLine { start, end, color, width: stroke_w } => {
                    let s = Point::new(start.x * sx + tx, start.y * sy + ty);
                    let e = Point::new(end.x * sx + tx, end.y * sy + ty);
                    Self::draw_line(buffer, width, height, s, e, *color, *stroke_w * sx.max(sy), clip);
                }
                DrawCommand::PushClip(r) => {
                    let offset_r = map_rect(r);
                    let min_x = clip.min_x().max(offset_r.min_x());
                    let min_y = clip.min_y().max(offset_r.min_y());
                    let max_x = clip.max_x().min(offset_r.max_x());
                    let max_y = clip.max_y().min(offset_r.max_y());
                    let new_clip = if max_x > min_x && max_y > min_y {
                        Rect::new(min_x, min_y, max_x - min_x, max_y - min_y)
                    } else {
                        Rect::ZERO
                    };
                    clip_stack.push(new_clip);
                }
                DrawCommand::PopClip => {
                    if clip_stack.len() > 1 {
                        clip_stack.pop();
                    }
                }
                DrawCommand::Translate(dx, dy) => {
                    if let Some(top) = transform_stack.last_mut() {
                        top.0 += dx * top.2;
                        top.1 += dy * top.3;
                    }
                }
                DrawCommand::Scale(sx_scale, sy_scale) => {
                    if let Some(top) = transform_stack.last_mut() {
                        top.2 *= sx_scale;
                        top.3 *= sy_scale;
                    }
                }
                DrawCommand::Save => {
                    transform_stack.push(*transform_stack.last().unwrap_or(&(0.0, 0.0, 1.0, 1.0)));
                    clip_stack.push(*clip_stack.last().unwrap_or(&Rect::new(0.0, 0.0, width as f32, height as f32)));
                }
                DrawCommand::Restore => {
                    if transform_stack.len() > 1 {
                        transform_stack.pop();
                    }
                    if clip_stack.len() > 1 {
                        clip_stack.pop();
                    }
                }
            }
        }
    }

    #[inline(always)]
    fn set_pixel(buffer: &mut [u32], width: u32, height: u32, x: i32, y: i32, color: Color, clip: Rect) {
        let xf = x as f32;
        let yf = y as f32;
        if xf >= clip.min_x() && xf < clip.max_x() && yf >= clip.min_y() && yf < clip.max_y() {
            if x >= 0 && x < width as i32 && y >= 0 && y < height as i32 {
                let idx = (y as u32 * width + x as u32) as usize;
                if color.a == 255 {
                    buffer[idx] = color.to_argb_u32();
                } else if color.a > 0 {
                    let dst = buffer[idx];
                    let dst_r = ((dst >> 16) & 0xFF) as u32;
                    let dst_g = ((dst >> 8) & 0xFF) as u32;
                    let dst_b = (dst & 0xFF) as u32;

                    let alpha = color.a as u32;
                    let inv_alpha = 255 - alpha;

                    let r = (color.r as u32 * alpha + dst_r * inv_alpha) / 255;
                    let g = (color.g as u32 * alpha + dst_g * inv_alpha) / 255;
                    let b = (color.b as u32 * alpha + dst_b * inv_alpha) / 255;

                    buffer[idx] = (0xFF << 24) | (r << 16) | (g << 8) | b;
                }
            }
        }
    }

    fn fill_rect(buffer: &mut [u32], width: u32, height: u32, rect: Rect, color: Color, clip: Rect) {
        let x0 = rect.min_x().max(clip.min_x()).max(0.0) as i32;
        let y0 = rect.min_y().max(clip.min_y()).max(0.0) as i32;
        let x1 = rect.max_x().min(clip.max_x()).min(width as f32) as i32;
        let y1 = rect.max_y().min(clip.max_y()).min(height as f32) as i32;

        for y in y0..y1 {
            for x in x0..x1 {
                Self::set_pixel(buffer, width, height, x, y, color, clip);
            }
        }
    }

    fn stroke_rect(buffer: &mut [u32], width: u32, height: u32, rect: Rect, color: Color, stroke_w: f32, clip: Rect) {
        let sw = stroke_w.max(1.0);
        Self::fill_rect(buffer, width, height, Rect::new(rect.origin.x, rect.origin.y, rect.size.width, sw), color, clip);
        Self::fill_rect(buffer, width, height, Rect::new(rect.origin.x, rect.origin.y + rect.size.height - sw, rect.size.width, sw), color, clip);
        Self::fill_rect(buffer, width, height, Rect::new(rect.origin.x, rect.origin.y, sw, rect.size.height), color, clip);
        Self::fill_rect(buffer, width, height, Rect::new(rect.origin.x + rect.size.width - sw, rect.origin.y, sw, rect.size.height), color, clip);
    }

    fn fill_rounded_rect(buffer: &mut [u32], width: u32, height: u32, rect: Rect, radius: BorderRadius, color: Color, clip: Rect) {
        let r = radius.top_left.min(rect.size.width / 2.0).min(rect.size.height / 2.0);
        if r <= 1.0 {
            Self::fill_rect(buffer, width, height, rect, color, clip);
            return;
        }

        let x0 = rect.min_x().max(clip.min_x()).max(0.0) as i32;
        let y0 = rect.min_y().max(clip.min_y()).max(0.0) as i32;
        let x1 = rect.max_x().min(clip.max_x()).min(width as f32) as i32;
        let y1 = rect.max_y().min(clip.max_y()).min(height as f32) as i32;

        for y in y0..y1 {
            let py = y as f32 + 0.5;
            for x in x0..x1 {
                let px = x as f32 + 0.5;

                let in_top_left = px < rect.min_x() + r && py < rect.min_y() + r;
                let in_top_right = px > rect.max_x() - r && py < rect.min_y() + r;
                let in_bottom_left = px < rect.min_x() + r && py > rect.max_y() - r;
                let in_bottom_right = px > rect.max_x() - r && py > rect.max_y() - r;

                let inside = if in_top_left {
                    let dx = px - (rect.min_x() + r);
                    let dy = py - (rect.min_y() + r);
                    dx * dx + dy * dy <= r * r
                } else if in_top_right {
                    let dx = px - (rect.max_x() - r);
                    let dy = py - (rect.min_y() + r);
                    dx * dx + dy * dy <= r * r
                } else if in_bottom_left {
                    let dx = px - (rect.min_x() + r);
                    let dy = py - (rect.max_y() - r);
                    dx * dx + dy * dy <= r * r
                } else if in_bottom_right {
                    let dx = px - (rect.max_x() - r);
                    let dy = py - (rect.max_y() - r);
                    dx * dx + dy * dy <= r * r
                } else {
                    true
                };

                if inside {
                    Self::set_pixel(buffer, width, height, x, y, color, clip);
                }
            }
        }
    }

    fn stroke_rounded_rect(buffer: &mut [u32], width: u32, height: u32, rect: Rect, radius: BorderRadius, color: Color, stroke_w: f32, clip: Rect) {
        let sw = stroke_w.max(1.0);
        let inner = rect.inset(Insets::all(sw));
        let r = radius.top_left.min(rect.size.width / 2.0).min(rect.size.height / 2.0);

        let x0 = rect.min_x().max(clip.min_x()).max(0.0) as i32;
        let y0 = rect.min_y().max(clip.min_y()).max(0.0) as i32;
        let x1 = rect.max_x().min(clip.max_x()).min(width as f32) as i32;
        let y1 = rect.max_y().min(clip.max_y()).min(height as f32) as i32;

        for y in y0..y1 {
            let py = y as f32 + 0.5;
            for x in x0..x1 {
                let px = x as f32 + 0.5;

                let in_outer = if px < rect.min_x() + r && py < rect.min_y() + r {
                    let dx = px - (rect.min_x() + r);
                    let dy = py - (rect.min_y() + r);
                    dx * dx + dy * dy <= r * r
                } else if px > rect.max_x() - r && py < rect.min_y() + r {
                    let dx = px - (rect.max_x() - r);
                    let dy = py - (rect.min_y() + r);
                    dx * dx + dy * dy <= r * r
                } else if px < rect.min_x() + r && py > rect.max_y() - r {
                    let dx = px - (rect.min_x() + r);
                    let dy = py - (rect.max_y() - r);
                    dx * dx + dy * dy <= r * r
                } else if px > rect.max_x() - r && py > rect.max_y() - r {
                    let dx = px - (rect.max_x() - r);
                    let dy = py - (rect.max_y() - r);
                    dx * dx + dy * dy <= r * r
                } else {
                    true
                };

                let in_inner = inner.contains(Point::new(px, py));

                if in_outer && !in_inner {
                    Self::set_pixel(buffer, width, height, x, y, color, clip);
                }
            }
        }
    }

    fn draw_line(buffer: &mut [u32], width: u32, height: u32, start: Point, end: Point, color: Color, stroke_w: f32, clip: Rect) {
        let dx = (end.x - start.x).abs();
        let dy = (end.y - start.y).abs();
        let steps = dx.max(dy).max(1.0) as usize;

        for i in 0..=steps {
            let t = i as f32 / steps as f32;
            let x = start.x + (end.x - start.x) * t;
            let y = start.y + (end.y - start.y) * t;
            Self::fill_rect(
                buffer,
                width,
                height,
                Rect::new(x - stroke_w / 2.0, y - stroke_w / 2.0, stroke_w, stroke_w),
                color,
                clip,
            );
        }
    }

    fn draw_text(buffer: &mut [u32], width: u32, height: u32, text: &str, origin: Point, color: Color, font_size: f32, clip: Rect) {
        let scale = (font_size / 14.0).max(1.0);
        let s = scale.round().max(1.0) as i32;
        let char_step = (8 * s) as i32;
        let line_height = (font_size * 1.4) as i32;

        let mut cur_x = origin.x as i32;
        let mut cur_y = (origin.y - font_size * 0.85) as i32;
        let start_x = cur_x;

        for ch in text.chars() {
            if ch == '\n' {
                cur_x = start_x;
                cur_y += line_height;
                continue;
            }

            Self::draw_char_bitmap(buffer, width, height, ch, cur_x, cur_y, color, s, clip);
            cur_x += char_step;
        }
    }

    fn draw_char_bitmap(buffer: &mut [u32], width: u32, height: u32, ch: char, ox: i32, oy: i32, color: Color, s: i32, clip: Rect) {
        let glyph = get_bitmap_glyph(ch);

        for row in 0..12 {
            let bits = glyph[row as usize];
            for col in 0..8 {
                if (bits & (1 << (7 - col))) != 0 {
                    for sy in 0..s {
                        for sx in 0..s {
                            Self::set_pixel(buffer, width, height, ox + col * s + sx, oy + row * s + sy, color, clip);
                        }
                    }
                }
            }
        }
    }
}

/// Embedded 8x12 bitmap font table
fn get_bitmap_glyph(ch: char) -> &'static [u8; 12] {
    match ch {
        'A' => &[0x38, 0x6C, 0xC6, 0xC6, 0xFE, 0xC6, 0xC6, 0xC6, 0x00, 0x00, 0x00, 0x00],
        'a' => &[0x00, 0x00, 0x00, 0x78, 0x0C, 0x7C, 0xCC, 0x76, 0x00, 0x00, 0x00, 0x00],
        'B' => &[0xFC, 0x66, 0x66, 0x7C, 0x66, 0x66, 0xFC, 0x00, 0x00, 0x00, 0x00, 0x00],
        'b' => &[0xE0, 0x60, 0x60, 0x7C, 0x66, 0x66, 0xDC, 0x00, 0x00, 0x00, 0x00, 0x00],
        'C' => &[0x3C, 0x66, 0xC0, 0xC0, 0xC0, 0x66, 0x3C, 0x00, 0x00, 0x00, 0x00, 0x00],
        'c' => &[0x00, 0x00, 0x00, 0x78, 0xCC, 0xC0, 0xCC, 0x78, 0x00, 0x00, 0x00, 0x00],
        'D' => &[0xF8, 0x6C, 0x66, 0x66, 0x66, 0x6C, 0xF8, 0x00, 0x00, 0x00, 0x00, 0x00],
        'd' => &[0x1C, 0x0C, 0x0C, 0x7C, 0xCC, 0xCC, 0x76, 0x00, 0x00, 0x00, 0x00, 0x00],
        'E' => &[0xFE, 0x62, 0x68, 0x78, 0x68, 0x62, 0xFE, 0x00, 0x00, 0x00, 0x00, 0x00],
        'e' => &[0x00, 0x00, 0x00, 0x78, 0xCC, 0xFC, 0xC0, 0x78, 0x00, 0x00, 0x00, 0x00],
        'F' => &[0xFE, 0x62, 0x68, 0x78, 0x68, 0x60, 0xF0, 0x00, 0x00, 0x00, 0x00, 0x00],
        'f' => &[0x38, 0x6C, 0x60, 0xF0, 0x60, 0x60, 0xF0, 0x00, 0x00, 0x00, 0x00, 0x00],
        'G' => &[0x3C, 0x66, 0xC0, 0xCE, 0xC6, 0x66, 0x3E, 0x00, 0x00, 0x00, 0x00, 0x00],
        'g' => &[0x00, 0x00, 0x00, 0x76, 0xCC, 0xCC, 0x7C, 0x0C, 0xF8, 0x00, 0x00, 0x00],
        'H' => &[0xC6, 0xC6, 0xC6, 0xFE, 0xC6, 0xC6, 0xC6, 0x00, 0x00, 0x00, 0x00, 0x00],
        'h' => &[0xE0, 0x60, 0x60, 0x6C, 0x76, 0x66, 0xE6, 0x00, 0x00, 0x00, 0x00, 0x00],
        'I' => &[0x3C, 0x18, 0x18, 0x18, 0x18, 0x18, 0x3C, 0x00, 0x00, 0x00, 0x00, 0x00],
        'i' => &[0x18, 0x00, 0x00, 0x78, 0x18, 0x18, 0x7E, 0x00, 0x00, 0x00, 0x00, 0x00],
        'J' => &[0x1E, 0x0C, 0x0C, 0x0C, 0xCC, 0xCC, 0x78, 0x00, 0x00, 0x00, 0x00, 0x00],
        'j' => &[0x0C, 0x00, 0x00, 0x3C, 0x0C, 0x0C, 0xCC, 0x78, 0x00, 0x00, 0x00, 0x00],
        'K' => &[0xE6, 0x66, 0x6C, 0x78, 0x6C, 0x66, 0xE6, 0x00, 0x00, 0x00, 0x00, 0x00],
        'k' => &[0xE0, 0x60, 0x60, 0x66, 0x7C, 0x6C, 0xE6, 0x00, 0x00, 0x00, 0x00, 0x00],
        'L' => &[0xF0, 0x60, 0x60, 0x60, 0x62, 0x66, 0xFE, 0x00, 0x00, 0x00, 0x00, 0x00],
        'l' => &[0x78, 0x18, 0x18, 0x18, 0x18, 0x18, 0x7E, 0x00, 0x00, 0x00, 0x00, 0x00],
        'M' => &[0xC6, 0xEE, 0xFE, 0xFE, 0xD6, 0xC6, 0xC6, 0x00, 0x00, 0x00, 0x00, 0x00],
        'm' => &[0x00, 0x00, 0x00, 0xEC, 0xFE, 0xD6, 0xD6, 0xD6, 0x00, 0x00, 0x00, 0x00],
        'N' => &[0xC6, 0xE6, 0xF6, 0xDE, 0xCE, 0xC6, 0xC6, 0x00, 0x00, 0x00, 0x00, 0x00],
        'n' => &[0x00, 0x00, 0x00, 0xDC, 0x66, 0x66, 0x66, 0x66, 0x00, 0x00, 0x00, 0x00],
        'O' => &[0x38, 0x6C, 0xC6, 0xC6, 0xC6, 0x6C, 0x38, 0x00, 0x00, 0x00, 0x00, 0x00],
        'o' => &[0x00, 0x00, 0x00, 0x78, 0xCC, 0xCC, 0xCC, 0x78, 0x00, 0x00, 0x00, 0x00],
        'P' => &[0xFC, 0x66, 0x66, 0x7C, 0x60, 0x60, 0xF0, 0x00, 0x00, 0x00, 0x00, 0x00],
        'p' => &[0x00, 0x00, 0x00, 0xDC, 0x66, 0x66, 0x7C, 0x60, 0xF0, 0x00, 0x00, 0x00],
        'Q' => &[0x38, 0x6C, 0xC6, 0xC6, 0xD6, 0x6C, 0x3C, 0x06, 0x00, 0x00, 0x00, 0x00],
        'q' => &[0x00, 0x00, 0x00, 0x76, 0xCC, 0xCC, 0x7C, 0x0C, 0x1E, 0x00, 0x00, 0x00],
        'R' => &[0xFC, 0x66, 0x66, 0x7C, 0x6C, 0x66, 0xE6, 0x00, 0x00, 0x00, 0x00, 0x00],
        'r' => &[0x00, 0x00, 0x00, 0xDC, 0x76, 0x60, 0x60, 0xF0, 0x00, 0x00, 0x00, 0x00],
        'S' => &[0x7C, 0xC6, 0x60, 0x38, 0x0C, 0xC6, 0x7C, 0x00, 0x00, 0x00, 0x00, 0x00],
        's' => &[0x00, 0x00, 0x00, 0x7C, 0xC0, 0x78, 0x0C, 0xF8, 0x00, 0x00, 0x00, 0x00],
        'T' => &[0xFE, 0xBA, 0x38, 0x38, 0x38, 0x38, 0x7C, 0x00, 0x00, 0x00, 0x00, 0x00],
        't' => &[0x30, 0x30, 0xFC, 0x30, 0x30, 0x30, 0x1C, 0x00, 0x00, 0x00, 0x00, 0x00],
        'U' => &[0xC6, 0xC6, 0xC6, 0xC6, 0xC6, 0xC6, 0x7C, 0x00, 0x00, 0x00, 0x00, 0x00],
        'u' => &[0x00, 0x00, 0x00, 0xCC, 0xCC, 0xCC, 0xCC, 0x76, 0x00, 0x00, 0x00, 0x00],
        'V' => &[0xC6, 0xC6, 0xC6, 0xC6, 0x6C, 0x38, 0x10, 0x00, 0x00, 0x00, 0x00, 0x00],
        'v' => &[0x00, 0x00, 0x00, 0xC6, 0xC6, 0x6C, 0x38, 0x10, 0x00, 0x00, 0x00, 0x00],
        'W' => &[0xC6, 0xC6, 0xD6, 0xFE, 0xFE, 0xEE, 0xC6, 0x00, 0x00, 0x00, 0x00, 0x00],
        'w' => &[0x00, 0x00, 0x00, 0xC6, 0xD6, 0xFE, 0x7C, 0x6C, 0x00, 0x00, 0x00, 0x00],
        'X' => &[0xC6, 0x6C, 0x38, 0x38, 0x6C, 0xC6, 0xC6, 0x00, 0x00, 0x00, 0x00, 0x00],
        'x' => &[0x00, 0x00, 0x00, 0xC6, 0x6C, 0x38, 0x6C, 0xC6, 0x00, 0x00, 0x00, 0x00],
        'Y' => &[0x66, 0x66, 0x66, 0x3C, 0x18, 0x18, 0x3C, 0x00, 0x00, 0x00, 0x00, 0x00],
        'y' => &[0x00, 0x00, 0x00, 0xC6, 0xC6, 0x7C, 0x0C, 0xF8, 0x00, 0x00, 0x00, 0x00],
        'Z' => &[0xFE, 0xC6, 0x0C, 0x18, 0x30, 0x63, 0xFE, 0x00, 0x00, 0x00, 0x00, 0x00],
        'z' => &[0x00, 0x00, 0x00, 0xFC, 0x18, 0x30, 0x60, 0xFC, 0x00, 0x00, 0x00, 0x00],
        '0' => &[0x38, 0x6C, 0xC6, 0xD6, 0xE6, 0x6C, 0x38, 0x00, 0x00, 0x00, 0x00, 0x00],
        '1' => &[0x18, 0x38, 0x18, 0x18, 0x18, 0x18, 0x7E, 0x00, 0x00, 0x00, 0x00, 0x00],
        '2' => &[0x7C, 0xC6, 0x06, 0x1C, 0x30, 0x60, 0xFE, 0x00, 0x00, 0x00, 0x00, 0x00],
        '3' => &[0x7C, 0xC6, 0x06, 0x3C, 0x06, 0xC6, 0x7C, 0x00, 0x00, 0x00, 0x00, 0x00],
        '4' => &[0x0C, 0x1C, 0x3C, 0x6C, 0xCC, 0xFE, 0x0C, 0x00, 0x00, 0x00, 0x00, 0x00],
        '5' => &[0xFE, 0xC0, 0xF8, 0x0C, 0x06, 0xC6, 0x7C, 0x00, 0x00, 0x00, 0x00, 0x00],
        '6' => &[0x38, 0x60, 0xC0, 0xFC, 0xC6, 0xC6, 0x7C, 0x00, 0x00, 0x00, 0x00, 0x00],
        '7' => &[0xFE, 0xC6, 0x0C, 0x18, 0x30, 0x30, 0x30, 0x00, 0x00, 0x00, 0x00, 0x00],
        '8' => &[0x7C, 0xC6, 0xC6, 0x7C, 0xC6, 0xC6, 0x7C, 0x00, 0x00, 0x00, 0x00, 0x00],
        '9' => &[0x7C, 0xC6, 0xC6, 0x7E, 0x06, 0x0C, 0x78, 0x00, 0x00, 0x00, 0x00, 0x00],
        '!' => &[0x18, 0x18, 0x18, 0x18, 0x00, 0x18, 0x18, 0x00, 0x00, 0x00, 0x00, 0x00],
        '?' => &[0x7C, 0xC6, 0x0C, 0x18, 0x18, 0x00, 0x18, 0x00, 0x00, 0x00, 0x00, 0x00],
        '.' => &[0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x18, 0x18, 0x00, 0x00, 0x00, 0x00],
        ',' => &[0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x18, 0x18, 0x10, 0x20, 0x00, 0x00],
        ':' => &[0x00, 0x18, 0x18, 0x00, 0x00, 0x18, 0x18, 0x00, 0x00, 0x00, 0x00, 0x00],
        ';' => &[0x00, 0x18, 0x18, 0x00, 0x00, 0x18, 0x18, 0x10, 0x20, 0x00, 0x00, 0x00],
        '-' => &[0x00, 0x00, 0x00, 0x7E, 0x7E, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00],
        '+' => &[0x00, 0x18, 0x18, 0x7E, 0x7E, 0x18, 0x18, 0x00, 0x00, 0x00, 0x00, 0x00],
        '=' => &[0x00, 0x7E, 0x00, 0x7E, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00],
        '(' => &[0x0C, 0x18, 0x30, 0x30, 0x30, 0x18, 0x0C, 0x00, 0x00, 0x00, 0x00, 0x00],
        ')' => &[0x30, 0x18, 0x0C, 0x0C, 0x0C, 0x18, 0x30, 0x00, 0x00, 0x00, 0x00, 0x00],
        '/' => &[0x06, 0x0C, 0x18, 0x30, 0x60, 0xC0, 0x80, 0x00, 0x00, 0x00, 0x00, 0x00],
        '%' => &[0xC6, 0xCC, 0x18, 0x30, 0x66, 0xC6, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00],
        '#' => &[0x6C, 0xFE, 0x6C, 0x6C, 0xFE, 0x6C, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00],
        '*' => &[0x66, 0x3C, 0xFF, 0x3C, 0x66, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00],
        '•' | '·' => &[0x00, 0x00, 0x3C, 0x7E, 0x7E, 0x3C, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00],
        '⚡' => &[0x0C, 0x1C, 0x3C, 0xFE, 0x0E, 0x1C, 0x18, 0x00, 0x00, 0x00, 0x00, 0x00],
        '✨' | '⭐' => &[0x18, 0x3C, 0xFF, 0x3C, 0x18, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00],
        '🎉' => &[0x18, 0x3C, 0x7E, 0xFF, 0x7E, 0x3C, 0x18, 0x00, 0x00, 0x00, 0x00, 0x00],
        '👉' | '>' => &[0x60, 0x30, 0x18, 0x0C, 0x18, 0x30, 0x60, 0x00, 0x00, 0x00, 0x00, 0x00],
        '<' => &[0x06, 0x0C, 0x18, 0x30, 0x18, 0x0C, 0x06, 0x00, 0x00, 0x00, 0x00, 0x00],
        '🔄' | '@' => &[0x3C, 0x66, 0x9E, 0xB6, 0x86, 0x66, 0x3C, 0x00, 0x00, 0x00, 0x00, 0x00],
        '_' => &[0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xFF],
        '|' => &[0x18, 0x18, 0x18, 0x18, 0x18, 0x18, 0x18, 0x18, 0x18, 0x18, 0x18, 0x18],
        '$' => &[0x18, 0x7C, 0xD8, 0x78, 0x1C, 0xD8, 0x7C, 0x18, 0x00, 0x00, 0x00, 0x00],
        '"' => &[0x66, 0x66, 0x66, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00],
        '\'' => &[0x18, 0x18, 0x18, 0x30, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00],
        '[' => &[0x3C, 0x30, 0x30, 0x30, 0x30, 0x30, 0x3C, 0x00, 0x00, 0x00, 0x00, 0x00],
        ']' => &[0x3C, 0x0C, 0x0C, 0x0C, 0x0C, 0x0C, 0x3C, 0x00, 0x00, 0x00, 0x00, 0x00],
        '{' => &[0x1C, 0x30, 0x30, 0x60, 0x30, 0x30, 0x1C, 0x00, 0x00, 0x00, 0x00, 0x00],
        '}' => &[0x38, 0x0C, 0x0C, 0x06, 0x0C, 0x0C, 0x38, 0x00, 0x00, 0x00, 0x00, 0x00],
        '&' => &[0x30, 0x48, 0x48, 0x30, 0x4A, 0x8C, 0x72, 0x00, 0x00, 0x00, 0x00, 0x00],
        '\\' => &[0x80, 0xC0, 0x60, 0x30, 0x18, 0x0C, 0x06, 0x00, 0x00, 0x00, 0x00, 0x00],
        '^' => &[0x18, 0x3C, 0x66, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00],
        '~' => &[0x00, 0x00, 0x72, 0x9C, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00],
        '🎚' | '🏷' | '📦' | '🦀' | '🔬' | '🔥' | '🌱' | '🚀' | '📊' | '✅' => &[0x18, 0x3C, 0x7E, 0xFF, 0x7E, 0x3C, 0x18, 0x00, 0x00, 0x00, 0x00, 0x00],
        '➕' => &[0x00, 0x18, 0x18, 0x7E, 0x7E, 0x18, 0x18, 0x00, 0x00, 0x00, 0x00, 0x00],
        '➖' => &[0x00, 0x00, 0x00, 0x7E, 0x7E, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00],
        ' ' => &[0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00],
        _ => &[0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00],
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use quick_core::geometry::Rect;

    #[test]
    fn test_software_rasterizer_clipping_and_translation() {
        let mut canvas = Canvas::new();
        canvas.clear(Color::BLACK);

        // Push clip (10..30, 10..30)
        canvas.push_clip(Rect::new(10.0, 10.0, 20.0, 20.0));
        // Fill rect (0..50, 0..50) with RED -> should only paint inside clip
        canvas.fill_rect(Rect::new(0.0, 0.0, 50.0, 50.0), Color::RED);
        canvas.pop_clip();

        // Translate (10, 10) and fill rect (0..5, 0..5) with GREEN -> at (10..15, 10..15)
        canvas.translate(10.0, 10.0);
        canvas.fill_rect(Rect::new(0.0, 0.0, 5.0, 5.0), Color::GREEN);

        let mut buffer = vec![0u32; 100 * 100];
        SoftwareRasterizer::render_to_buffer(&canvas, 100, 100, &mut buffer);

        // Outside clip (5, 5) must be BLACK
        assert_eq!(buffer[5 * 100 + 5], Color::BLACK.to_argb_u32());

        // Inside clip (12, 12) must be GREEN (overwriting RED due to translate)
        assert_eq!(buffer[12 * 100 + 12], Color::GREEN.to_argb_u32());

        // Inside clip but outside green (25, 25) must be RED
        assert_eq!(buffer[25 * 100 + 25], Color::RED.to_argb_u32());
    }

    #[test]
    fn test_software_rasterizer_scale_and_glyphs() {
        let mut canvas = Canvas::new();
        canvas.clear(Color::BLACK);

        // Scale by 2x
        canvas.scale(2.0, 2.0);
        canvas.fill_rect(Rect::new(5.0, 5.0, 10.0, 10.0), Color::WHITE);
        canvas.draw_text("$100 | _ok_", Point::new(0.0, 40.0), Color::RED, 14.0, None);

        let mut buffer = vec![0u32; 100 * 100];
        SoftwareRasterizer::render_to_buffer(&canvas, 100, 100, &mut buffer);

        // Rect at (5,5) size 10x10 scaled 2x -> (10..30, 10..30)
        assert_eq!(buffer[15 * 100 + 15], Color::WHITE.to_argb_u32());
        assert_eq!(buffer[28 * 100 + 28], Color::WHITE.to_argb_u32());
        assert_eq!(buffer[5 * 100 + 5], Color::BLACK.to_argb_u32());
    }
}
