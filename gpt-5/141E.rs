use std::io::{self, Read};

struct DSU {
    p: Vec<i32>,
    sz: Vec<i32>,
}
impl DSU {
    fn new(n: usize) -> Self {
        DSU { p: (0..n as i32).collect(), sz: vec![1; n] }
    }
    fn find(&mut self, x: usize) -> usize {
        if self.p[x] as usize != x {
            self.p[x] = self.find(self.p[x] as usize) as i32;
        }
        self.p[x] as usize
    }
    fn union(&mut self, a: usize, b: usize) -> bool {
        let mut x = self.find(a);
        let mut y = self.find(b);
        if x == y { return false; }
        if self.sz[x] < self.sz[y] { std::mem::swap(&mut x, &mut y); }
        self.p[y] = x as i32;
        self.sz[x] += self.sz[y];
        true
    }
}
#[derive(Clone)]
struct Edge {
    u: usize,
    v: usize,
    is_m: bool,
    idx: usize,
}
struct LCA {
    log: usize,
    par: Vec<Vec<i32>>,
    sedge: Vec<Vec<i32>>,
    depth: Vec<i32>,
}
fn build_lca(n: usize, adj: &Vec<Vec<(usize, usize)>>, edges: &Vec<Edge>, present: &Vec<bool>) -> LCA {
    let mut log = 1usize;
    while (1usize << log) <= n { log += 1; }
    let mut par = vec![vec![-1; n]; log];
    let mut sedge = vec![vec![-1; n]; log];
    let mut depth = vec![-1; n];
    let mut stack: Vec<(usize, usize, usize)> = Vec::with_capacity(n);
    stack.push((0usize, usize::MAX, usize::MAX));
    while let Some((v, p, eid)) = stack.pop() {
        if depth[v] != -1 { continue; }
        if p == usize::MAX {
            par[0][v] = -1;
            sedge[0][v] = -1;
            depth[v] = 0;
        } else {
            par[0][v] = p as i32;
            depth[v] = depth[p] + 1;
            sedge[0][v] = if eid != usize::MAX && !edges[eid].is_m { eid as i32 } else { -1 };
        }
        for &(to, eidx) in &adj[v] {
            if !present[eidx] { continue; }
            if to == p { continue; }
            stack.push((to, v, eidx));
        }
    }
    for k in 1..log {
        for v in 0..n {
            let pr = par[k - 1][v];
            if pr != -1 {
                par[k][v] = par[k - 1][pr as usize];
                sedge[k][v] = if sedge[k - 1][v] != -1 { sedge[k - 1][v] } else { sedge[k - 1][pr as usize] };
            } else {
                par[k][v] = -1;
                sedge[k][v] = sedge[k - 1][v];
            }
        }
    }
    LCA { log, par, sedge, depth }
}
fn find_s_edge_on_path(mut u: usize, mut v: usize, l: &LCA) -> Option<usize> {
    if l.depth[u] < l.depth[v] { std::mem::swap(&mut u, &mut v); }
    let mut ans: i32 = -1;
    let mut diff = (l.depth[u] - l.depth[v]) as usize;
    for k in (0..l.log).rev() {
        if (diff >> k) & 1 == 1 {
            if ans == -1 && l.sedge[k][u] != -1 { ans = l.sedge[k][u]; }
            u = if l.par[k][u] == -1 { u } else { l.par[k][u] as usize };
        }
    }
    if u == v {
        return if ans == -1 { None } else { Some(ans as usize) };
    }
    for k in (0..l.log).rev() {
        if l.par[k][u] != l.par[k][v] {
            if ans == -1 && l.sedge[k][u] != -1 { ans = l.sedge[k][u]; }
            if ans == -1 && l.sedge[k][v] != -1 { ans = l.sedge[k][v]; }
            u = l.par[k][u] as usize;
            v = l.par[k][v] as usize;
        }
    }
    if ans == -1 && l.sedge[0][u] != -1 { ans = l.sedge[0][u]; }
    if ans == -1 && l.sedge[0][v] != -1 { ans = l.sedge[0][v]; }
    if ans == -1 { None } else { Some(ans as usize) }
}
fn kruskal(n: usize, edges: &Vec<Edge>, s_edges: &Vec<usize>, m_edges: &Vec<usize>, first_is_m: bool) -> Option<(Vec<bool>, usize)> {
    let mut dsu = DSU::new(n);
    let mut present = vec![false; edges.len()];
    let mut cnt = 0usize;
    let mut mcnt = 0usize;
    if first_is_m {
        for &i in m_edges {
            let e = &edges[i];
            if dsu.union(e.u, e.v) {
                present[i] = true; cnt += 1; mcnt += 1;
                if cnt == n - 1 { break; }
            }
        }
        if cnt < n - 1 {
            for &i in s_edges {
                let e = &edges[i];
                if dsu.union(e.u, e.v) {
                    present[i] = true; cnt += 1;
                    if cnt == n - 1 { break; }
                }
            }
        }
    } else {
        for &i in s_edges {
            let e = &edges[i];
            if dsu.union(e.u, e.v) {
                present[i] = true; cnt += 1;
                if cnt == n - 1 { break; }
            }
        }
        if cnt < n - 1 {
            for &i in m_edges {
                let e = &edges[i];
                if dsu.union(e.u, e.v) {
                    present[i] = true; cnt += 1; mcnt += 1;
                    if cnt == n - 1 { break; }
                }
            }
        }
    }
    if cnt == n - 1 { Some((present, mcnt)) } else { None }
}
fn build_adj(n: usize, edges: &Vec<Edge>, present: &Vec<bool>) -> Vec<Vec<(usize, usize)>> {
    let mut adj = vec![Vec::<(usize, usize)>::new(); n];
    for i in 0..edges.len() {
        if present[i] {
            let e = &edges[i];
            adj[e.u].push((e.v, i));
            adj[e.v].push((e.u, i));
        }
    }
    adj
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();
    let n: usize = it.next().unwrap().parse().unwrap();
    let m: usize = it.next().unwrap().parse().unwrap();
    let mut edges: Vec<Edge> = Vec::with_capacity(m);
    let mut s_edges_idx: Vec<usize> = Vec::new();
    let mut m_edges_idx: Vec<usize> = Vec::new();
    for i in 0..m {
        let x: usize = it.next().unwrap().parse::<usize>().unwrap() - 1;
        let y: usize = it.next().unwrap().parse::<usize>().unwrap() - 1;
        let tstr = it.next().unwrap();
        let ch = tstr.as_bytes()[0];
        let is_m = ch == b'M';
        edges.push(Edge { u: x, v: y, is_m, idx: i + 1 });
        if is_m { m_edges_idx.push(i); } else { s_edges_idx.push(i); }
    }
    if n % 2 == 0 {
        println!("-1");
        return;
    }
    let k = (n - 1) / 2;

    let min_res = kruskal(n, &edges, &s_edges_idx, &m_edges_idx, false);
    if min_res.is_none() {
        println!("-1");
        return;
    }
    let (mut present, mut mcnt) = min_res.unwrap();
    if mcnt > k {
        println!("-1");
        return;
    }
    let max_res = kruskal(n, &edges, &s_edges_idx, &m_edges_idx, true);
    if max_res.is_none() {
        println!("-1");
        return;
    }
    let (max_present, umax) = max_res.unwrap();
    if umax < k {
        println!("-1");
        return;
    }
    if mcnt == k {
        let mut out: Vec<usize> = Vec::with_capacity(n - 1);
        for i in 0..m {
            if present[i] { out.push(edges[i].idx); }
        }
        println!("{}", out.len());
        for (i, v) in out.iter().enumerate() {
            if i > 0 { print!(" "); }
            print!("{}", v);
        }
        println!();
        return;
    }
    let mut adj = build_adj(n, &edges, &present);
    let mut lca = build_lca(n, &adj, &edges, &present);
    let mut cand_m: Vec<usize> = Vec::new();
    for i in 0..m {
        if max_present[i] && edges[i].is_m {
            cand_m.push(i);
        }
    }
    while mcnt < k {
        let mut progress = false;
        for &eid in &cand_m {
            if present[eid] { continue; }
            let e = &edges[eid];
            if e.u == e.v { continue; }
            if let Some(se) = find_s_edge_on_path(e.u, e.v, &lca) {
                present[eid] = true;
                adj[e.u].push((e.v, eid));
                adj[e.v].push((e.u, eid));
                present[se] = false;
                mcnt += 1;
                lca = build_lca(n, &adj, &edges, &present);
                progress = true;
                break;
            }
        }
        if !progress {
            for &eid in &m_edges_idx {
                if present[eid] { continue; }
                let e = &edges[eid];
                if e.u == e.v { continue; }
                if let Some(se) = find_s_edge_on_path(e.u, e.v, &lca) {
                    present[eid] = true;
                    adj[e.u].push((e.v, eid));
                    adj[e.v].push((e.u, eid));
                    present[se] = false;
                    mcnt += 1;
                    lca = build_lca(n, &adj, &edges, &present);
                    progress = true;
                    break;
                }
            }
        }
        if !progress {
            println!("-1");
            return;
        }
    }
    let mut out: Vec<usize> = Vec::with_capacity(n - 1);
    for i in 0..m {
        if present[i] { out.push(edges[i].idx); }
    }
    println!("{}", out.len());
    for (i, v) in out.iter().enumerate() {
        if i > 0 { print!(" "); }
        print!("{}", v);
    }
    println!();
}