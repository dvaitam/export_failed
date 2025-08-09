use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let n: u64 = input.trim().parse().unwrap();
    if n <= 2 {
        println!("No");
        return;
    }
    let sum = n * (n + 1) / 2;
    let mut d = 0u64;
    let mut i = 2u64;
    while i * i <= sum {
        if sum % i == 0 {
            d = i;
            break;
        }
        i += 1;
    }
    if d == 0 {
        println!("No");
        return;
    }
    println!("Yes");
    println!("1 {}", d);
    let mut line = String::new();
    line.push_str(&(n - 1).to_string());
    for num in 1..=n {
        if num == d {
            continue;
        }
        line.push(' ');
        line.push_str(&num.to_string());
    }
    println!("{}", line);
}