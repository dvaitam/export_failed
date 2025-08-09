use std::io::{self, Read};

fn main() {
    // Read entire input
    let mut s = String::new();
    io::stdin().read_to_string(&mut s).unwrap();
    let s = s.trim_end_matches(&['\n', '\r'][..]);

    // Extract jaws: '<' and '>'
    let signs: Vec<char> = s.chars().filter(|&c| c == '<' || c == '>').collect();
    let n = signs.len() + 1;

    if n == 0 {
        println!("false");
        return;
    }

    // DP: possible_end[i][d] = whether it's possible from position i with value d
    let mut possible_end = vec![vec![false; 10]; n];

    // Base case: last position can be any digit
    for d in 0..=9 {
        possible_end[n - 1][d] = true;
    }

    // Fill DP from back to front
    for i in (0..n - 1).rev() {
        let sign = signs[i];
        for d in 0..=9 {
            let mut ok = false;
            match sign {
                '<' => {
                    for d2 in (d + 1)..=9 {
                        if possible_end[i + 1][d2] {
                            ok = true;
                            break;
                        }
                    }
                }
                '>' => {
                    for d2 in 0..d {
                        if possible_end[i + 1][d2] {
                            ok = true;
                            break;
                        }
                    }
                }
                _ => {}
            }
            possible_end[i][d] = ok;
        }
    }

    // Check if any start digit works
    if !possible_end[0].iter().any(|&b| b) {
        println!("false");
        return;
    }

    // Reconstruct lexicographically smallest sequence
    let mut ans: Vec<u8> = Vec::with_capacity(n);
    for i in 0..n {
        let mut chosen: Option<u8> = None;
        for d in 0..=9u8 {
            if !possible_end[i][d as usize] {
                continue;
            }
            if i > 0 {
                let prev = ans[i - 1];
                let sign = signs[i - 1];
                if (sign == '<' && !(prev < d)) || (sign == '>' && !(prev > d)) {
                    continue;
                }
            }
            chosen = Some(d);
            break;
        }
        if let Some(d) = chosen {
            ans.push(d);
        } else {
            println!("false");
            return;
        }
    }

    // Output
    for (i, d) in ans.iter().enumerate() {
        if i > 0 {
            print!(" ");
        }
        print!("{}", d);
    }
    println!();
}