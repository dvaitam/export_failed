use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut reader = stdin.lock();
    let mut buffer = String::new();

    reader.read_line(&mut buffer).unwrap();
    let mut iter = buffer.split_whitespace();
    let n: usize = iter.next().unwrap().parse().unwrap();
    let m: usize = iter.next().unwrap().parse().unwrap();
    let k: usize = iter.next().unwrap().parse().unwrap();
    buffer.clear();

    let mut table: Vec<Vec<i32>> = Vec::with_capacity(n);
    for _ in 0..n {
        reader.read_line(&mut buffer).unwrap();
        let row: Vec<i32> = buffer
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        table.push(row);
        buffer.clear();
    }

    let mut row_map: Vec<usize> = (0..n).collect();
    let mut col_map: Vec<usize> = (0..m).collect();

    for _ in 0..k {
        reader.read_line(&mut buffer).unwrap();
        let mut parts = buffer.split_whitespace();
        let query_type = parts.next().unwrap().chars().next().unwrap();
        let x: usize = parts.next().unwrap().parse::<usize>().unwrap() - 1;
        let y: usize = parts.next().unwrap().parse::<usize>().unwrap() - 1;

        match query_type {
            'r' => {
                row_map.swap(x, y);
            }
            'с' => {
                col_map.swap(x, y);
            }
            'g' => {
                let physical_row = row_map[x];
                let physical_col = col_map[y];
                println!("{}", table[physical_row][physical_col]);
            }
            _ => {}
        }
        buffer.clear();
    }
}