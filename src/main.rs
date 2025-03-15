mod element_data;
mod element_types;
mod unit_manager;

use element_data::ElementStorage;

fn main() {
    let mut storage = ElementStorage::new();
    storage.load();
    storage.print_all_elements();
    storage.export();
}
