pub enum CapacitorType {
    Ceramic,
    Electrolytic,
    Tantalum,
}

pub enum PackageType {
    SMD,
    THT,
}

impl PackageType {
    fn get_name(&self) -> &'static str {
        match self {
            PackageType::SMD => "SMD",
            PackageType::THT => "THT",
        }
    }
}

pub enum ElementFeature {
    Resistance,
    Power,
    Tolerance,
    Package,
    Capacitance,
    Voltage,
    Inductance,
}

pub enum ElementType {
    Resistor,
    Capacitor,
    Inductor,
}

pub trait Element {
    fn print(&self);
}

pub struct Resistor {
    resistance: f32,
    power: f32,
    tolerance: i32,
    package_type: PackageType,
    element_type: ElementType,
    features: [ElementFeature; 4],
}

impl Resistor {
    pub fn new(resistance: f32, power: f32, tolerance: i32, package_type: PackageType) -> Resistor {
        Resistor {
            resistance,
            power,
            tolerance,
            package_type,
            element_type: ElementType::Resistor,
            features: [
                ElementFeature::Resistance,
                ElementFeature::Power,
                ElementFeature::Tolerance,
                ElementFeature::Package,
            ],
        }
    }
}

impl Element for Resistor {
    fn print(&self) {
        let package_name = self.package_type.get_name();
        println!(
            "Resistor ─⊏⊐─ with {} Ω ± {}% in {} package. {} W",
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
    element_type: ElementType,
    features: [ElementFeature; 4],
}

impl Capacitor {
    pub fn new(capacitance: f32, voltage: f32, tolerance: i32, package: PackageType) -> Capacitor {
        Capacitor {
            capacitance,
            voltage,
            tolerance,
            capacitor_type: CapacitorType::Ceramic,
            package,
            element_type: ElementType::Capacitor,
            features: [
                ElementFeature::Capacitance,
                ElementFeature::Voltage,
                ElementFeature::Tolerance,
                ElementFeature::Package,
            ],
        }
    }
}

impl Element for Capacitor {
    fn print(&self) {
        let package_name = self.package.get_name();
        println!(
            "Capacitor ─┤├─ with {} F ± {}% in {} package. {} V",
            self.capacitance, self.tolerance, package_name, self.voltage
        );
    }
}

pub struct Inductor {
    inductance: f32,
    voltage: f32,
    tolerance: i32,
    package: PackageType,
    element_type: ElementType,
    features: [ElementFeature; 4],
}

impl Inductor {
    pub fn new(inductance: f32, voltage: f32, tolerance: i32, package: PackageType) -> Inductor {
        Inductor {
            inductance,
            voltage,
            tolerance,
            package,
            element_type: ElementType::Inductor,
            features: [
                ElementFeature::Resistance,
                ElementFeature::Power,
                ElementFeature::Tolerance,
                ElementFeature::Package,
            ],
        }
    }
}
impl Element for Inductor {
    fn print(&self) {
        let package_name = self.package.get_name();
        println!(
            "Inductor ─◠◠─ with {} H ± {}% in {} package. {} V",
            self.inductance, self.tolerance, package_name, self.voltage
        );
    }
}
