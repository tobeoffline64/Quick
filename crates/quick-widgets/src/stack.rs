use crate::container::Container;
use quick_style::property::FlexDirection;

pub struct VStack;

impl VStack {
    pub fn new() -> Container {
        let mut container = Container::new();
        container.style.flex_direction = Some(FlexDirection::Column);
        container
    }
}

pub struct HStack;

impl HStack {
    pub fn new() -> Container {
        let mut container = Container::new();
        container.style.flex_direction = Some(FlexDirection::Row);
        container
    }
}
