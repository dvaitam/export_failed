use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut iter = input.split_whitespace();
    let n: usize = iter.next().unwrap().parse().unwrap();
    let mut mins = vec![i64::MAX; n];
    for i in 0..n {
        for j in 0..n {
            let val: i64 = iter.next().unwrap().parse().unwrap();
            if i != j && val < mins[i] {
                mins[i] = val;
            }
        }
    }
    print!("-1\n");
    for i in 0..n {
        if i > 0 {
            print!(" ");
        }
        print!("{}", mins[i]);
    }
    println!();
}