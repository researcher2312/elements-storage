pub enum CapacitorType {
    Ceramic,
    Electrolytic,
    Tantalum,
}

pub enum PackageType {
    SMD(SMDPackages),
    THT(THTPackages),
}

pub enum SMDPackages {
    T0603,
    T0804,
}

pub enum THTPackages {
    Axial,
    DIP8,
    DIP10,
    DIP14,
}

pub struct Resistor {
    resistance: f32,
    power: f32,
    tolerance: i32,
    package: PackageType,
}

impl Resistor {
    pub fn new(resistance: f32, power: f32, tolerance: i32, package: PackageType) -> Resistor {
        Resistor {
            resistance,
            power,
            tolerance,
            package,
        }
    }

    pub fn print(&self) {
        println!("Resistor with {} Ohm", self.resistance);
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
