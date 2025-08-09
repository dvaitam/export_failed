use std::io::{self, BufRead};

fn main() {
    let mut stdin = io::stdin().lock();
    let mut buffer = String::new();
    stdin.read_line(&mut buffer).unwrap();
    let mut iter = buffer.split_whitespace();
    let n: usize = iter.next().unwrap().parse().unwrap();
    let m: usize = iter.next().unwrap().parse().unwrap();
    let d: usize = iter.next().unwrap().parse().unwrap();

    buffer.clear();
    stdin.read_line(&mut buffer).unwrap();
    let c: Vec<usize> = buffer.split_whitespace().map(|s| s.parse().unwrap()).collect();

    let sum_c: usize = c.iter().sum();
    let total_water = n - sum_c;
    let num_gaps = m + 1;
    let max_water_capacity = num_gaps * (d - 1);

    if total_water > max_water_capacity {
        println!("NO");
        return;
    }

    println!("YES");

    let base_gap = total_water / num_gaps;
    let extra_gaps = total_water % num_gaps;

    let mut river = Vec::with_capacity(n);
    let mut gaps_distributed = 0;

    let gap_0_size = base_gap + if gaps_distributed < extra_gaps { 1 } else { 0 };
    gaps_distributed += 1;
    for _ in 0..gap_0_size {
        river.push(0);
    }

    for i in 0..m {
        for _ in 0..c[i] {
            river.push(i + 1);
        }

        let gap_size = base_gap + if gaps_distributed < extra_gaps { 1 } else { 0 };
        gaps_distributed += 1;
        for _ in 0..gap_size {
            river.push(0);
        }
    }

    let result_str: Vec<String> = river.iter().map(|&x| x.to_string()).collect();
    println!("{}", result_str.join(" "));
}