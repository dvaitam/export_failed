use std::io::Read;
fn main(){
    let mut s = String::new();
    std::io::stdin().read_to_string(&mut s).unwrap();
    let mut it = s.split_whitespace();
    let n: usize = it.next().unwrap().parse().unwrap();
    let k: usize = it.next().unwrap().parse().unwrap();
    let mut p = vec![0usize; n];
    let mut q = vec![0usize; n];
    for i in 0..n { p[i] = it.next().unwrap().parse::<usize>().unwrap() - 1; }
    for i in 0..n { q[i] = it.next().unwrap().parse::<usize>().unwrap() - 1; }
    let mut adj = vec![Vec::new(); n];
    for i in 0..n-1 {
        adj[p[i]].push(p[i+1]);
        adj[q[i]].push(q[i+1]);
    }
    let mut visited = vec![false; n];
    let mut order = Vec::with_capacity(n);
    for start in 0..n {
        if visited[start] { continue; }
        let mut stack: Vec<(usize, usize)> = Vec::new();
        visited[start] = true;
        stack.push((start, 0));
        while let Some((v, mut idx)) = stack.pop() {
            if idx < adj[v].len() {
                // resume
                stack.push((v, idx));
                let to = adj[v][idx];
                idx += 1;
                if !visited[to] {
                    visited[to] = true;
                    stack.push((v, idx));
                    stack.push((to, 0));
                } else {
                    // next will be processed in subsequent iterations
                }
            } else {
                order.push(v);
            }
        }
    }
    let mut comp = vec![-1isize; n];
    let mut cid = 0;
    let mut rev_adj = vec![Vec::new(); n];
    for u in 0..n {
        for &v in &adj[u] {
            rev_adj[v].push(u);
        }
    }
    for &v in order.iter().rev() {
        if comp[v] != -1 { continue; }
        let mut stack = vec![v];
        comp[v] = cid;
        while let Some(u) = stack.pop() {
            for &to in &rev_adj[u] {
                if comp[to] == -1 {
                    comp[to] = cid;
                    stack.push(to);
                }
            }
        }
        cid += 1;
    }
    let m = cid as usize;
    let mut cadj = vec![Vec::new(); m];
    for u in 0..n {
        for &v in &adj[u] {
            let a = comp[u] as usize;
            let b = comp[v] as usize;
            if a != b {
                cadj[a].push(b);
            }
        }
    }
    for vecu in &mut cadj {
        vecu.sort();
        vecu.dedup();
    }
    let mut indeg = vec![0usize; m];
    for u in 0..m {
        for &v in &cadj[u] {
            indeg[v] += 1;
        }
    }
    use std::collections::VecDeque;
    let mut dq = VecDeque::new();
    for i in 0..m { if indeg[i]==0 { dq.push_back(i); } }
    let mut dp = vec![1usize; m];
    let mut cnt = 0;
    while let Some(u) = dq.pop_front() {
        cnt += 1;
        for &v in &cadj[u] {
            if dp[v] < dp[u] + 1 { dp[v] = dp[u] + 1; }
            indeg[v] -= 1;
            if indeg[v]==0 { dq.push_back(v); }
        }
    }
    let maxdp = *dp.iter().max().unwrap_or(&1);
    if maxdp < k {
        println!("NO");
        return;
    }
    let mut comp_label = vec![0u8; m];
    for i in 0..m {
        let val = if dp[i] >= 26 { 25 } else { dp[i]-1 };
        comp_label[i] = val as u8;
    }
    let mut res = vec![b'a'; n];
    for i in 0..n {
        let c = comp[i] as usize;
        res[i] = b'a' + comp_label[c];
    }
    let sres = String::from_utf8(res).unwrap();
    println!("YES");
    println!("{}", sres);
}