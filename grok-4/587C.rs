use std::io::{self};
use std::cmp::Reverse;
use std::collections::BinaryHeap;

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lines();

    let first: Vec<usize> = lines.next().unwrap().unwrap().trim().split_whitespace().map(|x| x.parse().unwrap()).collect();
    let n = first[0];
    let m = first[1];
    let q = first[2];

    let mut adj: Vec<Vec<usize>> = vec![vec![]; n + 1];
    for _ in 0..n - 1 {
        let road: Vec<usize> = lines.next().unwrap().unwrap().trim().split_whitespace().map(|x| x.parse().unwrap()).collect();
        let v = road[0];
        let u = road[1];
        adj[v].push(u);
        adj[u].push(v);
    }

    let c: Vec<usize> = lines.next().unwrap().unwrap().trim().split_whitespace().map(|x| x.parse().unwrap()).collect();
    let mut city_people: Vec<Vec<usize>> = vec![vec![]; n + 1];
    for i in 1..=m {
        city_people[c[i - 1]].push(i);
    }
    for i in 1..=n {
        city_people[i].sort();
    }

    let mut depth = vec![0; n + 1];
    let mut parent = vec![0; n + 1];
    let mut sz = vec![0; n + 1];
    let mut heavy = vec![0; n + 1];
    dfs1(1, 0, &adj, &mut depth, &mut parent, &mut sz, &mut heavy);

    let mut chain_index = vec![0; n + 1];
    let mut chain_heads: Vec<usize> = vec![];
    let mut pos_in_base = vec![0; n + 1];
    let mut chain_bases: Vec<Vec<usize>> = vec![];
    let mut cur_chain: usize = 0;
    chain_heads.push(1);
    chain_bases.push(vec![]);
    hld_dfs(1, 0, &adj, &heavy, &mut chain_index, &mut pos_in_base, &mut chain_heads, &mut chain_bases, &mut cur_chain);

    let mut sum_trees: Vec<Vec<usize>> = vec![];
    let mut list_trees: Vec<Vec<Vec<usize>>> = vec![];
    for i in 0..chain_bases.len() {
        let chain_nodes = &chain_bases[i];
        let l = chain_nodes.len();
        if l == 0 {
            continue;
        }
        let mut stree = vec![0; 4 * l];
        build_sum(1, 0, l - 1, chain_nodes, &city_people, &mut stree);
        sum_trees.push(stree);

        let mut ltree: Vec<Vec<usize>> = vec![vec![]; 4 * l];
        build_lists(1, 0, l - 1, chain_nodes, &city_people, &mut ltree);
        list_trees.push(ltree);
    }

    let mut anc: Vec<Vec<usize>> = vec![vec![0; n + 1]; 18];
    anc[0] = parent.clone();
    for k in 1..18 {
        for node in 1..=n {
            let p = anc[k - 1][node];
            if p != 0 {
                anc[k][node] = anc[k - 1][p];
            }
        }
    }

    for _ in 0..q {
        let query: Vec<usize> = lines.next().unwrap().unwrap().trim().split_whitespace().map(|x| x.parse().unwrap()).collect();
        let v = query[0];
        let u = query[1];
        let a = query[2];

        if v == u {
            let num = city_people[v].len();
            let k = num.min(a);
            print!("{} ", k);
            for i in 0..k {
                print!("{} ", city_people[v][i]);
            }
            println!();
            continue;
        }

        let lca = get_lca(v, u, &depth, &anc);

        let (sum1, mut cand1) = query_up(v, lca, &chain_heads, &chain_index, &pos_in_base, &parent, &sum_trees, &list_trees);
        let (sum2, cand2) = query_up(u, lca, &chain_heads, &chain_index, &pos_in_base, &parent, &sum_trees, &list_trees);

        let count_lca = city_people[lca].len();
        let x = sum1 + sum2 - count_lca;

        let mut all_cand = vec![];
        all_cand.append(&mut cand1);
        all_cand.extend(cand2);
        all_cand.sort();
        all_cand.dedup();

        let k = x.min(a);
        print!("{} ", k);
        for i in 0..k {
            print!("{} ", all_cand[i]);
        }
        println!();
    }
}

