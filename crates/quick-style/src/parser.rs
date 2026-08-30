use crate::property::{AlignItems, Dimension, FlexDirection, JustifyContent, Style, TextAlignment};
use crate::rule::{StyleRule, StyleSheet};
use crate::selector::{PseudoState, Selector};
use memchr::{memchr, memchr2};
use quick_core::geometry::{BorderRadius, Color, Insets};

/// Parse inline CSS style string using SIMD-accelerated delimiter scanning.
pub fn parse_inline_style(input: &str) -> Style {
    let mut style = Style::default();
    let bytes = input.as_bytes();
    let mut start = 0;

    while start < bytes.len() {
        // Find next semicolon with SIMD memchr
        let len = match memchr(b';', &bytes[start..]) {
            Some(pos) => pos,
            None => bytes.len() - start,
        };

        let declaration = &input[start..start + len].trim();
        if !declaration.is_empty() {
            if let Some(colon_pos) = memchr(b':', declaration.as_bytes()) {
                let key = declaration[..colon_pos].trim();
                let val = declaration[colon_pos + 1..].trim();
                apply_property(&mut style, key, val);
            }
        }

        start += len + 1;
    }

    style
}

/// Parse a CSS stylesheet using SIMD delimiter matching.
pub fn parse_stylesheet(css: &str) -> StyleSheet {
    let mut sheet = StyleSheet::new();
    let bytes = css.as_bytes();
    let mut cursor = 0;

    while cursor < bytes.len() {
        if let Some(open_brace_rel) = memchr(b'{', &bytes[cursor..]) {
            let open_brace = cursor + open_brace_rel;
            let selector_str = css[cursor..open_brace].trim();
            cursor = open_brace + 1;

            if let Some(close_brace_rel) = memchr(b'}', &bytes[cursor..]) {
                let close_brace = cursor + close_brace_rel;
                let body = css[cursor..close_brace].trim();
                cursor = close_brace + 1;

                if !selector_str.is_empty() {
                    let selector = parse_selector(selector_str);
                    let style = parse_inline_style(body);
                    sheet.add_rule(selector, style);
                }
            } else {
                break;
            }
        } else {
            break;
        }
    }

    sheet
}

pub fn parse_selector(input: &str) -> Selector {
    let mut selector = Selector::default();
    let mut s = input.trim();

    if let Some(pseudo_idx) = memchr(b':', s.as_bytes()) {
        let pseudo = &s[pseudo_idx + 1..].trim();
        s = s[..pseudo_idx].trim();
        selector.pseudo_state = match *pseudo {
            "hover" => Some(PseudoState::Hover),
            "active" | "pressed" => Some(PseudoState::Active),
            "focus" | "focused" => Some(PseudoState::Focused),
            "disabled" => Some(PseudoState::Disabled),
            _ => None,
        };
    }

    if let Some(id_idx) = memchr(b'#', s.as_bytes()) {
        let id_part = &s[id_idx + 1..];
        s = &s[..id_idx];
        selector.id = Some(id_part.trim().to_string());
    }

    if let Some(cls_idx) = memchr(b'.', s.as_bytes()) {
        let cls_part = &s[cls_idx + 1..];
        s = &s[..cls_idx];
        selector.class = Some(cls_part.trim().to_string());
    }

    if !s.is_empty() {
        selector.element = Some(s.trim().to_string());
    }

    selector
}

