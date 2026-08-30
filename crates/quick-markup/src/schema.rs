use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UiNode {
    #[serde(rename = "type", default = "default_element")]
    pub element: String,
    pub id: Option<String>,
    pub class: Option<String>,
    pub style: Option<String>,
    pub text: Option<String>,
    pub placeholder: Option<String>,
    pub on_click: Option<String>,
    pub on_change: Option<String>,
    #[serde(default)]
    pub attributes: HashMap<String, String>,
    #[serde(default)]
    pub children: Vec<UiNode>,
}

fn default_element() -> String {
    "Container".to_string()
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UiDocument {
    pub styles: Option<String>,
    pub root: UiNode,
}
