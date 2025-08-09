use std::io::{self, BufRead};

const MOD: i64 = 1000000007;

fn mod_pow(mut base: i64, mut exp: i64, mod_: i64) -> i64 {
    let mut res = 1;
    while exp > 0 {
        if exp % 2 == 1 {
            res = res * base % mod_;
        }
        base = base * base % mod_;
        exp /= 2;
    }
    res
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lines();

    let first_line = lines.next().unwrap().unwrap();
    let mut iter = first_line.split_whitespace();
    let n: i64 = iter.next().unwrap().parse().unwrap();
    let m: i64 = iter.next().unwrap().parse().unwrap();
    let k: i64 = iter.next().unwrap().parse().unwrap();

    let mut s_vec: Vec<i64> = Vec::new();
    let mut invalid = false;

    for _ in 0..m {
        let line = lines.next().unwrap().unwrap();
        let mut iter = line.split_whitespace();
        let u: i64 = iter.next().unwrap().parse().unwrap();
        let v: i64 = iter.next().unwrap().parse().unwrap();
        let d = v - u;
        if d == 1 {
            continue;
        } else if d == k + 1 {
            s_vec.push(u);
        } else {
            invalid = true;
        }
    }

    if invalid {
        println!("0");
        return;
    }

    let max_start = n - k - 1;
    if max_start < 1 {
        println!("1");
        return;
    }

    let mut l: i64;
    let mut r: i64;
    if s_vec.is_empty() {
        l = 1;
        r = n;
    } else {
        let max_s = *s_vec.last().unwrap();
        let min_sp = s_vec[0] + k + 1;
        if max_s > min_sp {
            println!("0");
            return;
        }
        l = max_s;
        r = min_sp;
    }

    let mut p: Vec<i64> = Vec::new();
    let mut idx = 0;
    for pos in 1..=max_start {
        if idx < s_vec.len() && s_vec[idx] == pos {
            idx += 1;
            continue;
        }
        p.push(pos);
    }

    let t = p.len();
    let mut ans: i64 = 1;

    for ii in 0..t {
        let s = p[ii];
        let right = s + k + 1;
        let rightp = std::cmp::min(right, r);
        if l > rightp {
            continue;
        }

        // binary search for largest jj >= ii with p[jj] <= rightp
        let mut low = ii;
        let mut high = t - 1;
        let mut jj = ii - 1;
        while low <= high {
            let mid = low + (high - low) / 2;
            if p[mid] <= rightp {
                jj = mid;
                low = mid + 1;
            } else {
                if mid == 0 {
                    break;
                }
                high = mid - 1;
            }
        }

        if jj < ii {
            continue;
        }

        let d = (jj - ii) as i64;
        let contrib = mod_pow(2, d, MOD);
        ans = (ans + contrib) % MOD;
    }

    println!("{}", ans);
}