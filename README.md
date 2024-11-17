# Goniometry in Rust

A simple Rust library for performing **goniometry** operations (trigonometric functions like sine, cosine, tangent) and handling angle metrics (degrees and radians). This library allows you to define angles using values or fractions for greater precision, making it easy to calculate trigonometric values and convert between degrees and radians.

## Features

- **Trigonometric functions**: `sin`, `cos`, and `tan` for computing sine, cosine, and tangent of angles.
- **Angle metrics**: Support for angles in **Radians** and **Degrees**.
- **Flexible metric entries**: Define angles using exact values or fractions for precise angle definitions.

## Usage

### Adding to your project

Add the following to your `Cargo.toml` file to include this library as a dependency (or clone the repository if you prefer):

```toml
[dependencies]
goniometry = "*"
```

### Example

Here’s how you can use the library in your own project:

```rust
use goniometry::functions::*;
use goniometry::metrics::{Degree, Rad, Metric, MetricEntry};

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

The library provides the following trigonometric functions that accept either `Degree` or `Rad` objects:

- `sin(angle)` – Calculates the sine of the angle.
- `cos(angle)` – Calculates the cosine of the angle.
- `tan(angle)` – Calculates the tangent of the angle.

### Metric System

The library supports two types of angle metrics:

- **Degrees**: Represent angles in degrees (e.g., 90°).
- **Radians**: Represent angles in radians (e.g., π/2 radians).

#### Rad (Radians)

The `Rad` struct supports both value-based and fraction-based angle definitions. For example, you can create a radian value using a fraction like `π/2` (Fraction(1, 2)).

#### Degree

The `Degree` struct allows you to specify angles directly in degrees.

### Creating Angles

You can create angles using specific values or fractions:

```rust
let rad = Rad::new(MetricEntry::Fraction(1, 2)); // π/2 radians
let deg = Degree::new(MetricEntry::Value(90)); // 90 degrees
```

### Using Trigonometric Functions

You can calculate trigonometric values for angles in either radians or degrees:

```rust
let sine_value = sin(Degree::new(MetricEntry::Value(45)));  // sin(45 degrees)
let cosine_value = cos(Rad::new(MetricEntry::Fraction(1, 2))); // cos(π/2 radians)
let tangent_value = tan(Degree::new(MetricEntry::Value(36))); // tan(36 degrees)
```

## Installation

You can install this library in your project by adding it as a dependency in `Cargo.toml` or by cloning the repository.

Clone the repository:

```bash
git clone https://github.com/your-username/goniometry.git
```

## License

This project is licensed under the GNU License. See the [LICENSE](LICENSE) file for more details.
