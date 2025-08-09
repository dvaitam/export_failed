use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lines();
    let t: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();
    for _ in 0..t {
        let line: String = lines.next().unwrap().unwrap();
        let mut iter = line.split_whitespace();
        let n: usize = iter.next().unwrap().parse().unwrap();
        let m: usize = iter.next().unwrap().parse().unwrap();
        let mut a: Vec<Vec<i64>> = vec![];
        for _ in 0..n {
            let line: String = lines.next().unwrap().unwrap();
            let row: Vec<i64> = line.split_whitespace().map(|x| x.parse().unwrap()).collect();
            a.push(row);
        }
        for i in 0..n {
            for j in 0..m {
                let mut max_nb: i64 = 0;
                let dirs = [(-1i32, 0), (1, 0), (0, -1), (0, 1)];
                for (di, dj) in dirs.iter() {
                    let ni = i as i32 + di;
                    let nj = j as i32 + dj;
                    if ni >= 0 && ni < n as i32 && nj >= 0 && nj < m as i32 {
                        max_nb = max_nb.max(a[ni as usize][nj as usize]);
                    }
                }
                if a[i][j] > max_nb {
                    a[i][j] = max_nb;
                }
            }
        }
        for row in a.iter() {
            for &val in row.iter() {
                print!("{} ", val);
            }
            println!();
        }
    }
}