use std::cmp::{min, Ordering};
use std::collections::BinaryHeap;
use std::io::{self, Read};

#[derive(Clone, Eq, PartialEq)]
struct EntryMax {
    l: usize,
    r: usize,
}
impl Ord for EntryMax {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.l.cmp(&other.l) {
            Ordering::Equal => self.r.cmp(&other.r),
            o => o,
        }
    }
}
impl PartialOrd for EntryMax {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Clone, Eq, PartialEq)]
struct EntryMin {
    r: usize,
}
impl Ord for EntryMin {
    fn cmp(&self, other: &Self) -> Ordering {
        self.r.cmp(&other.r)
    }
}
impl PartialOrd for EntryMin {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn compute_bounds(n: usize, starts: Vec<Vec<(usize, usize)>>) -> (Vec<Option<usize>>, Vec<Option<usize>>) {
    let mut max_l = vec![None; n];
    let mut min_r = vec![None; n];
    let mut heap_max: BinaryHeap<EntryMax> = BinaryHeap::new();
    let mut heap_min: BinaryHeap<std::cmp::Reverse<EntryMin>> = BinaryHeap::new();

    for i in 0..n {
        for &(l, r) in &starts[i] {
            heap_max.push(EntryMax { l, r });
            heap_min.push(std::cmp::Reverse(EntryMin { r }));
        }
        while let Some(top) = heap_max.peek() {
            if top.r < i {
                heap_max.pop();
            } else {
                break;
            }
        }
        while let Some(top) = heap_min.peek() {
            if top.0.r < i {
                heap_min.pop();
            } else {
                break;
            }
        }
        if let Some(top) = heap_max.peek() {
            max_l[i] = Some(top.l);
        }
        if let Some(top) = heap_min.peek() {
            min_r[i] = Some(top.0.r);
        }
    }
    (max_l, min_r)
}

fn main() {
    // Fast input
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let t: usize = it.next().unwrap().parse().unwrap();
    let mut outputs = Vec::with_capacity(t);

    for _ in 0..t {
        let n: usize = it.next().unwrap().parse().unwrap();
        let m: usize = it.next().unwrap().parse().unwrap();

        if m == 0 {
            outputs.push("0".to_string());
            // consume nothing else
            continue;
        }

        let mut starts1 = vec![Vec::<(usize, usize)>::new(); n];
        let mut starts2 = vec![Vec::<(usize, usize)>::new(); n];

        for _ in 0..m {
            let a: usize = it.next().unwrap().parse::<usize>().unwrap() - 1;
            let b: usize = it.next().unwrap().parse::<usize>().unwrap() - 1;
            let u = a;
            let v = b;

            // arc1: [u..v-1]
            if v > 0 {
                starts1[u].push((u, v - 1));
            } else {
                // v == 0 cannot happen since a < b ensures v >= 1
                unreachable!();
            }

            // arc2 pieces: [0..u-1] and [v..n-1]
            if u > 0 {
                starts2[0].push((0, u - 1));
            }
            starts2[v].push((v, n - 1));
        }

        let (max_l1, min_r1) = compute_bounds(n, starts1);
        let (max_l2, min_r2) = compute_bounds(n, starts2);

        let inf = n;
        let mut best = 0usize;

        for i in 0..n {
            let left1 = match max_l1[i] {
                Some(l) => i - l,
                None => inf,
            };
            let right1 = match min_r1[i] {
                Some(r) => r - i,
                None => inf,
            };
            let left2 = match max_l2[i] {
                Some(l) => i - l,
                None => inf,
            };
            let right2 = match min_r2[i] {
                Some(r) => r - i,
                None => inf,
            };
            let left = min(left1, left2);
            let right = min(right1, right2);
            let len = 1 + left + right;
            if len > best {
                best = len;
            }
        }

        let ans = n - best;
        outputs.push(ans.to_string());
    }

    println!("{}", outputs.join("\n"));
}