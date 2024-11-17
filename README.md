# Goniometry in Rust

This is a simple Rust library that provides functionality for working with **goniometry** (trigonometric functions like sine, cosine, and tangent) and metrics (Degrees and Radians). The module offers a flexible way to handle angles, calculate trigonometric values, and convert between degrees and radians.

## Features

- **Trigonometric functions:** `sin`, `cos`, `tan` for calculating sine, cosine, and tangent.
- **Angle metrics:** Support for angles in both **Radians** and **Degrees**.
- **Metric entries:** Define angles using specific values or fractions for more precision.

## Usage

### Adding to your project

Add the following to your `Cargo.toml` to include this module as a dependency (or clone the repository):

```toml
[dependencies]
goniometry = "*"
```

### Example

```rust
use crate::goniometry::functions::*;
use crate::goniometry::metrics::{Degree, Rad, MetricEntry};

fn main() {
    // Create a Rad object with a fraction
    let rad = Rad::new(MetricEntry::Fraction(1, 2));
    println!("Rad1 (fraction): {}", rad);

    // Create a Rad object with the default value (π)
    let drad = Rad::default();
    println!("Rad2 (default): {}", drad);

    // Create a Degree object with a specific value
    let deg = Degree::new(MetricEntry::Value(90));
    println!("Deg1 (value): {}", deg);

    // Create a Degree object with the default value (180)
    let ddeg = Degree::default();
    println!("Deg2 (default): {}", ddeg);

    // Calculate trigonometric functions for Degree input
    let sen = sin(deg);
    println!("sin of Deg1: {}", sen);

    let cos = cos(ddeg);
    println!("cos of Deg2: {}", cos);

    let tan = tan(Degree::new(MetricEntry::Value(36)));
    println!("tan of 36 degrees: {}", tan);
}
```

### Goniometry Functions

The library provides trigonometric functions that accept `Degree` or `Rad` inputs and return the result of the respective calculations:

- `sin(angle)`
- `cos(angle)`
- `tan(angle)`

### Metric System

The library supports two metric types:

- **Degrees**: Angles in degrees (e.g., 90°).
- **Radians**: Angles in radians (e.g., π/2 radians).

#### Rad

The `Rad` struct supports both value-based and fraction-based angle definitions.

#### Degree

The `Degree` struct allows you to specify degrees directly.

### Examples

You can create angles with specific values or fractions:

```rust
let rad = Rad::new(MetricEntry::Fraction(1, 2)); // π/2
let deg = Degree::new(MetricEntry::Value(90)); // 90 degrees
```

### Functions

The trigonometric functions accept angles in either radians or degrees:

```rust
let sine_value = sin(Degree::new(MetricEntry::Value(45)));
let cosine_value = cos(Rad::new(MetricEntry::Fraction(1, 2))); // π/2 radians
let tangent_value = tan(Degree::new(MetricEntry::Value(36)));
```

## Installation

Clone the repository and add the `src` folder to your project, or use it as a dependency from Cargo.

```bash
git clone https://github.com/your-username/goniometry.git
```

## License

This project is licensed under the GNU License - see the [LICENSE](LICENSE) file for details.
