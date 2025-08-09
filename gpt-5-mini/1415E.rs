use std::io::{self, Read};
#[derive(Clone, Copy)]
struct Line { m: i128, b: i128, cnt: i64, used: bool }
fn eval_pair(line: &Line, x: i128) -> (i128, i64) {
    let val = line.m * x + line.b;
    (val, -line.cnt)
}
struct LiChao {
    n: usize,
    left: Vec<i32>,
    right: Vec<i32>,
    seg: Vec<Line>,
}
impl LiChao {
    fn new(n: usize) -> Self {
        let size = 4 * n + 5;
        let default = Line { m: 0, b: i128::MIN/4, cnt: 0, used: false };
        LiChao {
            n,
            left: vec![-1; size],
            right: vec![-1; size],
            seg: vec![default; size],
        }
    }
    fn better(a: (i128,i64), b: (i128,i64)) -> bool {
        if a.0 != b.0 { a.0 > b.0 } else { a.1 > b.1 }
    }
    fn insert_line(&mut self, idx: usize, l: i32, r: i32, mut nw: Line) {
        if !self.seg[idx].used {
            self.seg[idx] = nw;
            self.seg[idx].used = true;
            return;
        }
        let mid = (l + r) >> 1;
        let cur = self.seg[idx];
        let cur_mid = eval_pair(&cur, mid as i128);
        let nw_mid = eval_pair(&nw, mid as i128);
        if LiChao::better(nw_mid, cur_mid) {
            std::mem::swap(&mut self.seg[idx], &mut nw);
        }
        if l == r { return; }
        let cur = self.seg[idx];
        let cur_l = eval_pair(&cur, l as i128);
        let nw_l = eval_pair(&nw, l as i128);
        if LiChao::better(nw_l, cur_l) {
            if self.left[idx] == -1 {
                let ni = idx * 2;
                self.left[idx] = ni as i32;
            }
            let child = self.left[idx] as usize;
            self.insert_line(child, l, mid, nw);
        } else {
            let cur_r = eval_pair(&cur, r as i128);
            let nw_r = eval_pair(&nw, r as i128);
            if LiChao::better(nw_r, cur_r) {
                if self.right[idx] == -1 {
                    let ni = idx * 2 + 1;
                    self.right[idx] = ni as i32;
                }
                let child = self.right[idx] as usize;
                self.insert_line(child, mid+1, r, nw);
            }
        }
    }
    fn query(&self, idx: usize, l: i32, r: i32, x: i32) -> (i128,i64) {
        let mut res = (i128::MIN/4, i64::MIN/4);
        if self.seg[idx].used {
            res = eval_pair(&self.seg[idx], x as i128);
        }
        if l == r { return res; }
        let mid = (l + r) >> 1;
        if x <= mid {
            if self.left[idx] != -1 {
                let cand = self.query(self.left[idx] as usize, l, mid, x);
                if LiChao::better(cand, res) { return cand; }
            }
        } else {
            if self.right[idx] != -1 {
                let cand = self.query(self.right[idx] as usize, mid+1, r, x);
                if LiChao::better(cand, res) { return cand; }
            }
        }
        res
    }
}
fn solve_with_lambda(a: &Vec<i64>, s: &Vec<i128>, t: &Vec<i128>, lambda: i128) -> (i128, i64) {
    let n = a.len() - 1;
    let mut dp = vec![i128::MIN/4; n+1];
    let mut cnt = vec![0i64; n+1];
    dp[0] = 0;
    cnt[0] = 0;
    let mut lich = LiChao::new(n.max(1));
    // we will use nodes indexed starting from 1 for root
    // adapt insert/query to start at index 1
    // ensure seg vector size large enough; implemented in struct
    for r in 1..=n {
        // add line from l = r: uses dp[r-1], s[r-1], t[r-1], cnt[r-1]
        let m = -s[r-1];
        let b = dp[r-1] + t[r-1] - lambda;
        let base_cnt = cnt[r-1];
        let line = Line { m, b, cnt: base_cnt, used: true };
        lich.insert_line(1, 1, n as i32, line);
        let q = lich.query(1, 1, n as i32, r as i32);
        if q.0 <= i128::MIN/8 {
            dp[r] = i128::MIN/4;
            cnt[r] = 1e9 as i64;
        } else {
            dp[r] = (r as i128) * s[r] - t[r] + q.0;
            let best_prev_cnt = -q.1;
            cnt[r] = best_prev_cnt + 1;
        }
    }
    (dp[n], cnt[n])
}
fn main() {
    let mut s = String::new();
    io::stdin().read_to_string(&mut s).unwrap();
    let mut it = s.split_whitespace();
    let n: usize = it.next().unwrap().parse().unwrap();
    let k: usize = it.next().unwrap().parse().unwrap();
    let mut v = vec![0i64;n];
    for i in 0..n {
        v[i] = it.next().unwrap().parse().unwrap();
    }
    v.sort_by(|a,b| b.cmp(a));
    let mut a = vec![0i64; n+1];
    for i in 1..=n { a[i]=v[i-1]; }
    let mut S = vec![0i128; n+1];
    let mut T = vec![0i128; n+1];
    for i in 1..=n {
        S[i] = S[i-1] + a[i] as i128;
        T[i] = T[i-1] + (i as i128) * (a[i] as i128);
    }
    let K = (k+1) as i64;
    let mut low: i128 = -2_000_000_000_000_000_000i128;
    let mut high: i128 = 2_000_000_000_000_000_000i128;
    while low < high {
        let mid = low + ((high - low) >> 1);
        let (_val, cnt) = solve_with_lambda(&a, &S, &T, mid);
        if cnt <= K { high = mid; } else { low = mid + 1; }
    }
    let (dpval, cnt) = solve_with_lambda(&a, &S, &T, low);
    let ans = dpval + low * (K as i128);
    println!("{}", ans);
}