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
}

impl Selector {
    pub fn matches(
        &self,
        element: &str,
        classes: &[&str],
        id: Option<&str>,
        state: Option<PseudoState>,
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
        if self.pseudo_state.is_some() {
            score += 10;
        }
        if self.element.is_some() {
            score += 1;
        }
        score
    }
}
