mod element_data;
mod element_types;

use element_data::ElementStorage;
use element_types::{Element, ElementType};

fn main() {
    let mut storage = ElementStorage::new();
    storage.add_element(ElementType::Resistor)
}
