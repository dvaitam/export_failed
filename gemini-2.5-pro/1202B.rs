use std::collections::VecDeque;
use std::io::{self, BufRead, Write};

fn main() {
    let stdin = io::stdin();
    let mut s_str = String::new();
    stdin.lock().read_line(&mut s_str).unwrap();
    let s = s_str.trim().as_bytes();
    let n = s.len();

    let mut counts = [0; 26];
    for &c in s {
        counts[(c - b'A') as usize] += 1;
    }

    let mut odd_char_code = None;
    let mut odd_count = 0;
    for i in 0..26 {
        if counts[i] % 2 != 0 {
            odd_count += 1;
            odd_char_code = Some(i as u8);
        }
    }

    if odd_count > 1 {
        println!("NO SOLUTION");
        return;
    }

    let mut result_deque = VecDeque::with_capacity(n);

    if let Some(code) = odd_char_code {
        let c = (b'A' + code) as char;
        result_deque.push_back(c);
        counts[code as usize] -= 1;
    }

    for i in (0..26).rev() {
        let c = (b'A' + i as u8) as char;
        for _ in 0..(counts[i] / 2) {
            result_deque.push_front(c);
            result_deque.push_back(c);
        }
    }

    let stdout = io::stdout();
    let mut handle = stdout.lock();
    for c in result_deque {
        write!(handle, "{}", c).unwrap();
    }
    writeln!(handle).unwrap();
}