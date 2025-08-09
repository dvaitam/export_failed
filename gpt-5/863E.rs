use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();
    let n: usize = it.next().unwrap().parse().unwrap();
    let mut intervals: Vec<(i64, i64)> = Vec::with_capacity(n);
    let mut coords: Vec<i64> = Vec::with_capacity(2 * n);
    for _ in 0..n {
        let l: i64 = it.next().unwrap().parse().unwrap();
        let r: i64 = it.next().unwrap().parse().unwrap();
        intervals.push((l, r));
        coords.push(l);
        coords.push(r + 1);
    }
    coords.sort_unstable();
    coords.dedup();
    let m = coords.len();
    let mut diff = vec![0i64; m];
    let mut idxs: Vec<(usize, usize)> = Vec::with_capacity(n);
    for &(l, r) in &intervals {
        let r1 = r + 1;
        let l_idx = coords.binary_search(&l).unwrap();
        let r_idx = coords.binary_search(&r1).unwrap();
        diff[l_idx] += 1;
        diff[r_idx] -= 1;
        idxs.push((l_idx, r_idx));
    }
    let mut one_len = vec![0i64; m.saturating_sub(1)];
    let mut c = 0i64;
    for j in 0..m - 1 {
        c += diff[j];
        let len = coords[j + 1] - coords[j];
        if c == 1 {
            one_len[j] = len;
        }
    }
    let mut pref = vec![0i64; m];
    for j in 0..m - 1 {
        pref[j + 1] = pref[j] + one_len[j];
    }
    for (i, &(l_idx, r_idx)) in idxs.iter().enumerate() {
        if pref[r_idx] - pref[l_idx] == 0 {
            println!("{}", i + 1);
            return;
        }
    }
    println!("-1");
}