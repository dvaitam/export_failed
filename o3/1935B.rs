use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();
    let t: usize = it.next().unwrap().parse().unwrap();
    let mut out = String::new();
    for _ in 0..t {
        let n: usize = it.next().unwrap().parse().unwrap();
        let mut a = Vec::with_capacity(n);
        for _ in 0..n {
            a.push(it.next().unwrap().parse::<usize>().unwrap());
        }
        let mut present = vec![false; n + 2];
        for &v in &a {
            if v <= n {
                present[v] = true;
            }
        }
        let mut mex = 0usize;
        while present[mex] {
            mex += 1;
        }
        if mex == n {
            out.push_str("-1\n");
            continue;
        }
        if mex == 0 {
            out.push_str("2\n1 1\n");
            out.push_str(&format!("2 {}\n", n));
            continue;
        }
        let g = mex;
        let mut freq = vec![0usize; g];
        for &v in &a {
            if v < g {
                freq[v] += 1;
            }
        }
        if freq.iter().any(|&c| c < 2) {
            out.push_str("-1\n");
            continue;
        }
        let mut rem = freq.clone();
        let mut missing_suffix = 0usize;
        let mut visited = vec![0usize; g];
        let mut seg_id = 1usize;
        let mut need = g;
        let mut start = 1usize;
        let mut segs: Vec<(usize, usize)> = Vec::new();
        for i in 0..n {
            let val = a[i];
            if val < g {
                rem[val] -= 1;
                if rem[val] == 0 {
                    missing_suffix += 1;
                }
                if visited[val] != seg_id {
                    visited[val] = seg_id;
                    need -= 1;
                }
            }
            if need == 0 {
                let can_cut = if i == n - 1 {
                    !segs.is_empty()
                } else {
                    missing_suffix == 0
                };
                if can_cut {
                    segs.push((start, i + 1));
                    start = i + 2;
                    seg_id += 1;
                    need = g;
                }
            }
        }
        if start <= n {
            if need == 0 {
                segs.push((start, n));
            } else {
                out.push_str("-1\n");
                continue;
            }
        }
        if segs.len() < 2 {
            out.push_str("-1\n");
            continue;
        }
        out.push_str(&format!("{}\n", segs.len()));
        for (l, r) in segs {
            out.push_str(&format!("{} {}\n", l, r));
        }
    }
    print!("{}", out);
}