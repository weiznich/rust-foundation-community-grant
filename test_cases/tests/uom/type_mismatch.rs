use uom::si::f32::*;
use uom::si::length::meter;
use uom::si::time::second;

fn main() {
    // Setup length and time quantities using different units.
    let l1 = Length::new::<meter>(15.0);
    let t1 = Time::new::<second>(50.0);
    let error = l1 + t1;
}
