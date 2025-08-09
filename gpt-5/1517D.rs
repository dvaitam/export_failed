use std::io::{self, Read};

struct Scanner {
    input: Vec<u8>,
    index: usize,
}
impl Scanner {
    fn new() -> Self {
        let mut input = String::new();
        io::stdin().read_to_string(&mut input).unwrap();
        Self { input: input.into_bytes(), index: 0 }
    }
    fn next_i64(&mut self) -> i64 {
        let mut neg = false;
        while self.index < self.input.len() && self.input[self.index].is_ascii_whitespace() {
            self.index += 1;
        }
        if self.index < self.input.len() && self.input[self.index] == b'-' {
            neg = true;
            self.index += 1;
        }
        let mut x: i64 = 0;
        while self.index < self.input.len() && !self.input[self.index].is_ascii_whitespace() {
            x = x * 10 + (self.input[self.index] - b'0') as i64;
            self.index += 1;
        }
        if neg { -x } else { x }
    }
    fn next_usize(&mut self) -> usize {
        self.next_i64() as usize
    }
}

fn main() {
    let mut sc = Scanner::new();
    let n = sc.next_usize();
    let m = sc.next_usize();
    let k = sc.next_usize();

    let mut h = vec![vec![0i64; m.saturating_sub(1)]; n];
    for i in 0..n {
        for j in 0..m.saturating_sub(1) {
            h[i][j] = sc.next_i64();
        }
    }
    let mut v = vec![vec![0i64; m]; n.saturating_sub(1)];
    for i in 0..n.saturating_sub(1) {
        for j in 0..m {
            v[i][j] = sc.next_i64();
        }
    }

    let mut out = String::new();
    if k % 2 == 1 {
        for i in 0..n {
            for j in 0..m {
                if j > 0 { out.push(' '); }
                out.push_str("-1");
            }
            out.push('\n');
        }
        print!("{}", out);
        return;
    }

    let half = k / 2;
    let inf: i64 = 1_000_000_000_000i64;
    let mut dp = vec![vec![0i64; m]; n];

    for _ in 0..half {
        let mut ndp = vec![vec![inf; m]; n];
        for i in 0..n {
            for j in 0..m {
                if j > 0 {
                    let val = dp[i][j - 1] + h[i][j - 1];
                    if val < ndp[i][j] { ndp[i][j] = val; }
                }
                if j + 1 < m {
                    let val = dp[i][j + 1] + h[i][j];
                    if val < ndp[i][j] { ndp[i][j] = val; }
                }
                if i > 0 {
                    let val = dp[i - 1][j] + v[i - 1][j];
                    if val < ndp[i][j] { ndp[i][j] = val; }
                }
                if i + 1 < n {
                    let val = dp[i + 1][j] + v[i][j];
                    if val < ndp[i][j] { ndp[i][j] = val; }
                }
            }
        }
        dp = ndp;
    }

    for i in 0..n {
        for j in 0..m {
            if j > 0 { out.push(' '); }
            let ans = dp[i][j] * 2;
            out.push_str(&ans.to_string());
        }
        out.push('\n');
    }
    print!("{}", out);
}