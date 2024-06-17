pub enum CapacitorType {
    Ceramic,
    Electrolytic,
    Tantalum,
}

pub enum PackageType {
    SMD,
    THT,
}

pub enum ElementType {
    Passive,
    Analog,
    Active,
    Digital,
}

pub struct Resistor {
    resistance: f32,
    power: f32,
    tolerance: i32,
    package_type: PackageType,
    element_type: ElementType,
}

impl Resistor {
    pub fn new(resistance: f32, power: f32, tolerance: i32, package_type: PackageType) -> Resistor {
        Resistor {
            resistance,
            power,
            tolerance,
            package_type,
            element_type: ElementType::Passive,
        }
    }

    fn get_package_type(package: &PackageType) -> &'static str {
        match package {
            PackageType::THT => "THT",
            PackageType::SMD => "SMD",
        }
    }

    pub fn print(&self) {
        let package_name = Resistor::get_package_type(&self.package_type);
        println!(
            "Resistor with {} Ω ± {}% in {} package. {} W",
            self.resistance, self.tolerance, package_name, self.power
        );
    }
}

pub struct Capacitor {
    capacitance: f32,
    voltage: f32,
    tolerance: i32,
    capacitor_type: CapacitorType,
    package: PackageType,
}

pub struct Inductor {
    inductance: f32,
    voltage: f32,
    package: PackageType,
}
