use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();
    let q: usize = it.next().unwrap().parse().unwrap();
    let mut res = Vec::with_capacity(q);
    for _ in 0..q {
        let n: usize = it.next().unwrap().parse().unwrap();
        let k: i64 = it.next().unwrap().parse().unwrap();
        let mut low: i64 = std::i64::MIN;
        let mut high: i64 = std::i64::MAX;
        for _ in 0..n {
            let a: i64 = it.next().unwrap().parse().unwrap();
            low = low.max(a - k);
            high = high.min(a + k);
        }
        if low <= high && high >= 1 {
            res.push(high.to_string());
        } else {
            res.push("-1".to_string());
        }
    }
    println!("{}", res.join("\n"));
}