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
                    .push(Resistor::new(10.0, 0.25, 5, Mounting::THT, Package::Axial));
            }
            ElementType::Capacitor => {
                self.capacitors
                    .push(Capacitor::new(20.0, 5.0, 2, Mounting::THT, Package::Axial));
            }
            ElementType::Inductor => {
                self.inductors
                    .push(Inductor::new(15.0, 3.0, 4, Mounting::THT, Package::Axial));
            }
        }
    }

    pub fn print_all_elements(&self) {
        for element in &self.resistors {
            element.print();
        }
        for element in &self.capacitors {
            element.print();
        }
        for element in &self.inductors {
            element.print()
        }
    }

    // pub fn export(&self) -> Result<(), Box<dyn Error>> {
    //     let file_path = "elements.csv";
    //     let file = File::open(file_path)?;
    //     let mut writer = Writer::from_writer(file)?;
    //     for res in &self.resistors {
    //         writer.write_record(&[res.export()])
    //     }
    //     Ok(())
    // }
}
