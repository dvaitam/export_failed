use std::io::{self, BufRead, Write, BufWriter};

fn solve() {
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

        if n == 0 {
            writeln!(stdout, "0").unwrap();
            // Read the empty line for the array if n=0, though constraints say n>=1
            if let Ok(bytes_read) = stdin.read_line(&mut buffer) {
                if bytes_read > 0 {
                     buffer.clear();
                }
            }
            continue;
        }

        stdin.read_line(&mut buffer).unwrap();
        let mut a: Vec<i32> = buffer
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        buffer.clear();

        a.sort_unstable();

        let mut pairs = 0;
        let mut singles = 0;
        
        let mut i = 0;
        while i < n {
            let mut j = i;
            while j < n && a[j] == a[i] {
                j += 1;
            }
            let count = j - i;
            pairs += count / 2;
            singles += count % 2;
            i = j;
        }
        
        let result = pairs + singles / 2;
        writeln!(stdout, "{}", result).unwrap();
    }
}

fn main() {
    solve();
}