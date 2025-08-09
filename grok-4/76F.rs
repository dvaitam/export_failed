use std::io::{self};

#[derive(Clone)]
struct OuterNode {
    left: usize,
    right: usize,
    inner_root: usize,
}

#[derive(Clone)]
struct InnerNode {
    max_val: i32,
    left: usize,
    right: usize,
}

struct Solver {
    outer_nodes: Vec<OuterNode>,
    inner_nodes: Vec<InnerNode>,
    mr: i32,
}

impl Solver {
    fn new() -> Self {
        let mut s = Solver {
            outer_nodes: Vec::new(),
            inner_nodes: Vec::new(),
            mr: 0,
        };
        s.outer_nodes.reserve(3000000);
        s.inner_nodes.reserve(30000000);
        s
    }

    fn new_outer_node(&mut self) -> usize {
        let idx = self.outer_nodes.len();
        self.outer_nodes.push(OuterNode {
            left: 0,
            right: 0,
            inner_root: 0,
        });
        idx
    }

    fn new_inner_node(&mut self) -> usize {
        let idx = self.inner_nodes.len();
        self.inner_nodes.push(InnerNode {
            max_val: -1,
            left: 0,
            right: 0,
        });
        idx
    }

    fn update_inner(&mut self, node: usize, lo: i32, hi: i32, pos: i32, val: i32) {
        if lo == hi {
            self.inner_nodes[node].max_val = std::cmp::max(self.inner_nodes[node].max_val, val);
            return;
        }
        let mid = lo + (hi - lo) / 2;
        if pos <= mid {
            let mut l = self.inner_nodes[node].left;
            if l == 0 {
                l = self.new_inner_node();
                self.inner_nodes[node].left = l;
            }
            self.update_inner(l, lo, mid, pos, val);
        } else {
            let mut r = self.inner_nodes[node].right;
            if r == 0 {
                r = self.new_inner_node();
                self.inner_nodes[node].right = r;
            }
            self.update_inner(r, mid + 1, hi, pos, val);
        }
        let mut mv = -1;
        let l = self.inner_nodes[node].left;
        if l != 0 {
            mv = std::cmp::max(mv, self.inner_nodes[l].max_val);
        }
        let r = self.inner_nodes[node].right;
        if r != 0 {
            mv = std::cmp::max(mv, self.inner_nodes[r].max_val);
        }
        self.inner_nodes[node].max_val = mv;
    }

    fn query_inner(&self, node: usize, lo: i32, hi: i32, qlo: i32, qhi: i32) -> i32 {
        if node == 0 || lo > qhi || hi < qlo {
            return -1;
        }
        if qlo <= lo && hi <= qhi {
            return self.inner_nodes[node].max_val;
        }
        let mid = lo + (hi - lo) / 2;
        let mut res = -1;
        res = std::cmp::max(res, self.query_inner(self.inner_nodes[node].left, lo, mid, qlo, qhi));
        res = std::cmp::max(res, self.query_inner(self.inner_nodes[node].right, mid + 1, hi, qlo, qhi));
        res
    }

    fn update_outer(&mut self, node: usize, lo: i32, hi: i32, pos: i32, r_pos: i32, val: i32) {
        let mut i = self.outer_nodes[node].inner_root;
        if i == 0 {
            i = self.new_inner_node();
            self.outer_nodes[node].inner_root = i;
        }
        self.update_inner(i, 1, self.mr, r_pos, val);
        if lo == hi {
            return;
        }
        let mid = lo + (hi - lo) / 2;
        if pos <= mid {
            let mut l = self.outer_nodes[node].left;
            if l == 0 {
                l = self.new_outer_node();
                self.outer_nodes[node].left = l;
            }
            self.update_outer(l, lo, mid, pos, r_pos, val);
        } else {
            let mut r = self.outer_nodes[node].right;
            if r == 0 {
                r = self.new_outer_node();
                self.outer_nodes[node].right = r;
            }
            self.update_outer(r, mid + 1, hi, pos, r_pos, val);
        }
    }

    fn query_outer(&self, node: usize, lo: i32, hi: i32, qlo: i32, qhi: i32, qr: i32) -> i32 {
        if node == 0 || lo > qhi || hi < qlo {
            return -1;
        }
        if qlo <= lo && hi <= qhi {
            let i = self.outer_nodes[node].inner_root;
            if i == 0 {
                return -1;
            }
            return self.query_inner(i, 1, self.mr, 1, qr);
        }
        let mid = lo + (hi - lo) / 2;
        let mut res = -1;
        res = std::cmp::max(res, self.query_outer(self.outer_nodes[node].left, lo, mid, qlo, qhi, qr));
        res = std::cmp::max(res, self.query_outer(self.outer_nodes[node].right, mid + 1, hi, qlo, qhi, qr));
        res
    }

