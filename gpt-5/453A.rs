use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();
    let m: usize = it.next().unwrap().parse().unwrap();
    let n: i32 = it.next().unwrap().parse().unwrap();
    let mf = m as f64;
    let mut ans = 0.0f64;
    for k in 1..=m {
        let x = (k as f64 - 1.0) / mf;
        ans += 1.0 - x.powi(n);
    }
    println!("{:.10}", ans);
}