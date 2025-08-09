use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();
    let t: usize = it.next().unwrap().parse().unwrap();
    let mut outputs = Vec::with_capacity(t);
    for _ in 0..t {
        let n: usize = it.next().unwrap().parse().unwrap();
        let mut k: Vec<i64> = Vec::with_capacity(n);
        for _ in 0..n {
            k.push(it.next().unwrap().parse().unwrap());
        }
        let mut h: Vec<i64> = Vec::with_capacity(n);
        for _ in 0..n {
            h.push(it.next().unwrap().parse().unwrap());
        }

        let mut l0 = k[0] - h[0] + 1;
        let mut r0 = k[0];
        let mut ans: i128 = 0;

        for i in 1..n {
            let l = k[i] - h[i] + 1;
            let r = k[i];
            if l <= r0 {
                l0 = l0.min(l);
                r0 = r;
            } else {
                let len = (r0 - l0 + 1) as i128;
                ans += len * (len + 1) / 2;
                l0 = l;
                r0 = r;
            }
        }
        let len = (r0 - l0 + 1) as i128;
        ans += len * (len + 1) / 2;

        outputs.push(ans.to_string());
    }
    println!("{}", outputs.join("\n"));
}