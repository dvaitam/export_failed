use std::cmp::max;
use std::io::{self, Read};

struct Scanner {
    buf: Vec<u8>,
    idx: usize,
}
impl Scanner {
    fn new() -> Self {
        let mut input = String::new();
        io::stdin().read_to_string(&mut input).unwrap();
        Self { buf: input.into_bytes(), idx: 0 }
    }
    fn next_i64(&mut self) -> i64 {
        let n = self.buf.len();
        while self.idx < n && self.buf[self.idx].is_ascii_whitespace() {
            self.idx += 1;
        }
        let mut sign = 1i64;
        if self.idx < n && self.buf[self.idx] == b'-' {
            sign = -1;
            self.idx += 1;
        }
        let mut x = 0i64;
        while self.idx < n && self.buf[self.idx].is_ascii_digit() {
            x = x * 10 + (self.buf[self.idx] - b'0') as i64;
            self.idx += 1;
        }
        x * sign
    }
}

fn main() {
    let mut sc = Scanner::new();
    let t = sc.next_i64() as usize;
    let mut out = String::new();
    for _ in 0..t {
        let n = sc.next_i64() as usize;
        let mut v = Vec::with_capacity(n);
        let mut min_b = i64::MAX;
        for _ in 0..n {
            let a = sc.next_i64();
            let b = sc.next_i64();
            if b < min_b {
                min_b = b;
            }
            v.push((a, b));
        }
        v.sort_by_key(|&(a, _)| a);
        let mut suf = vec![0i64; n];
        if n > 0 {
            suf[n - 1] = v[n - 1].1;
            for i in (0..n - 1).rev() {
                suf[i] = max(v[i].1, suf[i + 1]);
            }
        }
        let mut ans = i64::MAX;
        let mut i = 0usize;
        while i < n {
            let val = v[i].0;
            let mut j = i;
            while j + 1 < n && v[j + 1].0 == val {
                j += 1;
            }
            let y = if j + 1 < n { suf[j + 1] } else { min_b };
            let cand = (val - y).abs();
            if cand < ans {
                ans = cand;
            }
            i = j + 1;
        }
        out.push_str(&format!("{}\n", ans));
    }
    print!("{}", out);
}