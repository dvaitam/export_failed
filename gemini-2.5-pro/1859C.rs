use std::io;
use std::cmp::max;

fn main() {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();
    let t: usize = buffer.trim().parse().unwrap();

    for _ in 0..t {
        buffer.clear();
        io::stdin().read_line(&mut buffer).unwrap();
        let n: i64 = buffer.trim().parse().unwrap();
        
        let mut sum_sq: i64 = 0;
        for i in 1..=n {
            sum_sq += i * i;
        }

        let mut max_cost = sum_sq - n * n;

        for k in 1..n {
            let current_sum = sum_sq - (n - k).pow(2);
            let max_prod = max((n - 1).pow(2), n * k);
            let current_cost = current_sum - max_prod;
            if current_cost > max_cost {
                max_cost = current_cost;
            }
        }
        
        println!("{}", max_cost);
    }
}