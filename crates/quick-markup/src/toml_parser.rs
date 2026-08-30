use crate::schema::UiDocument;

pub fn parse_toml(toml_content: &str) -> Result<UiDocument, String> {
    toml::from_str::<UiDocument>(toml_content)
        .map_err(|err| format!("TOML parse error: {}", err))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_toml_ui() {
        let toml_str = r#"
        styles = "Button { background: #3b82f6; }"

        [root]
        type = "VStack"
        id = "main"
        style = "padding: 16px; gap: 8px;"

        [[root.children]]
        type = "Text"
        id = "counter-text"
        text = "Hello from TOML"

        [[root.children]]
        type = "Button"
        id = "inc-button"
        text = "Click Me"
        on_click = "increment"
        "#;

        let doc = parse_toml(toml_str).expect("Should parse valid TOML UI");
        assert!(doc.styles.is_some());
        assert_eq!(doc.root.element, "VStack");
        assert_eq!(doc.root.id, Some("main".to_string()));
        assert_eq!(doc.root.children.len(), 2);
        assert_eq!(doc.root.children[1].on_click, Some("increment".to_string()));
    }
}
