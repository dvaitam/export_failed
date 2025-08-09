use std::io::{self, Read};
use std::collections::VecDeque;

fn main() {
    const MOD: i64 = 1_000_000_009;
    let mut s = String::new();
    io::stdin().read_to_string(&mut s).unwrap();
    let mut it = s.split_whitespace();
    let n: usize = it.next().unwrap().parse().unwrap();
    let m: usize = it.next().unwrap().parse().unwrap();
    let mut patterns: Vec<String> = Vec::new();
    let mut max_len = 0usize;
    for _ in 0..m {
        let p = it.next().unwrap().to_string();
        if p.len() > max_len { max_len = p.len() }
        patterns.push(p);
    }

    struct Node { next: [i32;4], link: usize, out: usize }
    let mut trie: Vec<Node> = Vec::new();
    trie.push(Node { next: [-1,-1,-1,-1], link: 0, out: 0 });

    let idx = |c: char| -> usize {
        match c {
            'A' => 0,
            'C' => 1,
            'G' => 2,
            'T' => 3,
            _ => unreachable!(),
        }
    };

    for p in patterns.iter() {
        let mut v: usize = 0;
        for ch in p.chars() {
            let c = idx(ch);
            if trie[v].next[c] == -1 {
                trie.push(Node { next: [-1,-1,-1,-1], link: 0, out: 0 });
                let ni = (trie.len()-1) as i32;
                trie[v].next[c] = ni;
            }
            v = trie[v].next[c] as usize;
        }
        if trie[v].out < p.len() { trie[v].out = p.len(); }
    }

    // build links
    let mut q: VecDeque<usize> = VecDeque::new();
    // root adjustments
    for c in 0..4 {
        if trie[0].next[c] != -1 {
            let u = trie[0].next[c] as usize;
            trie[u].link = 0;
            q.push_back(u);
        } else {
            trie[0].next[c] = 0;
        }
    }
    while let Some(v) = q.pop_front() {
        let linkv = trie[v].link;
        for c in 0..4 {
            if trie[v].next[c] != -1 {
                let u = trie[v].next[c] as usize;
                trie[u].link = trie[linkv].next[c] as usize;
                let lnk = trie[u].link;
                if trie[u].out < trie[lnk].out { trie[u].out = trie[lnk].out; }
                q.push_back(u);
            } else {
                trie[v].next[c] = trie[linkv].next[c];
            }
        }
    }

    let nodes = trie.len();
    let L = max_len;
    let mut dp = vec![vec![vec![0i64; L+1]; nodes]; n+1];
    dp[0][0][0] = 1;
    for pos in 0..n {
        for st in 0..nodes {
            for d in 0..=L {
                let val = dp[pos][st][d];
                if val == 0 { continue; }
                for c in 0..4 {
                    let nxt = trie[st].next[c] as usize;
                    let outlen = trie[nxt].out;
                    let mut nd = if d+1 > outlen { d+1 - outlen } else { 0 };
                    if nd > L { continue; }
                    let cur = &mut dp[pos+1][nxt][nd];
                    *cur = (*cur + val) % MOD;
                }
            }
        }
    }
    let mut ans: i64 = 0;
    for st in 0..nodes {
        ans = (ans + dp[n][st][0]) % MOD;
    }
    println!("{}", ans);
}