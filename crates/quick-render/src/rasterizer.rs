use crate::canvas::{Canvas, DrawCommand};
use fontdue::{Font, FontSettings};
use quick_core::geometry::{BorderRadius, Color, Insets, Point, Rect};
use quick_style::theme::tokens::Shadow;
use std::sync::OnceLock;

static DEFAULT_INTER_FONT: OnceLock<Font> = OnceLock::new();

fn get_default_font() -> &'static Font {
    DEFAULT_INTER_FONT.get_or_init(|| {
        let settings = FontSettings {
            scale: 40.0,
            ..FontSettings::default()
        };
        Font::from_bytes(quick_style::fonts::INTER_VARIABLE, settings)
            .or_else(|_| Font::from_bytes(quick_style::fonts::INTER_REGULAR, FontSettings::default()))
            .expect("Failed to initialize embedded Inter font")
    })
}

pub struct SoftwareRasterizer;

impl SoftwareRasterizer {
    /// Rasterize all Canvas draw commands into a 32-bit ARGB pixel buffer with high-definition anti-aliasing.
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
                DrawCommand::DrawShadow { rect, radius, shadow } => {
                    let scaled_rad = BorderRadius::new(
                        radius.top_left * sx.max(sy),
                        radius.top_right * sx.max(sy),
                        radius.bottom_right * sx.max(sy),
                        radius.bottom_left * sx.max(sy),
                    );
                    Self::draw_shadow(buffer, width, height, map_rect(rect), scaled_rad, *shadow, clip);
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
    fn set_pixel_alpha(buffer: &mut [u32], width: u32, height: u32, x: i32, y: i32, r_val: u8, g_val: u8, b_val: u8, alpha: u8, clip: Rect) {
        if alpha == 0 {
            return;
        }
        let xf = x as f32;
        let yf = y as f32;
        if xf >= clip.min_x() && xf < clip.max_x() && yf >= clip.min_y() && yf < clip.max_y() {
            if x >= 0 && x < width as i32 && y >= 0 && y < height as i32 {
                let idx = (y as u32 * width + x as u32) as usize;
                if alpha == 255 {
                    buffer[idx] = (0xFF << 24) | ((r_val as u32) << 16) | ((g_val as u32) << 8) | (b_val as u32);
                } else {
                    let dst = buffer[idx];
                    let dst_r = ((dst >> 16) & 0xFF) as u32;
                    let dst_g = ((dst >> 8) & 0xFF) as u32;
                    let dst_b = (dst & 0xFF) as u32;

                    let a = alpha as u32;
                    let inv_a = 255 - a;

                    let out_r = (r_val as u32 * a + dst_r * inv_a) / 255;
                    let out_g = (g_val as u32 * a + dst_g * inv_a) / 255;
                    let out_b = (b_val as u32 * a + dst_b * inv_a) / 255;

                    buffer[idx] = (0xFF << 24) | (out_r << 16) | (out_g << 8) | out_b;
                }
            }
        }
    }

    #[inline(always)]
    fn set_pixel(buffer: &mut [u32], width: u32, height: u32, x: i32, y: i32, color: Color, clip: Rect) {
        Self::set_pixel_alpha(buffer, width, height, x, y, color.r, color.g, color.b, color.a, clip);
    }

    fn fill_rect(buffer: &mut [u32], width: u32, height: u32, rect: Rect, color: Color, clip: Rect) {
        let x0 = rect.min_x().max(clip.min_x()).max(0.0) as i32;
        let y0 = rect.min_y().max(clip.min_y()).max(0.0) as i32;
        let x1 = rect.max_x().min(clip.max_x()).min(width as f32).ceil() as i32;
        let y1 = rect.max_y().min(clip.max_y()).min(height as f32).ceil() as i32;

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
        let max_r = (rect.size.width / 2.0).min(rect.size.height / 2.0);
        let r_tl = radius.top_left.min(max_r);
        let r_tr = radius.top_right.min(max_r);
        let r_br = radius.bottom_right.min(max_r);
        let r_bl = radius.bottom_left.min(max_r);

        if r_tl <= 0.5 && r_tr <= 0.5 && r_br <= 0.5 && r_bl <= 0.5 {
            Self::fill_rect(buffer, width, height, rect, color, clip);
            return;
        }

        let x0 = (rect.min_x() - 1.0).max(clip.min_x()).max(0.0) as i32;
        let y0 = (rect.min_y() - 1.0).max(clip.min_y()).max(0.0) as i32;
        let x1 = (rect.max_x() + 1.0).min(clip.max_x()).min(width as f32).ceil() as i32;
        let y1 = (rect.max_y() + 1.0).min(clip.max_y()).min(height as f32).ceil() as i32;

        for y in y0..y1 {
            let py = y as f32 + 0.5;
            for x in x0..x1 {
                let px = x as f32 + 0.5;

                // Check bounding box
                if px < rect.min_x() || px > rect.max_x() || py < rect.min_y() || py > rect.max_y() {
                    continue;
                }

                let in_tl = px < rect.min_x() + r_tl && py < rect.min_y() + r_tl;
                let in_tr = px > rect.max_x() - r_tr && py < rect.min_y() + r_tr;
                let in_br = px > rect.max_x() - r_br && py > rect.max_y() - r_br;
                let in_bl = px < rect.min_x() + r_bl && py > rect.max_y() - r_bl;

                let coverage = if in_tl && r_tl > 0.0 {
                    let dx = px - (rect.min_x() + r_tl);
                    let dy = py - (rect.min_y() + r_tl);
                    let dist = (dx * dx + dy * dy).sqrt() - r_tl;
                    (0.5 - dist).clamp(0.0, 1.0)
                } else if in_tr && r_tr > 0.0 {
                    let dx = px - (rect.max_x() - r_tr);
                    let dy = py - (rect.min_y() + r_tr);
                    let dist = (dx * dx + dy * dy).sqrt() - r_tr;
                    (0.5 - dist).clamp(0.0, 1.0)
                } else if in_br && r_br > 0.0 {
                    let dx = px - (rect.max_x() - r_br);
                    let dy = py - (rect.max_y() - r_br);
                    let dist = (dx * dx + dy * dy).sqrt() - r_br;
                    (0.5 - dist).clamp(0.0, 1.0)
                } else if in_bl && r_bl > 0.0 {
                    let dx = px - (rect.min_x() + r_bl);
                    let dy = py - (rect.max_y() - r_bl);
                    let dist = (dx * dx + dy * dy).sqrt() - r_bl;
                    (0.5 - dist).clamp(0.0, 1.0)
                } else {
                    1.0
                };

                if coverage > 0.005 {
                    let alpha = ((color.a as f32 * coverage) + 0.5) as u8;
                    Self::set_pixel_alpha(buffer, width, height, x, y, color.r, color.g, color.b, alpha, clip);
                }
            }
        }
    }

    fn stroke_rounded_rect(buffer: &mut [u32], width: u32, height: u32, rect: Rect, radius: BorderRadius, color: Color, stroke_w: f32, clip: Rect) {
        let sw = stroke_w.max(1.0);
        let max_r = (rect.size.width / 2.0).min(rect.size.height / 2.0);
        let r = radius.top_left.min(max_r);
        let r_inner = (r - sw).max(0.0);

        let x0 = (rect.min_x() - 1.0).max(clip.min_x()).max(0.0) as i32;
        let y0 = (rect.min_y() - 1.0).max(clip.min_y()).max(0.0) as i32;
        let x1 = (rect.max_x() + 1.0).min(clip.max_x()).min(width as f32).ceil() as i32;
        let y1 = (rect.max_y() + 1.0).min(clip.max_y()).min(height as f32).ceil() as i32;

        let inner_rect = rect.inset(Insets::all(sw));

        for y in y0..y1 {
            let py = y as f32 + 0.5;
            for x in x0..x1 {
                let px = x as f32 + 0.5;

                if px < rect.min_x() || px > rect.max_x() || py < rect.min_y() || py > rect.max_y() {
                    continue;
                }

                // Outer shape coverage
                let in_tl = px < rect.min_x() + r && py < rect.min_y() + r;
                let in_tr = px > rect.max_x() - r && py < rect.min_y() + r;
                let in_br = px > rect.max_x() - r && py > rect.max_y() - r;
                let in_bl = px < rect.min_x() + r && py > rect.max_y() - r;

                let outer_cov = if in_tl && r > 0.0 {
                    let dx = px - (rect.min_x() + r);
                    let dy = py - (rect.min_y() + r);
                    (0.5 - ((dx * dx + dy * dy).sqrt() - r)).clamp(0.0, 1.0)
                } else if in_tr && r > 0.0 {
                    let dx = px - (rect.max_x() - r);
                    let dy = py - (rect.min_y() + r);
                    (0.5 - ((dx * dx + dy * dy).sqrt() - r)).clamp(0.0, 1.0)
                } else if in_br && r > 0.0 {
                    let dx = px - (rect.max_x() - r);
                    let dy = py - (rect.max_y() - r);
                    (0.5 - ((dx * dx + dy * dy).sqrt() - r)).clamp(0.0, 1.0)
                } else if in_bl && r > 0.0 {
                    let dx = px - (rect.min_x() + r);
                    let dy = py - (rect.max_y() - r);
                    (0.5 - ((dx * dx + dy * dy).sqrt() - r)).clamp(0.0, 1.0)
                } else {
                    1.0
                };

                // Inner cutout coverage
                let inner_cov = if px >= inner_rect.min_x() && px <= inner_rect.max_x() && py >= inner_rect.min_y() && py <= inner_rect.max_y() {
                    let in_in_tl = px < inner_rect.min_x() + r_inner && py < inner_rect.min_y() + r_inner;
                    let in_in_tr = px > inner_rect.max_x() - r_inner && py < inner_rect.min_y() + r_inner;
                    let in_in_br = px > inner_rect.max_x() - r_inner && py > inner_rect.max_y() - r_inner;
                    let in_in_bl = px < inner_rect.min_x() + r_inner && py > inner_rect.max_y() - r_inner;

                    if in_in_tl && r_inner > 0.0 {
                        let dx = px - (inner_rect.min_x() + r_inner);
                        let dy = py - (inner_rect.min_y() + r_inner);
                        (0.5 - ((dx * dx + dy * dy).sqrt() - r_inner)).clamp(0.0, 1.0)
                    } else if in_in_tr && r_inner > 0.0 {
                        let dx = px - (inner_rect.max_x() - r_inner);
                        let dy = py - (inner_rect.min_y() + r_inner);
                        (0.5 - ((dx * dx + dy * dy).sqrt() - r_inner)).clamp(0.0, 1.0)
                    } else if in_in_br && r_inner > 0.0 {
                        let dx = px - (inner_rect.max_x() - r_inner);
                        let dy = py - (inner_rect.max_y() - r_inner);
                        (0.5 - ((dx * dx + dy * dy).sqrt() - r_inner)).clamp(0.0, 1.0)
                    } else if in_in_bl && r_inner > 0.0 {
                        let dx = px - (inner_rect.min_x() + r_inner);
                        let dy = py - (inner_rect.max_y() - r_inner);
                        (0.5 - ((dx * dx + dy * dy).sqrt() - r_inner)).clamp(0.0, 1.0)
                    } else {
                        1.0
                    }
                } else {
                    0.0
                };

                let ring_coverage = (outer_cov - inner_cov).clamp(0.0, 1.0);
                if ring_coverage > 0.005 {
                    let alpha = ((color.a as f32 * ring_coverage) + 0.5) as u8;
                    Self::set_pixel_alpha(buffer, width, height, x, y, color.r, color.g, color.b, alpha, clip);
                }
            }
        }
    }

    fn draw_shadow(
        buffer: &mut [u32],
        width: u32,
        height: u32,
        rect: Rect,
        radius: BorderRadius,
        shadow: Shadow,
        clip: Rect,
    ) {
        if shadow.color.a == 0 {
            return;
        }

        let offset_x = shadow.offset_x;
        let offset_y = shadow.offset_y;
        let spread = shadow.spread_radius;
        let blur = shadow.blur_radius.max(0.5);

        let shadow_rect = Rect::new(
            rect.origin.x + offset_x - spread,
            rect.origin.y + offset_y - spread,
            (rect.size.width + spread * 2.0).max(0.0),
            (rect.size.height + spread * 2.0).max(0.0),
        );

        let pad = blur * 2.5;
        let min_x = (shadow_rect.min_x() - pad).max(clip.min_x()).max(0.0) as i32;
        let min_y = (shadow_rect.min_y() - pad).max(clip.min_y()).max(0.0) as i32;
        let max_x = (shadow_rect.max_x() + pad).min(clip.max_x()).min(width as f32).ceil() as i32;
        let max_y = (shadow_rect.max_y() + pad).min(clip.max_y()).min(height as f32).ceil() as i32;

        let center_x = shadow_rect.origin.x + shadow_rect.size.width / 2.0;
        let center_y = shadow_rect.origin.y + shadow_rect.size.height / 2.0;
        let half_w = shadow_rect.size.width / 2.0;
        let half_h = shadow_rect.size.height / 2.0;
        let r = radius.top_left.min(half_w).min(half_h);

        let shadow_color = shadow.color;

        for y in min_y..max_y {
            let py = y as f32 + 0.5;
            let dy = (py - center_y).abs() - (half_h - r);

            for x in min_x..max_x {
                let px = x as f32 + 0.5;
                let dx = (px - center_x).abs() - (half_w - r);

                let dist = if dx > 0.0 && dy > 0.0 {
                    (dx * dx + dy * dy).sqrt() - r
                } else {
                    dx.max(dy) - r
                };

                let alpha_factor = if dist <= -blur {
                    1.0
                } else if dist >= blur {
                    0.0
                } else {
                    let t = (dist + blur) / (2.0 * blur);
                    (1.0 - t).clamp(0.0, 1.0)
                };

                if alpha_factor > 0.005 {
                    let pixel_alpha = ((shadow_color.a as f32) * alpha_factor).round() as u8;
                    if pixel_alpha > 0 {
                        Self::set_pixel_alpha(
                            buffer,
                            width,
                            height,
                            x,
                            y,
                            shadow_color.r,
                            shadow_color.g,
                            shadow_color.b,
                            pixel_alpha,
                            clip,
                        );
                    }
                }
            }
        }
    }

    fn draw_line(buffer: &mut [u32], width: u32, height: u32, start: Point, end: Point, color: Color, stroke_w: f32, clip: Rect) {
        let sw = stroke_w.max(1.0);
        let half_sw = sw / 2.0;

        let min_x = (start.x.min(end.x) - half_sw - 1.0).max(clip.min_x()).max(0.0) as i32;
        let min_y = (start.y.min(end.y) - half_sw - 1.0).max(clip.min_y()).max(0.0) as i32;
        let max_x = (start.x.max(end.x) + half_sw + 1.0).min(clip.max_x()).min(width as f32).ceil() as i32;
        let max_y = (start.y.max(end.y) + half_sw + 1.0).min(clip.max_y()).min(height as f32).ceil() as i32;

        let seg_dx = end.x - start.x;
        let seg_dy = end.y - start.y;
        let len_sq = seg_dx * seg_dx + seg_dy * seg_dy;

        for y in min_y..max_y {
            let py = y as f32 + 0.5;
            for x in min_x..max_x {
                let px = x as f32 + 0.5;

                let dist = if len_sq == 0.0 {
                    let dx = px - start.x;
                    let dy = py - start.y;
                    (dx * dx + dy * dy).sqrt()
                } else {
                    let t = (((px - start.x) * seg_dx + (py - start.y) * seg_dy) / len_sq).clamp(0.0, 1.0);
                    let proj_x = start.x + t * seg_dx;
                    let proj_y = start.y + t * seg_dy;
                    let dx = px - proj_x;
                    let dy = py - proj_y;
                    (dx * dx + dy * dy).sqrt()
                };

                let coverage = (0.5 - (dist - half_sw)).clamp(0.0, 1.0);
                if coverage > 0.005 {
                    let alpha = ((color.a as f32 * coverage) + 0.5) as u8;
                    Self::set_pixel_alpha(buffer, width, height, x, y, color.r, color.g, color.b, alpha, clip);
                }
            }
        }
    }

    fn draw_text(buffer: &mut [u32], width: u32, height: u32, text: &str, origin: Point, color: Color, font_size: f32, clip: Rect) {
        let font = get_default_font();
        let scale = font_size.max(8.0);
        let line_height = scale * 1.35;

        let mut cur_x = origin.x;
        let mut cur_y = origin.y;
        let start_x = cur_x;

        for ch in text.chars() {
            if ch == '\n' {
                cur_x = start_x;
                cur_y += line_height;
                continue;
            }

            // Check if font has the glyph
            let (metrics, bitmap) = font.rasterize(ch, scale);
            if metrics.width > 0 && metrics.height > 0 && !bitmap.is_empty() {
                let glyph_x0 = (cur_x + metrics.xmin as f32).round() as i32;
                // In fontdue, ymin is distance from baseline to bottom of glyph.
                // Origin.y is passed as baseline y coordinate.
                let glyph_y0 = (cur_y - metrics.height as f32 - metrics.ymin as f32).round() as i32;

                for gy in 0..metrics.height {
                    let py = glyph_y0 + gy as i32;
                    for gx in 0..metrics.width {
                        let px = glyph_x0 + gx as i32;
                        let alpha_coverage = bitmap[gy * metrics.width + gx];
                        if alpha_coverage > 0 {
                            let effective_alpha = ((color.a as u32 * alpha_coverage as u32) / 255) as u8;
                            Self::set_pixel_alpha(buffer, width, height, px, py, color.r, color.g, color.b, effective_alpha, clip);
                        }
                    }
                }
                cur_x += metrics.advance_width;
            } else if ch == ' ' {
                cur_x += scale * 0.28;
            } else {
                // Symbol / emoji fallback
                let s = (scale / 14.0).round().max(1.0) as i32;
                Self::draw_char_bitmap(buffer, width, height, ch, cur_x as i32, (cur_y - scale * 0.85) as i32, color, s, clip);
                cur_x += (8 * s) as f32;
            }
        }
    }

    fn draw_char_bitmap(
        buffer: &mut [u32],
        width: u32,
        height: u32,
        ch: char,
        x0: i32,
        y0: i32,
        color: Color,
        scale: i32,
        clip: Rect,
    ) {
        let bitmap = get_fallback_glyph_bitmap(ch);
        for row in 0..12 {
            let row_byte = bitmap[row];
            for col in 0..8 {
                if (row_byte & (0x80 >> col)) != 0 {
                    for dy in 0..scale {
                        for dx in 0..scale {
                            let px = x0 + col * scale + dx;
                            let py = y0 + (row as i32) * scale + dy;
                            Self::set_pixel(buffer, width, height, px, py, color, clip);
                        }
                    }
                }
            }
        }
    }
}

fn get_fallback_glyph_bitmap(c: char) -> &'static [u8; 12] {
    match c {
        '⚡' => &[0x0C, 0x1C, 0x3C, 0xFE, 0x0E, 0x1C, 0x18, 0x00, 0x00, 0x00, 0x00, 0x00],
        '✨' | '⭐' => &[0x18, 0x3C, 0xFF, 0x3C, 0x18, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00],
        '🎉' => &[0x18, 0x3C, 0x7E, 0xFF, 0x7E, 0x3C, 0x18, 0x00, 0x00, 0x00, 0x00, 0x00],
        '👉' | '>' => &[0x60, 0x30, 0x18, 0x0C, 0x18, 0x30, 0x60, 0x00, 0x00, 0x00, 0x00, 0x00],
        '<' => &[0x06, 0x0C, 0x18, 0x30, 0x18, 0x0C, 0x06, 0x00, 0x00, 0x00, 0x00, 0x00],
        '▼' | '▾' => &[0x00, 0x00, 0x7E, 0x3C, 0x18, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00],
        '▶' | '▸' => &[0x00, 0x18, 0x38, 0x78, 0x38, 0x18, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00],
        '−' | '-' => &[0x00, 0x00, 0x00, 0x7E, 0x7E, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00],
        '+' => &[0x00, 0x18, 0x18, 0x7E, 0x7E, 0x18, 0x18, 0x00, 0x00, 0x00, 0x00, 0x00],
        '✓' | '✔' => &[0x00, 0x02, 0x06, 0x8C, 0xD8, 0x70, 0x20, 0x00, 0x00, 0x00, 0x00, 0x00],
        '•' | '·' => &[0x00, 0x00, 0x3C, 0x7E, 0x7E, 0x3C, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00],
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

        canvas.push_clip(Rect::new(10.0, 10.0, 20.0, 20.0));
        canvas.fill_rect(Rect::new(0.0, 0.0, 50.0, 50.0), Color::RED);
        canvas.pop_clip();

        canvas.translate(10.0, 10.0);
        canvas.fill_rect(Rect::new(0.0, 0.0, 5.0, 5.0), Color::GREEN);

        let mut buffer = vec![0u32; 100 * 100];
        SoftwareRasterizer::render_to_buffer(&canvas, 100, 100, &mut buffer);

        assert_eq!(buffer[5 * 100 + 5], Color::BLACK.to_argb_u32());
        assert_eq!(buffer[12 * 100 + 12], Color::GREEN.to_argb_u32());
        assert_eq!(buffer[25 * 100 + 25], Color::RED.to_argb_u32());
    }

