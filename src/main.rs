mod element_types;

use element_types::PackageType;

fn main() {
    let resistor = element_types::Resistor::new(10.0, 0.25, 5, PackageType::THT);
    let capacitor = element_types::Capacitor::new(20.0, 5.0, 2, PackageType::THT);
    let inductor = element_types::Inductor::new(15.0, 3.0, 4, PackageType::THT);
    resistor.print();
    capacitor.print();
    inductor.print();
}
