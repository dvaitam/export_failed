use std::cmp::{max, Reverse};
use std::collections::BinaryHeap;
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
    fn next_i64(&mut self) -> i64 {
        let n = self.buf.len();
        while self.idx < n && Self::is_whitespace(self.buf[self.idx]) {
            self.idx += 1;
        }
        let mut sign = 1i64;
        if self.idx < n && self.buf[self.idx] == b'-' {
            sign = -1;
            self.idx += 1;
        }
        let mut x: i64 = 0;
        while self.idx < n && !Self::is_whitespace(self.buf[self.idx]) {
            x = x * 10 + (self.buf[self.idx] - b'0') as i64;
            self.idx += 1;
        }
        x * sign
    }
    fn next_usize(&mut self) -> usize {
        self.next_i64() as usize
    }
}

fn main() {
    let mut sc = Scanner::new();
    let t = sc.next_usize();
    const INF: i64 = 9_000_000_000_000_000_000;
    const NEG_INF: i64 = -9_000_000_000_000_000_000;

    let mut outputs = Vec::with_capacity(t);
    for _ in 0..t {
        let n = sc.next_usize();
        let m = sc.next_usize();
        let p = sc.next_i64();

        let mut w: Vec<i64> = Vec::with_capacity(n);
        for _ in 0..n {
            w.push(sc.next_i64());
        }

        let mut adj: Vec<Vec<(usize, i64)>> = vec![Vec::new(); n];
        for _ in 0..m {
            let a = sc.next_usize() - 1;
            let b = sc.next_usize() - 1;
            let s = sc.next_i64();
            adj[a].push((b, s));
        }

        // Unique sorted w values and index map
        let mut uniq = w.clone();
        uniq.sort_unstable();
        uniq.dedup();
        let k = uniq.len();
        let mut widx_of_node = vec![0usize; n];
        // Build map: value -> index via binary search
        for i in 0..n {
            let idx = uniq.binary_search(&w[i]).unwrap();
            widx_of_node[i] = idx;
        }

        // Dijkstra over augmented states (node, widx)
        let mut dist = vec![vec![INF; k]; n];
        let mut coins = vec![vec![NEG_INF; k]; n];

        let start_idx = widx_of_node[0];
        dist[0][start_idx] = 0;
        coins[0][start_idx] = p;

        let mut heap: BinaryHeap<(Reverse<i64>, Reverse<i64>, usize, usize)> = BinaryHeap::new();
        heap.push((Reverse(0), Reverse(-p), 0, start_idx));

        let mut ans: Option<i64> = None;

        while let Some((Reverse(shows), Reverse(neg_coin), v, wi)) = heap.pop() {
            let coin_here = -neg_coin;
            if shows != dist[v][wi] || coin_here != coins[v][wi] {
                continue;
            }
            if v == n - 1 {
                ans = Some(shows);
                break;
            }
            let wmax = uniq[wi];

            for &(u, cost) in &adj[v] {
                let mut c = coin_here;
                let mut add = 0i64;
                if c < cost {
                    let need = cost - c;
                    let extra = (need + wmax - 1) / wmax;
                    c += extra * wmax;
                    add += extra;
                }
                c -= cost;
                let new_wi = if widx_of_node[u] > wi { widx_of_node[u] } else { wi };
                let new_shows = shows + add;

                if new_shows < dist[u][new_wi] || (new_shows == dist[u][new_wi] && c > coins[u][new_wi]) {
                    dist[u][new_wi] = new_shows;
                    coins[u][new_wi] = c;
                    heap.push((Reverse(new_shows), Reverse(-c), u, new_wi));
                }
            }
        }

        outputs.push(match ans {
            Some(x) => x.to_string(),
            None => "-1".to_string(),
        });
    }

    println!("{}", outputs.join("\n"));
}