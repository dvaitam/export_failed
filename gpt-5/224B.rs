use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace().map(|s| s.parse::<usize>().unwrap());

    let n = it.next().unwrap();
    let k = it.next().unwrap();

    let mut a = Vec::with_capacity(n);
    let mut maxv = 0usize;
    for _ in 0..n {
        let val = it.next().unwrap();
        if val > maxv { maxv = val; }
        a.push(val);
    }

    if k == 0 {
        println!("-1 -1");
        return;
    }

    let mut cnt = vec![0usize; maxv + 2];
    let mut distinct = 0usize;
    let mut l = 0usize;
    let mut ans: Option<(usize, usize)> = None;

    for r in 0..n {
        let v = a[r];
        if cnt[v] == 0 { distinct += 1; }
        cnt[v] += 1;

        if distinct == k {
            while cnt[a[l]] > 1 {
                cnt[a[l]] -= 1;
                l += 1;
            }
            ans = Some((l + 1, r + 1));
            break;
        }
    }

    match ans {
        Some((x, y)) => println!("{} {}", x, y),
        None => println!("-1 -1"),
    }
}