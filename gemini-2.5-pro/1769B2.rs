```rust
use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [u64; n],
    }

    let total_sum = a.iter().sum::<u64>() as u128;
    let mut dp = vec