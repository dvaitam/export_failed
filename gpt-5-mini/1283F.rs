use std::io::{self, Read};
use std::collections::BinaryHeap;

fn main() {
    // read input
    let mut s = String::new();
    io::stdin().read_to_string(&mut s).unwrap();
    let mut it = s.split_whitespace();
    let n: usize = it.next().unwrap().parse().unwrap();
    let mut a: Vec<usize> = Vec::with_capacity(n-1);
    for _ in 0..(n-1) {
        a.push(it.next().unwrap().parse().unwrap());
    }

    // collect first-occurrence sequence
    let mut seen = vec![false; n+1];
    let mut seq: Vec<usize> = Vec::new();
    for &x in &a {
        if !seen[x] {
            seq.push(x);
            seen[x] = true;
        }
    }

    // build heap of unused nodes (nodes not in seq)
    let mut heap = BinaryHeap::new();
    for v in 1..=n {
        if !seen[v] {
            heap.push(v);
        }
    }
    if heap.is_empty() {
        println!("-1");
        return;
    }

    // build edges
    let mut edges: Vec<(usize, usize)> = Vec::with_capacity(n-1);
    // chain from seq
    if seq.len() >= 2 {
        for i in 0..(seq.len()-1) {
            edges.push((seq[i], seq[i+1]));
        }
    }
    // attach largest unused as child of last seq element
    let last_main = *seq.last().unwrap_or(&0);
    let cur = heap.pop().unwrap();
    edges.push((last_main, cur));

    // mark first occurrences skip
    let mut first_seen = vec![false; n+1];
    // iterate a in order, skip first occurrence, for others attach largest unused
    for &x in &a {
        if !first_seen[x] {
            first_seen[x] = true;
            continue;
        } else {
            match heap.pop() {
                Some(ch) => {
                    edges.push((x, ch));
                }
                None => {
                    println!("-1");
                    return;
                }
            }
        }
    }

    if edges.len() != n-1 {
        println!("-1");
        return;
    }

    // compute in-degree (child usage) to find root
    let mut in_deg = vec![0usize; n+1];
    for &(_, v) in &edges {
        if v == 0 || v > n { println!("-1"); return; }
        in_deg[v] += 1;
        if in_deg[v] > 1 {
            // a node assigned as child more than once -> invalid
            println!("-1");
            return;
        }
    }
    let mut root = 0usize;
    for v in 1..=n {
        if in_deg[v] == 0 {
            if root != 0 {
                // more than one root candidate -> invalid
                println!("-1");
                return;
            }
            root = v;
        }
    }
    if root == 0 {
        println!("-1");
        return;
    }

    // build adjacency and perform DFS to compute parent, depth, subtree_max
    let mut adj = vec![Vec::<usize>::new(); n+1];
    for &(u,v) in &edges {
        adj[u].push(v);
        adj[v].push(u);
    }
    // iterative DFS postorder
    let mut parent = vec![0usize; n+1];
    let mut depth = vec![0usize; n+1];
    let mut subtree_max = vec![0usize; n+1];
    let mut stack: Vec<(usize, usize, bool)> = Vec::new();
    stack.push((root, 0, false));
    parent[root] = 0;
    depth[root] = 0;
    while let Some((v, p, visited)) = stack.pop() {
        if !visited {
            stack.push((v, p, true));
            for &to in &adj[v] {
                if to == p { continue; }
                parent[to] = v;
                depth[to] = depth[v] + 1;
                stack.push((to, v, false));
            }
        } else {
            let mut mx = v;
            for &to in &adj[v] {
                if to == p { continue; }
                if subtree_max[to] > mx { mx = subtree_max[to]; }
            }
            subtree_max[v] = mx;
        }
    }

    // build edges info by actual parent relation (child nodes)
    let mut edge_info: Vec<(usize, usize, usize)> = Vec::with_capacity(n-1);
    for v in 1..=n {
        if v == root { continue; }
        let p = parent[v];
        if p == 0 {
            println!("-1");
            return;
        }
        edge_info.push((subtree_max[v], depth[v], p));
    }

    // sort by subtree_max desc, depth asc
    edge_info.sort_by(|a, b| {
        if a.0 != b.0 {
            b.0.cmp(&a.0) // desc by subtree_max
        } else {
            a.1.cmp(&b.1) // asc by depth
        }
    });

    // produce mains sequence
    let mut produced: Vec<usize> = edge_info.iter().map(|t| t.2).collect();
    // compare to input a
    if produced.len() != a.len() {
        println!("-1");
        return;
    }
    let mut ok = true;
    for i in 0..a.len() {
        if produced[i] != a[i] {
            ok = false;
            break;
        }
    }
    if !ok {
        println!("-1");
        return;
    }

    // final check: ensure edges represent parent->child consistent with parent[] from DFS
    // (they should by construction). Print result.
    println!("{}", root);
    for &(u,v) in &edges {
        println!("{} {}", u, v);
    }
}