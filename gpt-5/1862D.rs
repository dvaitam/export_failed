use std::io::{self, Read};

fn tri(x: u128) -> u128 {
    x * (x + 1) / 2
}

fn minimal_balls(n: u128) -> u128 {
    let mut lo: u128 = 0;
    let mut hi: u128 = 2_000_000_000;
    while lo < hi {
        let mid = (lo + hi) / 2;
        if tri(mid) >= n {
            hi = mid;
        } else {
            lo = mid + 1;
        }
    }
    let k = lo;
    let tk = tri(k);
    if tk == n {
        k + 1
    } else {
        let tprev = tri(k - 1);
        k + (n - tprev)
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();
    let t: usize = it.next().unwrap().parse().unwrap();
    let mut out = String::new();
    for _ in 0..t {
        let n: u128 = it.next().unwrap().parse::<u128>().unwrap();
        out.push_str(&format!("{}\n", minimal_balls(n)));
    }
    print!("{}", out);
}