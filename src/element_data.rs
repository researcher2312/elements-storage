use crate::element_types::*;
use crate::unit_manager::UnitValue;
use csv::{ReaderBuilder, Writer};
use std::error::Error;
use std::fmt::format;
use std::fs::File;

pub struct ElementContainer<T: Element> {
    elements: Vec<T>,
    filename: String,
}

impl<T: Element + std::fmt::Debug> ElementContainer<T> {
    pub fn new() -> ElementContainer<T> {
        ElementContainer {
            elements: Vec::new(),
            filename: format!("{}.csv", T::get_filename()),
        }
    }

    pub fn add_element(&mut self, element: T) {
        self.elements.push(element);
    }

    pub fn print_elements(&self) {
        for element in &self.elements {
            element.print();
        }
    }

    pub fn get_elements_as_text(&self) -> String {
        let iter = self.elements.iter().map(|e| format!("{:?}", e));
        iter.collect::<Vec<String>>().join("\n")
    }

    pub fn export(&self) -> Result<(), Box<dyn Error>> {
        let file = File::create(&self.filename)?;
        let mut writer = Writer::from_writer(file);
        writer.write_record(T::get_header())?;
        for res in &self.elements {
            writer.write_record(res.export())?
        }
        Ok(())
    }

    pub fn load(&mut self) {
        let file_path = format!("{}.csv", T::get_filename());
        if let Ok(file) = File::open(file_path) {
            let mut reader = ReaderBuilder::new().has_headers(true).from_reader(file);
            for record in reader.records() {
                let record = record.unwrap();
                let elem = T::from_record(&record);
                self.add_element(elem);
            }
        }
    }
}

pub struct ElementStorage {
    resistors: ElementContainer<Resistor>,
    capacitors: ElementContainer<Capacitor>,
    inductors: ElementContainer<Inductor>,
}

impl ElementStorage {
    pub fn new() -> ElementStorage {
        ElementStorage {
            resistors: ElementContainer::new(),
            capacitors: ElementContainer::new(),
            inductors: ElementContainer::new(),
        }
    }

    pub fn print_all_elements(&self) {
        self.resistors.print_elements();
        self.capacitors.print_elements();
        self.inductors.print_elements();
    }

    pub fn load(&mut self) {
        self.resistors.load();
        self.capacitors.load();
        self.inductors.load();
    }

    pub fn export(&self) {
        self.resistors.export().unwrap();
        self.capacitors.export().unwrap();
        self.inductors.export().unwrap();
    }

    pub fn get_resistors(self) -> String {
        self.resistors.get_elements_as_text()
    }
}
