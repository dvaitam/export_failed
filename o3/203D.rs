use std::io::{self, Read};

fn reflect(mut pos: f64, limit: f64) -> f64 {
    let period = 2.0 * limit;
    pos = pos.rem_euclid(period);
    if pos > limit {
        period - pos
    } else {
        pos
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let nums: Vec<f64> = input
        .split_whitespace()
        .map(|s| s.parse::<f64>().unwrap())
        .collect();

    let (a, b, m) = (nums[0], nums[1], nums[2]);
    let (vx, vy, vz) = (nums[3], nums[4], nums[5]);

    let t = m / -vy;
    let x_unfold = a / 2.0 + vx * t;
    let z_unfold = vz * t;

    let x0 = reflect(x_unfold, a);
    let z0 = reflect(z_unfold, b);

    println!("{:.10} {:.10}", x0, z0);
}