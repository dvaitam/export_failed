use std::io::{self, Read, Write};

fn main() {
    // Read input
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();
    let n: usize = it.next().unwrap().parse().unwrap();

    let mut adj = vec![Vec::<usize>::new(); n + 1];
    for _ in 0..n - 1 {
        let a: usize = it.next().unwrap().parse().unwrap();
        let b: usize = it.next().unwrap().parse().unwrap();
        adj[a].push(b);
        adj[b].push(a);
    }

    // Root the tree at 1 and compute parents
    let root = 1usize;
    let mut parent = vec![0usize; n + 1];
    let mut stack = Vec::with_capacity(n);
    parent[root] = root;
    stack.push(root);
    while let Some(v) = stack.pop() {
        for &u in &adj[v] {
            if parent[u] == 0 {
                parent[u] = v;
                stack.push(u);
            }
        }
    }

    let mut out = String::new();
    // Number of nodes in T-decomposition
    out.push_str(&format!("{}\n", n));

    // Bags: one per vertex; root bag is {root}, others are {v, parent[v]}
    for v in 1..=n {
        if v == root {
            out.push_str(&format!("1 {}\n", v));
        } else {
            out.push_str(&format!("2 {} {}\n", v, parent[v]));
        }
    }

    // Tree edges between bags: connect bag(v) to bag(parent(v)) for v != root
    for v in 1..=n {
        if v != root {
            out.push_str(&format!("{} {}\n", v, parent[v]));
        }
    }

    let mut writer = io::BufWriter::new(io::stdout());
    writer.write_all(out.as_bytes()).unwrap();
}