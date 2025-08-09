use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let k_line = lines.next().unwrap().unwrap();
    let k: f64 = k_line.parse().unwrap();

    let xy_line = lines.next().unwrap().unwrap();
    let coords: Vec<f64> = xy_line
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    let x = coords[0];
    let y = coords[1];

    let angle_rad = k.to_radians();

    let new_x = x * angle_rad.cos() - y * angle_rad.sin();
    let new_y = x * angle_rad.sin() + y * angle_rad.cos();

    println!("{} {}", new_x, new_y);
}