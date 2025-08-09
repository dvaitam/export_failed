use std::io::{self, Read};
use std::cmp::min;

struct Edge { to: usize, rev: usize, cap: i32 }
struct Dinic {
    n: usize,
    g: Vec<Vec<Edge>>,
    level: Vec<i32>,
    it: Vec<usize>,
}
impl Dinic {
    fn new(n: usize) -> Self {
        let mut g: Vec<Vec<Edge>> = Vec::new();
        g.resize_with(n, Vec::new);
        Dinic { n, g, level: vec![-1; n], it: vec![0; n] }
    }
    fn add_edge(&mut self, u: usize, v: usize, c: i32) {
        let a = Edge { to: v, rev: self.g[v].len(), cap: c };
        let b = Edge { to: u, rev: self.g[u].len(), cap: 0 };
        self.g[u].push(a);
        self.g[v].push(b);
    }
    fn bfs(&mut self, s: usize, t: usize) -> bool {
        for x in self.level.iter_mut() { *x = -1; }
        let mut q = std::collections::VecDeque::new();
        self.level[s] = 0;
        q.push_back(s);
        while let Some(v) = q.pop_front() {
            for e in &self.g[v] {
                if e.cap > 0 && self.level[e.to] < 0 {
                    self.level[e.to] = self.level[v] + 1;
                    q.push_back(e.to);
                }
            }
        }
        self.level[t] >= 0
    }
    fn dfs(&mut self, v: usize, t: usize, f: i32) -> i32 {
        if v == t { return f; }
        while self.it[v] < self.g[v].len() {
            let i = self.it[v];
            let cap = self.g[v][i].cap;
            let to = self.g[v][i].to;
            if cap > 0 && self.level[v] < self.level[to] {
                let ret = self.dfs(to, t, min(f, cap));
                if ret > 0 {
                    let rev = self.g[v][i].rev;
                    self.g[v][i].cap -= ret;
                    self.g[to][rev].cap += ret;
                    return ret;
                }
            }
            self.it[v] += 1;
        }
        0
    }
    fn maxflow(&mut self, s: usize, t: usize, lim: i32) -> i32 {
        let mut flow = 0;
        while self.bfs(s, t) {
            for x in self.it.iter_mut() { *x = 0; }
            loop {
                let f = self.dfs(s, t, lim - flow);
                if f == 0 { break; }
                flow += f;
                if flow >= lim { return flow; }
            }
        }
        flow
    }
    fn reachable_from(&self, s: usize) -> Vec<bool> {
        let mut vis = vec![false; self.n];
        let mut q = std::collections::VecDeque::new();
        vis[s] = true;
        q.push_back(s);
        while let Some(v) = q.pop_front() {
            for e in &self.g[v] {
                if e.cap > 0 && !vis[e.to] {
                    vis[e.to] = true;
                    q.push_back(e.to);
                }
            }
        }
        vis
    }
}

fn main() {
    let mut s = String::new();
    io::stdin().read_to_string(&mut s).unwrap();
    let mut it = s.split_whitespace();
    let t: usize = it.next().unwrap().parse().unwrap();
    let mut outputs: Vec<String> = Vec::new();
    const INF: i32 = 1_000_000_000;
    for _ in 0..t {
        let n: usize = it.next().unwrap().parse().unwrap();
        let m: usize = it.next().unwrap().parse().unwrap();
        let mut grid: Vec<Vec<char>> = Vec::new();
        for _ in 0..n {
            let row: String = it.next().unwrap().to_string();
            grid.push(row.chars().collect());
        }
        let mut adj_initial = vec![vec![false; m]; n];
        for i in 0..n {
            for j in 0..m {
                if grid[i][j] == '#' {
                    let dirs = [(-1,0),(1,0),(0,-1),(0,1)];
                    for (dx,dy) in dirs {
                        let ni = i as isize + dx;
                        let nj = j as isize + dy;
                        if ni>=0 && ni<n as isize && nj>=0 && nj<m as isize {
                            adj_initial[ni as usize][nj as usize] = true;
                        }
                    }
                }
            }
        }
        let mut best_flow = INF;
        let mut best_choice: Option<(usize, Vec<Vec<char>>)> = None;
        for parity in 0..2 {
            let nm = n*m;
            let nodes = nm*2 + 2;
            let sidx = nodes-2;
            let tidx = nodes-1;
            let mut din = Dinic::new(nodes);
            let cell_index = |i:usize,j:usize| -> usize { i*m + j };
            for i in 0..n {
                for j in 0..m {
                    if grid[i][j] == '#' { continue; }
                    let idx = cell_index(i,j);
                    let vin = idx*2;
                    let vout = vin+1;
                    let cap = if grid[i][j] == '.' {
                        if ((i+j)&1) == parity && !adj_initial[i][j] {
                            1
                        } else {
                            INF
                        }
                    } else { INF };
                    din.add_edge(vin, vout, cap);
                    if i == 0 {
                        din.add_edge(sidx, vin, INF);
                    }
                    if i == n-1 {
                        din.add_edge(vout, tidx, INF);
                    }
                    let dirs = [(-1,0),(1,0),(0,-1),(0,1)];
                    for (dx,dy) in dirs {
                        let ni = i as isize + dx;
                        let nj = j as isize + dy;
                        if ni>=0 && ni<n as isize && nj>=0 && nj<m as isize {
                            let niu = ni as usize;
                            let nju = nj as usize;
                            if grid[niu][nju] == '#' { continue; }
                            let nidx = cell_index(niu,nju);
                            let nin = nidx*2;
                            din.add_edge(vout, nin, INF);
                        }
                    }
                }
            }
            let flow = din.maxflow(sidx, tidx, INF);
            if flow < best_flow {
                if flow >= INF { continue; }
                let vis = din.reachable_from(sidx);
                let mut res_grid = grid.clone();
                for i in 0..n {
                    for j in 0..m {
                        if grid[i][j] == '#' { continue; }
                        let idx = cell_index(i,j);
                        let vin = idx*2;
                        let vout = vin+1;
                        if vis[vin] && !vis[vout] {
                            res_grid[i][j] = '#';
                        }
                    }
                }
                best_flow = flow;
                best_choice = Some((parity, res_grid));
            }
        }
        if let Some((_p, outg)) = best_choice {
            outputs.push("YES".to_string());
            for i in 0..n {
                outputs.push(outg[i].iter().collect());
            }
        } else {
            outputs.push("NO".to_string());
        }
    }
    println!("{}", outputs.join("\n"));
}