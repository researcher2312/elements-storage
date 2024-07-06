use crate::element_types::*;

pub struct ElementStorage {
    resistors: Vec<Resistor>,
    capacitors: Vec<Capacitor>,
    inductors: Vec<Inductor>,
}

impl ElementStorage {
    pub fn new() -> ElementStorage {
        ElementStorage {
            resistors: Vec::new(),
            capacitors: Vec::new(),
            inductors: Vec::new(),
        }
    }

    pub fn add_element(&mut self, element_type: ElementType) {
        match element_type {
            ElementType::Resistor => {
                self.resistors
                    .push(Resistor::new(10.0, 0.25, 5, PackageType::THT));
            }
            ElementType::Capacitor => {
                self.capacitors
                    .push(Capacitor::new(20.0, 5.0, 2, PackageType::THT));
            }
            ElementType::Inductor => {
                self.inductors
                    .push(Inductor::new(15.0, 3.0, 4, PackageType::THT));
            }
        }
    }
}
