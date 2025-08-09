use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lines();
    let n: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();
    let a: Vec<i64> = lines.next().unwrap().unwrap().trim().split_whitespace().map(|x| x.parse().unwrap()).collect();
    let b: Vec<i64> = lines.next().unwrap().unwrap().trim().split_whitespace().map(|x| x.parse().unwrap()).collect();
    let total_a: i64 = a.iter().sum();
    let mut bb = b.clone();
    bb.sort_by(|x, y| y.cmp(x));
    let mut sum_b = 0i64;
    let mut min_k = 0usize;
    for i in 0..n {
        sum_b += bb[i];
        if sum_b >= total_a {
            min_k = i + 1;
            break;
        }
    }
    if min_k == 0 {
        min_k = n;
    }
    const MAX_S: usize = 100 * 100 + 10;
    let mut dp: Vec<Vec<i64>> = vec![vec![-1; MAX_S]; n + 1];
    dp[0][0] = 0;
    for i in 0..n {
        let ai = a[i];
        let bi = b[i] as usize;
        for jj in (1..=n).rev() {
            for ss in (bi..MAX_S).rev() {
                let prev_s = ss - bi;
                let prev_a = dp[jj - 1][prev_s];
                if prev_a != -1 {
                    let new_a = prev_a + ai;
                    if dp[jj][ss] < new_a {
                        dp[jj][ss] = new_a;
                    }
                }
            }
        }
    }
    let mut max_a = -1i64;
    for ss in 0..MAX_S {
        if (ss as i64) >= total_a && dp[min_k][ss] > max_a {
            max_a = dp[min_k][ss];
        }
    }
    let t = total_a - max_a;
    println!("{} {}", min_k, t);
}