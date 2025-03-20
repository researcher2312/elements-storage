#!allow[unused]
mod element_data;
mod element_types;
mod unit_manager;

use element_data::ElementStorage;
use iocraft::prelude::*;

fn main() {
    let mut storage = ElementStorage::new();
    storage.load();
    storage.export();

    let mut elem = element! {
        View(
            border_style: BorderStyle::Round,
            border_color: Color::Blue,
        ) {
            Text(content: storage.get_resistors())
        }
    };

    smol::block_on(elem.render_loop()).unwrap()
}
