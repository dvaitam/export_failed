use std::io::{self, Read};
use std::collections::BinaryHeap;

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let n: usize = it.next().unwrap().parse().unwrap();
    let k: usize = it.next().unwrap().parse().unwrap();

    let mut adj = vec![Vec::<usize>::new(); n + 1];
    for _ in 0..n - 1 {
        let a: usize = it.next().unwrap().parse().unwrap();
        let b: usize = it.next().unwrap().parse().unwrap();
        adj[a].push(b);
        adj[b].push(a);
    }

    let mcount = 2 * k;
    let mut marked = vec![false; n + 1];
    for _ in 0..mcount {
        let c: usize = it.next().unwrap().parse().unwrap();
        marked[c] = true;
    }

    let root = 1usize;
    let mut parent = vec![0usize; n + 1];
    let mut order = Vec::with_capacity(n);
    let mut stack: Vec<(usize, usize, bool)> = Vec::new();
    stack.push((root, 0, false));
    while let Some((v, p, visited)) = stack.pop() {
        if visited {
            order.push(v);
        } else {
            parent[v] = p;
            stack.push((v, p, true));
            for &u in adj[v].iter() {
                if u == p { continue; }
                stack.push((u, v, false));
            }
        }
    }

    let total = mcount;
    let mut cnt = vec![0usize; n + 1];
    for &v in order.iter() {
        let mut sum = if marked[v] { 1 } else { 0 };
        for &u in adj[v].iter() {
            if parent[u] == v {
                sum += cnt[u];
            }
        }
        cnt[v] = sum;
    }

    let mut best_v = 1usize;
    let mut best_val = total;
    for v in 1..=n {
        let mut mx = total - cnt[v];
        for &u in adj[v].iter() {
            if parent[u] == v {
                if cnt[u] > mx { mx = cnt[u]; }
            }
        }
        if mx < best_val {
            best_val = mx;
            best_v = v;
        }
    }

    let mut buckets: Vec<Vec<usize>> = Vec::new();
    for &u0 in adj[best_v].iter() {
        let mut vec_nodes: Vec<usize> = Vec::new();
        let mut st: Vec<(usize, usize)> = Vec::new();
        st.push((u0, best_v));
        while let Some((v, p)) = st.pop() {
            if marked[v] { vec_nodes.push(v); }
            for &w in adj[v].iter() {
                if w == p || w == best_v { continue; }
                st.push((w, v));
            }
        }
        if !vec_nodes.is_empty() {
            buckets.push(vec_nodes);
        }
    }
    if marked[best_v] {
        buckets.push(vec![best_v]);
    }

    let mut heap: BinaryHeap<(usize, usize)> = BinaryHeap::new();
    for (i, vec_nodes) in buckets.iter().enumerate() {
        if !vec_nodes.is_empty() {
            heap.push((vec_nodes.len(), i));
        }
    }

    let mut pairs: Vec<(usize, usize, usize)> = Vec::with_capacity(k);
    for _ in 0..k {
        let (sa, ia) = heap.pop().unwrap();
        let (sb, ib) = heap.pop().unwrap();
        let ua = buckets[ia].pop().unwrap();
        let vb = buckets[ib].pop().unwrap();
        pairs.push((ua, vb, best_v));
        if sa - 1 > 0 {
            heap.push((sa - 1, ia));
        }
        if sb - 1 > 0 {
            heap.push((sb - 1, ib));
        }
    }

    println!("1");
    println!("{}", best_v);
    for (u, v2, x) in pairs {
        println!("{} {} {}", u, v2, x);
    }
}