use std::cmp::min;
use std::io::{self, Read};
use std::collections::VecDeque;

struct Scanner {
    buf: Vec<u8>,
    idx: usize,
}
impl Scanner {
    fn new() -> Self {
        let mut input = Vec::new();
        io::stdin().read_to_end(&mut input).unwrap();
        Scanner { buf: input, idx: 0 }
    }
    fn is_whitespace(b: u8) -> bool {
        b == b' ' || b == b'\n' || b == b'\r' || b == b'\t'
    }
    fn next_usize(&mut self) -> usize {
        while self.idx < self.buf.len() && Self::is_whitespace(self.buf[self.idx]) {
            self.idx += 1;
        }
        let mut x = 0usize;
        while self.idx < self.buf.len() && !Self::is_whitespace(self.buf[self.idx]) {
            x = x * 10 + (self.buf[self.idx] - b'0') as usize;
            self.idx += 1;
        }
        x
    }
}

fn dfs_bridge(
    v: usize,
    parent_eid: usize,
    adj: &Vec<Vec<(usize, usize)>>,
    tin: &mut [i32],
    low: &mut [i32],
    timer: &mut i32,
    is_bridge: &mut [bool],
) {
    tin[v] = *timer;
    low[v] = *timer;
    *timer += 1;
    for &(to, eid) in &adj[v] {
        if eid == parent_eid {
            continue;
        }
        if tin[to] != -1 {
            low[v] = min(low[v], tin[to]);
        } else {
            dfs_bridge(to, eid, adj, tin, low, timer, is_bridge);
            low[v] = min(low[v], low[to]);
            if low[to] > tin[v] {
                is_bridge[eid] = true;
            }
        }
    }
}

fn main() {
    let mut sc = Scanner::new();
    let n = sc.next_usize();
    let m = sc.next_usize();
    let mut adj: Vec<Vec<(usize, usize)>> = vec![Vec::new(); n];
    let mut u: Vec<usize> = vec![0; m];
    let mut v: Vec<usize> = vec![0; m];
    for i in 0..m {
        let a = sc.next_usize() - 1;
        let b = sc.next_usize() - 1;
        u[i] = a;
        v[i] = b;
        adj[a].push((b, i));
        adj[b].push((a, i));
    }

    let mut tin = vec![-1i32; n];
    let mut low = vec![0i32; n];
    let mut timer = 0i32;
    let mut is_bridge = vec![false; m];

    for i in 0..n {
        if tin[i] == -1 {
            dfs_bridge(i, m, &adj, &mut tin, &mut low, &mut timer, &mut is_bridge);
        }
    }

    let mut comp_id = vec![usize::MAX; n];
    let mut comps: Vec<Vec<usize>> = Vec::new();
    let mut comp_count = 0usize;
    for i in 0..n {
        if comp_id[i] == usize::MAX {
            let mut q = VecDeque::new();
            q.push_back(i);
            comp_id[i] = comp_count;
            let mut comp_vertices = Vec::new();
            comp_vertices.push(i);
            while let Some(x) = q.pop_front() {
                for &(to, eid) in &adj[x] {
                    if is_bridge[eid] {
                        continue;
                    }
                    if comp_id[to] == usize::MAX {
                        comp_id[to] = comp_count;
                        comp_vertices.push(to);
                        q.push_back(to);
                    }
                }
            }
            comps.push(comp_vertices);
            comp_count += 1;
        }
    }

    if comp_count == 1 {
        println!("0");
        return;
    }

    let mut deg = vec![0usize; comp_count];
    for i in 0..m {
        if is_bridge[i] {
            let c1 = comp_id[u[i]];
            let c2 = comp_id[v[i]];
            if c1 != c2 {
                deg[c1] += 1;
                deg[c2] += 1;
            }
        }
    }

    let mut leaves: Vec<usize> = Vec::new();
    for c in 0..comp_count {
        if deg[c] == 1 {
            leaves.push(c);
        }
    }

    if leaves.is_empty() {
        println!("0");
        return;
    }

    let mut output = String::new();

    if comp_count == 2 {
        // Find the unique bridge endpoints between the two components
        let mut a_end = usize::MAX;
        let mut b_end = usize::MAX;
        for i in 0..m {
            if is_bridge[i] {
                let c1 = comp_id[u[i]];
                let c2 = comp_id[v[i]];
                if c1 != c2 {
                    a_end = u[i];
                    b_end = v[i];
                    break;
                }
            }
        }
        let ca = comp_id[a_end];
        let cb = comp_id[b_end];
        let size_a = comps[ca].len();
        let size_b = comps[cb].len();
        if size_a == 1 && size_b == 1 {
            println!("-1");
            return;
        }
        let u_new;
        let v_new;
        if size_a >= 2 {
            // pick u != a_end
            let mut found = None;
            for &x in &comps[ca] {
                if x != a_end {
                    found = Some(x);
                    break;
                }
            }
            u_new = found.unwrap();
            v_new = comps[cb][0];
        } else {
            // size_b >= 2
            u_new = a_end;
            let mut found = None;
            for &y in &comps[cb] {
                if y != b_end {
                    found = Some(y);
                    break;
                }
            }
            v_new = found.unwrap();
        }
        output.push_str("1\n");
        output.push_str(&(format!("{} {}\n", u_new + 1, v_new + 1)));
        print!("{}", output);
        return;
    }

    let l = leaves.len();
    let t = (l + 1) / 2;
    output.push_str(&(format!("{}\n", t)));
    for i in 0..t {
        let j = (i + t) % l;
        let c1 = leaves[i];
        let c2 = leaves[j];
        let u_rep = comps[c1][0];
        let v_rep = comps[c2][0];
        output.push_str(&(format!("{} {}\n", u_rep + 1, v_rep + 1)));
    }
    print!("{}", output);
}