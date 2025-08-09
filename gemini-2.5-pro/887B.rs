use std::collections::HashSet;
use std::io::{self, BufRead};
use itertools::Itertools;

fn get_digits(mut n: u32) -> HashSet<u32> {
    let mut digits = HashSet::new();
    if n == 0 {
        digits.insert(0);
        return digits;
    }
    while n > 0 {
        digits.insert(n % 10);
        n /= 10;
    }
    digits
}