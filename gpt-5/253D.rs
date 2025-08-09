use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let n: usize = it.next().unwrap().parse().unwrap();
    let m: usize = it.next().unwrap().parse().unwrap();
    let k: i32 = it.next().unwrap().parse::<i64>().unwrap() as i32;

    let mut grid: Vec<Vec<u8>> = Vec::with_capacity(n);
    for _ in 0..n {
        grid.push(it.next().unwrap().as_bytes().to_vec());
    }

    let mut col_psa = vec![vec![0i32; m]; n + 1];
    for i in 0..n {
        for j in 0..m {
            col_psa[i + 1][j] = col_psa[i][j] + if grid[i][j] == b'a' { 1 } else { 0 };
        }
    }

    let mut ans: u128 = 0;

    for r1 in 0..n - 1 {
        for r2 in (r1 + 1)..n {
            let mut l: usize = 0;
            let mut sum: i32 = 0;
            let mut counts = [0u64; 26];
            let row1 = &grid[r1];
            let row2 = &grid[r2];

            for r in 0..m {
                sum += col_psa[r2 + 1][r] - col_psa[r1][r];
                while sum > k {
                    sum -= col_psa[r2 + 1][l] - col_psa[r1][l];
                    if row1[l] == row2[l] {
                        let idx = (row1[l] - b'a') as usize;
                        counts[idx] -= 1;
                    }
                    l += 1;
                }
                if row1[r] == row2[r] {
                    let idx = (row1[r] - b'a') as usize;
                    ans += counts[idx] as u128;
                    counts[idx] += 1;
                }
            }
        }
    }

    println!("{}", ans);
}