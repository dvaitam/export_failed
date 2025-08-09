use std::io::{self, BufWriter, Write};

fn main() {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();
    let t: i32 = buffer.trim().parse().unwrap();
    let mut out = BufWriter::new(io::stdout().lock());

    for _ in 0..t {
        buffer.clear();
        io::stdin().read_line(&mut buffer).unwrap();
        let n: usize = buffer.trim().parse().unwrap();

        buffer.clear();
        io::stdin().read_line(&mut buffer).unwrap();
        let mut p: Vec<i32> = buffer
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
    }
}