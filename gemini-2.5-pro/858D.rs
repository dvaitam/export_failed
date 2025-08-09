use std::io::{self, BufRead, Write, BufWriter};
use std::collections::{HashMap, HashSet};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let n: usize = lines.next().unwrap().unwrap().parse().unwrap();

    let phone_numbers: Vec<String> = lines.take(n).map(|l| l.unwrap()).collect();

    let mut counts: HashMap<String, u32> = HashMap::new();

    for number in &phone_numbers {
        let mut unique_subs_in_number = HashSet::with_capacity(45);
        for len in 1..=9 {
            for start in 0..=(9 - len) {
                unique_subs_in_number.insert(&number[start..start + len]);
            }
        }

        for sub_slice in unique_subs_in_number {
            *counts.entry(sub_slice.to_string()).or_insert(0) += 1;
        }
    }

    let stdout = io::stdout();
    let mut out = BufWriter::new(stdout.lock());

    for number in &phone_numbers {
        'search: for len in 1..=9 {
            for start in 0..=(9 - len) {
                let sub = &number[start..start + len];
                if *counts.get(sub).unwrap() == 1 {
                    writeln!(out, "{}", sub).unwrap();
                    break 'search;
                }
            }
        }
    }
}