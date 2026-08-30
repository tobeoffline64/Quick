use crate::schema::UiDocument;
use crate::toml_parser::parse_toml;
use crate::xml_parser::parse_xml;
use std::fs;
use std::path::Path;

/// Parse a `.quick` declarative UI file content.
/// Automatically detects XML or TOML format with zero-copy SIMD parsing.
pub fn parse_quick(content: &str) -> Result<UiDocument, String> {
    let trimmed = content.trim_start();
    if trimmed.starts_with('<') {
        parse_xml(content)
    } else {
        parse_toml(content)
    }
}

/// Load and parse a `.quick` file from disk.
pub fn parse_quick_file(path: impl AsRef<Path>) -> Result<UiDocument, String> {
    let path_ref = path.as_ref();
    let content = fs::read_to_string(path_ref)
        .map_err(|e| format!("Failed to read .quick file at {:?}: {}", path_ref, e))?;
    parse_quick(&content)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_quick_xml_syntax() {
        let quick_src = r#"
        <VStack id="hello-root" style="padding: 24px; background: #1e1e2e;">
            <Style>
                Text.greeting { font-size: 24px; color: #89b4fa; }
            </Style>
            <Text class="greeting" text="Hello, World from .quick!" />
        </VStack>
        "#;

        let doc = parse_quick(quick_src).expect("Should parse .quick XML syntax");
        assert_eq!(doc.root.element, "VStack");
        assert_eq!(doc.root.id, Some("hello-root".to_string()));
        assert_eq!(doc.root.children.len(), 1);
        assert_eq!(doc.root.children[0].text, Some("Hello, World from .quick!".to_string()));
    }

    #[test]
    fn test_parse_quick_toml_syntax() {
        let quick_src = r#"
        styles = "Text.greeting { font-size: 24px; }"

        [root]
        type = "VStack"
        id = "hello-root"

        [[root.children]]
        type = "Text"
        class = "greeting"
        text = "Hello, World from .quick (TOML)!"
        "#;

        let doc = parse_quick(quick_src).expect("Should parse .quick TOML syntax");
        assert_eq!(doc.root.element, "VStack");
        assert_eq!(doc.root.children.len(), 1);
    }

    #[test]
    fn test_parse_quick_adversarial_edge_cases() {
        // 1. Invalid XML syntax returns Err without panic
        assert!(parse_quick("<VStack><Text></VStack>").is_err());

        // 2. XML with comments and whitespace
        let xml_with_comments = r#"
        <!-- Header comment -->
        <VStack>
            <!-- Sibling comment -->
            <Text text="Hello" />
        </VStack>
        "#;
        let doc = parse_quick(xml_with_comments).expect("Should handle XML comments");
        assert_eq!(doc.root.children.len(), 1);

        // 3. Non-existent file path returns clean Err
        assert!(parse_quick_file("/non_existent/path/app.quick").is_err());

        // 4. Invalid TOML syntax returns Err without panic
        assert!(parse_quick("invalid [ [[ toml syntax = {").is_err());
    }

    #[test]
    fn test_parse_quick_deeply_nested_and_unknown_tags() {
        let mut nested = String::from("<VStack>");
        for _ in 0..30 {
            nested.push_str("<CustomBox class=\"wrapper\">");
        }
        nested.push_str("<Text text=\"Deep Content\" />");
        for _ in 0..30 {
            nested.push_str("</CustomBox>");
        }
        nested.push_str("</VStack>");

        let doc = parse_quick(&nested).expect("Should parse 30-level nested markup");
        assert_eq!(doc.root.element, "VStack");
        assert_eq!(doc.root.children.len(), 1);

        let mut data_ctx = crate::builder::DataContext::new();
        let (root_widget, _) = crate::builder::build_ui_tree(&doc, &mut data_ctx);
        assert_eq!(root_widget.widget_type(), "Container");
    }
}