    #[test]
    fn test_software_rasterizer_antialiased_rounded_rect() {
        let mut canvas = Canvas::new();
        canvas.clear(Color::BLACK);
        canvas.fill_rounded_rect(Rect::new(10.0, 10.0, 80.0, 80.0), BorderRadius::all(16.0), Color::WHITE);

        let mut buffer = vec![0u32; 100 * 100];
        SoftwareRasterizer::render_to_buffer(&canvas, 100, 100, &mut buffer);

        // Center must be white
        assert_eq!(buffer[50 * 100 + 50], Color::WHITE.to_argb_u32());
        // Far outside must be black
        assert_eq!(buffer[2 * 100 + 2], Color::BLACK.to_argb_u32());
    }

    #[test]
    fn test_software_rasterizer_hd_text_rendering() {
        let mut canvas = Canvas::new();
        canvas.clear(Color::BLACK);
        canvas.draw_text("Quick Framework", Point::new(10.0, 30.0), Color::WHITE, 16.0, None);

        let mut buffer = vec![0u32; 200 * 50];
        SoftwareRasterizer::render_to_buffer(&canvas, 200, 50, &mut buffer);

        // Some pixels must be non-black (rendered text glyphs)
        let non_black_count = buffer.iter().filter(|&&p| p != Color::BLACK.to_argb_u32()).count();
        assert!(non_black_count > 50, "text glyphs should render non-black pixels, found {}", non_black_count);
    }
}
