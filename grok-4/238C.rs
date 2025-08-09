use std::io::{self, BufRead};
use std::cmp;

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lines();

    let n: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();

    let mut adj: Vec<Vec<(usize, bool)>> = vec![vec![]; n + 1];

    for _ in 0..(n - 1) {
        let line: String = lines.next().unwrap().unwrap();
        let mut iter = line.split_whitespace();
        let a: usize = iter.next().unwrap().parse().unwrap();
        let b: usize = iter.next().unwrap().parse().unwrap();
        adj[a].push((b, true));
        adj[b].push((a, false));
    }

    // Compute min for single source
    let min_single = if n == 1 { 0 } else { compute_min_single(&adj, n) };

    // Compute min for double sources
    let mut min_double = i32::MAX;

    for m in 1..=n {
        let d = adj[m].len();
        if d < 2 { continue; }

        let mut extras: Vec<i32> = vec![];
        let mut constant: i32 = 0;

        for &(x, out_from_m) in &adj[m] {
            // Compute for this x
            let mut children: Vec<Vec<usize>> = vec![vec![]; n + 1];
            let mut sub_vertices: Vec<usize> = vec![];
            build_tree(x, 0, m, &adj, &mut children, &mut sub_vertices);

            if sub_vertices.is_empty() { continue; }

            let initial_cost = compute_current_cost(x, &children, &adj);
            let mut all_costs: Vec<i32> = vec![0; n + 1];
            reroot(x, 0, initial_cost, &mut all_costs, &children, &adj);

            let mut min_special = i32::MAX;
            for &k in &sub_vertices {
                min_special = cmp::min(min_special, all_costs[k]);
            }
            let cost_other = all_costs[x];

            let flip_in = if !out_from_m { 0 } else { 1 };
            let flip_out = if out_from_m { 0 } else { 1 };

            let extra = flip_in + min_special - (flip_out + cost_other);
            extras.push(extra);

            constant += flip_out + cost_other;
        }

        let d_ex = extras.len();
        if d_ex < 2 { continue; }

        let mut sorted = extras.clone();
        sorted.sort();
        let min_sum = sorted[0] + sorted[1];

        let this_cost = constant + min_sum;
        min_double = cmp::min(min_double, this_cost);
    }

    let ans = if min_double == i32::MAX { min_single } else { cmp::min(min_single, min_double) };
    println!("{}", ans);
}

fn build_tree(u: usize, p: usize, excl: usize, adj: &Vec<Vec<(usize, bool)>>, children: &mut Vec<Vec<usize>>, sub_vertices: &mut Vec<usize>) {
    sub_vertices.push(u);
    for &(v, _) in &adj[u] {
        if v == p || v == excl { continue; }
        children[u].push(v);
        build_tree(v, u, excl, adj, children, sub_vertices);
    }
}

fn compute_current_cost(u: usize, children: &Vec<Vec<usize>>, adj: &Vec<Vec<(usize, bool)>>) -> i32 {
    let mut cnt: i32 = 0;
    for &v in &children[u] {
        cnt += compute_current_cost(v, children, adj);
        let out = adj[u].iter().find(|&& (nb, _)| nb == v).unwrap().1;
        if !out { cnt += 1; }
    }
    cnt
}

fn reroot(u: usize, p: usize, current_cost: i32, all_costs: &mut Vec<i32>, children: &Vec<Vec<usize>>, adj: &Vec<Vec<(usize, bool)>>) {
    all_costs[u] = current_cost;
    for &v in &children[u] {
        let out = adj[u].iter().find(|&& (nb, _)| nb == v).unwrap().1;
        let contrib_old = if !out { 1 } else { 0 };
        let new_cost = current_cost + 1 - 2 * contrib_old;
        reroot(v, u, new_cost, all_costs, children, adj);
    }
}

fn compute_min_single(adj: &Vec<Vec<(usize, bool)>>, n: usize) -> i32 {
    let root = 1;
    let mut children: Vec<Vec<usize>> = vec![vec![]; n + 1];
    let mut sub_vertices: Vec<usize> = vec![];
    build_tree_whole(root, 0, adj, &mut children, &mut sub_vertices);

    let initial_cost = compute_current_cost(root, &children, adj);
    let mut all_costs: Vec<i32> = vec![0; n + 1];
    reroot(root, 0, initial_cost, &mut all_costs, &children, adj);

    let mut min_single = i32::MAX;
    for i in 1..=n {
        if all_costs[i] != 0 || n == 1 {
            min_single = cmp::min(min_single, all_costs[i]);
        }
    }
    min_single
}

fn build_tree_whole(u: usize, p: usize, adj: &Vec<Vec<(usize, bool)>>, children: &mut Vec<Vec<usize>>, sub_vertices: &mut Vec<usize>) {
    sub_vertices.push(u);
    for &(v, _) in &adj[u] {
        if v == p { continue; }
        children[u].push(v);
        build_tree_whole(v, u, adj, children, sub_vertices);
    }
}