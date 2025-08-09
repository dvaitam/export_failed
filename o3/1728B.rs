use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut iter = input.split_whitespace();
    let t: usize = iter.next().unwrap().parse().unwrap();
    let mut out = String::new();
    for _ in 0..t {
        let n: usize = iter.next().unwrap().parse().unwrap();
        let mut p = Vec::with_capacity(n);
        if n == 4 {
            p.extend_from_slice(&[2, 1, 3, 4]);
        } else {
            if n > 5 {
                for k in (4..=n - 2).rev() {
                    p.push(k);
                }
            }
            p.extend_from_slice(&[2, 3, 1, n - 1, n]);
        }
        for (i, v) in p.iter().enumerate() {
            if i > 0 {
                out.push(' ');
            }
            out.push_str(&v.to_string());
        }
        out.push('\n');
    }
    print!("{}", out);
}