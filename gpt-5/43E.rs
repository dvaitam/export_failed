use std::io::{self, Read};

fn sign(x: i64) -> i32 {
    if x > 0 { 1 } else if x < 0 { -1 } else { 0 }
}

fn count_pair(a: &Vec<(i64, i64)>, b: &Vec<(i64, i64)>) -> i64 {
    let total_a: i64 = a.iter().map(|&(_, t)| t).sum();
    let total_b: i64 = b.iter().map(|&(_, t)| t).sum();
    let limit_time = total_a.min(total_b);
    let mut ia = 0usize;
    let mut ib = 0usize;
    let mut rem_a = a[0].1;
    let mut rem_b = b[0].1;
    let mut va = a[0].0;
    let mut vb = b[0].0;

    let mut t_elapsed: i64 = 0;
    let mut d: i64 = 0; // position difference: pos_a - pos_b
    let mut prev_m: i64 = 0;
    let mut have_prev = false;

    let mut cnt: i64 = 0;

    while t_elapsed < limit_time {
        while rem_a == 0 && ia + 1 < a.len() {
            ia += 1;
            va = a[ia].0;
            rem_a = a[ia].1;
        }
        while rem_b == 0 && ib + 1 < b.len() {
            ib += 1;
            vb = b[ib].0;
            rem_b = b[ib].1;
        }

        let mut dt = limit_time - t_elapsed;
        if rem_a < dt { dt = rem_a; }
        if rem_b < dt { dt = rem_b; }
        if dt == 0 { break; }

        let m = va - vb;

        if d == 0 && t_elapsed > 0 {
            if have_prev && prev_m != 0 && m != 0 && sign(prev_m) == sign(m) {
                cnt += 1;
            }
        }

        let d_after = d + m * dt;

        if d != 0 && d_after != 0 && sign(d) != sign(d_after) {
            cnt += 1;
        }

        t_elapsed += dt;
        d = d_after;

        rem_a -= dt;
        rem_b -= dt;

        prev_m = m;
        have_prev = true;
    }

    cnt
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace().map(|s| s.parse::<i64>().unwrap());

    let n = it.next().unwrap() as usize;
    let _s = it.next().unwrap(); // not needed explicitly

    let mut cars: Vec<Vec<(i64, i64)>> = Vec::with_capacity(n);

    for _ in 0..n {
        let k = it.next().unwrap() as usize;
        let mut segs = Vec::with_capacity(k);
        for _ in 0..k {
            let v = it.next().unwrap();
            let t = it.next().unwrap();
            segs.push((v, t));
        }
        cars.push(segs);
    }

    let mut ans: i64 = 0;
    for i in 0..n {
        for j in (i + 1)..n {
            ans += count_pair(&cars[i], &cars[j]);
        }
    }

    println!("{}", ans);
}