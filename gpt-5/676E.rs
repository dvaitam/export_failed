use std::io::{self, Read};

const BASE: u32 = 1_000_000_000;

#[derive(Clone)]
struct Big {
    sign: i8,        // -1, 0, 1
    d: Vec<u32>,     // little-endian base 1e9
}

impl Big {
    fn new() -> Self {
        Big { sign: 0, d: Vec::new() }
    }
    fn from_i64(x: i64) -> Self {
        if x == 0 {
            return Big::new();
        }
        let mut a = Big { sign: if x < 0 { -1 } else { 1 }, d: Vec::new() };
        let mut v = x.abs() as u64;
        while v > 0 {
            a.d.push((v % BASE as u64) as u32);
            v /= BASE as u64;
        }
        a
    }
    fn is_zero(&self) -> bool {
        self.sign == 0
    }
    fn normalize(&mut self) {
        while let Some(&last) = self.d.last() {
            if last == 0 { self.d.pop(); } else { break; }
        }
        if self.d.is_empty() {
            self.sign = 0;
        }
    }
    fn mul_small(&mut self, m: i64) {
        if self.is_zero() || m == 0 {
            self.sign = 0;
            self.d.clear();
            return;
        }
        let msign = if m < 0 { -1 } else { 1 };
        self.sign = (self.sign as i32 * msign as i32).signum() as i8;
        let mm = m.abs() as u64;
        let mut carry: u64 = 0;
        for x in &mut self.d {
            let cur = *x as u64 * mm + carry;
            *x = (cur % BASE as u64) as u32;
            carry = cur / BASE as u64;
        }
        while carry > 0 {
            self.d.push((carry % BASE as u64) as u32);
            carry /= BASE as u64;
        }
        self.normalize();
    }
    fn abs_add_small(&mut self, a: u64) {
        let mut carry = a;
        let mut i = 0usize;
        while carry > 0 {
            if i == self.d.len() {
                self.d.push(0);
            }
            let cur = self.d[i] as u64 + carry;
            self.d[i] = (cur % BASE as u64) as u32;
            carry = cur / BASE as u64;
            i += 1;
        }
    }
    fn abs_ge_small(&self, a: u64) -> bool {
        if self.d.len() > 1 {
            return true;
        }
        if self.d.is_empty() {
            return false;
        }
        self.d[0] as u64 >= a
    }
    fn abs_sub_small(&mut self, a: u64) {
        // assumes |self| >= a
        let mut borrow: i64 = a as i64;
        let mut i = 0usize;
        while borrow > 0 && i < self.d.len() {
            let cur = self.d[i] as i64 - (borrow % BASE as i64);
            if cur < 0 {
                self.d[i] = (cur + BASE as i64) as u32;
                borrow = borrow / BASE as i64 + 1;
            } else {
                self.d[i] = cur as u32;
                borrow = borrow / BASE as i64;
            }
            i += 1;
        }
        self.normalize();
    }
    fn add_small(&mut self, a: i64) {
        if a == 0 {
            return;
        }
        if self.sign == 0 {
            *self = Big::from_i64(a);
            return;
        }
        let a_sign = if a < 0 { -1 } else { 1 };
        let aval = a.abs() as u64;
        if self.sign == a_sign {
            self.abs_add_small(aval);
        } else {
            if self.abs_ge_small(aval) {
                self.abs_sub_small(aval);
                if self.is_zero() {
                    self.sign = 0;
                }
            } else {
                // result = aval - |self|
                // construct new from (aval - |self|)
                // compute |self| as u64 (only if fits), but if it doesn't fit, then |self| > aval, handled above.
                let mut tmp = aval;
                // subtract self.abs from tmp
                let mut carry: i128 = tmp as i128;
                let mut p: i128 = 1;
                let mut res_digits: Vec<u32> = Vec::new();
                // convert aval to base and subtract self.d
                // We can process digit-wise:
                let mut i = 0usize;
                let mut borrow: i64 = 0;
                let mut remaining = aval;
                while i < self.d.len() || remaining > 0 || borrow != 0 {
                    let a_digit = (remaining % BASE as u64) as i64;
                    remaining /= BASE as u64;
                    let mut cur = a_digit - borrow;
                    if i < self.d.len() {
                        cur -= self.d[i] as i64;
                    }
                    if cur < 0 {
                        cur += BASE as i64;
                        borrow = 1;
                    } else {
                        borrow = 0;
                    }
                    res_digits.push(cur as u32);
                    i += 1;
                }
                // remove leading zeros
                while let Some(&last) = res_digits.last() {
                    if last == 0 { res_digits.pop(); } else { break; }
                }
                if res_digits.is_empty() {
                    self.sign = 0;
                    self.d.clear();
                } else {
                    self.sign = a_sign;
                    self.d = res_digits;
                }
            }
        }
        self.normalize();
    }
}

fn main() {
    // Read input
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let n: usize = it.next().unwrap().parse().unwrap();
    let k: i64 = it.next().unwrap().parse().unwrap();

    let mut coeff: Vec<Option<i64>> = Vec::with_capacity(n + 1);
    let mut unknown = 0usize;
    for _ in 0..=n {
        let s = it.next().unwrap();
        if s == "?" {
            coeff.push(None);
            unknown += 1;
        } else {
            let v: i64 = s.parse().unwrap();
            coeff.push(Some(v));
        }
    }

    if unknown == 0 {
        // Evaluate P(k) exactly using Big
        let mut v = Big::from_i64(0);
        for i in (0..=n).rev() {
            v.mul_small(k);
            v.add_small(coeff[i].unwrap());
        }
        println!("{}", if v.is_zero() { "Yes" } else { "No" });
        return;
    }

    if k == 0 {
        // P(0) = a0
        match coeff[0] {
            None => println!("Yes"),
            Some(v) => println!("{}", if v == 0 { "Yes" } else { "No" }),
        }
        return;
    }

    // With at least one unknown and k != 0, if coefficients can be any real/integer (unbounded),
    // it's always possible to set them to make P(k) = 0.
    println!("Yes");
}