use std::collections::VecDeque;
use std::io::{self, BufRead};

fn solve() {
    let mut lines = io::stdin().lock().lines();

    let first_line = lines.next().unwrap().unwrap();
    let dims: Vec<usize> = first_line
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    let (_k, n, _m) = (dims[0], dims[1], dims[2]);

    let mut grid: Vec<Vec<char>> = (0..n)
        .map(|_| lines.next().unwrap().unwrap().chars().collect())
        .collect();
}