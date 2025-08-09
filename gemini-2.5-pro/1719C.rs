```rust
use std::collections::VecDeque;
use std::io::{self, BufRead, BufWriter, Write};

fn main() {
    let stdin = io::stdin();
    let mut handle = stdin.lock();
    let stdout = io::stdout();
    let mut out = BufWriter::new(stdout.lock());
    solve(&mut handle, &mut out);
}

fn solve(stdin: &mut dyn BufRead, out: &mut dyn Write) {
    let mut buffer = String::new();
    stdin.read_line(&mut buffer).unwrap();
    let t: usize = buffer.trim().parse().unwrap();

    for _ in 0..t {
        buffer.clear();
        stdin.read_line(&mut buffer).unwrap();
        let mut parts = buffer.trim().split_whitespace();
        let n: usize = parts.next().unwrap().parse().unwrap();
        let q_count: usize = parts.next().unwrap().parse().unwrap();

        buffer.clear();
        stdin.read_line(&mut buffer).unwrap();
        let a: Vec<usize> = buffer
            .trim()
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();

        let champ_id = a.iter().position(|&s| s == n).unwrap() + 1;

        let mut contestants: VecDeque<usize> = (1..=n).collect();
        let mut win_rounds: Vec<Vec<usize>> = vec