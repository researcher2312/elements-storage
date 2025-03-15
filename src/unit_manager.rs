pub enum Unit {
    Ohm,
    Farad,
    Henr,
    Volt,
    Ampere,
    Watt,
}

impl Unit {
    fn to_str(&self) -> &'static str {
        match self {
            Unit::Ohm => "om",
            Unit::Farad => "F",
            Unit::Henr => "H",
            Unit::Volt => "V",
            Unit::Ampere => "A",
            Unit::Watt => "W",
        }
    }
}

pub struct UnitValue {
    value: f64,
    unit: Unit,
}

impl UnitValue {
    fn prefix(&self) -> &'static str {
        match self.value {
            1e-12..1e-9 => "p",
            1e-9..1e-6 => "n",
            1e-6..1e-3 => "u",
            1e-3..1.0 => "m",
            1.0..1e3 => "",
            1e3..1e6 => "K",
            1e6..1e9 => "M",
            1e9..1e12 => "G",
            _ => "XXX",
        }
    }

    fn to_str(&self) -> String {
        return format!("{} {}{}", self.value, self.prefix(), self.unit.to_str());
    }
}
