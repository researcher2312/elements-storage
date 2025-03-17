use crate::unit_manager::UnitValue;

use std::str::FromStr;

#[derive(Debug)]
pub enum CapacitorType {
    Ceramic,
    Electrolytic,
    Tantalum,
}

#[derive(Debug)]
pub enum Package {
    Axial,
    DIP(i32),
    SOT(i32),
}

impl Package {
    fn get_name(&self) -> String {
        match self {
            Package::Axial => String::from_str("Axial").unwrap(),
            Package::DIP(number) => format!["DIP-{}", &number.to_string()],
            Package::SOT(number) => format!["SOT-{}", &number.to_string()],
        }
    }
}

#[derive(Debug)]
pub enum Mounting {
    SMD,
    THT,
}

impl Mounting {
    fn get_name(&self) -> &'static str {
        match self {
            Mounting::SMD => "SMD",
            Mounting::THT => "THT",
        }
    }
}

impl FromStr for Mounting {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "SMD" => Ok(Mounting::SMD),
            "THT" => Ok(Mounting::THT),
            _ => Err(()),
        }
    }
}

pub enum ElementFeature {
    Resistance(UnitValue),
    Power(UnitValue),
    Tolerance(i32),
    Package(Package),
    Mounting(Mounting),
    Capacitance(UnitValue),
    Voltage(UnitValue),
    Inductance(UnitValue),
}

pub enum ElementType {
    Resistor,
    Capacitor,
    Inductor,
}

pub trait Element {
    fn get_type(&self) -> ElementType;
    fn get_filename() -> &'static str;
    fn get_header() -> Vec<&'static str>;
    fn print(&self);
    fn export(&self) -> Vec<String>;
    fn from_record(record: &csv::StringRecord) -> Self;
}

#[derive(Debug)]
pub struct Resistor {
    resistance: f32,
    power: f32,
    tolerance: i32,
    mounting: Mounting,
    package: Package,
}

impl Resistor {
    pub fn new(
        resistance: f32,
        power: f32,
        tolerance: i32,
        mounting: Mounting,
        package: Package,
    ) -> Resistor {
        Resistor {
            resistance,
            power,
            tolerance,
            mounting,
            package,
        }
    }
}

impl Element for Resistor {
    fn print(&self) {
        let mounting_name = self.mounting.get_name();
        let package_name = self.package.get_name();
        println!(
            "─⊏⊐─ {} Ω ± {}% {} W resistor in {} {} package",
            self.resistance, self.tolerance, self.power, mounting_name, package_name
        );
    }

    fn export(&self) -> Vec<String> {
        let vector = vec![
            self.resistance.to_string(),
            self.power.to_string(),
            self.tolerance.to_string(),
            self.mounting.get_name().to_owned(),
            self.package.get_name().to_owned(),
        ];
        return vector;
    }

    fn get_type(&self) -> ElementType {
        return ElementType::Resistor;
    }

    fn get_header() -> Vec<&'static str> {
        return vec!["resistance", "power", "tolerance", "mounting", "package"];
    }

    fn get_filename() -> &'static str {
        return "resistors";
    }

    fn from_record(record: &csv::StringRecord) -> Resistor {
        Resistor::new(
            record[0].parse().unwrap(),
            record[1].parse().unwrap(),
            record[2].parse().unwrap(),
            Mounting::from_str(&record[3]).unwrap(),
            Package::Axial,
        )
    }
}

#[derive(Debug)]
pub struct Capacitor {
    capacitance: f32,
    voltage: f32,
    tolerance: i32,
    capacitor_type: CapacitorType,
    mounting: Mounting,
    package: Package,
}

impl Capacitor {
    pub fn new(
        capacitance: f32,
        voltage: f32,
        tolerance: i32,
        mounting: Mounting,
        package: Package,
    ) -> Capacitor {
        Capacitor {
            capacitance,
            voltage,
            tolerance,
            capacitor_type: CapacitorType::Ceramic,
            mounting,
            package,
        }
    }
}

impl Element for Capacitor {
    fn print(&self) {
        let mounting_name = self.mounting.get_name();
        let package_name = self.package.get_name();
        println!(
            "─┤├─ {} F ± {}% {} V capacitor in {} {} package",
            self.capacitance, self.tolerance, self.voltage, mounting_name, package_name
        );
    }

    fn get_type(&self) -> ElementType {
        return ElementType::Capacitor;
    }

    fn get_filename() -> &'static str {
        return "capacitors";
    }

    fn get_header() -> Vec<&'static str> {
        return vec!["capacitance", "voltage", "tolerance", "mounting", "package"];
    }

    fn export(&self) -> Vec<String> {
        let vector = vec![
            self.capacitance.to_string(),
            self.voltage.to_string(),
            self.tolerance.to_string(),
            self.mounting.get_name().to_owned(),
            self.package.get_name().to_owned(),
        ];
        vector
    }

    fn from_record(record: &csv::StringRecord) -> Capacitor {
        Capacitor::new(
            record[0].parse().unwrap(),
            record[1].parse().unwrap(),
            record[2].parse().unwrap(),
            Mounting::from_str(&record[3]).unwrap(),
            Package::Axial,
        )
    }
}

#[derive(Debug)]
pub struct Inductor {
    inductance: f32,
    voltage: f32,
    tolerance: i32,
    mounting: Mounting,
    package: Package,
}

impl Inductor {
    pub fn new(
        inductance: f32,
        voltage: f32,
        tolerance: i32,
        mounting: Mounting,
        package: Package,
    ) -> Inductor {
        Inductor {
            inductance,
            voltage,
            tolerance,
            mounting,
            package,
        }
    }
}
impl Element for Inductor {
    fn print(&self) {
        let mounting_name = self.mounting.get_name();
        let package_name = self.package.get_name();
        println!(
            "─◠◠─ {} H ± {}% {} V inductor in {} {} package",
            self.inductance, self.tolerance, self.voltage, mounting_name, package_name
        );
    }

    fn get_type(&self) -> ElementType {
        return ElementType::Inductor;
    }

    fn get_filename() -> &'static str {
        return "inductors";
    }

    fn get_header() -> Vec<&'static str> {
        return vec!["inductance", "voltage", "tolerance", "mounting", "package"];
    }

    fn export(&self) -> Vec<String> {
        let vector = vec![
            self.inductance.to_string(),
            self.voltage.to_string(),
            self.tolerance.to_string(),
            self.mounting.get_name().to_owned(),
            self.package.get_name().to_owned(),
        ];
        return vector;
    }

    fn from_record(record: &csv::StringRecord) -> Inductor {
        Inductor::new(
            record[0].parse().unwrap(),
            record[1].parse().unwrap(),
            record[2].parse().unwrap(),
            Mounting::from_str(&record[3]).unwrap(),
            Package::Axial,
        )
    }
}
