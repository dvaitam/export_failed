use std::io::{self, Read};

fn minimal_even(length: usize) -> String {
    let half = length / 2;
    let mut s = String::with_capacity(length);
    s.extend(std::iter::repeat('4').take(half));
    s.extend(std::iter::repeat('7').take(half));
    s
}

fn build_candidate(n: &[u8]) -> Option<String> {
    let len = n.len();
    if len % 2 == 1 {
        return Some(minimal_even(len + 1));
    }
    let k = len / 2;
    let mut tried = vec![0u8; len];
    let mut selected = vec![b' '; len];
    let mut prev_greater = vec![false; len];
    let mut active = vec![false; len];

    let mut cnt4 = 0usize;
    let mut cnt7 = 0usize;
    let mut greater = false;
    let mut idx: i64 = 0;

    loop {
        if idx == len as i64 {
            return Some(String::from_utf8(selected).unwrap());
        }
        if idx < 0 {
            break;
        }
        let i = idx as usize;

        let mut placed = false;
        while tried[i] < 2 {
            let d = if tried[i] == 0 { b'4' } else { b'7' };
            tried[i] += 1;

            if d == b'4' && cnt4 == k || d == b'7' && cnt7 == k {
                continue;
            }
            if !greater && d < n[i] {
                continue;
            }

            let new_cnt4 = cnt4 + if d == b'4' { 1 } else { 0 };
            let new_cnt7 = cnt7 + if d == b'7' { 1 } else { 0 };
            let remain = len - i - 1;
            let rem4 = k - new_cnt4;
            let rem7 = k - new_cnt7;
            if rem4 > remain || rem7 > remain {
                continue;
            }

            selected[i] = d;
            active[i] = true;
            prev_greater[i] = greater;
            if d == b'4' {
                cnt4 += 1;
            } else {
                cnt7 += 1;
            }
            if !greater && d > n[i] {
                greater = true;
            }
            idx += 1;
            placed = true;
            break;
        }

        if placed {
            continue;
        }

        tried[i] = 0;
        idx -= 1;
        if idx < 0 {
            break;
        }
        let back = idx as usize;
        if active[back] {
            let ch = selected[back];
            if ch == b'4' {
                cnt4 -= 1;
            } else {
                cnt7 -= 1;
            }
            active[back] = false;
            greater = prev_greater[back];
        }
    }
    None
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let s = input.trim();
    let bytes = s.as_bytes();

    if let Some(ans) = build_candidate(bytes) {
        println!("{}", ans);
    } else {
        let len = bytes.len();
        println!("{}", minimal_even(len + 2));
    }
}