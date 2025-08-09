use std::io::{self, Read};

fn main() {
    // Read input
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();
    let n: usize = it.next().unwrap().parse().unwrap();
    let half = n / 2;

    // Store indices by residue mod 3
    let mut idx: [Vec<usize>; 3] = [Vec::new(), Vec::new(), Vec::new()];
    for i in 0..n {
        let v: i64 = it.next().unwrap().parse().unwrap();
        let r = (v % 3) as usize;
        idx[r].push(i);
    }

    let n0 = idx[0].len();
    let n12 = n - n0;

    let (z, mut black): (i32, Vec<usize>) = if n0 <= half {
        // choose Z = 0
        let mut black = idx[0].clone();
        let mut need = half - black.len();
        for &v in &idx[1] {
            if need == 0 { break }
            black.push(v);
            need -= 1;
        }
        for &v in &idx[2] {
            if need == 0 { break }
            black.push(v);
            need -= 1;
        }
        (0, black)
    } else {
        // choose Z = 2
        let mut black = Vec::with_capacity(half);
        black.extend(&idx[1]);
        black.extend(&idx[2]);
        let mut need = half - black.len();
        for &v in &idx[0] {
            if need == 0 { break }
            black.push(v);
            need -= 1;
        }
        (2, black)
    };

    // Build answer string
    let mut ans = vec![b'1'; n];
    for i in black {
        ans[i] = b'0';
    }
    let s = String::from_utf8(ans).unwrap();

    // Output
    println!("{}", z);
    println!("{}", s);
}