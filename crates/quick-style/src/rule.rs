use std::collections::HashMap;
use crate::property::Style;
use crate::selector::{PseudoState, Selector};
use serde::{Deserialize, Serialize};
use serde::de::DeserializeOwned;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StyleRule {
    pub selector: Selector,
    pub style: Style,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ResourceDictionary {
    pub values: HashMap<String, serde_json::Value>,
}

impl ResourceDictionary {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set<T: Serialize>(&mut self, key: impl Into<String>, val: T) {
        if let Ok(val) = serde_json::to_value(val) {
            self.values.insert(key.into(), val);
        }
    }

    pub fn get<T: DeserializeOwned>(&self, key: &str) -> Option<T> {
        self.values.get(key).and_then(|v| serde_json::from_value(v.clone()).ok())
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct StyleSheet {
    pub rules: Vec<StyleRule>,
    pub resources: ResourceDictionary,
}

impl StyleSheet {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_rule(&mut self, selector: Selector, style: Style) {
        self.rules.push(StyleRule { selector, style });
    }

    /// Resolve effective style based on element type, class list, ID and pseudo-state.
    pub fn resolve(
        &self,
        element: &str,
        classes: &[&str],
        id: Option<&str>,
        state: Option<PseudoState>,
    ) -> Style {
        let mut matched: Vec<(&StyleRule, u32)> = self
            .rules
            .iter()
            .filter(|rule| rule.selector.matches(element, classes, id, state))
            .map(|rule| (rule, rule.selector.specificity()))
            .collect();

        // Sort by specificity ascending so higher specificity rules override earlier ones
        matched.sort_by_key(|(_, spec)| *spec);

        let mut computed = Style::default();
        for (rule, _) in matched {
            computed.merge_with(&rule.style);
        }
        computed
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_resource_dictionary() {
        let mut dict = ResourceDictionary::new();
        dict.set("font_size", 16.0f32);
        dict.set("theme", "dark".to_string());

        assert_eq!(dict.get::<f32>("font_size"), Some(16.0));
        assert_eq!(dict.get::<String>("theme"), Some("dark".to_string()));
        assert_eq!(dict.get::<i32>("non_existent"), None);
    }
}