fn dfs1(node: usize, par: usize, adj: &Vec<Vec<usize>>, depth: &mut Vec<usize>, parent: &mut Vec<usize>, sz: &mut Vec<usize>, heavy: &mut Vec<usize>) {
    parent[node] = par;
    depth[node] = if node == 1 { 0 } else { depth[par] + 1 };
    sz[node] = 1;
    let mut max_child_sz = 0;
    let mut heavy_child = 0;
    for &child in adj[node].iter() {
        if child != par {
            dfs1(child, node, adj, depth, parent, sz, heavy);
            sz[node] += sz[child];
            if sz[child] > max_child_sz {
                max_child_sz = sz[child];
                heavy_child = child;
            }
        }
    }
    heavy[node] = heavy_child;
}

fn hld_dfs(node: usize, par: usize, adj: &Vec<Vec<usize>>, heavy: &Vec<usize>, chain_index: &mut Vec<usize>, pos_in_base: &mut Vec<usize>, chain_heads: &mut Vec<usize>, chain_bases: &mut Vec<Vec<usize>>, cur_chain: &mut usize) {
    chain_index[node] = *cur_chain;
    pos_in_base[node] = chain_bases[*cur_chain].len();
    chain_bases[*cur_chain].push(node);
    if heavy[node] != 0 {
        hld_dfs(heavy[node], node, adj, heavy, chain_index, pos_in_base, chain_heads, chain_bases, cur_chain);
    }
    for &child in adj[node].iter() {
        if child != par && child != heavy[node] {
            *cur_chain += 1;
            chain_heads.push(child);
            chain_bases.push(vec![]);
            hld_dfs(child, node, adj, heavy, chain_index, pos_in_base, chain_heads, chain_bases, cur_chain);
        }
    }
}

fn build_sum(idx: usize, start: usize, end: usize, chain_nodes: &Vec<usize>, city_people: &Vec<Vec<usize>>, tree: &mut Vec<usize>) {
    if start == end {
        tree[idx] = city_people[chain_nodes[start]].len();
        return;
    }
    let mid = (start + end) / 2;
    build_sum(2 * idx, start, mid, chain_nodes, city_people, tree);
    build_sum(2 * idx + 1, mid + 1, end, chain_nodes, city_people, tree);
    tree[idx] = tree[2 * idx] + tree[2 * idx + 1];
}

fn build_lists(idx: usize, start: usize, end: usize, chain_nodes: &Vec<usize>, city_people: &Vec<Vec<usize>>, tree: &mut Vec<Vec<usize>>) {
    if start == end {
        tree[idx] = city_people[chain_nodes[start]].clone();
        return;
    }
    let mid = (start + end) / 2;
    build_lists(2 * idx, start, mid, chain_nodes, city_people, tree);
    build_lists(2 * idx + 1, mid + 1, end, chain_nodes, city_people, tree);
    let left = &tree[2 * idx];
    let right = &tree[2 * idx + 1];
    let mut merged: Vec<usize> = Vec::with_capacity(left.len() + right.len());
    let (mut i, mut j) = (0, 0);
    while i < left.len() && j < right.len() {
        if left[i] < right[j] {
            merged.push(left[i]);
            i += 1;
        } else {
            merged.push(right[j]);
            j += 1;
        }
    }
    merged.extend_from_slice(&left[i..]);
    merged.extend_from_slice(&right[j..]);
    tree[idx] = merged;
}

