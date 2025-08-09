use std::io::{self, Read};

fn check(a: &Vec<i64>, n: usize, k: usize, m: i64) -> bool {
    let mut prefix = Vec::with_capacity(n + 1);
    prefix.push(0i64);
    for &v in a {
        let b = if v >= m { 1 } else { -1 };
        let prev = *prefix.last().unwrap();
        prefix.push(prev + b);
    }
    let mut min_pref = 0i64;
    for i in k..=n {
        if prefix[i - k] < min_pref {
            min_pref = prefix[i - k];
        }
        if prefix[i] - min_pref >= 1 {
            return true;
        }
    }
    false
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();
    let n: usize = it.next().unwrap().parse().unwrap();
    let k: usize = it.next().unwrap().parse().unwrap();
    let mut a: Vec<i64> = Vec::with_capacity(n);
    for _ in 0..n {
        a.push(it.next().unwrap().parse::<i64>().unwrap());
    }
    let mut lo: i64 = 1;
    let mut hi: i64 = *a.iter().max().unwrap();
    while lo < hi {
        let mid = (lo + hi + 1) / 2;
        if check(&a, n, k, mid) {
            lo = mid;
        } else {
            hi = mid - 1;
        }
    }
    println!("{}", lo);
}