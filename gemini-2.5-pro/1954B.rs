use std::io::{self, BufReader, BufRead};
use std::fmt::Write;

fn solve() {
    let n: usize = read_line().split_whitespace().next().unwrap().parse().unwrap();
    let a: Vec<i32> = read_line()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    if n == 0 {
        println!("0");
        return;
    }

    let mut runs = Vec::new();
    let mut counts = Vec::new();
    if n > 0 {
        runs.push(a[0]);
        counts.push(1);
    }
    for i in 1..n {
        if a[i] == a[i - 1] {
            *counts.last_mut().unwrap() += 1;
        } else {
            runs.push(a[i]);
            counts.push(1);
        }
    }

    let k = runs.len();
    if k <= 1 {
        println!("-1");
        return;
    }

    let mut is_palindrome = true;
    for i in 0..k / 2 {
        if runs[i] != runs[k - 1 - i] {
            is_palindrome = false;
            break;
        }
    }

    if !is_palindrome {
        println!("0");
        return;
    }

    if k % 2 == 1 {
        let mid_idx = k / 2;
        if counts[mid_idx] < 2 {
             println!("-1");
        } else {
             println!("{}", counts[mid_idx] + 1);
        }
    } else {
        println!("0");
    }
}

fn main() {
    let mut reader = BufReader::new(io::stdin());
    let t: usize = read_line_from_buf(&mut reader).trim().parse().unwrap();
    let mut out = String::new();
    for _ in 0..t {
        let n: usize = read_line_from_buf(&mut reader).trim().parse().unwrap();
        let a_str = read_line_from_buf(&mut reader);
        let a: Vec<i32> = a_str.split_whitespace().map(|s| s.parse().unwrap()).collect();
        
        solve_case(n, &a, &mut out);
    }
    print!("{}", out);
}

fn solve_case(n: usize, a: &[i32], out: &mut String) {
    if n == 0 {
        writeln!(out, "0").unwrap();
        return;
    }

    let mut runs = Vec::new();
    let mut counts = Vec::new();

    if n > 0 {
        runs.push(a[0]);
        counts.push(1);
        for i in 1..n {
            if a[i] == a[i - 1] {
                if let Some(last) = counts.last_mut() {
                    *last += 1;
                }
            } else {
                runs.push(a[i]);
                counts.push(1);
            }
        }
    }

    let k = runs.len();
    if k <= 1 {
        writeln!(out, "-1").unwrap();
        return;
    }

    let mut left = 0;
    let mut right = k - 1;

    while left < right {
        if runs[left] != runs[right] || counts[left] + counts[right] < 3 {
            writeln!(out, "0").unwrap();
            return;
        }
        left += 1;
        right -= 1;
    }

    if left > right {
        writeln!(out, "0").unwrap();
        return;
    }
    
    // Now left == right, k is odd
    if counts[left] < 2 {
        writeln!(out, "-1").unwrap();
    } else {
        writeln!(out, "{}", counts[left] + 1).unwrap();
    }
}


fn read_line_from_buf(reader: &mut BufReader<io::Stdin>) -> String {
    let mut line = String::new();
    reader.read_line(&mut line).expect("Failed to read line");
    line
}

fn read_line() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    input.trim().to_string()
}