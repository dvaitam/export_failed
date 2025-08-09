```rust
use std::io::{self, BufRead, Write};

fn main() {
    let stdin = io::stdin();
    let mut handle = stdin.lock();
    let mut stdout = io::stdout();

    let mut buffer = String::new();
    handle.read_line(&mut buffer).unwrap();
    let t: usize = buffer.trim().parse().unwrap();
    
    for _ in 0..t {
        buffer.clear();
        handle.read_line(&mut buffer).unwrap();
        let n: usize = buffer.trim().parse().unwrap();

        buffer.clear();
        handle.read_line(&mut buffer).unwrap();
        let s_bytes = buffer.trim().as_bytes();

        buffer.clear();
        handle.read_line(&mut buffer).unwrap();
        let a_bytes = buffer.trim().as_bytes();

        if n % 2 != 0 {
            writeln!(stdout, "-1").unwrap();
            continue;
        }

        let s_bits: Vec<u8> = s_bytes.iter().map(|&b| if b == b'(' { 0 } else { 1 }).collect();
        let a_bits: Vec<u8> = a_bytes.iter().map(|&b| if b == b'0' { 0 } else { 1 }).collect();

        let inf = (n + 1) as i32;
        let mut dp_prev = vec