    fn compute_max(&mut self, events: &mut Vec<Event>, include_dummy: bool, ml: i32, mr: i32, ls: &Vec<i64>, rs: &Vec<i64>) -> i32 {
        self.mr = mr;
        let mut root = self.new_outer_node();
        let num = events.len();
        let mut ranked: Vec<(i64, usize)> = (0..num).map(|id| (events[id].t, id)).collect();
        ranked.sort_by_key(|&(tt, _)| tt);
        let mut dp = vec![0i32; num];
        let mut current_t = -1i64;
        let mut group: Vec<usize> = Vec::new();
        for &(tt, id) in &ranked {
            if tt != current_t {
                for &gid in &group {
                    let lval = events[gid].l;
                    let rval = events[gid].r;
                    let lrank = (ls.binary_search(&lval).unwrap() as i32) + 1;
                    let rrank = (rs.binary_search(&rval).unwrap() as i32) + 1;
                    let max_prev = self.query_outer(root, 1, ml, lrank, ml, rrank);
                    let base = if include_dummy {
                        if max_prev == -1 { 0 } else { 1 + max_prev }
                    } else {
                        1 + std::cmp::max(0, max_prev)
                    };
                    dp[gid] = base;
                }
                for &gid in &group {
                    let lval = events[gid].l;
                    let rval = events[gid].r;
                    let lrank = (ls.binary_search(&lval).unwrap() as i32) + 1;
                    let rrank = (rs.binary_search(&rval).unwrap() as i32) + 1;
                    self.update_outer(root, 1, ml, lrank, rrank, dp[gid]);
                }
                group.clear();
                current_t = tt;
            }
            group.push(id);
        }
        // last group
        for &gid in &group {
            let lval = events[gid].l;
            let rval = events[gid].r;
            let lrank = (ls.binary_search(&lval).unwrap() as i32) + 1;
            let rrank = (rs.binary_search(&rval).unwrap() as i32) + 1;
            let max_prev = self.query_outer(root, 1, ml, lrank, ml, rrank);
            let base = if include_dummy {
                if max_prev == -1 { 0 } else { 1 + max_prev }
            } else {
                1 + std::cmp::max(0, max_prev)
            };
            dp[gid] = base;
        }
        for &gid in &group {
            let lval = events[gid].l;
            let rval = events[gid].r;
            let lrank = (ls.binary_search(&lval).unwrap() as i32) + 1;
            let rrank = (rs.binary_search(&rval).unwrap() as i32) + 1;
            self.update_outer(root, 1, ml, lrank, rrank, dp[gid]);
        }
        let mut maxx = 0i32;
        for i in 0..num {
            if include_dummy && events[i].t == 0 {
                continue;
            }
            maxx = std::cmp::max(maxx, dp[i]);
        }
        maxx
    }
}

#[derive(Clone)]
struct Event {
    x: i64,
    t: i64,
    l: i64,
    r: i64,
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lines();

    let n: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();
    let mut events1 = Vec::with_capacity(n + 1);
    let mut events2 = Vec::with_capacity(n);
    for _ in 0..n {
        let line: String = lines.next().unwrap().unwrap();
        let mut iter = line.split_whitespace();
        let x: i64 = iter.next().unwrap().parse().unwrap();
        let t: i64 = iter.next().unwrap().parse().unwrap();
        events1.push(Event { x, t, l: 0, r: 0 });
        events2.push(Event { x, t, l: 0, r: 0 });
    }
    let v: i64 = lines.next().unwrap().unwrap().trim().parse().unwrap();

    // compute l r
    for e in events1.iter_mut() {
        e.l = e.x - v * e.t;
        e.r = e.x + v * e.t;
    }
    for e in events2.iter_mut() {
        e.l = e.x - v * e.t;
        e.r = e.x + v * e.t;
    }

    // for first
    events1.push(Event { x: 0, t: 0, l: 0, r: 0 });
    let mut ls1: Vec<i64> = events1.iter().map(|e| e.l).collect();
    ls1.sort();
    ls1.dedup();
    let ml1 = ls1.len() as i32;
    let mut rs1: Vec<i64> = events1.iter().map(|e| e.r).collect();
    rs1.sort();
    rs1.dedup();
    let mr1 = rs1.len() as i32;

    // for second
    let mut ls2: Vec<i64> = events2.iter().map(|e| e.l).collect();
    ls2.sort();
    ls2.dedup();
    let ml2 = ls2.len() as i32;
    let mut rs2: Vec<i64> = events2.iter().map(|e| e.r).collect();
    rs2.sort();
    rs2.dedup();
    let mr2 = rs2.len() as i32;

    let mut solver = Solver::new();
    let ans1 = solver.compute_max(&mut events1, true, ml1, mr1, &ls1, &rs1);
    solver.outer_nodes.clear();
    solver.inner_nodes.clear();
    let ans2 = solver.compute_max(&mut events2, false, ml2, mr2, &ls2, &rs2);

    println!("{} {}", ans1, ans2);
}