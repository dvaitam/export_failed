```rust
use std::io::{self, BufRead};
use std::cmp::max;

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    
    let line1 = lines.next().unwrap().unwrap();
    let mut iter = line1.split_whitespace().map(|s| s.parse::<usize>().unwrap());
    let n: usize = iter.next().unwrap();
    let m: usize = iter.next().unwrap();
    let c0: usize = iter.next().unwrap();
    let d0: usize = iter.next().unwrap();

    let mut stuffings = Vec::new();
    for _ in 0..m {
        let line = lines.next().unwrap().unwrap();
        let mut iter = line.split_whitespace().map(|s| s.parse::<usize>().unwrap());
        let a = iter.next().unwrap();
        let b = iter.next().unwrap();
        let c = iter.next().unwrap();
        let d = iter.next().unwrap();
        stuffings.push((a, b, c, d));
    }

    let mut dp = vec