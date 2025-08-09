use std::io::{self, BufWriter, Read, Write};
use std::str::SplitWhitespace;

struct Scanner<'a> {
    iter: SplitWhitespace<'a>,
}

impl<'a> Scanner<'a> {
    fn new(s: &'a str) -> Self {
        Self {
            iter: s.split_whitespace(),
        }
    }

    fn next<T: std::str::FromStr>(&mut self) -> T {
        self.iter.next().unwrap().parse().ok().unwrap()
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut scanner = Scanner::new(&input);
    let stdout = io::stdout();
    let mut out = BufWriter::new(stdout.lock());

    let n: i32 = scanner.next();
    let m: i32 = scanner.next();

    writeln!(out, "{}", n + m).unwrap();
}