use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace().map(|s| s.parse::<usize>().unwrap());
    let n = it.next().unwrap();
    let k = it.next().unwrap();

    let mut edges: Vec<(usize, usize)> = Vec::with_capacity(n - 1);
    let d: usize;

    if k == 2 {
        d = n - 1;
        for i in 1..n {
            edges.push((i, i + 1));
        }
    } else {
        let rem = n - 1 - k;
        let p = rem / k;
        let r = rem % k;
        d = 2 * p + 2 + std::cmp::min(2, r);

        let mut next_id = 2;
        for i in 0..k {
            let extra = p + if i < r { 1 } else { 0 };
            let mut prev = 1;
            for _ in 0..extra {
                edges.push((prev, next_id));
                prev = next_id;
                next_id += 1;
            }
            edges.push((prev, next_id));
            next_id += 1;
        }
    }

    let mut output = String::new();
    output.push_str(&format!("{}\n", d));
    for (u, v) in edges {
        output.push_str(&format!("{} {}\n", u, v));
    }
    print!("{}", output);
}