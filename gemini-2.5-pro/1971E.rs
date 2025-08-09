use std::io::{BufRead, Write};

fn solve(stdin: &mut dyn BufRead, _stdout: &mut dyn Write) {
    let mut line = String::new();
    stdin.read_line(&mut line).unwrap();
    let mut iter = line.split_whitespace();
    let _n: u64 = iter.next().unwrap().parse().unwrap();
    let _k: usize = iter.next().unwrap().parse().unwrap();
    let _q: usize = iter.next().unwrap().parse().unwrap();

    line.clear();
    stdin.read_line(&mut line).unwrap();
    let _a_in: Vec<u64> = line.split_whitespace().map(|s| s.parse().unwrap()).collect();

    line.clear();
    stdin.read_line(&mut line).unwrap();
    let _b_in: Vec<u64> = line.split_whitespace().map(|s| s.parse().unwrap()).collect();
}

fn main() {
    let mut stdin = std::io::stdin().lock();
    let mut stdout = std::io::stdout().lock();
    solve(&mut stdin, &mut stdout);
}