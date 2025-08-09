use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();
    let n: usize = it.next().unwrap().parse().unwrap();
    let m: f64 = it.next().unwrap().parse::<f64>().unwrap();
    let mut min_price = f64::MAX;
    for _ in 0..n {
        let a: f64 = it.next().unwrap().parse().unwrap();
        let b: f64 = it.next().unwrap().parse().unwrap();
        let price = a / b;
        if price < min_price {
            min_price = price;
        }
    }
    println!("{:.10}", min_price * m);
}