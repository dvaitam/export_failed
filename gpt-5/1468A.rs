use std::cmp::max;
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
    fn is_whitespace(b: u8) -> bool {
        b == b' ' || b == b'\n' || b == b'\t' || b == b'\r'
    }
    fn next<T: std::str::FromStr>(&mut self) -> T {
        while self.idx < self.buf.len() && Self::is_whitespace(self.buf[self.idx]) {
            self.idx += 1;
        }
        let start = self.idx;
        while self.idx < self.buf.len() && !Self::is_whitespace(self.buf[self.idx]) {
            self.idx += 1;
        }
        std::str::from_utf8(&self.buf[start..self.idx]).unwrap().parse().ok().unwrap()
    }
}

#[derive(Clone, Copy)]
struct Node {
    l: u32,
    r: u32,
    val: i32,
}

struct PersistentSegTree {
    nodes: Vec<Node>,
    m: i32,
}
impl PersistentSegTree {
    fn new(m: usize, capacity_nodes: usize) -> Self {
        let mut nodes = Vec::with_capacity(capacity_nodes.max(1));
        nodes.push(Node { l: 0, r: 0, val: 0 });
        Self { nodes, m: m as i32 }
    }
    fn query(&self, idx: u32, l: i32, r: i32, ql: i32, qr: i32) -> i32 {
        if idx == 0 || ql > r || qr < l {
            return 0;
        }
        if ql <= l && r <= qr {
            return self.nodes[idx as usize].val;
        }
        let mid = (l + r) >> 1;
        let left = self.nodes[idx as usize].l;
        let right = self.nodes[idx as usize].r;
        let mut res = 0;
        if ql <= mid {
            let v = self.query(left, l, mid, ql, qr);
            if v > res { res = v; }
        }
        if qr > mid {
            let v = self.query(right, mid + 1, r, ql, qr);
            if v > res { res = v; }
        }
        res
    }
    fn update(&mut self, idx: u32, l: i32, r: i32, pos: i32, val: i32) -> u32 {
        if l == r {
            let old_val = if idx == 0 { 0 } else { self.nodes[idx as usize].val };
            let new_val = if val > old_val { val } else { old_val };
            self.nodes.push(Node { l: 0, r: 0, val: new_val });
            return (self.nodes.len() - 1) as u32;
        }
        let mid = (l + r) >> 1;
        let (old_l, old_r) = if idx == 0 {
            (0u32, 0u32)
        } else {
            let n = self.nodes[idx as usize];
            (n.l, n.r)
        };
        let (new_l, new_r);
        if pos <= mid {
            new_l = self.update(old_l, l, mid, pos, val);
            new_r = old_r;
        } else {
            new_l = old_l;
            new_r = self.update(old_r, mid + 1, r, pos, val);
        }
        let left_val = if new_l == 0 { 0 } else { self.nodes[new_l as usize].val };
        let right_val = if new_r == 0 { 0 } else { self.nodes[new_r as usize].val };
        let new_val = if left_val > right_val { left_val } else { right_val };
        self.nodes.push(Node { l: new_l, r: new_r, val: new_val });
        (self.nodes.len() - 1) as u32
    }
}

struct MaxSeg {
    n: usize,
    data: Vec<i32>,
}
impl MaxSeg {
    fn new(size: usize) -> Self {
        let mut n = 1usize;
        while n < size { n <<= 1; }
        Self { n, data: vec![0; n << 1] }
    }
    fn point_update(&mut self, pos: usize, val: i32) {
        let mut i = self.n + (pos - 1);
        self.data[i] = max(self.data[i], val);
        i >>= 1;
        while i > 0 {
            let v = max(self.data[i << 1], self.data[(i << 1) | 1]);
            self.data[i] = v;
            i >>= 1;
        }
    }
    fn range_max(&self, l: usize, r: usize) -> i32 {
        if l > r { return 0; }
        let mut l = l + self.n - 1;
        let mut r = r + self.n - 1;
        let mut res = 0;
        while l <= r {
            if (l & 1) == 1 {
                res = max(res, self.data[l]);
                l += 1;
            }
            if (r & 1) == 0 {
                res = max(res, self.data[r]);
                if r == 0 { break; }
                r -= 1;
            }
            l >>= 1;
            r >>= 1;
            if l == 0 || r == 0 { break; }
        }
        res
    }
}

fn main() {
    let mut scanner = Scanner::new();
    let t: usize = scanner.next();
    let mut outputs = Vec::with_capacity(t);
    for _ in 0..t {
        let n: usize = scanner.next();
        let mut a = vec![0usize; n];
        let mut m = 0usize;
        for i in 0..n {
            let v: usize = scanner.next();
            a[i] = v;
            if v > m { m = v; }
        }

        let mut last_gt = MaxSeg::new(m.max(1));
        // Estimate capacity for nodes: ~ (ceil_log2(m)+4) * n + 1
        let mut depth = 0usize;
        let mut p = 1usize;
        while p < m.max(1) { p <<= 1; depth += 1; }
        let cap = (depth + 4).saturating_mul(n).max(1);
        let mut pst = PersistentSegTree::new(m.max(1), cap);
        let mut roots = vec![0u32; n + 1];

        let mut ans = 0i32;
        for i in 1..=n {
            let v = a[i - 1] as i32;
            let v_usize = v as usize;

            let g = if v_usize < m { last_gt.range_max(v_usize + 1, m) } else { 0 };
            let up = 1 + if v_usize >= 1 { pst.query(roots[i - 1], 1, pst.m, 1, v) } else { 0 };
            let down = if g > 0 {
                2 + if v_usize >= 1 { pst.query(roots[(g as usize) - 1], 1, pst.m, 1, v) } else { 0 }
            } else { 0 };
            let dp = max(up, down);
            ans = max(ans, dp);
            roots[i] = pst.update(roots[i - 1], 1, pst.m, v, dp);
            last_gt.point_update(v_usize, i as i32);
        }

        outputs.push(ans.to_string());
    }
    println!("{}", outputs.join("\n"));
}