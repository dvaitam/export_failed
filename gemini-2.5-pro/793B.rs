use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let mut parts = input.split_whitespace();
    let n: usize = parts.next().unwrap().parse().unwrap();
    let m: usize = parts.next().unwrap().parse().unwrap();

    let mut grid: Vec<Vec<char>> = Vec::with_capacity(n);
    let mut start = (0, 0);
    let mut target = (0, 0);

    for r in 0..n {
        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        let row: Vec<char> = input.trim().chars().collect();
        for c in 0..m {
            if row[c] == 'S' {
                start = (r, c);
            } else if row[c] == 'T' {
                target = (r, c);
            }
        }
        grid.push(row);
    }

    let (_sr, _sc) = start;
    let (_tr, _tc) = target;
}