use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PseudoState {
    Hover,
    Active,
    Focused,
    Disabled,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
pub struct Selector {
    pub element: Option<String>,
    pub class: Option<String>,
    pub id: Option<String>,
    pub pseudo_state: Option<PseudoState>,
    pub attribute: Option<(String, String)>,
}

impl Selector {
    pub fn matches(
        &self,
        element: &str,
        classes: &[&str],
        id: Option<&str>,
        state: Option<PseudoState>,
    ) -> bool {
        self.matches_with_attrs(element, classes, id, state, None)
    }

    pub fn matches_with_attrs(
        &self,
        element: &str,
        classes: &[&str],
        id: Option<&str>,
        state: Option<PseudoState>,
        attributes: Option<&std::collections::HashMap<String, String>>,
    ) -> bool {
        if let Some(ref elem) = self.element {
            if elem != element && elem != "*" {
                return false;
            }
        }
        if let Some(ref cls) = self.class {
            if !classes.iter().any(|c| *c == cls) {
                return false;
            }
        }
        if let Some(ref self_id) = self.id {
            if id != Some(self_id.as_str()) {
                return false;
            }
        }
        if self.pseudo_state.is_some() && self.pseudo_state != state {
            return false;
        }
        if let Some((ref attr_key, ref attr_val)) = self.attribute {
            if let Some(attrs) = attributes {
                if let Some(val) = attrs.get(attr_key) {
                    if !attr_val.is_empty() && val.to_lowercase() != attr_val.to_lowercase() {
                        return false;
                    }
                } else {
                    return false;
                }
            } else {
                return false;
            }
        }
        true
    }

    pub fn specificity(&self) -> u32 {
        let mut score = 0;
        if self.id.is_some() {
            score += 100;
        }
        if self.class.is_some() {
            score += 10;
        }
        if self.attribute.is_some() {
            score += 10;
        }
        if self.pseudo_state.is_some() {
            score += 10;
        }
        if self.element.is_some() {
            score += 1;
        }
        score
    }
}
