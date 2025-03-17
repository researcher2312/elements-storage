mod element_data;
mod element_types;
mod unit_manager;

use cursive::views::{Dialog, ListView, TextView};
use element_data::ElementStorage;

fn main() {
    let mut storage = ElementStorage::new();
    storage.load();
    storage.print_all_elements();
    storage.export();

    let mut siv = cursive::default();

    storage.display_on_list(&mut siv);

    // Starts the event loop.
    siv.run();
}
