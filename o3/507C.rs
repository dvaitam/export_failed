use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();
    let h: u64 = it.next().unwrap().parse().unwrap();
    let mut n: u64 = it.next().unwrap().parse().unwrap();
    let mut ans: u128 = 1;
    let mut dir: u8 = 0;
    let mut height = h;
    while height > 0 {
        let half = 1u128 << (height - 1);
        let left = (n as u128) <= half;
        if (left && dir == 0) || (!left && dir == 1) {
            if height > 1 {
                ans += 1;
            }
            if !left {
                n -= half as u64;
            }
            dir ^= 1;
        } else {
            ans += 1u128 << height;
            if !left {
                n -= half as u64;
            }
        }
        height -= 1;
    }
    println!("{}", ans);
}