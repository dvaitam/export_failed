use std::io::{self, Read};

const MOD: u64 = 998_244_353;

fn mod_pow(mut base: u64, mut exp: u64) -> u64 {
    let mut res = 1u64;
    base %= MOD;
    while exp > 0 {
        if exp & 1 == 1 {
            res = res * base % MOD;
        }
        base = base * base % MOD;
        exp >>= 1;
    }
    res
}

struct Scanner {
    data: Vec<u8>,
    idx: usize,
}

impl Scanner {
    fn new() -> Self {
        let mut input = String::new();
        io::stdin().read_to_string(&mut input).unwrap();
        Self {
            data: input.into_bytes(),
            idx: 0,
        }
    }
    fn next_u64(&mut self) -> u64 {
        while self.idx < self.data.len() && self.data[self.idx].is_ascii_whitespace() {
            self.idx += 1;
        }
        let start = self.idx;
        while self.idx < self.data.len() && !self.data[self.idx].is_ascii_whitespace() {
            self.idx += 1;
        }
        std::str::from_utf8(&self.data[start..self.idx])
            .unwrap()
            .parse::<u64>()
            .unwrap()
    }
}

fn main() {
    let mut sc = Scanner::new();
    let t = sc.next_u64() as usize;
    for _ in 0..t {
        let n = sc.next_u64() as usize;
        let m = sc.next_u64() as usize;
        let k = sc.next_u64();
        let q = sc.next_u64() as usize;
        let mut xs = Vec::with_capacity(q);
        let mut ys = Vec::with_capacity(q);
        for _ in 0..q {
            xs.push(sc.next_u64() as usize);
            ys.push(sc.next_u64() as usize);
        }
        let mut row_seen = vec![false; n + 1];
        let mut col_seen = vec![false; m + 1];
        let mut row_cnt = 0usize;
        let mut col_cnt = 0usize;
        let mut effective = 0u64;
        for i in (0..q).rev() {
            let r = xs[i];
            let c = ys[i];
            let mut useful = false;
            if row_cnt < n && !row_seen[r] {
                useful = true;
            }
            if col_cnt < m && !col_seen[c] {
                useful = true;
            }
            if useful {
                effective += 1;
            }
            if !row_seen[r] {
                row_seen[r] = true;
                row_cnt += 1;
            }
            if !col_seen[c] {
                col_seen[c] = true;
                col_cnt += 1;
            }
            if row_cnt == n && col_cnt == m {
                break;
            }
        }
        let ans = mod_pow(k, effective);
        println!("{}", ans);
    }
}