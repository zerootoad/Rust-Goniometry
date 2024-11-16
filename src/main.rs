use crate::goniometry::functions::*;
use crate::goniometry::metrics::Degree;
use crate::goniometry::metrics::Metric;
use crate::goniometry::metrics::MetricEntry;
use crate::goniometry::metrics::Rad;

pub mod goniometry;

fn main() {
    let rad = Rad::new(MetricEntry::Fraction(1, 2));
    println!("Rad1 (fraction): {}", rad);

    let drad = Rad::default();
    println!("Rad2 (default): {}", drad);

    let deg = Degree::new(MetricEntry::Value(90));
    println!("Deg1 (value): {}", deg);

    let ddeg = Degree::default();
    println!("Deg2 (default): {}", ddeg);

    let sen = sin(deg);
    println!("sin of Rad1: {}", sen);

    let cos = cos(ddeg);
    println!("cos of Deg1: {}", cos);

    let tan = tan(Degree::new(MetricEntry::Value(36)));
    println!("tan of 36 degrees: {}", tan);
}
