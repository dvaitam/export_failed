use std::cmp::min;
use std::collections::VecDeque;
use std::io::{self, Read};

struct Scanner {
    buf: Vec<u8>,
    idx: usize,
}
impl Scanner {
    fn new() -> Self {
        let mut input = String::new();
        io::stdin().read_to_string(&mut input).unwrap();
        Self { buf: input.into_bytes(), idx: 0 }
    }
    fn next_usize(&mut self) -> usize {
        let n = self.buf.len();
        while self.idx < n && self.buf[self.idx].is_ascii_whitespace() {
            self.idx += 1;
        }
        let mut x = 0usize;
        while self.idx < n && !self.buf[self.idx].is_ascii_whitespace() {
            x = x * 10 + (self.buf[self.idx] - b'0') as usize;
            self.idx += 1;
        }
        x
    }
}

#[derive(Clone, Copy)]
struct Frame {
    v: usize,
    pe: usize,
    it: usize,
}

fn main() {
    let mut sc = Scanner::new();
    let n = sc.next_usize();
    let m = sc.next_usize();

    let mut edges: Vec<(usize, usize)> = Vec::with_capacity(m);
    let mut adj: Vec<Vec<(usize, usize)>> = vec![Vec::new(); n];

    for eid in 0..m {
        let mut u = sc.next_usize();
        let mut v = sc.next_usize();
        u -= 1;
        v -= 1;
        edges.push((u, v));
        adj[u].push((v, eid));
        adj[v].push((u, eid));
    }

    // Iterative DFS to find bridges
    let mut visited = vec![false; n];
    let mut tin = vec![0usize; n];
    let mut low = vec![0usize; n];
    let mut timer: usize = 1;
    let mut is_bridge = vec![false; m];

    let invalid: usize = usize::MAX;

    for s in 0..n {
        if visited[s] {
            continue;
        }
        let mut st: Vec<Frame> = Vec::new();
        visited[s] = true;
        tin[s] = timer;
        low[s] = timer;
        timer += 1;
        st.push(Frame { v: s, pe: invalid, it: 0 });

        while let Some(_) = st.last() {
            // Use a block to manage borrows
            let process_next = {
                let fr = st.last_mut().unwrap();
                let v = fr.v;
                if fr.it < adj[v].len() {
                    let (to, eid) = adj[v][fr.it];
                    fr.it += 1;
                    if eid == fr.pe {
                        true
                    } else if !visited[to] {
                        visited[to] = true;
                        tin[to] = timer;
                        low[to] = timer;
                        timer += 1;
                        st.push(Frame { v: to, pe: eid, it: 0 });
                        true
                    } else {
                        low[v] = min(low[v], tin[to]);
                        true
                    }
                } else {
                    false
                }
            };
            if process_next {
                continue;
            }
            let frame = st.pop().unwrap();
            if let Some(par) = st.last_mut() {
                let p = par.v;
                // frame.pe is the edge connecting p - frame.v
                if frame.pe != invalid {
                    if low[frame.v] > tin[p] {
                        is_bridge[frame.pe] = true;
                    }
                    if low[p] > low[frame.v] {
                        low[p] = low[frame.v];
                    }
                }
            }
        }
    }

    // Compress into 2-edge-connected components (skip bridges)
    let mut comp = vec![usize::MAX; n];
    let mut comp_id = 0usize;
    let mut stack: Vec<usize> = Vec::new();

    for i in 0..n {
        if comp[i] != usize::MAX {
            continue;
        }
        comp[i] = comp_id;
        stack.clear();
        stack.push(i);
        while let Some(v) = stack.pop() {
            for &(to, eid) in &adj[v] {
                if is_bridge[eid] {
                    continue;
                }
                if comp[to] == usize::MAX {
                    comp[to] = comp_id;
                    stack.push(to);
                }
            }
        }
        comp_id += 1;
    }

    if comp_id <= 1 {
        println!("0");
        return;
    }

    // Build bridge tree
    let mut tree: Vec<Vec<usize>> = vec![Vec::new(); comp_id];
    for eid in 0..m {
        if is_bridge[eid] {
            let (u, v) = edges[eid];
            let cu = comp[u];
            let cv = comp[v];
            tree[cu].push(cv);
            tree[cv].push(cu);
        }
    }

    // BFS to find farthest node
    fn bfs(start: usize, g: &Vec<Vec<usize>>) -> (usize, i32) {
        let n = g.len();
        let mut dist = vec![-1i32; n];
        let mut q = VecDeque::new();
        dist[start] = 0;
        q.push_back(start);
        let mut best_node = start;
        while let Some(v) = q.pop_front() {
            let dv = dist[v];
            if dv > dist[best_node] {
                best_node = v;
            }
            for &to in &g[v] {
                if dist[to] == -1 {
                    dist[to] = dv + 1;
                    q.push_back(to);
                }
            }
        }
        (best_node, dist[best_node])
    }

    let (a, _) = bfs(0, &tree);
    let (_, diameter) = bfs(a, &tree);

    println!("{}", diameter);
}