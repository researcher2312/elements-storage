use std::str::FromStr;

pub enum CapacitorType {
    Ceramic,
    Electrolytic,
    Tantalum,
}

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

pub enum ElementFeature {
    Resistance(i32),
    Power(f32),
    Tolerance(i32),
    Package(Package),
    Mounting(Mounting),
    Capacitance(f32),
    Voltage(f32),
    Inductance(f32),
}

pub enum ElementType {
    Resistor,
    Capacitor,
    Inductor,
}

pub trait Element {
    fn print(&self);
    fn export(&self) -> Vec<String> {
        vec![String::from_str("default export").unwrap()]
    }
}

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
        ];
        return vector;
    }
}

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
}

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
}
