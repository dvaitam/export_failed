use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let numbers: Vec<i32> = input
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    let coefficients = [1, 1, 2, 7, 4];

    let result: i32 = numbers
        .iter()
        .zip(coefficients.iter())
        .map(|(num, coeff)| num * coeff)
        .sum();

    println!("{}", result);
}