use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace().map(|s| s.parse::<i64>().unwrap());
    let _n = it.next().unwrap();
    let a1 = it.next().unwrap();
    let a2 = it.next().unwrap();
    let a3 = it.next().unwrap();
    let b1 = it.next().unwrap();
    let b2 = it.next().unwrap();
    let b3 = it.next().unwrap();

    let maxw = std::cmp::min(a1, b2) + std::cmp::min(a2, b3) + std::cmp::min(a3, b1);
    let minw = [
        a1 - (b1 + b3),
        a2 - (b2 + b1),
        a3 - (b3 + b2),
    ]
    .iter()
    .map(|&x| if x > 0 { x } else { 0 })
    .sum::<i64>();

    println!("{} {}", minw, maxw);
}