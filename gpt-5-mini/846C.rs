use std::io::{self, Read};
fn main() {
    let mut s = String::new();
    io::stdin().read_to_string(&mut s).unwrap();
    let mut it = s.split_whitespace();
    let n: usize = it.next().unwrap().parse().unwrap();
    let mut a = vec![0i64; n];
    for i in 0..n {
        a[i] = it.next().unwrap().parse().unwrap();
    }
    let mut p = vec![0i64; n+1];
    for i in 0..n {
        p[i+1] = p[i] + a[i];
    }
    let mut pref_val = vec![0i64; n+1];
    let mut pref_pos = vec![0usize; n+1];
    pref_val[0] = p[0];
    pref_pos[0] = 0;
    for j in 1..=n {
        if p[j] > pref_val[j-1] {
            pref_val[j] = p[j];
            pref_pos[j] = j;
        } else {
            pref_val[j] = pref_val[j-1];
            pref_pos[j] = pref_pos[j-1];
        }
    }
    let mut suff_val = vec![0i64; n+1];
    let mut suff_pos = vec![0usize; n+1];
    suff_val[n] = p[n];
    suff_pos[n] = n;
    for j in (0..n).rev() {
        if p[j] >= suff_val[j+1] {
            suff_val[j] = p[j];
            suff_pos[j] = j;
        } else {
            suff_val[j] = suff_val[j+1];
            suff_pos[j] = suff_pos[j+1];
        }
    }
    let mut best = i128::MIN;
    let mut bd0 = 0usize;
    let mut bd1 = 0usize;
    let mut bd2 = 0usize;
    for j in 0..=n {
        let cur = pref_val[j] as i128 - p[j] as i128 + suff_val[j] as i128;
        if cur > best {
            best = cur;
            bd0 = pref_pos[j];
            bd1 = j;
            bd2 = suff_pos[j];
        }
    }
    println!("{} {} {}", bd0, bd1, bd2);
}