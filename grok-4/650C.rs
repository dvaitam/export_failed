use std::collections::HashMap;
use std::io::{self, BufRead, BufWriter, Write};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lines();
    let first: Vec<usize> = lines.next().unwrap().unwrap().trim().split_whitespace().map(|x| x.parse().unwrap()).collect();
    let n = first[0];
    let m = first[1];
    let mut a: Vec<Vec<u32>> = Vec::with_capacity(n);
    for _ in 0..n {
        let line: Vec<u32> = lines.next().unwrap().unwrap().trim().split_whitespace().map(|x| x.parse().unwrap()).collect();
        a.push(line);
    }
    let nm = n * m;
    let mut parent: Vec<usize> = (0..nm).collect();
    let mut urank: Vec<i32> = vec![0; nm];
    for i in 0..n {
        let mut vals: Vec<(u32, usize)> = (0..m).map(|j| (a[i][j], j)).collect();
        vals.sort_by_key(|&(v, _)| v);
        let mut k = 0;
        while k < m {
            let curr_v = vals[k].0;
            let start = k;
            k += 1;
            while k < m && vals[k].0 == curr_v {
                k += 1;
            }
            if k - start > 1 {
                let first_idx = i * m + vals[start].1;
                for p in start + 1..k {
                    let this_idx = i * m + vals[p].1;
                    union(&mut parent, &mut urank, first_idx, this_idx);
                }
            }
        }
    }
    for j in 0..m {
        let mut vals: Vec<(u32, usize)> = (0..n).map(|i| (a[i][j], i)).collect();
        vals.sort_by_key(|&(v, _)| v);
        let mut k = 0;
        while k < n {
            let curr_v = vals[k].0;
            let start = k;
            k += 1;
            while k < n && vals[k].0 == curr_v {
                k += 1;
            }
            if k - start > 1 {
                let first_idx = vals[start].1 * m + j;
                for p in start + 1..k {
                    let this_idx = vals[p].1 * m + j;
                    union(&mut parent, &mut urank, first_idx, this_idx);
                }
            }
        }
    }
    let mut comp_pos: HashMap<usize, Vec<(usize, usize)>> = HashMap::new();
    for i in 0..n {
        for j in 0..m {
            let idx = i * m + j;
            let root = find(&mut parent, idx);
            comp_pos.entry(root).or_insert(Vec::new()).push((i, j));
        }
    }
    let mut comps: Vec<Comp> = Vec::new();
    for (&root, pos) in &comp_pos {
        let mut rset: Vec<usize> = pos.iter().map(|&(i, _)| i).collect();
        rset.sort();
        rset.dedup();
        let mut cset: Vec<usize> = pos.iter().map(|&(_, j)| j).collect();
        cset.sort();
        cset.dedup();
        let v = a[pos[0].0][pos[0].1];
        comps.push(Comp {
            root,
            v,
            rows: rset,
            cols: cset,
        });
    }
    comps.sort_by_key(|c| c.v);
    let mut row_max: Vec<i32> = vec![0; n];
    let mut col_max: Vec<i32> = vec![0; m];
    let mut root_to_rank: HashMap<usize, i32> = HashMap::new();
    let mut ii = 0;
    while ii < comps.len() {
        let curr_v = comps[ii].v;
        let mut batch: Vec<usize> = Vec::new();
        while ii < comps.len() && comps[ii].v == curr_v {
            batch.push(ii);
            ii += 1;
        }
        let mut batch_ranks: Vec<i32> = vec![0; batch.len()];
        for (bi, &ci) in batch.iter().enumerate() {
            let c = &comps[ci];
            let mut max_r: i32 = 0;
            for &r in &c.rows {
                if row_max[r] > max_r {
                    max_r = row_max[r];
                }
            }
            let mut max_c: i32 = 0;
            for &cl in &c.cols {
                if col_max[cl] > max_c {
                    max_c = col_max[cl];
                }
            }
            let max_pred = if max_r > max_c { max_r } else { max_c };
            batch_ranks[bi] = 1 + max_pred;
        }
        for (bi, &ci) in batch.iter().enumerate() {
            let c = &comps[ci];
            root_to_rank.insert(c.root, batch_ranks[bi]);
        }
        for (bi, &ci) in batch.iter().enumerate() {
            let c = &comps[ci];
            let rank = batch_ranks[bi];
            for &r in &c.rows {
                if rank > row_max[r] {
                    row_max[r] = rank;
                }
            }
            for &cl in &c.cols {
                if rank > col_max[cl] {
                    col_max[cl] = rank;
                }
            }
        }
    }
    let mut cell_root: Vec<usize> = vec![0; nm];
    for idx in 0..nm {
        cell_root[idx] = find(&mut parent, idx);
    }
    let stdout = io::stdout();
    let mut writer = BufWriter::new(stdout);
    for i in 0..n {
        for j in 0..m {
            let root = cell_root[i * m + j];
            let val = root_to_rank[&root];
            write!(&mut writer, "{} ", val).unwrap();
        }
        writeln!(&mut writer).unwrap();
    }
}

fn find(parent: &mut Vec<usize>, x: usize) -> usize {
    let mut root = x;
    while parent[root] != root {
        root = parent[root];
    }
    let mut curr = x;
    while curr != root {
        let next = parent[curr];
        parent[curr] = root;
        curr = next;
    }
    root
}

fn union(parent: &mut Vec<usize>, urank: &mut Vec<i32>, x: usize, y: usize) {
    let mut px = find(parent, x);
    let mut py = find(parent, y);
    if px == py {
        return;
    }
    if urank[px] < urank[py] {
        std::mem::swap(&mut px, &mut py);
    }
    parent[py] = px;
    if urank[px] == urank[py] {
        urank[px] += 1;
    }
}

struct Comp {
    root: usize,
    v: u32,
    rows: Vec<usize>,
    cols: Vec<usize>,
}