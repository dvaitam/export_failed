use std::io::{self, Read};

fn main() {
    const NEG: i64 = -9_000_000_000_000_000_000;
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let n: usize = it.next().unwrap().parse().unwrap();
    let m: usize = it.next().unwrap().parse().unwrap();

    let mut sums = vec![0i64; n];
    let mut prefs = vec![0i64; n];
    let mut suffs = vec![0i64; n];
    let mut bests = vec![0i64; n];

    for i in 0..n {
        let l: usize = it.next().unwrap().parse().unwrap();
        let mut arr = Vec::with_capacity(l);
        for _ in 0..l {
            let v: i64 = it.next().unwrap().parse().unwrap();
            arr.push(v);
        }

        let mut sum = 0i64;
        for &v in &arr {
            sum += v;
        }
        sums[i] = sum;

        let mut s = 0i64;
        let mut pref = NEG;
        for &v in &arr {
            s += v;
            if s > pref {
                pref = s;
            }
        }
        prefs[i] = pref;

        s = 0;
        let mut suff = NEG;
        for &v in arr.iter().rev() {
            s += v;
            if s > suff {
                suff = s;
            }
        }
        suffs[i] = suff;

        let mut cur = NEG;
        let mut best = NEG;
        for &v in &arr {
            let t = if cur + v > v { cur + v } else { v };
            cur = t;
            if cur > best {
                best = cur;
            }
        }
        bests[i] = best;
    }

    let mut res = NEG;
    let mut cur_end = NEG;
    for _ in 0..m {
        let idx: usize = it.next().unwrap().parse::<usize>().unwrap() - 1;
        let s = sums[idx];
        let p = prefs[idx];
        let su = suffs[idx];
        let b = bests[idx];

        if b > res {
            res = b;
        }
        let cross = cur_end + p;
        if cross > res {
            res = cross;
        }
        let new_cur = (cur_end + s).max(su);
        cur_end = new_cur;
    }

    println!("{}", res);
}