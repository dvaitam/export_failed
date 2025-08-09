use std::io::{self, Read};

fn main() {
    let mut s = String::new();
    io::stdin().read_to_string(&mut s).unwrap();
    let mut it = s.split_whitespace();
    let n: usize = it.next().unwrap().parse().unwrap();
    let k: usize = it.next().unwrap().parse().unwrap();
    let A: f64 = it.next().unwrap().parse::<f64>().unwrap();
    let mut b = vec![0usize; n];
    let mut l = vec![0usize; n];
    for i in 0..n {
        b[i] = it.next().unwrap().parse().unwrap();
        l[i] = it.next().unwrap().parse().unwrap();
    }
    let mut caps = vec![0usize; n];
    for i in 0..n {
        caps[i] = ((100 - l[i]) / 10) as usize;
    }
    let mut candies = vec![0usize; n];
    let mut best = 0f64;
    fn evaluate(n: usize, A: f64, b: &Vec<usize>, l: &Vec<usize>, candies: &Vec<usize>) -> f64 {
        let mut p = vec![0f64; n];
        for i in 0..n {
            let li = (l[i] as isize + 10 * (candies[i] as isize)) as i32;
            let li = if li > 100 { 100 } else { li } as f64;
            p[i] = li / 100.0;
        }
        let mut total = 0f64;
        let lim = 1usize << n;
        for mask in 0..lim {
            let mut prob = 1f64;
            for i in 0..n {
                if ((mask >> i) & 1) == 1 {
                    prob *= p[i];
                } else {
                    prob *= 1.0 - p[i];
                }
                if prob == 0.0 { break; }
            }
            if prob == 0.0 { continue; }
            let pos = (mask as u32).count_ones() as usize;
            if pos * 2 > n {
                total += prob;
            } else {
                let mut B = 0f64;
                for i in 0..n {
                    if ((mask >> i) & 1) == 0 {
                        B += b[i] as f64;
                    }
                }
                total += prob * (A / (A + B));
            }
        }
        total
    }
    fn dfs(idx: usize, rem: usize, n: usize, k: usize, caps: &Vec<usize>, candies: &mut Vec<usize>, b: &Vec<usize>, l: &Vec<usize>, A: f64, best: &mut f64) {
        if idx == n {
            let val = evaluate(n, A, b, l, candies);
            if val > *best { *best = val; }
            return;
        }
        let max_give = std::cmp::min(rem, caps[idx]);
        for give in 0..=max_give {
            candies[idx] = give;
            dfs(idx+1, rem-give, n, k, caps, candies, b, l, A, best);
        }
        candies[idx] = 0;
    }
    dfs(0, k, n, k, &caps, &mut candies, &b, &l, A, &mut best);
    println!("{:.10}", best);
}