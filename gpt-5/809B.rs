use std::io::{self, Write};

fn read_line() -> String {
    let mut s = String::new();
    io::stdin().read_line(&mut s).unwrap();
    s
}

fn ask(x: i64, y: i64) -> bool {
    println!("1 {} {}", x, y);
    io::stdout().flush().unwrap();
    let resp = read_line();
    resp.trim().starts_with('T')
}

fn find_min(mut l: i64, mut r: i64) -> i64 {
    while l < r {
        let m = (l + r) / 2;
        if ask(m, m + 1) {
            r = m;
        } else {
            l = m + 1;
        }
    }
    l
}

fn main() {
    let first = read_line();
    let mut it = first.split_whitespace();
    let n: i64 = it.next().unwrap().parse().unwrap();
    let _k: i64 = it.next().unwrap().parse().unwrap();

    let s1 = find_min(1, n);

    let mut found_right = false;
    let mut right_prev = s1;
    if s1 < n {
        let mut pos = s1;
        let mut step = 1i64;
        while pos < n {
            let res = ask(pos, pos + 1);
            if !res {
                found_right = true;
                break;
            }
            right_prev = pos;
            if pos == n - 1 {
                break;
            }
            pos = (pos + step).min(n - 1);
            step <<= 1;
        }
    }

    let s2: i64;
    if found_right {
        let l = right_prev + 1;
        let r = n;
        s2 = find_min(l, r);
    } else {
        let mut found_left = false;
        let mut left_pos = s1 - 1;
        if s1 > 1 {
            let mut pos = s1 - 1;
            let mut step = 1i64;
            loop {
                let res = ask(pos, pos + 1);
                if res {
                    found_left = true;
                    left_pos = pos;
                    break;
                }
                if pos == 1 {
                    break;
                }
                let next_pos = pos.saturating_sub(step);
                if next_pos < 1 {
                    pos = 1;
                } else {
                    pos = next_pos;
                }
                step <<= 1;
            }
        }
        if found_left {
            s2 = find_min(1, left_pos);
        } else {
            // If not found on right (found_right==false) and not on left (found_left==false),
            // k>=2 guarantees this case shouldn't happen. Fallback: choose neighbor.
            let cand = if s1 < n { s1 + 1 } else { s1 - 1 };
            s2 = cand;
        }
    }

    println!("2 {} {}", s1, s2);
    io::stdout().flush().unwrap();
}