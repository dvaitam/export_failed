use std::collections::HashMap;
use std::collections::HashSet;
use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lines();
    let first: Vec<usize> = lines.next().unwrap().unwrap().trim().split_whitespace().map(|x| x.parse().unwrap()).collect();
    let n = first[0];
    let m = first[1];
    let mut adj: Vec<Vec<bool>> = vec![vec![false; n + 1]; n + 1];
    let mut deg: Vec<usize> = vec![0; n + 1];
    for _ in 0..m {
        let edge: Vec<usize> = lines.next().unwrap().unwrap().trim().split_whitespace().map(|x| x.parse().unwrap()).collect();
        let u = edge[0];
        let v = edge[1];
        adj[u][v] = true;
        adj[v][u] = true;
        deg[u] += 1;
        deg[v] += 1;
    }
    let full = n - 1;
    let mut b: Vec<usize> = vec![];
    let mut vp: Vec<usize> = vec![];
    for i in 1..=n {
        if deg[i] == full {
            b.push(i);
        } else {
            vp.push(i);
        }
    }
    let kp = vp.len();
    let mut possible = false;
    let mut assignment: Vec<char> = vec![' '; n + 1];
    if kp == 0 {
        possible = true;
        for i in 1..=n {
            assignment[i] = 'b';
        }
    } else {
        let mut deg_set: HashSet<usize> = HashSet::new();
        for &i in &vp {
            deg_set.insert(deg[i]);
        }
        let num_d = deg_set.len();
        if num_d > 2 {
            // no
        } else if num_d == 2 {
            let mut ds: Vec<usize> = deg_set.into_iter().collect();
            ds.sort_by(|a, b| b.cmp(a));
            let d1 = ds[0];
            let d2 = ds[1];
            let mut x: Vec<usize> = vec![];
            let mut y: Vec<usize> = vec![];
            for &i in &vp {
                if deg[i] == d1 {
                    x.push(i);
                } else {
                    y.push(i);
                }
            }
            let sx = x.len();
            let sy = y.len();
            if d1 == n - 1 - sy && d2 == n - 1 - sx {
                let mut is_cliq_x = true;
                'check_x: for (ii, &u) in x.iter().enumerate() {
                    for jj in ii + 1..sx {
                        let v = x[jj];
                        if !adj[u][v] {
                            is_cliq_x = false;
                            break 'check_x;
                        }
                    }
                }
                let mut is_cliq_y = true;
                'check_y: for (ii, &u) in y.iter().enumerate() {
                    for jj in ii + 1..sy {
                        let v = y[jj];
                        if !adj[u][v] {
                            is_cliq_y = false;
                            break 'check_y;
                        }
                    }
                }
                let mut no_bet = true;
                'outer: for &u in &x {
                    for &v in &y {
                        if adj[u][v] {
                            no_bet = false;
                            break 'outer;
                        }
                    }
                }
                if is_cliq_x && is_cliq_y && no_bet {
                    possible = true;
                    for &i in &b {
                        assignment[i] = 'b';
                    }
                    for &i in &x {
                        assignment[i] = 'a';
                    }
                    for &i in &y {
                        assignment[i] = 'c';
                    }
                }
            }
        } else {
            let d = *deg_set.iter().next().unwrap();
            if kp % 2 == 0 {
                let s = kp / 2;
                if d == n - 1 - s {
                    let mut idx_map: HashMap<usize, usize> = HashMap::new();
                    for (ii, &v) in vp.iter().enumerate() {
                        idx_map.insert(v, ii);
                    }
                    let mut adj_vp: Vec<Vec<usize>> = vec![vec![]; kp];
                    for (ii, &u) in vp.iter().enumerate() {
                        for jj in ii + 1..kp {
                            let v = vp[jj];
                            if adj[u][v] {
                                adj_vp[ii].push(jj);
                                adj_vp[jj].push(ii);
                            }
                        }
                    }
                    let mut visited = vec![false; kp];
                    let mut components: Vec<Vec<usize>> = vec![];
                    for start in 0..kp {
                        if !visited[start] {
                            let mut comp: Vec<usize> = vec![];
                            let mut stack: Vec<usize> = vec![start];
                            visited[start] = true;
                            while let Some(u) = stack.pop() {
                                comp.push(u);
                                for &v in &adj_vp[u] {
                                    if !visited[v] {
                                        visited[v] = true;
                                        stack.push(v);
                                    }
                                }
                            }
                            components.push(comp);
                        }
                    }
                    if components.len() == 2 {
                        let mut ok = true;
                        for comp in &components {
                            let m = comp.len();
                            if m != s {
                                ok = false;
                                break;
                            }
                            for &u in comp {
                                if adj_vp[u].len() != m - 1 {
                                    ok = false;
                                    break;
                                }
                            }
                            if !ok {
                                break;
                            }
                        }
                        if ok {
                            possible = true;
                            for &i in &b {
                                assignment[i] = 'b';
                            }
                            let comp1 = &components[0];
                            for &loc in comp1 {
                                let vert = vp[loc];
                                assignment[vert] = 'a';
                            }
                            let comp2 = &components[1];
                            for &loc in comp2 {
                                let vert = vp[loc];
                                assignment[vert] = 'c';
                            }
                        }
                    }
                }
            }
        }
    }
    if possible {
        println!("Yes");
        for i in 1..=n {
            print!("{}", assignment[i]);
        }
        println!();
    } else {
        println!("No");
    }
}