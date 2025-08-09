use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();
    let w: usize = it.next().unwrap().parse().unwrap();
    let b: usize = it.next().unwrap().parse().unwrap();

    let mut p = vec![vec![0f64; b + 3]; w + 3];

    for wi in 0..=w {
        for bi in 0..=b {
            if wi == 0 {
                p[wi][bi] = 0.0;
                continue;
            }
            let total = (wi + bi) as f64;
            let mut res = wi as f64 / total;

            if bi > 0 {
                let bi1 = bi - 1;
                let q = if bi1 == 0 {
                    0.0
                } else {
                    let total_q = (wi + bi1) as f64;
                    let after_total = (wi + bi1 - 1) as f64;
                    let p_jump_white = wi as f64 / after_total;
                    let p_jump_black = (bi1 - 1) as f64 / after_total;
                    let val1 = if wi >= 1 && bi1 >= 1 { p[wi - 1][bi1 - 1] } else { 0.0 };
                    let val2 = if bi1 >= 2 { p[wi][bi1 - 2] } else { 0.0 };
                    (bi1 as f64) / total_q * (p_jump_white * val1 + p_jump_black * val2)
                };
                res += (bi as f64) / total * q;
            }
            p[wi][bi] = res;
        }
    }

    println!("{:.10}", p[w][b]);
}