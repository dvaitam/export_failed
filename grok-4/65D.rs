use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lines();

    let n: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();
    let s: String = lines.next().unwrap().unwrap().trim().to_string();
    let chars: Vec<char> = s.chars().collect();

    let mut future_fixed: Vec<Vec<i32>> = vec![vec![0; 4]; n + 2];

    for i in (1..=n).rev() {
        future_fixed[i] = future_fixed[i + 1].clone();
        let ch = chars[i - 1];
        if ch != '?' {
            let h = match ch {
                'G' => 0,
                'H' => 1,
                'R' => 2,
                'S' => 3,
                _ => unreachable!(),
            };
            future_fixed[i][h] += 1;
        }
    }

    let names = vec![
        "Gryffindor".to_string(),
        "Hufflepuff".to_string(),
        "Ravenclaw".to_string(),
        "Slytherin".to_string(),
    ];

    let mut possible: Vec<String> = vec![];

    for t in 0..4 {
        let mut counts: Vec<i32> = vec![0; 4];
        for i in 1..=n {
            let ch = chars[i - 1];
            if ch != '?' {
                let h = match ch {
                    'G' => 0,
                    'H' => 1,
                    'R' => 2,
                    'S' => 3,
                    _ => unreachable!(),
                };
                counts[h] += 1;
            } else {
                let mut m = i32::MAX;
                for &c in &counts {
                    if c < m {
                        m = c;
                    }
                }
                let mut s_set: Vec<usize> = vec![];
                for h in 0..4 {
                    if counts[h] == m {
                        s_set.push(h);
                    }
                }
                let mut candidates: Vec<usize> = s_set.iter().filter(|&&h| h != t).cloned().collect();
                if candidates.is_empty() {
                    counts[t] += 1;
                } else {
                    let mut best_h = candidates[0];
                    let mut max_fut = future_fixed[i + 1][best_h];
                    for &h in &candidates {
                        let fut = future_fixed[i + 1][h];
                        if fut > max_fut || (fut == max_fut && h < best_h) {
                            max_fut = fut;
                            best_h = h;
                        }
                    }
                    counts[best_h] += 1;
                }
            }
        }
        let mut min_other = i32::MAX;
        for h in 0..4 {
            if h != t {
                if counts[h] < min_other {
                    min_other = counts[h];
                }
            }
        }
        if counts[t] <= min_other {
            possible.push(names[t].clone());
        }
    }

    possible.sort();
    for house in possible {
        println!("{}", house);
    }
}