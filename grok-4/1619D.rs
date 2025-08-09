use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lines();
    let t: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();
    for _ in 0..t {
        let _ = lines.next();
        let line: String = lines.next().unwrap().unwrap();
        let mut parts = line.split_whitespace();
        let m: usize = parts.next().unwrap().parse().unwrap();
        let n: usize = parts.next().unwrap().parse().unwrap();
        let mut p: Vec<Vec<i64>> = vec![];
        for _ in 0..m {
            let line: String = lines.next().unwrap().unwrap();
            let row: Vec<i64> = line.split_whitespace().map(|x| x.parse().unwrap()).collect();
            p.push(row);
        }
        let mut max_p: Vec<i64> = vec![0; n];
        for j in 0..n {
            let mut mx = 0i64;
            for i in 0..m {
                if p[i][j] > mx {
                    mx = p[i][j];
                }
            }
            max_p[j] = mx;
        }
        let mut low: i64 = 1;
        let mut high: i64 = 1000000000;
        let mut ans = 0i64;
        while low <= high {
            let mid = low + (high - low) / 2;
            let mut all_have = true;
            for j in 0..n {
                if max_p[j] < mid {
                    all_have = false;
                    break;
                }
            }
            if !all_have {
                high = mid - 1;
                continue;
            }
            let mut max_cnt: usize = 0;
            for i in 0..m {
                let mut cnt: usize = 0;
                for j in 0..n {
                    if p[i][j] >= mid {
                        cnt += 1;
                    }
                }
                if cnt > max_cnt {
                    max_cnt = cnt;
                }
            }
            if max_cnt >= 2 {
                ans = mid;
                low = mid + 1;
            } else {
                high = mid - 1;
            }
        }
        println!("{}", ans);
    }
}