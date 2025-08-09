```rust
use std::io::{self, BufRead};
use std::cmp::{min, max};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let first_line = lines.next().unwrap().unwrap();
    let mut nums = first_line.split_whitespace().map(|s| s.parse::<i64>().unwrap());
    
    let n = nums.next().unwrap() as usize;
    let x1 = nums.next().unwrap();
    let y1 = nums.next().unwrap();
    let x2 = nums.next().unwrap();
    let y2 = nums.next().unwrap();

    let mut dists: Vec<(i64, i64)> = Vec::with_capacity(n);

    for _ in 0..n {
        let line = lines.next().unwrap().unwrap();
        let mut flower_coords = line.split_whitespace().map(|s| s.parse::<i64>().unwrap());
        let xi = flower_coords.next().unwrap();
        let yi = flower_coords.next().unwrap();

        let dx1 = xi - x1;
        let dy1 = yi - y1;
        let d1_sq = dx1 * dx1 + dy1 * dy1;

        let dx2 = xi - x2;
        let dy2 = yi - y2;
        let d2_sq = dx2 * dx2 + dy2 * dy2;
        
        dists.push((d1_sq, d2_sq));
    }
    
    dists.sort_unstable_by_key(|p| p.0);

    let mut max_d2_suffix = vec