mod element_types;

use element_types::PackageType;
use element_types::THTPackages;

fn main() {
    let resistor =
        element_types::Resistor::new(10.0, 0.25, 5, PackageType::THT(THTPackages::Axial));
    resistor.print();
}
