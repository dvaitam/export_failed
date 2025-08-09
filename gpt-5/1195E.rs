use std::io::{self, Read};

fn main() {
    // Fast input
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let n: usize = it.next().unwrap().parse().unwrap();
    let m: usize = it.next().unwrap().parse().unwrap();
    let a: usize = it.next().unwrap().parse().unwrap();
    let b: usize = it.next().unwrap().parse().unwrap();

    let mut g: u64 = it.next().unwrap().parse().unwrap();
    let x: u64 = it.next().unwrap().parse().unwrap();
    let y: u64 = it.next().unwrap().parse().unwrap();
    let z: u64 = it.next().unwrap().parse().unwrap();

    let cols2 = m - b + 1;
    let _rows2 = n - a + 1;

    let mut rowmins: Vec<u32> = vec![0u32; n * cols2];

    // First pass: compute horizontal sliding minima for each row
    for i in 0..n {
        let mut idx: Vec<usize> = Vec::with_capacity(b + 5);
        let mut val: Vec<u32> = Vec::with_capacity(b + 5);
        let mut head: usize = 0;

        for j in 0..m {
            let cur = g as u32;

            while val.len() > head && *val.last().unwrap() >= cur {
                val.pop();
                idx.pop();
            }
            val.push(cur);
            idx.push(j);

            while idx.len() > head && idx[head] + b <= j {
                head += 1;
            }

            if j + 1 >= b {
                let pos = i * cols2 + (j + 1 - b);
                rowmins[pos] = val[head];
            }

            g = (g.wrapping_mul(x).wrapping_add(y)) % z;
        }
    }

    // Second pass: compute vertical sliding minima for each column over rowmins and sum
    let mut ans: u64 = 0;

    for c in 0..cols2 {
        let mut idx: Vec<usize> = Vec::with_capacity(a + 5);
        let mut val: Vec<u32> = Vec::with_capacity(a + 5);
        let mut head: usize = 0;

        for r in 0..n {
            let cur = rowmins[r * cols2 + c];

            while val.len() > head && *val.last().unwrap() >= cur {
                val.pop();
                idx.pop();
            }
            val.push(cur);
            idx.push(r);

            while idx.len() > head && idx[head] + a <= r {
                head += 1;
            }

            if r + 1 >= a {
                ans += val[head] as u64;
            }
        }
    }

    println!("{}", ans);
}