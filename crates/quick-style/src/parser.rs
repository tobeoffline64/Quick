use crate::property::{AlignItems, Dimension, FlexDirection, JustifyContent, Style, TextAlignment};
use crate::rule::StyleSheet;
use crate::selector::{PseudoState, Selector};
use memchr::memchr;
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

fn strip_css_comments(input: &str) -> String {
    let mut output = String::with_capacity(input.len());
    let mut chars = input.chars().peekable();
    while let Some(ch) = chars.next() {
        if ch == '/' && chars.peek() == Some(&'*') {
            chars.next(); // consume '*'
            while let Some(c) = chars.next() {
                if c == '*' && chars.peek() == Some(&'/') {
                    chars.next(); // consume '/'
                    break;
                }
            }
        } else {
            output.push(ch);
        }
    }
    output
}

/// Parse a CSS stylesheet using SIMD delimiter matching.
pub fn parse_stylesheet(css: &str) -> StyleSheet {
    let clean_css = strip_css_comments(css);
    let mut sheet = StyleSheet::new();
    let bytes = clean_css.as_bytes();
    let mut cursor = 0;

    while cursor < bytes.len() {
        if let Some(open_brace_rel) = memchr(b'{', &bytes[cursor..]) {
            let open_brace = cursor + open_brace_rel;
            let selector_str = clean_css[cursor..open_brace].trim();
            cursor = open_brace + 1;

            if let Some(close_brace_rel) = memchr(b'}', &bytes[cursor..]) {
                let close_brace = cursor + close_brace_rel;
                let body = clean_css[cursor..close_brace].trim();
                cursor = close_brace + 1;

                if !selector_str.is_empty() {
                    let style = parse_inline_style(body);
                    for single_selector in selector_str.split(',') {
                        let s = single_selector.trim();
                        if !s.is_empty() {
                            let selector = parse_selector(s);
                            sheet.add_rule(selector, style.clone());
                        }
                    }
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
        let pseudo = s[pseudo_idx + 1..].trim();
        s = s[..pseudo_idx].trim();
        selector.pseudo_state = match pseudo {
            "hover" => Some(PseudoState::Hover),
            "active" | "pressed" => Some(PseudoState::Active),
            "focus" | "focused" => Some(PseudoState::Focused),
            "disabled" => Some(PseudoState::Disabled),
            _ => None,
        };
    }

    let dot_pos = memchr(b'.', s.as_bytes());
    let hash_pos = memchr(b'#', s.as_bytes());

    let (elem_str, class_str, id_str) = match (dot_pos, hash_pos) {
        (Some(d), Some(h)) if d < h => (
            &s[..d],
            Some(&s[d + 1..h]),
            Some(&s[h + 1..]),
        ),
        (Some(d), Some(h)) => (
            &s[..h],
            Some(&s[d + 1..]),
            Some(&s[h + 1..d]),
        ),
        (Some(d), None) => (
            &s[..d],
            Some(&s[d + 1..]),
            None,
        ),
        (None, Some(h)) => (
            &s[..h],
            None,
            Some(&s[h + 1..]),
        ),
        (None, None) => (
            s,
            None,
            None,
        ),
    };

    let elem_trimmed = elem_str.trim();
    if !elem_trimmed.is_empty() {
        selector.element = Some(elem_trimmed.to_string());
    }

    if let Some(cls) = class_str {
        let cls_trimmed = cls.trim();
        if !cls_trimmed.is_empty() {
            selector.class = Some(cls_trimmed.to_string());
        }
    }

    if let Some(id) = id_str {
        let id_trimmed = id.trim();
        if !id_trimmed.is_empty() {
            selector.id = Some(id_trimmed.to_string());
        }
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
        "padding-top" => {
            if let Ok(num) = val.trim_end_matches("px").trim().parse::<f32>() {
                style.padding.get_or_insert_with(Insets::default).top = num;
            }
        }
        "padding-right" => {
            if let Ok(num) = val.trim_end_matches("px").trim().parse::<f32>() {
                style.padding.get_or_insert_with(Insets::default).right = num;
            }
        }
        "padding-bottom" => {
            if let Ok(num) = val.trim_end_matches("px").trim().parse::<f32>() {
                style.padding.get_or_insert_with(Insets::default).bottom = num;
            }
        }
        "padding-left" => {
            if let Ok(num) = val.trim_end_matches("px").trim().parse::<f32>() {
                style.padding.get_or_insert_with(Insets::default).left = num;
            }
        }

        "margin" => style.margin = parse_insets(val),
        "margin-top" => {
            if let Ok(num) = val.trim_end_matches("px").trim().parse::<f32>() {
                style.margin.get_or_insert_with(Insets::default).top = num;
            }
        }
        "margin-right" => {
            if let Ok(num) = val.trim_end_matches("px").trim().parse::<f32>() {
                style.margin.get_or_insert_with(Insets::default).right = num;
            }
        }
        "margin-bottom" => {
            if let Ok(num) = val.trim_end_matches("px").trim().parse::<f32>() {
                style.margin.get_or_insert_with(Insets::default).bottom = num;
            }
        }
        "margin-left" => {
            if let Ok(num) = val.trim_end_matches("px").trim().parse::<f32>() {
                style.margin.get_or_insert_with(Insets::default).left = num;
            }
        }

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
        "border" => {
            let parts: Vec<&str> = val.split_whitespace().collect();
            for part in parts {
                if let Ok(w) = part.trim_end_matches("px").parse::<f32>() {
                    style.border_width = Some(w);
                } else if let Ok(color) = Color::from_hex(part) {
                    style.border_color = Some(color);
                }
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
        3 => Some(Insets::new(parts[0], parts[1], parts[2], parts[1])),
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

    #[test]
    fn test_composite_selectors() {
        let s1 = parse_selector("Button.btn-primary:hover");
        assert_eq!(s1.element, Some("Button".to_string()));
        assert_eq!(s1.class, Some("btn-primary".to_string()));
        assert_eq!(s1.pseudo_state, Some(PseudoState::Hover));

        let s2 = parse_selector("Container.card#main");
        assert_eq!(s2.element, Some("Container".to_string()));
        assert_eq!(s2.class, Some("card".to_string()));
        assert_eq!(s2.id, Some("main".to_string()));

        let s3 = parse_selector(".badge");
        assert_eq!(s3.element, None);
        assert_eq!(s3.class, Some("badge".to_string()));

        let s4 = parse_selector("#header");
        assert_eq!(s4.element, None);
        assert_eq!(s4.id, Some("header".to_string()));
    }

    #[test]
    fn test_property_parsing() {
        let inline = "width: 500px; height: 50%; opacity: 0.85; gap: 16px; margin: 10px 20px; border: 2px solid #30363d;";
        let style = parse_inline_style(inline);

        assert_eq!(style.width, Some(Dimension::Px(500.0)));
        assert_eq!(style.height, Some(Dimension::Percent(50.0)));
        assert_eq!(style.opacity, Some(0.85));
        assert_eq!(style.gap, Some(16.0));
        assert_eq!(style.margin, Some(Insets::symmetric(10.0, 20.0)));
        assert_eq!(style.border_width, Some(2.0));
        assert_eq!(style.border_color, Some(Color::from_hex("#30363d").unwrap()));
    }

    #[test]
    fn test_insets_three_values() {
        let inline = "margin: 10px 20px 30px;";
        let style = parse_inline_style(inline);
        assert_eq!(style.margin, Some(Insets::new(10.0, 20.0, 30.0, 20.0)));
    }

    #[test]
    fn test_individual_insets() {
        let inline = "padding-top: 15px; padding-left: 25px; margin-bottom: 35px;";
        let style = parse_inline_style(inline);
        assert_eq!(style.padding.unwrap().top, 15.0);
        assert_eq!(style.padding.unwrap().left, 25.0);
        assert_eq!(style.margin.unwrap().bottom, 35.0);
    }

    #[test]
    fn test_css_comments_and_comma_selectors() {
        let css = r#"
            /* General styling */
            Button.primary, Button.secondary {
                padding: 12px;
                /* Inside rule comment */
                border-radius: 4px;
            }
        "#;
        let sheet = parse_stylesheet(css);
        assert_eq!(sheet.rules.len(), 2);

        let p = sheet.resolve("Button", &["primary"], None, None);
        assert_eq!(p.padding, Some(Insets::all(12.0)));

        let s = sheet.resolve("Button", &["secondary"], None, None);
        assert_eq!(s.padding, Some(Insets::all(12.0)));
    }
}
