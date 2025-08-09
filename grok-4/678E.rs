use std::io;

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lines();

    let n: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();
    let mut p: Vec<Vec<f64>> = vec![vec![0.0; n + 1]; n + 1];
    for i in 1..=n {
        let line: String = lines.next().unwrap().unwrap();
        let probs: Vec<f64> = line.trim().split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        for j in 1..=n {
            p[i][j] = probs[j - 1];
        }
    }

    let maxmask: usize = (1 << n) - 1;
    let mut dp: Vec<Vec<f64>> = vec![vec![0.0; maxmask + 1]; n + 1];

    for mask in 0..=maxmask {
        let bits = mask.count_ones() as usize;
        for c in 0..=n {
            let mut valid = true;
            if c != 0 && (mask & (1 << (c - 1))) != 0 {
                valid = false;
            }
            if !valid {
                dp[c][mask] = 0.0;
                continue;
            }
            if bits == 0 {
                dp[c][mask] = if c == 1 { 1.0 } else { 0.0 };
                continue;
            }
            let mut val = 0.0;
            if c != 0 {
                let mut mx: f64 = 0.0;
                for ib in 0..n {
                    if (mask & (1 << ib)) != 0 {
                        let x = ib + 1;
                        let newmask = mask & !(1 << ib);
                        let prob = p[c][x] * dp[c][newmask] + p[x][c] * dp[x][newmask];
                        mx = mx.max(prob);
                    }
                }
                val = mx;
            } else {
                if bits == 1 {
                    let mut x = 0;
                    for ib in 0..n {
                        if (mask & (1 << ib)) != 0 {
                            x = ib + 1;
                        }
                    }
                    val = if x == 1 { 1.0 } else { 0.0 };
                } else if bits >= 2 {
                    let mut mx: f64 = 0.0;
                    for ib in 0..n {
                        if (mask & (1 << ib)) == 0 { continue; }
                        let x = ib + 1;
                        for jb in ib + 1..n {
                            if (mask & (1 << jb)) == 0 { continue; }
                            let y = jb + 1;
                            let newmask = mask & !((1 << ib) | (1 << jb));
                            let prob = p[x][y] * dp[x][newmask] + p[y][x] * dp[y][newmask];
                            mx = mx.max(prob);
                        }
                    }
                    val = mx;
                } else {
                    val = 0.0;
                }
            }
            dp[c][mask] = val;
        }
    }

    println!("{:.10}", dp[0][maxmask]);
}