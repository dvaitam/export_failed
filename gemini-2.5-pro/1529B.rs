use std::io::{self, BufRead};
use std::cmp::min;

fn solve() {
    let mut line = String::new();
    let stdin = io::stdin();
    let mut handle = stdin.lock();

    handle.read_line(&mut line).unwrap(); // n line, unused

    line.clear();
    handle.read_line(&mut line).unwrap();
    let a: Vec<i64> = line
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    let mut non_positives = Vec::new();
    let mut positives = Vec::new();

    for &x in &a {
        if x <= 0 {
            non_positives.push(x);
        } else {
            positives.push(x);
        }
    }

    let k = non_positives.len();
    let mut result = k;

    if !positives.is_empty() {
        let min_positive = *positives.iter().min().unwrap();
        non_positives.sort_unstable();

        let mut can_add = true;
        if k >= 2 {
            let mut min_diff = i64::MAX;
            for i in 0..(k - 1) {
                min_diff = min(min_diff, non_positives[i + 1] - non_positives[i]);
            }
            if min_diff < min_positive {
                can_add = false;
            }
        }

        if can_add {
            result += 1;
        }
    }

    println!("{}", result);
}

fn main() {
    let mut line = String::new();
    let stdin = io::stdin();
    let mut handle = stdin.lock();
    handle.read_line(&mut line).unwrap();
    let t: usize = line.trim().parse().unwrap();
    for _ in 0..t {
        solve();
    }
}