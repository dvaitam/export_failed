use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();
    let t: usize = it.next().unwrap().parse().unwrap();

    const MAX: usize = 10_000_000;
    let mut sigma = vec![0u32; MAX + 1];

    for i in 1..=MAX {
        let mut j = i;
        while j <= MAX {
            sigma[j] += i as u32;
            j += i;
        }
    }

    let mut best = vec![u32::MAX; MAX + 1];
    for n in 1..=MAX {
        let s = sigma[n] as usize;
        if s <= MAX && best[s] > n as u32 {
            best[s] = n as u32;
        }
    }

    let mut out = String::new();
    for _ in 0..t {
        let c: usize = it.next().unwrap().parse().unwrap();
        let ans = best[c];
        if ans == u32::MAX {
            out.push_str("-1\n");
        } else {
            out.push_str(&format!("{}\n", ans));
        }
    }
    print!("{}", out);
}