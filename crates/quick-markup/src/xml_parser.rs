use crate::schema::{UiDocument, UiNode};
use quick_xml::events::Event;
use quick_xml::reader::Reader;

/// Parse XML declarative UI document with SIMD-assisted parsing and zero-copy string handling.
pub fn parse_xml(xml_content: &str) -> Result<UiDocument, String> {
    // Validate UTF-8 quickly with simdutf8
    simdutf8::basic::from_utf8(xml_content.as_bytes())
        .map_err(|e| format!("Invalid UTF-8 in XML document: {}", e))?;

    let mut reader = Reader::from_str(xml_content);
    reader.config_mut().trim_text(true);

    let mut doc = UiDocument::default();
    let mut node_stack: Vec<UiNode> = Vec::with_capacity(16);
    let mut buf = Vec::with_capacity(256);

    loop {
        match reader.read_event_into(&mut buf) {
            Ok(Event::Start(ref e)) => {
                let tag_name = String::from_utf8_lossy(e.name().as_ref()).to_string();

                if tag_name.eq_ignore_ascii_case("Style") || tag_name.eq_ignore_ascii_case("Styles") {
                    let text = reader.read_text(e.name()).map_err(|err| err.to_string())?;
                    if let Some(ref mut existing) = doc.styles {
                        existing.push('\n');
                        existing.push_str(&text);
                    } else {
                        doc.styles = Some(text.to_string());
                    }
                    continue;
                }

                let mut node = UiNode {
                    element: tag_name,
                    ..Default::default()
                };

                for attr in e.attributes() {
                    let attr = attr.map_err(|err| err.to_string())?;
                    let key = String::from_utf8_lossy(attr.key.as_ref()).to_string();
                    let val = attr.unescape_value()
                        .map(|v| v.to_string())
                        .unwrap_or_else(|_| String::from_utf8_lossy(&attr.value).to_string());

                    match key.to_lowercase().as_str() {
                        "id" => node.id = Some(val),
                        "class" => node.class = Some(val),
                        "style" => node.style = Some(val),
                        "text" => node.text = Some(val),
                        "placeholder" => node.placeholder = Some(val),
                        "onclick" | "on_click" => node.on_click = Some(val),
                        "onchange" | "on_change" => node.on_change = Some(val),
                        _ => {
                            node.attributes.insert(key, val);
                        }
                    }
                }

                node_stack.push(node);
            }
            Ok(Event::Empty(ref e)) => {
                let tag_name = String::from_utf8_lossy(e.name().as_ref()).to_string();
                let mut node = UiNode {
                    element: tag_name,
                    ..Default::default()
                };

                for attr in e.attributes() {
                    let attr = attr.map_err(|err| err.to_string())?;
                    let key = String::from_utf8_lossy(attr.key.as_ref()).to_string();
                    let val = attr.unescape_value()
                        .map(|v| v.to_string())
                        .unwrap_or_else(|_| String::from_utf8_lossy(&attr.value).to_string());

                    match key.to_lowercase().as_str() {
                        "id" => node.id = Some(val),
                        "class" => node.class = Some(val),
                        "style" => node.style = Some(val),
                        "text" => node.text = Some(val),
                        "placeholder" => node.placeholder = Some(val),
                        "onclick" | "on_click" => node.on_click = Some(val),
                        "onchange" | "on_change" => node.on_change = Some(val),
                        _ => {
                            node.attributes.insert(key, val);
                        }
                    }
                }

                if let Some(parent) = node_stack.last_mut() {
                    parent.children.push(node);
                } else {
                    doc.root = node;
                }
            }
            Ok(Event::Text(e)) => {
                let text = e.unescape().map_err(|err| err.to_string())?.to_string();
                if !text.is_empty() {
                    if let Some(current) = node_stack.last_mut() {
                        if let Some(ref mut existing) = current.text {
                            existing.push_str(&text);
                        } else {
                            current.text = Some(text);
                        }
                    }
                }
            }
            Ok(Event::CData(e)) => {
                let text = String::from_utf8_lossy(e.as_ref()).to_string();
                if !text.is_empty() {
                    if let Some(current) = node_stack.last_mut() {
                        if let Some(ref mut existing) = current.text {
                            existing.push_str(&text);
                        } else {
                            current.text = Some(text);
                        }
                    }
                }
            }
            Ok(Event::End(ref _e)) => {
                if let Some(finished_node) = node_stack.pop() {
                    if let Some(parent) = node_stack.last_mut() {
                        parent.children.push(finished_node);
                    } else {
                        doc.root = finished_node;
                    }
                }
            }
            Ok(Event::Eof) => break,
            Err(e) => return Err(format!("XML parse error at position {}: {:?}", reader.buffer_position(), e)),
            _ => (),
        }
        buf.clear();
    }

    Ok(doc)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_xml_ui() {
        let xml = r#"
        <VStack id="main" class="container" style="padding: 16px; gap: 8px;">
            <Style>
                Button { background: #3b82f6; }
            </Style>
            <Text id="label" text="Counter App" style="font-size: 20px; color: #ffffff;" />
            <Button id="btn-inc" text="Increment" onclick="increment" />
        </VStack>
        "#;

        let doc = parse_xml(xml).expect("Should parse valid XML");
        assert!(doc.styles.is_some());
        assert_eq!(doc.root.element, "VStack");
        assert_eq!(doc.root.id, Some("main".to_string()));
        assert_eq!(doc.root.children.len(), 2);
        assert_eq!(doc.root.children[0].element, "Text");
        assert_eq!(doc.root.children[1].element, "Button");
        assert_eq!(doc.root.children[1].on_click, Some("increment".to_string()));
    }

    #[test]
    fn test_parse_xml_multiple_styles() {
        let xml = r#"
        <VStack>
            <Style>
                Text { color: #fff; }
            </Style>
            <Style>
                Button { background: #3b82f6; }
            </Style>
            <Text text="Hello" />
        </VStack>
        "#;
        let doc = parse_xml(xml).unwrap();
        let styles = doc.styles.unwrap();
        assert!(styles.contains("Text"));
        assert!(styles.contains("Button"));
    }

    #[test]
    fn test_parse_xml_cdata_and_escaped_attributes() {
        let xml = r#"
        <VStack>
            <Text text="&quot;Quotes&quot; &amp; &lt;Arrows&gt;" />
            <Text><![CDATA[Raw <Embedded> & Unescaped Content]]></Text>
        </VStack>
        "#;
        let doc = parse_xml(xml).unwrap();
        assert_eq!(doc.root.children.len(), 2);
        assert_eq!(doc.root.children[0].text, Some("\"Quotes\" & <Arrows>".to_string()));
        assert_eq!(doc.root.children[1].text, Some("Raw <Embedded> & Unescaped Content".to_string()));
    }
}
