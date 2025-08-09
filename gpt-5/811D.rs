use std::collections::VecDeque;
use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let n: usize = it.next().unwrap().parse().unwrap();
    let m: usize = it.next().unwrap().parse().unwrap();

    let mut grid: Vec<Vec<char>> = Vec::with_capacity(n);
    for _ in 0..n {
        let row = it.next().unwrap().chars().collect::<Vec<char>>();
        grid.push(row);
    }

    let mut fx = 0usize;
    let mut fy = 0usize;
    for i in 0..n {
        for j in 0..m {
            if grid[i][j] == 'F' {
                fx = i;
                fy = j;
            }
        }
    }

    let mut visited = vec![vec![false; m]; n];
    let mut parent: Vec<Vec<Option<(usize, usize, char)>>> = vec![vec![None; m]; n];

    let dirs: [(isize, isize, char); 4] = [(-1, 0, 'U'), (1, 0, 'D'), (0, -1, 'L'), (0, 1, 'R')];

    let mut q = VecDeque::new();
    visited[0][0] = true;
    q.push_back((0usize, 0usize));

    'bfs: while let Some((x, y)) = q.pop_front() {
        for &(dx, dy, ch) in &dirs {
            let nx = x as isize + dx;
            let ny = y as isize + dy;
            if nx < 0 || ny < 0 || nx >= n as isize || ny >= m as isize {
                continue;
            }
            let ux = nx as usize;
            let uy = ny as usize;
            if visited[ux][uy] || grid[ux][uy] == '*' {
                continue;
            }
            visited[ux][uy] = true;
            parent[ux][uy] = Some((x, y, ch));
            if grid[ux][uy] == 'F' {
                break 'bfs;
            }
            q.push_back((ux, uy));
        }
    }

    let mut path = Vec::new();
    let mut cx = fx;
    let mut cy = fy;

    while !(cx == 0 && cy == 0) {
        if let Some((px, py, ch)) = parent[cx][cy] {
            path.push(ch);
            cx = px;
            cy = py;
        } else {
            break;
        }
    }

    path.reverse();
    let result: String = path.into_iter().collect();
    println!("{}", result);
}