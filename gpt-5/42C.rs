use std::cmp::Ordering;
use std::collections::{HashMap, VecDeque};
use std::io::{self, Read};

fn all_ones(a: &[u64; 4]) -> bool {
    a.iter().all(|&x| x == 1)
}
fn count_odds(a: &[u64; 4]) -> usize {
    a.iter().filter(|&&x| x & 1 == 1).count()
}
fn max_val(a: &[u64; 4]) -> u64 {
    *a.iter().max().unwrap()
}
fn apply_add(a: &mut [u64; 4], i: usize, ops: &mut Vec<String>) {
    a[i] += 1;
    a[(i + 1) % 4] += 1;
    ops.push(format!("+{}", i + 1));
}
fn apply_div(a: &mut [u64; 4], i: usize, ops: &mut Vec<String>) {
    // precondition: both even
    if (a[i] & 1) != 0 || (a[(i + 1) % 4] & 1) != 0 {
        // Should not happen in our logic
        // But guard anyway by skipping (won't be called)
        return;
    }
    a[i] /= 2;
    a[(i + 1) % 4] /= 2;
    ops.push(format!("/{}", i + 1));
}
fn parity_fix_edges(a: &[u64; 4]) -> [bool; 4] {
    // Solve x4+x1=r1; x1+x2=r2; x2+x3=r3; x3+x4=r4 over GF(2)
    let r1 = (a[0] & 1) as u8;
    let r2 = (a[1] & 1) as u8;
    let r3 = (a[2] & 1) as u8;
    let r4 = (a[3] & 1) as u8;
    let mut x = [0u8; 4];
    // t = 0 solution
    x[0] = 0;
    x[1] = r2;
    x[2] = r2 ^ r3;
    x[3] = r2 ^ r3 ^ r4;
    let sum0 = (x[0] + x[1] + x[2] + x[3]) as i32;
    if sum0 > 2 {
        // take t = 1 solution = x ^ (1,1,1,1)
        for i in 0..4 {
            x[i] ^= 1;
        }
    }
    [x[0] == 1, x[1] == 1, x[2] == 1, x[3] == 1]
}

fn pack_state(a: &[u32; 4]) -> u64 {
    ((a[0] as u64) << 48) | ((a[1] as u64) << 32) | ((a[2] as u64) << 16) | (a[3] as u64)
}
fn unpack_state(k: u64) -> [u32; 4] {
    [
        ((k >> 48) & 0xFFFF) as u32,
        ((k >> 32) & 0xFFFF) as u32,
        ((k >> 16) & 0xFFFF) as u32,
        (k & 0xFFFF) as u32,
    ]
}
fn bfs_solve(start: [u64; 4], bound: u32) -> Option<Vec<String>> {
    let s = [start[0] as u32, start[1] as u32, start[2] as u32, start[3] as u32];
    if s.iter().any(|&x| x == 0 || x > bound) {
        return None;
    }
    let goal = [1u32, 1, 1, 1];
    let start_key = pack_state(&s);
    let goal_key = pack_state(&goal);
    if start_key == goal_key {
        return Some(vec![]);
    }

    let mut prev: HashMap<u64, (u64, u8)> = HashMap::new();
    let mut q = VecDeque::new();
    prev.insert(start_key, (0, 255));
    q.push_back(start_key);

    while let Some(cur_k) = q.pop_front() {
        let cur = unpack_state(cur_k);
        for i in 0..4 {
            // Addition
            {
                let mut nxt = cur;
                let i2 = (i + 1) % 4;
                nxt[i] = nxt[i].saturating_add(1);
                nxt[i2] = nxt[i2].saturating_add(1);
                if nxt[i] <= bound && nxt[i2] <= bound {
                    let nk = pack_state(&nxt);
                    if !prev.contains_key(&nk) {
                        prev.insert(nk, (cur_k, (b'+' as u8) << 3 | (i as u8)));
                        if nk == goal_key {
                            // reconstruct
                            let mut ops = Vec::<String>::new();
                            let mut at = nk;
                            while at != start_key {
                                let (p, op) = prev[&at];
                                let edge = (op & 7) as usize;
                                let ch = (op >> 3) as u8 as char;
                                ops.push(format!("{}{}", ch, edge + 1));
                                at = p;
                            }
                            ops.reverse();
                            return Some(ops);
                        }
                        q.push_back(nk);
                    }
                }
            }
            // Division
            {
                let i2 = (i + 1) % 4;
                if (cur[i] & 1) == 0 && (cur[i2] & 1) == 0 {
                    let mut nxt = cur;
                    nxt[i] >>= 1;
                    nxt[i2] >>= 1;
                    let nk = pack_state(&nxt);
                    if !prev.contains_key(&nk) {
                        prev.insert(nk, (cur_k, (b'/' as u8) << 3 | (i as u8)));
                        if nk == goal_key {
                            // reconstruct
                            let mut ops = Vec::<String>::new();
                            let mut at = nk;
                            while at != start_key {
                                let (p, op) = prev[&at];
                                let edge = (op & 7) as usize;
                                let ch = (op >> 3) as u8 as char;
                                ops.push(format!("{}{}", ch, edge + 1));
                                at = p;
                            }
                            ops.reverse();
                            return Some(ops);
                        }
                        q.push_back(nk);
                    }
                }
            }
        }
    }
    None
}

