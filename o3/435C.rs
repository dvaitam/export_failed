use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut nums = input.split_whitespace().map(|s| s.parse::<i32>().unwrap());
    let n = nums.next().unwrap() as usize;
    let a: Vec<i32> = (0..n).map(|_| nums.next().unwrap()).collect();
    let width: usize = a.iter().map(|&v| v as usize).sum();

    let mut y = 0;
    let mut min_y = 0;
    let mut max_y = 0;
    for (i, &len) in a.iter().enumerate() {
        let dir = if i % 2 == 0 { 1 } else { -1 };
        y += dir * len;
        min_y = min_y.min(y);
        max_y = max_y.max(y);
    }
    let height = (max_y - min_y) as usize;
    let mut grid = vec![vec![b' '; width]; height];

    let mut x = 0usize;
    let mut cur_y = 0i32;
    for (i, &len) in a.iter().enumerate() {
        let dir = if i % 2 == 0 { 1 } else { -1 };
        for k in 0..len {
            let col = x + k as usize;
            let y_char = if dir == 1 {
                cur_y + k
            } else {
                cur_y - k - 1
            };
            let row = (max_y - 1 - y_char) as usize;
            grid[row][col] = if dir == 1 { b'/' } else { b'\\' };
        }
        x += len as usize;
        cur_y += dir * len;
    }

    for (idx, row) in grid.iter().enumerate() {
        let line = String::from_utf8(row.clone()).unwrap();
        if idx + 1 == height {
            print!("{}", line);
        } else {
            println!("{}", line);
        }
    }
}