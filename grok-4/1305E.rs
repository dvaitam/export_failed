use std::io::{self, BufRead};
use std::collections::HashMap;

fn main() {
    let stdin = io::stdin();
    let mut line = String::new();
    stdin.read_line(&mut line).unwrap();
    let words: Vec<String> = line.trim().split_whitespace().map(String::from).collect();
    let n: usize = words[0].parse().unwrap();
    let m_i64: i64 = words[1].parse().unwrap();
    if m_i64 < 0 {
        println!("-1");
        return;
    }
    let m = m_i64 as u64;
    let mut maxm = vec![0u64; n + 1];
    for k in 1..=n {
        let x = (k as u64) - 1;
        maxm[k] = (x * x) / 4;
    }
    if m > maxm[n] {
        println!("-1");
        return;
    }
    let res = build(n, m, &maxm);
    if res.is_empty() {
        println!("-1");
    } else {
        let output: Vec<String> = res.iter().map(|&v| v.to_string()).collect();
        println!("{}", output.join(" "));
    }
}

fn build(n: usize, m: u64, maxm: &Vec<u64>) -> Vec<u64> {
    if m == maxm[n] {
        return (1..=n).map(|i| i as u64).collect();
    }
    if n == 0 {
        return vec![];
    }
    let base_m = if m > maxm[n - 1] { maxm[n - 1] } else { m };
    let r = m - base_m;
    let prefix = build(n - 1, base_m, maxm);
    if prefix.is_empty() {
        return vec![];
    }
    let prev = *prefix.last().unwrap();
    let pn = prefix.len();
    let mut count: HashMap<u64, u64> = HashMap::new();
    for i in 0..pn {
        for j in (i + 1)..pn {
            if let Some(s) = prefix[i].checked_add(prefix[j]) {
                *count.entry(s).or_insert(0) += 1;
            } else {
                return vec![];
            }
        }
    }
    if r == 0 {
        let mut large_sums: Vec<u64> = count.keys().filter(|&&s| s > prev && s <= 1_000_000_000).cloned().collect();
        large_sums.sort();
        let mut curr = if prev + 1 > prev { prev + 1 } else { return vec![]; };
        if curr > 1_000_000_000 {
            return vec![];
        }
        let mut chosen = 0;
        let mut i = 0;
        let ls_len = large_sums.len();
        while i < ls_len {
            if curr < large_sums[i] {
                chosen = curr;
                break;
            } else if curr == large_sums[i] {
                curr = large_sums[i].checked_add(1).unwrap_or(u64::MAX);
                if curr > 1_000_000_000 {
                    return vec![];
                }
                i += 1;
            } else {
                i += 1;
            }
        }
        if chosen == 0 {
            if curr <= 1_000_000_000 {
                chosen = curr;
            } else {
                return vec![];
            }
        }
        let mut res = prefix;
        res.push(chosen);
        return res;
    } else {
        let mut candidates: Vec<u64> = vec![];
        for (&s, &c) in &count {
            if s > prev && s <= 1_000_000_000 && c == r {
                candidates.push(s);
            }
        }
        if candidates.is_empty() {
            return vec![];
        }
        candidates.sort();
        let chosen = candidates[0];
        let mut res = prefix;
        res.push(chosen);
        return res;
    }
}