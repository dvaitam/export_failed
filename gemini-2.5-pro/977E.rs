```rust
use std::io::{self, BufRead};
use std::collections::VecDeque;

fn main() {
    let mut buffer = String::new();
    let stdin = io::stdin();
    let mut handle = stdin.lock();

    handle.read_line(&mut buffer).unwrap();
    let mut line_iter = buffer.split_whitespace();
    let n: usize = line_iter.next().unwrap().parse().unwrap();
    let m: usize = line_iter.next().unwrap().parse().unwrap();

    let mut adj: Vec<Vec<usize>> = vec