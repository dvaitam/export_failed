use std::io::{self, Read};
use std::collections::{BinaryHeap, VecDeque};
use std::cmp::Reverse;

fn dijkstra(n: usize, adj: &Vec<Vec<(usize,i64)>>, src: usize) -> Vec<i64> {
    let mut dist = vec![i64::MAX / 4; n];
    let mut pq = BinaryHeap::new();
    dist[src] = 0;
    pq.push(Reverse((0i64, src)));
    while let Some(Reverse((d,u))) = pq.pop() {
        if d != dist[u] { continue; }
        for &(v,w) in &adj[u] {
            let nd = d + w;
            if nd < dist[v] {
                dist[v] = nd;
                pq.push(Reverse((nd, v)));
            }
        }
    }
    dist
}

struct HopcroftKarp {
    n: usize,
    m: usize,
    adj: Vec<Vec<usize>>,
    pair_u: Vec<isize>,
    pair_v: Vec<isize>,
    dist: Vec<i32>,
}

impl HopcroftKarp {
    fn new(n: usize, m: usize, adj: Vec<Vec<usize>>) -> Self {
        HopcroftKarp {
            n, m, adj,
            pair_u: vec![-1; n],
            pair_v: vec![-1; m],
            dist: vec![0; n],
        }
    }

    fn bfs(&mut self) -> bool {
        let inf = i32::MAX;
        let mut q = VecDeque::new();
        for u in 0..self.n {
            if self.pair_u[u] == -1 {
                self.dist[u] = 0;
                q.push_back(u);
            } else {
                self.dist[u] = inf;
            }
        }
        let mut found = false;
        while let Some(u) = q.pop_front() {
            for &v in &self.adj[u] {
                let pu = self.pair_v[v];
                if pu == -1 {
                    found = true;
                } else {
                    let pu_usize = pu as usize;
                    if self.dist[pu_usize] == inf {
                        self.dist[pu_usize] = self.dist[u] + 1;
                        q.push_back(pu_usize);
                    }
                }
            }
        }
        found
    }

    fn dfs(&mut self, u: usize) -> bool {
        let neighbors = self.adj[u].clone();
        for v in neighbors {
            let pu = self.pair_v[v];
            if pu == -1 || (self.dist[pu as usize] == self.dist[u] + 1 && self.dfs(pu as usize)) {
                self.pair_u[u] = v as isize;
                self.pair_v[v] = u as isize;
                return true;
            }
        }
        self.dist[u] = i32::MAX;
        false
    }

    fn max_matching(&mut self, cap: usize) -> usize {
        let mut result = 0;
        while self.bfs() {
            for u in 0..self.n {
                if self.pair_u[u] == -1 {
                    if self.dfs(u) {
                        result += 1;
                        if result >= cap { return result; }
                    }
                }
            }
        }
        result
    }
}

fn main() {
    let mut s = String::new();
    io::stdin().read_to_string(&mut s).unwrap();
    let mut it = s.split_whitespace();
    let v: usize = it.next().unwrap().parse().unwrap();
    let e: usize = it.next().unwrap().parse().unwrap();
    let n: usize = it.next().unwrap().parse().unwrap();
    let k: usize = it.next().unwrap().parse().unwrap();
    let mut starts = Vec::with_capacity(n);
    for _ in 0..n {
        let x: usize = it.next().unwrap().parse().unwrap();
        starts.push(x-1);
    }
    let mut adj = vec![Vec::new(); v];
    for _ in 0..e {
        let a: usize = it.next().unwrap().parse::<usize>().unwrap() - 1;
        let b: usize = it.next().unwrap().parse::<usize>().unwrap() - 1;
        let t: i64 = it.next().unwrap().parse().unwrap();
        adj[a].push((b,t));
        adj[b].push((a,t));
    }

    use std::collections::HashMap;
    let mut unique_map = HashMap::new();
    let mut unique_starts = Vec::new();
    for &s0 in &starts {
        if !unique_map.contains_key(&s0) {
            let idx = unique_starts.len();
            unique_map.insert(s0, idx);
            unique_starts.push(s0);
        }
    }
    let mut dists_per_unique = Vec::new();
    let mut maxd: i64 = 0;
    for &us in &unique_starts {
        let d = dijkstra(v, &adj, us);
        for &val in &d {
            if val < i64::MAX/4 && val > maxd { maxd = val; }
        }
        dists_per_unique.push(d);
    }
    let hi_bound: i64 = 1731311;
    let mut hi = if maxd as i64 > hi_bound { hi_bound } else { maxd };
    let mut lo: i64 = 0;
    // check possibility at INF (all finite edges)
    let mut adj_full = vec![Vec::new(); n];
    for (i, &st) in starts.iter().enumerate() {
        let uid = unique_map[&st];
        let dvec = &dists_per_unique[uid];
        for city in 0..v {
            if dvec[city] < i64::MAX/4 {
                adj_full[i].push(city);
            }
        }
    }
    let mut hk_full = HopcroftKarp::new(n, v, adj_full);
    let max_full = hk_full.max_matching(k);
    if max_full < k {
        println!("-1");
        return;
    }
    // binary search
    let mut ans = hi;
    while lo <= hi {
        let mid = (lo + hi) / 2;
        let mut adj_mid = vec![Vec::new(); n];
        for (i, &st) in starts.iter().enumerate() {
            let uid = unique_map[&st];
            let dvec = &dists_per_unique[uid];
            for city in 0..v {
                if dvec[city] <= mid {
                    adj_mid[i].push(city);
                }
            }
        }
        let mut hk = HopcroftKarp::new(n, v, adj_mid);
        let mm = hk.max_matching(k);
        if mm >= k {
            ans = mid;
            if mid == 0 { break; }
            hi = mid - 1;
        } else {
            lo = mid + 1;
        }
    }
    println!("{}", ans);
}