fn main() {
    // Read input
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();
    let mut a = [0u64; 4];
    for i in 0..4 {
        a[i] = it.next().unwrap().parse::<u64>().unwrap();
    }

    let mut ops: Vec<String> = Vec::new();

    const OPS_LIMIT: usize = 1000;
    const BFS_START_LIMIT: u64 = 20;
    const BFS_BOUND: u32 = 40;

    // Main loop
    loop {
        if all_ones(&a) {
            break;
        }
        if ops.len() > OPS_LIMIT {
            println!("-1");
            return;
        }

        // Small-state fallback BFS
        if max_val(&a) <= BFS_START_LIMIT {
            if let Some(path) = bfs_solve(a, BFS_BOUND) {
                if ops.len() + path.len() > OPS_LIMIT {
                    println!("-1");
                    return;
                }
                // apply to a and append
                for s in path.iter() {
                    let bytes = s.as_bytes();
                    if bytes[0] == b'+' {
                        let idx = (bytes[1] - b'1') as usize;
                        apply_add(&mut a, idx, &mut ops);
                    } else {
                        let idx = (bytes[1] - b'1') as usize;
                        apply_div(&mut a, idx, &mut ops);
                    }
                }
                break;
            } else {
                println!("-1");
                return;
            }
        }

        let k = count_odds(&a);
        if k % 2 == 0 {
            // Make all even using at most 2 additions
            if k != 0 {
                let edges = parity_fix_edges(&a);
                for i in 0..4 {
                    if edges[i] {
                        apply_add(&mut a, i, &mut ops);
                        if ops.len() > OPS_LIMIT {
                            println!("-1");
                            return;
                        }
                    }
                }
            }
            // Now all even: halve all via /1 and /3
            apply_div(&mut a, 0, &mut ops);
            if ops.len() > OPS_LIMIT {
                println!("-1");
                return;
            }
            apply_div(&mut a, 2, &mut ops);
            if ops.len() > OPS_LIMIT {
                println!("-1");
                return;
            }
        } else {
            // Odd number of odd entries: adjust by dividing some even-even pair
            // Prefer the even-even adjacent pair with maximum sum
            let mut best_idx: Option<usize> = None;
            let mut best_sum: u64 = 0;
            for i in 0..4 {
                let j = (i + 1) % 4;
                if (a[i] & 1) == 0 && (a[j] & 1) == 0 {
                    let s = a[i] + a[j];
                    if best_idx.is_none() || s > best_sum {
                        best_sum = s;
                        best_idx = Some(i);
                    }
                }
            }
            if let Some(i) = best_idx {
                apply_div(&mut a, i, &mut ops);
                if ops.len() > OPS_LIMIT {
                    println!("-1");
                    return;
                }
            } else {
                // No even-even pair exists, so there must be an odd-odd pair. Make it even-even by + then next loop will divide.
                let mut choose: Option<usize> = None;
                let mut best_pair_sum: u64 = 0;
                for i in 0..4 {
                    let j = (i + 1) % 4;
                    if (a[i] & 1) == 1 && (a[j] & 1) == 1 {
                        let s = a[i] + a[j];
                        if choose.is_none() || s > best_pair_sum {
                            best_pair_sum = s;
                            choose = Some(i);
                        }
                    }
                }
                if let Some(i) = choose {
                    apply_add(&mut a, i, &mut ops);
                    if ops.len() > OPS_LIMIT {
                        println!("-1");
                        return;
                    }
                } else {
                    // Should not happen
                    println!("-1");
                    return;
                }
            }
        }
    }

    // Output operations
    for s in ops {
        println!("{}", s);
    }
}