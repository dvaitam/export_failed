```rust
use std::io::{self, BufRead};
use std::collections::VecDeque;

fn dfs(r: usize, c: usize, n: usize, m: usize, grid: &mut Vec<Vec<char>>, visited: &mut Vec<Vec<bool>>, to_change: &mut usize) {
    if visited[r][c] || grid[r][c] == '#' {
        return;
    }
    visited[r][c] = true;

    let dr: [i32; 4] = [-1, 1, 0, 0];
    let dc: [i32; 4] = [0, 0, -1, 1];

    for i in 0..4 {
        let nr = r as i32 + dr[i];
        let nc = c as i32 + dc[i];

        if nr >= 0 && nr < n as i32 && nc >= 0 && nc < m as i32 {
            dfs(nr as usize, nc as usize, n, m, grid, visited, to_change);
        }
    }

    if *to_change > 0 {
        grid[r][c] = 'X';
        *to_change -= 1;
    }
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let first_line = lines.next().unwrap().unwrap();
    let mut parts = first_line.split_whitespace();
    let n: usize = parts.next().unwrap().parse().unwrap();
    let m: usize = parts.next().unwrap().parse().unwrap();
    let k: usize = parts.next().unwrap().parse().unwrap();

    let mut grid: Vec<Vec<char>> = Vec::with_capacity(n);
    let mut start_pos: Option<(usize, usize)> = None;
    let mut total_dots = 0;

    for r in 0..n {
        let row_str = lines.next().unwrap().unwrap();
        let row: Vec<char> = row_str.chars().collect();
        for c in 0..m {
            if row[c] == '.' {
                if start_pos.is_none() {
                    start_pos = Some((r, c));
                }
                total_dots += 1;
            }
        }
        grid.push(row);
    }
    
    if start_pos.is_none() {
        for row in grid {
            println!("{}", row.iter().collect::<String>());
        }
        return;
    }
    
    let (start_r, start_c) = start_pos.unwrap();
    
    let mut to_change = total_dots - k;

    if to_change > 0 {
        let mut visited = vec