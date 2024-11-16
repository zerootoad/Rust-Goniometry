use std::f64::consts::PI;

pub fn deg_to_rad(n: f64) -> f64 {
    // convert degrees to radiants
    if n.fract() == 0.0 {
        n * (PI / 180 as f64)
    } else {
        n
    }
}

pub fn f(n: u128) -> u128 {
    // fractional functions
    if n == 1 || n == 0 {
        1
    } else {
        n * f(n - 1)
    }
}

pub fn sin(entry: f64) -> f64 {
    // Taylor expansion of sinx, check reference here:
    // https://stackoverflow.com/questions/67556351/trigonometric-functions-how-do-i-write-sine-and-cosine-functions-code-without
    let mut x = deg_to_rad(entry);

    while x >= PI {
        x -= PI;
    }

    let mut sinx: f64 = 0 as f64;
    let aprox = 12;

    if PI > x && x > PI / 2 as f64 {
        x = PI - x;
    }

    for i in 0..aprox {
        let n = (-1 as f64).powi(i as i32) * x.powi(2 * i + 1);
        let d = f((2 * i + 1) as u128) as f64;
        sinx += n / d;
    }

    sinx
}

pub fn cos(entry: f64) -> f64 {
    let x = deg_to_rad(entry);

    sin(PI / 2 as f64 - x) // get cosine using sin identity: cosx = sin(90 - x)

    /*
    second approach would be using the fundamental goniometric circle formula:
        sen^2x + cos^2 = 1
        cos^2x = 1 - sen^2x
        cos = +- sqrt(1 - senx)

    then picking + or - based on where the angle is found on the Cartesian Plane.
    */
}

pub fn tan(entry: f64) -> f64 {
    let x = deg_to_rad(entry);

    let sinx = sin(x);
    let cosx = cos(x);

    sinx / cosx // use the second fundamental formula of goniometry to get the tan.
}
