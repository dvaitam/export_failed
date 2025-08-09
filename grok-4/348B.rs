use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lines();

    let n: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();
    let mut a: Vec<u64> = vec![0; n + 1];
    let line: String = lines.next().unwrap().unwrap();
    let mut iter = line.split_whitespace();
    for i in 1..=n {
        a[i] = iter.next().unwrap().parse().unwrap();
    }

    let mut adj: Vec<Vec<usize>> = vec![vec![]; n + 1];
    for _ in 0..(n - 1) {
        let line: String = lines.next().unwrap().unwrap();
        let mut iter = line.split_whitespace();
        let x: usize = iter.next().unwrap().parse().unwrap();
        let y: usize = iter.next().unwrap().parse().unwrap();
        adj[x].push(y);
        adj[y].push(x);
    }

    let (.., m_root) = dfs(1, 0, &adj, &a);

    let mut total: u64 = 0;
    for i in 1..=n {
        total += a[i];
    }

    println!("{}", total - m_root);
}

fn dfs(node: usize, par: usize, adj: &Vec<Vec<usize>>, a: &Vec<u64>) -> (u64, u64) {
    let mut children: Vec<usize> = vec![];
    for &nei in &adj[node] {
        if nei != par {
            children.push(nei);
        }
    }
    let k = children.len() as u64;
    if k == 0 {
        return (1, a[node]);
    }
    let mut child_pairs: Vec<(u64, u64)> = vec![];
    for &ch in &children {
        child_pairs.push(dfs(ch, node, adj, a));
    }
    if k == 1 {
        return child_pairs[0];
    }
    let mut min_m = u64::MAX;
    for &(_, mi) in &child_pairs {
        if mi < min_m {
            min_m = mi;
        }
    }
    if min_m == 0 {
        return (1, 0);
    }
    let mut current_l: u128 = 1;
    let min_m_128 = min_m as u128;
    'lcm_loop: for &(di, _) in &child_pairs {
        let di_128 = di as u128;
        let g = gcd_u128(current_l, di_128);
        let part = di_128 / g;
        let new_l: u128;
        if current_l > u128::MAX / part {
            new_l = u128::MAX;
        } else {
            new_l = current_l * part;
        }
        current_l = new_l;
        if current_l > min_m_128 {
            current_l = u128::MAX;
            break 'lcm_loop;
        }
    }
    let l_opt: Option<u64> = if current_l == u128::MAX || current_l > min_m_128 {
        None
    } else {
        Some(current_l as u64)
    };
    let max_inter: u64 = match l_opt {
        Some(l) => (min_m / l) * l,
        None => 0,
    };
    let m_v_128 = (k as u128) * (max_inter as u128);
    let m_v = m_v_128 as u64;
    let d_v_128 = if l_opt.is_none() {
        u128::MAX
    } else {
        (k as u128) * (l_opt.unwrap() as u128)
    };
    if d_v_128 > m_v_128 || d_v_128 > (u64::MAX as u128) {
        (1, 0)
    } else {
        (d_v_128 as u64, m_v)
    }
}

fn gcd_u128(mut a: u128, mut b: u128) -> u128 {
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    a
}