fn get_lca(mut x: usize, mut y: usize, depth: &Vec<usize>, anc: &Vec<Vec<usize>>) -> usize {
    if depth[x] > depth[y] {
        std::mem::swap(&mut x, &mut y);
    }
    let diff = depth[y] - depth[x];
    for k in 0..18 {
        if (diff >> k) & 1 == 1 {
            y = anc[k][y];
        }
    }
    if x == y {
        return x;
    }
    for k in (0..18).rev() {
        if anc[k][x] != anc[k][y] {
            x = anc[k][x];
            y = anc[k][y];
        }
    }
    anc[0][x]
}

fn query_up(mut desc: usize, anc: usize, chain_heads: &Vec<usize>, chain_index: &Vec<usize>, pos_in_base: &Vec<usize>, parent: &Vec<usize>, sum_trees: &Vec<Vec<usize>>, list_trees: &Vec<Vec<Vec<usize>>>) -> (usize, Vec<usize>) {
    let mut total_sum = 0;
    let mut candidates: Vec<usize> = vec![];
    while chain_index[desc] != chain_index[anc] {
        let ch = chain_index[desc];
        let head = chain_heads[ch];
        let l = sum_trees[ch].len() / 4;
        let low = pos_in_base[head];
        let high = pos_in_base[desc];
        total_sum += query_sum(1, 0, l - 1, low, high, &sum_trees[ch]);
        let small_10 = get_smallest_k(10, 1, 0, l - 1, low, high, &list_trees[ch]);
        candidates.extend(small_10);
        desc = parent[head];
    }
    let ch = chain_index[desc];
    let l = sum_trees[ch].len() / 4;
    let low = pos_in_base[anc];
    let high = pos_in_base[desc];
    total_sum += query_sum(1, 0, l - 1, low, high, &sum_trees[ch]);
    let small_10 = get_smallest_k(10, 1, 0, l - 1, low, high, &list_trees[ch]);
    candidates.extend(small_10);
    (total_sum, candidates)
}

fn query_sum(idx: usize, start: usize, end: usize, ql: usize, qr: usize, tree: &Vec<usize>) -> usize {
    if ql > end || qr < start {
        return 0;
    }
    if ql <= start && end <= qr {
        return tree[idx];
    }
    let mid = (start + end) / 2;
    query_sum(2 * idx, start, mid, ql, qr, tree) + query_sum(2 * idx + 1, mid + 1, end, ql, qr, tree)
}

fn get_smallest_k<'a>(maxk: usize, idx: usize, start: usize, end: usize, ql: usize, qr: usize, tree: &'a Vec<Vec<usize>>) -> Vec<usize> {
    let mut lists: Vec<&'a Vec<usize>> = vec![];
    get_range_lists(idx, start, end, ql, qr, tree, &mut lists);
    let mut pq: BinaryHeap<Reverse<(usize, usize, usize)>> = BinaryHeap::new();
    for i in 0..lists.len() {
        if !lists[i].is_empty() {
            pq.push(Reverse((lists[i][0], i, 0)));
        }
    }
    let mut result: Vec<usize> = vec![];
    let mut used = 0;
    while !pq.is_empty() && used < maxk {
        let Reverse((val, lid, eid)) = pq.pop().unwrap();
        result.push(val);
        used += 1;
        if eid + 1 < lists[lid].len() {
            let next_val = lists[lid][eid + 1];
            pq.push(Reverse((next_val, lid, eid + 1)));
        }
    }
    result
}

fn get_range_lists<'a>(idx: usize, start: usize, end: usize, ql: usize, qr: usize, tree: &'a Vec<Vec<usize>>, res: &mut Vec<&'a Vec<usize>>) {
    if ql > end || qr < start {
        return;
    }
    if ql <= start && end <= qr {
        res.push(&tree[idx]);
        return;
    }
    let mid = (start + end) / 2;
    get_range_lists(2 * idx, start, mid, ql, qr, tree, res);
    get_range_lists(2 * idx + 1, mid + 1, end, ql, qr, tree, res);
}