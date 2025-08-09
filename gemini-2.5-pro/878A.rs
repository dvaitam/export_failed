use std::io::{self, BufRead, BufWriter, Write};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let n: usize = lines.next().unwrap().unwrap().parse().unwrap();

    let mut val0: u16 = 0;
    let mut val1: u16 = 1023;

    for _ in 0..n {
        let line = lines.next().unwrap().unwrap();
        let mut parts = line.split_whitespace();
        let op = parts.next().unwrap();
        let x: u16 = parts.next().unwrap().parse().unwrap();

        match op {
            "&" => {
                val0 &= x;
                val1 &= x;
            }
            "|" => {
                val0 |= x;
                val1 |= x;
            }
            "^" => {
                val0 ^= x;
                val1 ^= x;
            }
            _ => {}
        }
    }

    let mut const_xor: u16 = 0;
    let mut const_or: u16 = 0;
    let mut const_and: u16 = 1023;

    for i in 0..10 {
        let bit_from_0 = (val0 >> i) & 1;
        let bit_from_1 = (val1 >> i) & 1;

        if bit_from_0 == 1 && bit_from_1 == 0 {
            // Negation: achieved by XOR with 1
            const_xor |= 1 << i;
        } else if bit_from_0 == 1 && bit_from_1 == 1 {
            // Constant 1: achieved by OR with 1
            const_or |= 1 << i;
        } else if bit_from_0 == 0 && bit_from_1 == 0 {
            // Constant 0: achieved by AND with 0
            const_and &= !(1 << i);
        }
        // Identity (0, 1) is the default and requires no changes to the constants
    }

    let mut result_ops: Vec<(char, u16)> = Vec::new();

    // The order of operations is fixed to match the logic used to derive the constants
    if const_xor != 0 {
        result_ops.push(('^', const_xor));
    }
    if const_or != 0 {
        result_ops.push(('|', const_or));
    }
    if const_and != 1023 {
        result_ops.push(('&', const_and));
    }
    
    let stdout = io::stdout();
    let mut out = BufWriter::new(stdout.lock());
    
    writeln!(out, "{}", result_ops.len()).unwrap();
    for (op, val) in result_ops {
        writeln!(out, "{} {}", op, val).unwrap();
    }
}