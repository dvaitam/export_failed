use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lines();

    let line1: String = lines.next().unwrap().unwrap();
    let mut iter = line1.split_whitespace();
    let x1: i64 = iter.next().unwrap().parse().unwrap();
    let y1: i64 = iter.next().unwrap().parse().unwrap();

    let line2: String = lines.next().unwrap().unwrap();
    let mut iter = line2.split_whitespace();
    let x2: i64 = iter.next().unwrap().parse().unwrap();
    let y2: i64 = iter.next().unwrap().parse().unwrap();

    let n: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();

    let mut count = 0;
    for _ in 0..n {
        let line: String = lines.next().unwrap().unwrap();
        let mut iter = line.split_whitespace();
        let a: i64 = iter.next().unwrap().parse().unwrap();
        let b: i64 = iter.next().unwrap().parse().unwrap();
        let c: i64 = iter.next().unwrap().parse().unwrap();

        let val1 = a * x1 + b * y1 + c;
        let val2 = a * x2 + b * y2 + c;

        if (val1 > 0 && val2 < 0) || (val1 < 0 && val2 > 0) {
            count += 1;
        }
    }

    println!("{}", count);
}