use std::io::{self, BufRead, BufWriter, Write};

fn main() {
    let mut stdin = io::stdin().lock();
    let mut stdout = BufWriter::new(io::stdout().lock());
    let mut buffer = String::new();

    stdin.read_line(&mut buffer).unwrap();
    let t: usize = buffer.trim().parse().unwrap();
    buffer.clear();

    for _ in 0..t {
        stdin.read_line(&mut buffer).unwrap();
        let n: usize = buffer.trim().parse().unwrap();
        buffer.clear();

        stdin.read_line(&mut buffer).unwrap();
        let p: Vec<i32> = buffer
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        buffer.clear();

        let mut s: Vec<i32> = Vec::new();
        s.push(p[0]);

        for i in 1..(n - 1) {
            if (p[i - 1] < p[i] && p[i] > p[i + 1]) || (p[i - 1] > p[i] && p[i] < p[i + 1]) {
                s.push(p[i]);
            }
        }
        
        s.push(p[n - 1]);

        writeln!(stdout, "{}", s.len()).unwrap();
        for (i, &val) in s.iter().enumerate() {
            write!(stdout, "{}", val).unwrap();
            if i < s.len() - 1 {
                write!(stdout, " ").unwrap();
            }
        }
        writeln!(stdout).unwrap();
    }
}