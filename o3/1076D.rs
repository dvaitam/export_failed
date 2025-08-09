use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::io::{self, Read};

#[derive(Eq)]
struct State {
    dist: i64,
    v: usize,
}
impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .dist
            .cmp(&self.dist)
            .then_with(|| self.v.cmp(&other.v))
    }
}
impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl PartialEq for State {
    fn eq(&self, other: &Self) -> bool {
        self.dist == other.dist && self.v == other.v
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();
    let n: usize = it.next().unwrap().parse().unwrap();
    let m: usize = it.next().unwrap().parse().unwrap();
    let k: usize = it.next().unwrap().parse().unwrap();
    let mut g: Vec<Vec<(usize, i64, usize)>> = vec![Vec::new(); n + 1];
    for idx in 1..=m {
        let x: usize = it.next().unwrap().parse().unwrap();
        let y: usize = it.next().unwrap().parse().unwrap();
        let w: i64 = it.next().unwrap().parse().unwrap();
        g[x].push((y, w, idx));
        g[y].push((x, w, idx));
    }

    let mut dist = vec![i64::MAX / 4; n + 1];
    let mut par_edge = vec![0usize; n + 1];
    let mut pq = BinaryHeap::new();
    dist[1] = 0;
    pq.push(State { dist: 0, v: 1 });
    while let Some(State { dist: d, v: u }) = pq.pop() {
        if d != dist[u] {
            continue;
        }
        for &(v, w, idx) in &g[u] {
            let nd = d + w;
            if nd < dist[v] {
                dist[v] = nd;
                par_edge[v] = idx;
                pq.push(State { dist: nd, v });
            }
        }
    }

    let mut verts: Vec<(i64, usize)> = (2..=n).map(|v| (dist[v], v)).collect();
    verts.sort_unstable();

    let limit = std::cmp::min(k, n - 1);
    let mut ans = Vec::with_capacity(limit);
    for &(_, v) in verts.iter() {
        if ans.len() == limit {
            break;
        }
        ans.push(par_edge[v]);
    }

    println!("{}", ans.len());
    if !ans.is_empty() {
        for (i, e) in ans.iter().enumerate() {
            if i > 0 {
                print!(" ");
            }
            print!("{}", e);
        }
    }
    println!();
}