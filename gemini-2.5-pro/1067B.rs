```rust
use std::io::{self, BufRead};
use std::collections::VecDeque;

fn bfs(start_node: usize, n: usize, adj: &Vec<Vec<usize>>) -> Vec<i32> {
    let mut dist = vec