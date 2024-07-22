mod element_data;
mod element_types;

use element_data::ElementStorage;
use element_types::ElementType;

fn main() {
    let mut storage = ElementStorage::new();
    storage.add_element(ElementType::Resistor);
    storage.add_element(ElementType::Resistor);
    storage.add_element(ElementType::Capacitor);
    storage.add_element(ElementType::Inductor);
    storage.print_all_elements();
    storage.export().expect("file error");
}
