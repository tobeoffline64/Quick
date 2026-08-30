use crate::container::Container;
use quick_style::property::{AlignItems, FlexDirection};

pub struct VStack;

impl VStack {
    pub fn new() -> Container {
        let mut container = Container::new();
        container.style.flex_direction = Some(FlexDirection::Column);
        container.style.align_items = Some(AlignItems::Stretch);
        container
    }
}

pub struct HStack;

impl HStack {
    pub fn new() -> Container {
        let mut container = Container::new();
        container.style.flex_direction = Some(FlexDirection::Row);
        container.style.align_items = Some(AlignItems::Stretch);
        container
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use quick_style::property::FlexDirection;

    #[test]
    fn test_stack_directions() {
        let vstack = VStack::new();
        assert_eq!(vstack.style.flex_direction, Some(FlexDirection::Column));

        let hstack = HStack::new();
        assert_eq!(hstack.style.flex_direction, Some(FlexDirection::Row));
    }
}

