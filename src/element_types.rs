use std::str::FromStr;

pub enum CapacitorType {
    Ceramic,
    Electrolytic,
    Tantalum,
}

pub enum Package {
    Axial,
    DIP(i32),
}

impl Package {
    fn get_name(&self) -> String {
        match self {
            Package::Axial => String::from_str("Axial").unwrap(),
            Package::DIP(number) => String::from_str("DIP-").unwrap() + &number.to_string(),
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
    fn export(&self) -> Vec<String>;
}

pub struct Resistor {
    resistance: f32,
    power: f32,
    tolerance: i32,
    mounting: Mounting,
}

impl Resistor {
    pub fn new(resistance: f32, power: f32, tolerance: i32, mounting: Mounting) -> Resistor {
        Resistor {
            resistance,
            power,
            tolerance,
            mounting,
        }
    }
}

impl Element for Resistor {
    fn print(&self) {
        let package_name = self.mounting.get_name();
        println!(
            "Resistor ─⊏⊐─ with {} Ω ± {}% in {} package. {} W",
            self.resistance, self.tolerance, package_name, self.power
        );
    }

    fn export(&self) -> Vec<String> {
        let vector = vec![1, 2];
        return vector;
    }
}

pub struct Capacitor {
    capacitance: f32,
    voltage: f32,
    tolerance: i32,
    capacitor_type: CapacitorType,
    mounting: Mounting,
}

impl Capacitor {
    pub fn new(capacitance: f32, voltage: f32, tolerance: i32, mounting: Mounting) -> Capacitor {
        Capacitor {
            capacitance,
            voltage,
            tolerance,
            capacitor_type: CapacitorType::Ceramic,
            mounting,
        }
    }
}

impl Element for Capacitor {
    fn print(&self) {
        let package_name = self.mounting.get_name();
        println!(
            "Capacitor ─┤├─ with {} F ± {}% in {} package. {} V",
            self.capacitance, self.tolerance, package_name, self.voltage
        );
    }

    fn export(&self) -> Vec<String> {
        let vector = vec![1, 2];
        return vector;
    }
}

pub struct Inductor {
    inductance: f32,
    voltage: f32,
    tolerance: i32,
    mounting: Mounting,
}

impl Inductor {
    pub fn new(inductance: f32, voltage: f32, tolerance: i32, mounting: Mounting) -> Inductor {
        Inductor {
            inductance,
            voltage,
            tolerance,
            mounting,
        }
    }
}
impl Element for Inductor {
    fn print(&self) {
        let package_name = self.mounting.get_name();
        println!(
            "Inductor ─◠◠─ with {} H ± {}% in {} package. {} V",
            self.inductance, self.tolerance, package_name, self.voltage
        );
    }

    fn export(&self) -> Vec<String> {
        let vector = vec![1, 2];
        return vector;
    }
}
