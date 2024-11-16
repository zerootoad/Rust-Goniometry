use std::f64::consts::PI;

pub enum MetricEntry<T> {
    Value(T),
    Fraction(T, T),
}

pub trait Metric {
    fn new<T>(entry: MetricEntry<T>) -> f64
    where
        T: Into<f64>;
    fn default() -> f64;
}

#[derive(Debug)]
pub struct Rad(pub f64);
#[derive(Debug)]
pub struct Degree(pub f64);

impl Metric for Rad {
    fn new<T>(entry: MetricEntry<T>) -> f64
    where
        T: Into<f64>,
    {
        match entry {
            MetricEntry::Fraction(num, den) => {
                let den = den.into();
                assert!(den != 0 as f64, "Denominator can't assume 0 as value.");
                let value = (num.into() / den) * PI;
                Rad(value).0
            }
            MetricEntry::Value(value) => Rad(value.into() as f64).0,
        }
    }

    fn default() -> f64 {
        Rad(PI).0
    }
}

impl Metric for Degree {
    fn new<T>(entry: MetricEntry<T>) -> f64
    where
        T: Into<f64>,
    {
        match entry {
            MetricEntry::Fraction(_, _) => panic!("Cannot initialize degrees using fractions!"),
            MetricEntry::Value(value) => Degree(value.into() as f64).0,
        }
    }

    fn default() -> f64 {
        Degree(180 as f64).0
    }
}
