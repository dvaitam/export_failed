use std::collections::{HashSet, VecDeque};
use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lines();

    let first_line = lines.next().unwrap().unwrap();
    let mut iter = first_line.split_whitespace();
    let n: usize = iter.next().unwrap().parse().unwrap();
    let m: usize = iter.next().unwrap().parse().unwrap();

    let mut adj: Vec<Vec<(usize, i32)>> = vec![vec![]; n + 1];
    for _ in 0..m {
        let line = lines.next().unwrap().unwrap();
        let mut iter = line.split_whitespace();
        let x: usize = iter.next().unwrap().parse().unwrap();
        let y: usize = iter.next().unwrap().parse().unwrap();
        let z: i32 = iter.next().unwrap().parse().unwrap();
        adj[x].push((y, z));
        adj[y].push((x, z));
    }

    let mut dist: Vec<usize> = vec![usize::MAX; n + 1];
    dist[1] = 0;
    let mut q: VecDeque<usize> = VecDeque::new();
    q.push_back(1);
    while let Some(u) = q.pop_front() {
        for &(v, _) in &adj[u] {
            if dist[v] == usize::MAX {
                dist[v] = dist[u] + 1;
                q.push_back(v);
            }
        }
    }

    let d_n = dist[n];
    if d_n == usize::MAX {
        // Should not happen as graph is connected
        return;
    }

    let mut levels: Vec<Vec<usize>> = vec![vec![]; d_n + 1];
    for i in 1..=n {
        if dist[i] != usize::MAX {
            levels[dist[i]].push(i);
        }
    }

    let mut max_good: Vec<i32> = vec![i32::MIN; n + 1];
    max_good[1] = 0;
    let mut pred: Vec<usize> = vec![0; n + 1];

    for d in 0..d_n {
        for &u in &levels[d] {
            for &(v, z) in &adj[u] {
                if dist[v] == dist[u] + 1 {
                    let new_score = max_good[u] + z;
                    if new_score > max_good[v] {
                        max_good[v] = new_score;
                        pred[v] = u;
                    }
                }
            }
        }
    }

    // Reconstruct path
    let mut path: Vec<usize> = vec![];
    let mut cur = n;
    loop {
        path.push(cur);
        if cur == 1 {
            break;
        }
        cur = pred[cur];
        if cur == 0 {
            // Error, but assume connected
            return;
        }
    }
    path.reverse();

    // Path edge set
    let mut path_edge_set: HashSet<(usize, usize)> = HashSet::new();
    for i in 0..path.len() - 1 {
        let a = path[i];
        let b = path[i + 1];
        let aa = a.min(b);
        let bb = a.max(b);
        path_edge_set.insert((aa, bb));
    }

    // Collect changes
    let mut changes: Vec<(usize, usize, i32)> = vec![];

    // Repairs
    for i in 0..path.len() - 1 {
        let a = path[i];
        let b = path[i + 1];
        let mut zz = -1;
        for &(to, zval) in &adj[a] {
            if to == b {
                zz = zval;
                break;
            }
        }
        if zz == 0 {
            changes.push((a, b, 1));
        }
    }

    // Blow-ups
    for u in 1..=n {
        for &(v, z) in &adj[u] {
            if u < v {
                let aa = u;
                let bb = v;
                if z == 1 && !path_edge_set.contains(&(aa, bb)) {
                    changes.push((u, v, 0));
                }
            }
        }
    }

    println!("{}", changes.len());
    for (x, y, z) in changes {
        println!("{} {} {}", x, y, z);
    }
}