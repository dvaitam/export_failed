```rust
use std::collections::HashMap;
use std::io::{self, BufRead};

fn check_possible(
    start_idx: usize,
    needed: &HashMap<i64, i32>,
    suffix_counts: &Vec<HashMap<i64, i32>>,
) -> bool {
    if start_idx >= suffix_counts.len() {
        return needed.is_empty();
    }

    let available = &suffix_counts[start_idx];
    for (val, &count) in needed.iter() {
        if available.get(val).cloned().unwrap_or(0) < count {
            return false;
        }
    }
    true
}

fn compute_optimal(
    k: usize,
    a: &Vec<i64>,
    suffix_counts: &Vec<HashMap<i64, i32>>,
) -> Vec<i64> {
    let mut sorted_a = a.clone();
    sorted_a.sort_unstable_by(|x, y| y.cmp(x));

    let mut s_initial = HashMap::new();
    for i in 0..k {
        *s_initial.entry(sorted_a[i]).or_insert(0) += 1;
    }

    let mut b = Vec::with_capacity(k);
    let mut s_current = s_initial;
    let mut last_idx: i32 = -1;
    let n = a.len();

    for _ in 0..k {
        let mut unique_keys: Vec<_> = s_current.keys().cloned().collect();
        unique_keys.sort_unstable();

        for val_candidate in unique_keys {
            let mut found_idx: Option<usize> = None;
            let start_search_idx = (last_idx + 1) as usize;
            for i in start_search_idx..n {
                if a[i] == val_candidate {
                    found_idx = Some(i);
                    break;
                }
            }

            if let Some(i) = found_idx {
                let original_count = s_current.get(&val_candidate).cloned().unwrap();
                if original_count == 1 {
                    s_current.remove(&val_candidate);
                } else {
                    *s_current.get_mut(&val_candidate).unwrap() -= 1;
                }

                if check_possible(i + 1, &s_current, suffix_counts) {
                    b.push(val_candidate);
                    last_idx = i as i32;
                    break;
                } else {
                    *s_current.entry(val_candidate).or_insert(0) += 1;
                }
            }
        }
    }
    b
}

fn main() {
    let stdin = io::stdin();
    let mut iterator = stdin.lock().lines();

    let n: usize = iterator.next().unwrap().unwrap().parse().unwrap();
    let a: Vec<i64> = iterator
        .next()
        .unwrap()
        .unwrap()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    let m: usize = iterator.next().unwrap().unwrap().parse().unwrap();

    let mut suffix_counts: Vec<HashMap<i64, i32>> = vec