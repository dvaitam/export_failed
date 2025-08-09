use std::io::{self, Read};
use std::collections::VecDeque;

fn main() {
    // Read input
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let n: usize = it.next().unwrap().parse().unwrap();

    let mut a: Vec<i64> = Vec::with_capacity(n);
    for _ in 0..n {
        a.push(it.next().unwrap().parse::<i64>().unwrap());
    }
    let a_orig = a.clone();

    let mut b: Vec<i32> = Vec::with_capacity(n);
    for _ in 0..n {
        let x: i64 = it.next().unwrap().parse().unwrap();
        if x == -1 {
            b.push(-1);
        } else {
            b.push((x as i32) - 1);
        }
    }

    // Build indegrees
    let mut indeg = vec![0usize; n];
    for &bi in &b {
        if bi != -1 {
            indeg[bi as usize] += 1;
        }
    }

    // Kahn-like processing with positive/negative split
    let mut q = VecDeque::new();
    for i in 0..n {
        if indeg[i] == 0 {
            q.push_back(i);
        }
    }

    let mut order_pos: Vec<usize> = Vec::with_capacity(n);
    let mut order_neg: Vec<usize> = Vec::new();

    while let Some(v) = q.pop_front() {
        if a[v] >= 0 {
            order_pos.push(v);
            if b[v] != -1 {
                let p = b[v] as usize;
                a[p] += a[v];
                indeg[p] -= 1;
                if indeg[p] == 0 {
                    q.push_back(p);
                }
            }
        } else {
            order_neg.push(v);
            if b[v] != -1 {
                let p = b[v] as usize;
                indeg[p] -= 1;
                if indeg[p] == 0 {
                    q.push_back(p);
                }
            }
        }
    }

    // Final order: positives first, then negatives reversed
    let mut order: Vec<usize> = Vec::with_capacity(n);
    order.extend(order_pos);
    order_neg.reverse();
    order.extend(order_neg);

    // Simulate to compute maximum ans
    let mut a_sim = a_orig.clone();
    let mut ans: i64 = 0;
    for &v in &order {
        ans += a_sim[v];
        if b[v] != -1 {
            let p = b[v] as usize;
            a_sim[p] += a_sim[v];
        }
    }

    // Output
    println!("{}", ans);
    for (i, &v) in order.iter().enumerate() {
        if i > 0 { print!(" "); }
        print!("{}", v + 1);
    }
    println!();
}