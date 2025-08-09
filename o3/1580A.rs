use std::io::{self, Read};
fn solve_case(grid: &Vec<Vec<u8>>, n: usize, m: usize) -> i32 {
    let mut pref = vec![vec![0i32; m]; n + 1];
    for i in 0..n {
        for j in 0..m {
            pref[i + 1][j] = pref[i][j] + grid[i][j] as i32;
        }
    }
    let mut best = std::i32::MAX;
    let mut vert_zero = vec![0i32; m];
    let mut sum_col = vec![0i32; m];
    let mut ps = vec![0i32; m + 1];
    for r1 in 0..n {
        for r2 in (r1 + 4)..n {
            let h = (r2 - r1 + 1) as i32;
            for j in 0..m {
                let interior_ones = pref[r2][j] - pref[r1 + 1][j];
                vert_zero[j] = (h - 2) - interior_ones;
                let top_bottom_missing =
                    2 - (grid[r1][j] as i32 + grid[r2][j] as i32);
                sum_col[j] = interior_ones + top_bottom_missing;
            }
            ps[0] = 0;
            for j in 0..m {
                ps[j + 1] = ps[j] + sum_col[j];
            }
            let mut min_val = std::i32::MAX;
            for c2 in 3..m {
                let c1 = c2 - 3;
                let candidate = vert_zero[c1] - ps[c1 + 1];
                if candidate < min_val {
                    min_val = candidate;
                }
                let cost = vert_zero[c2] + ps[c2] + min_val;
                if cost < best {
                    best = cost;
                }
            }
        }
    }
    best
}
fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();
    let t: usize = it.next().unwrap().parse().unwrap();
    for _ in 0..t {
        let n: usize = it.next().unwrap().parse().unwrap();
        let m: usize = it.next().unwrap().parse().unwrap();
        let mut grid = vec![vec![0u8; m]; n];
        for i in 0..n {
            let s = it.next().unwrap().as_bytes();
            for j in 0..m {
                grid[i][j] = (s[j] - b'0') as u8;
            }
        }
        let ans = solve_case(&grid, n, m);
        println!("{}", ans);
    }
}