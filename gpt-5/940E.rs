use std::cmp::max;
use std::collections::VecDeque;
use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let n: usize = it.next().unwrap().parse().unwrap();
    let c: usize = it.next().unwrap().parse().unwrap();
    let mut a: Vec<i64> = Vec::with_capacity(n);
    for _ in 0..n {
        a.push(it.next().unwrap().parse::<i64>().unwrap());
    }

    if c == 1 {
        println!("0");
        return;
    }

    let total: i128 = a.iter().map(|&x| x as i128).sum();

    if n < c {
        println!("{}", total);
        return;
    }

    let m = n - c + 1;
    let mut w = vec![0i64; m];
    let mut dq: VecDeque<usize> = VecDeque::new();
    for i in 0..n {
        while let Some(&idx) = dq.back() {
            if a[idx] >= a[i] {
                dq.pop_back();
            } else {
                break;
            }
        }
        dq.push_back(i);
        while let Some(&idx) = dq.front() {
            if idx + c <= i {
                dq.pop_front();
            } else {
                break;
            }
        }
        if i + 1 >= c {
            w[i + 1 - c] = a[*dq.front().unwrap()];
        }
    }

    let k = n / c;
    let r = n % c;

    if k == 0 {
        println!("{}", total);
        return;
    }

    let cols = r + 1;
    let mut prev = vec![0i64; cols];
    for t in 0..cols {
        prev[t] = w[t];
    }

    for j in 1..k {
        let base = j * c;
        let mut pref = vec![prev[0]; cols];
        for t in 1..cols {
            pref[t] = max(pref[t - 1], prev[t]);
        }
        let mut cur = vec![0i64; cols];
        for t in 0..cols {
            cur[t] = pref[t] + w[base + t];
        }
        prev = cur;
    }

    let best = *prev.iter().max().unwrap() as i128;
    let ans = total - best;
    println!("{}", ans);
}