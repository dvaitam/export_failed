use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace().map(|s| s.parse::<usize>().unwrap());
    let n = it.next().unwrap();
    let mut a = Vec::with_capacity(n);
    for _ in 0..n {
        a.push(it.next().unwrap());
    }
    let max_val = *a.iter().max().unwrap();
    let mut spf = vec![0usize; max_val + 1];
    for i in 2..=max_val {
        if spf[i] == 0 {
            spf[i] = i;
            if i * i <= max_val {
                let mut j = i * i;
                while j <= max_val {
                    if spf[j] == 0 {
                        spf[j] = i;
                    }
                    j += i;
                }
            }
        }
    }
    let mut d1 = Vec::with_capacity(n);
    let mut d2 = Vec::with_capacity(n);
    for &val in &a {
        let p = spf[val] as i32;
        let mut x = val;
        while x % p as usize == 0 {
            x /= p as usize;
        }
        if x > 1 {
            d1.push(p);
            d2.push(x as i32);
        } else {
            d1.push(-1);
            d2.push(-1);
        }
    }
    let mut out = String::new();
    for (i, v) in d1.iter().enumerate() {
        if i > 0 {
            out.push(' ');
        }
        out.push_str(&v.to_string());
    }
    out.push('\n');
    for (i, v) in d2.iter().enumerate() {
        if i > 0 {
            out.push(' ');
        }
        out.push_str(&v.to_string());
    }
    out.push('\n');
    print!("{}", out);
}