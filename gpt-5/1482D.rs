use std::cmp::Ordering;
use std::collections::BTreeSet;
use std::io::{self, Read};

fn gcd(mut a: i64, mut b: i64) -> i64 {
    while b != 0 {
        let t = a % b;
        a = b;
        b = t;
    }
    a.abs()
}

struct DSU {
    p: Vec<usize>,
}
impl DSU {
    fn new(n: usize) -> Self {
        let mut p = Vec::with_capacity(n + 1);
        for i in 0..=n {
            p.push(i);
        }
        DSU { p }
    }
    fn find(&mut self, x: usize) -> usize {
        if self.p[x] == x {
            x
        } else {
            let r = self.find(self.p[x]);
            self.p[x] = r;
            r
        }
    }
    fn erase(&mut self, x: usize) {
        let n = self.p.len() - 1;
        let nxt = if x + 1 <= n { x + 1 } else { n };
        let r = self.find(nxt);
        self.p[x] = r;
    }
}

fn next_alive(i: usize, n: usize, dsu: &mut DSU) -> usize {
    let mut j = if i + 1 < n { dsu.find(i + 1) } else { dsu.find(0) };
    if j == n {
        j = dsu.find(0);
    }
    j
}

fn main() {
    // Fast input
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let t: usize = it.next().unwrap().parse().unwrap();
    let mut out = String::new();

    for _ in 0..t {
        let n: usize = it.next().unwrap().parse().unwrap();
        let mut a = vec![0i64; n];
        for i in 0..n {
            a[i] = it.next().unwrap().parse::<i64>().unwrap();
        }

        if n == 0 {
            out.push_str("0\n");
            continue;
        }

        let mut dsu = DSU::new(n);
        let mut alive = vec![true; n];
        let mut set: BTreeSet<usize> = BTreeSet::new();

        for i in 0..n {
            let j = if n == 1 { i } else { next_alive(i, n, &mut dsu) };
            if gcd(a[i], a[j]) == 1 {
                set.insert(i);
            }
        }

        let mut cur: usize = 0;
        let mut ans: Vec<usize> = Vec::new();

        while !set.is_empty() {
            let i = if let Some(&v) = set.range(cur..).next() {
                v
            } else if let Some(&v) = set.iter().next() {
                v
            } else {
                break;
            };
            set.remove(&i);
            if !alive[i] {
                continue;
            }

            let j = if n == 1 { i } else { next_alive(i, n, &mut dsu) };
            if j >= n || !alive[j] {
                continue;
            }
            if gcd(a[i], a[j]) != 1 {
                continue;
            }

            ans.push(j + 1);
            set.remove(&j);
            alive[j] = false;
            dsu.erase(j);

            if j == i {
                break;
            }

            let r = if n == 1 { i } else { next_alive(i, n, &mut dsu) };
            if alive[i] && gcd(a[i], a[r]) == 1 {
                set.insert(i);
            }
            cur = r;
        }

        out.push_str(&format!("{}", ans.len()));
        if !ans.is_empty() {
            out.push(' ');
            for (k, v) in ans.iter().enumerate() {
                if k > 0 {
                    out.push(' ');
                }
                out.push_str(&v.to_string());
            }
        }
        out.push('\n');
    }

    print!("{}", out);
}