fn apply_property(style: &mut Style, key: &str, val: &str) {
    match key {
        "width" => style.width = parse_dimension(val),
        "height" => style.height = parse_dimension(val),
        "min-width" => style.min_width = parse_dimension(val),
        "min-height" => style.min_height = parse_dimension(val),
        "max-width" => style.max_width = parse_dimension(val),
        "max-height" => style.max_height = parse_dimension(val),

        "padding" => style.padding = parse_insets(val),
        "margin" => style.margin = parse_insets(val),

        "flex-direction" => {
            style.flex_direction = match val {
                "row" => Some(FlexDirection::Row),
                "column" => Some(FlexDirection::Column),
                "row-reverse" => Some(FlexDirection::RowReverse),
                "column-reverse" => Some(FlexDirection::ColumnReverse),
                _ => None,
            };
        }
        "justify-content" => {
            style.justify_content = match val {
                "flex-start" | "start" => Some(JustifyContent::FlexStart),
                "center" => Some(JustifyContent::Center),
                "flex-end" | "end" => Some(JustifyContent::FlexEnd),
                "space-between" => Some(JustifyContent::SpaceBetween),
                "space-around" => Some(JustifyContent::SpaceAround),
                "space-evenly" => Some(JustifyContent::SpaceEvenly),
                _ => None,
            };
        }
        "align-items" => {
            style.align_items = match val {
                "flex-start" | "start" => Some(AlignItems::FlexStart),
                "center" => Some(AlignItems::Center),
                "flex-end" | "end" => Some(AlignItems::FlexEnd),
                "stretch" => Some(AlignItems::Stretch),
                "baseline" => Some(AlignItems::Baseline),
                _ => None,
            };
        }
        "gap" => {
            if let Ok(num) = val.trim_end_matches("px").trim().parse::<f32>() {
                style.gap = Some(num);
            }
        }

        "background" | "background-color" => {
            if let Ok(color) = Color::from_hex(val) {
                style.background_color = Some(color);
            }
        }
        "color" | "text-color" => {
            if let Ok(color) = Color::from_hex(val) {
                style.text_color = Some(color);
            }
        }
        "border-color" => {
            if let Ok(color) = Color::from_hex(val) {
                style.border_color = Some(color);
            }
        }
        "border-width" => {
            if let Ok(num) = val.trim_end_matches("px").trim().parse::<f32>() {
                style.border_width = Some(num);
            }
        }
        "border-radius" => {
            if let Ok(num) = val.trim_end_matches("px").trim().parse::<f32>() {
                style.border_radius = Some(BorderRadius::all(num));
            }
        }
        "opacity" => {
            if let Ok(num) = val.trim().parse::<f32>() {
                style.opacity = Some(num);
            }
        }
        "font-family" => {
            style.font_family = Some(val.trim_matches('\'').trim_matches('"').to_string());
        }
        "font-size" => {
            if let Ok(num) = val.trim_end_matches("px").trim().parse::<f32>() {
                style.font_size = Some(num);
            }
        }
        "font-weight" => {
            if let Ok(weight) = val.parse::<u16>() {
                style.font_weight = Some(weight);
            } else if val == "bold" {
                style.font_weight = Some(700);
            } else if val == "normal" {
                style.font_weight = Some(400);
            }
        }
        "text-align" => {
            style.text_align = match val {
                "left" => Some(TextAlignment::Left),
                "center" => Some(TextAlignment::Center),
                "right" => Some(TextAlignment::Right),
                "justify" => Some(TextAlignment::Justify),
                _ => None,
            };
        }
        _ => {}
    }
}

fn parse_dimension(val: &str) -> Option<Dimension> {
    let val = val.trim();
    if val == "auto" {
        Some(Dimension::Auto)
    } else if let Some(pct) = val.strip_suffix('%') {
        pct.trim().parse::<f32>().ok().map(Dimension::Percent)
    } else {
        val.trim_end_matches("px").trim().parse::<f32>().ok().map(Dimension::Px)
    }
}

fn parse_insets(val: &str) -> Option<Insets> {
    let parts: Vec<f32> = val
        .split_whitespace()
        .filter_map(|p| p.trim_end_matches("px").parse::<f32>().ok())
        .collect();

    match parts.len() {
        1 => Some(Insets::all(parts[0])),
        2 => Some(Insets::symmetric(parts[0], parts[1])),
        4 => Some(Insets::new(parts[0], parts[1], parts[2], parts[3])),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simd_css_parsing() {
        let css = r#"
            Button.primary {
                background: #3b82f6;
                padding: 8px 16px;
                border-radius: 6px;
            }
            Button.primary:hover {
                background: #2563eb;
            }
        "#;
        let sheet = parse_stylesheet(css);
        assert_eq!(sheet.rules.len(), 2);

        let style_normal = sheet.resolve("Button", &["primary"], None, None);
        assert_eq!(style_normal.background_color, Some(Color::from_hex("#3b82f6").unwrap()));
        assert_eq!(style_normal.padding, Some(Insets::symmetric(8.0, 16.0)));

        let style_hover = sheet.resolve("Button", &["primary"], None, Some(PseudoState::Hover));
        assert_eq!(style_hover.background_color, Some(Color::from_hex("#2563eb").unwrap()));
    }
}
