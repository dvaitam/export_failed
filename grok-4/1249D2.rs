use std::io::{self, BufRead};

struct SegTree {
    tree: Vec<i64>,
    lazy: Vec<i64>,
    n: usize,
}

impl SegTree {
    fn new(size: usize) -> Self {
        Self {
            tree: vec![0; 4 * size],
            lazy: vec![0; 4 * size],
            n: size,
        }
    }

    fn propagate(&mut self, node: usize, start: usize, end: usize) {
        if self.lazy[node] != 0 {
            self.tree[node] += self.lazy[node];
            if start != end {
                self.lazy[2 * node] += self.lazy[node];
                self.lazy[2 * node + 1] += self.lazy[node];
            }
            self.lazy[node] = 0;
        }
    }

    fn update(&mut self, node: usize, start: usize, end: usize, l: usize, r: usize, val: i64) {
        self.propagate(node, start, end);
        if start > end || start > r || end < l {
            return;
        }
        if l <= start && end <= r {
            self.lazy[node] += val;
            self.propagate(node, start, end);
            return;
        }
        let mid = (start + end) / 2;
        self.update(2 * node, start, mid, l, r, val);
        self.update(2 * node + 1, mid + 1, end, l, r, val);
        self.tree[node] = self.tree[2 * node].max(self.tree[2 * node + 1]);
    }

    fn query(&mut self, node: usize, start: usize, end: usize, l: usize, r: usize) -> i64 {
        self.propagate(node, start, end);
        if start > end || start > r || end < l {
            return i64::MIN;
        }
        if l <= start && end <= r {
            return self.tree[node];
        }
        let mid = (start + end) / 2;
        let p1 = self.query(2 * node, start, mid, l, r);
        let p2 = self.query(2 * node + 1, mid + 1, end, l, r);
        p1.max(p2)
    }
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lines();
    let first: String = lines.next().unwrap().unwrap();
    let mut iter = first.split_whitespace();
    let n: usize = iter.next().unwrap().parse().unwrap();
    let k: usize = iter.next().unwrap().parse().unwrap();
    let mut segments: Vec<(usize, usize, usize)> = Vec::with_capacity(n);
    for i in 1..=n {
        let line: String = lines.next().unwrap().unwrap();
        let mut iter = line.split_whitespace();
        let l: usize = iter.next().unwrap().parse().unwrap();
        let r: usize = iter.next().unwrap().parse().unwrap();
        segments.push((l, r, i));
    }
    segments.sort_by_key(|&(l, r, _)| (r, l));
    const MAX: usize = 200010;
    let mut st = SegTree::new(MAX);
    let mut removed: Vec<usize> = Vec::new();
    for (l, r, idx) in segments {
        let max_cur = st.query(1, 1, MAX, l, r);
        if max_cur + 1 <= k as i64 {
            st.update(1, 1, MAX, l, r, 1);
        } else {
            removed.push(idx);
        }
    }
    println!("{}", removed.len());
    for &id in &removed {
        print!("{} ", id);
    }
    println!();
}