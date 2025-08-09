use std::io::{self, Read};
use std::collections::{HashSet,HashMap,VecDeque};
use std::str::FromStr;
use std::cmp::Ordering;
use std::hash::{Hash, Hasher};
use std::fmt;

#[derive(Clone, Eq, PartialEq, Hash)]
struct Big {
    digits: Vec<u32>, // little-endian base 1e9
}

impl Big {
    fn new() -> Self { Big { digits: vec![0] } }
    fn normalize(&mut self) {
        while self.digits.len() > 1 && *self.digits.last().unwrap() == 0 {
            self.digits.pop();
        }
    }
    fn from_u32(mut n: u32) -> Self {
        let base = 1_000_000_000u32;
        if n == 0 { return Big::new(); }
        let mut d = Vec::new();
        while n > 0 {
            d.push(n % base);
            n /= base;
        }
        Big { digits: d }
    }
    fn mul_small(&self, m: u32) -> Self {
        if m == 0 { return Big::new(); }
        let base = 1_000_000_000u64;
        let mut carry: u64 = 0;
        let mut res = Vec::with_capacity(self.digits.len()+1);
        for &dig in &self.digits {
            let prod = (dig as u64) * (m as u64) + carry;
            res.push((prod % base) as u32);
            carry = prod / base;
        }
        if carry > 0 { res.push(carry as u32); }
        let mut b = Big { digits: res };
        b.normalize();
        b
    }
    fn add_small(&self, a: u32) -> Self {
        let base = 1_000_000_000u64;
        let mut res = self.digits.clone();
        let mut carry = a as u64;
        for v in &mut res {
            let sum = (*v as u64) + carry;
            *v = (sum % base) as u32;
            carry = sum / base;
            if carry == 0 { break; }
        }
        if carry > 0 { res.push(carry as u32); }
        let mut b = Big { digits: res };
        b.normalize();
        b
    }
    // returns None if self < other
    fn sub(&self, other: &Big) -> Option<Big> {
        if self < other { return None; }
        let mut res = self.digits.clone();
        let mut borrow: i64 = 0;
        let base = 1_000_000_000i64;
        for i in 0..res.len() {
            let ov = if i < other.digits.len() { other.digits[i] as i64 } else { 0 };
            let mut cur = res[i] as i64 - ov - borrow;
            if cur < 0 {
                cur += base;
                borrow = 1;
            } else {
                borrow = 0;
            }
            res[i] = cur as u32;
        }
        let mut b = Big { digits: res };
        b.normalize();
        Some(b)
    }
    fn to_string_dec(&self) -> String {
        let mut s = String::new();
        if self.digits.is_empty() { return "0".to_string(); }
        if let Some(&last) = self.digits.last() {
            s += &format!("{}", last);
        } else {
            s += "0";
        }
        for &d in self.digits.iter().rev().skip(1) {
            s += &format!("{:09}", d);
        }
        s
    }
}

impl FromStr for Big {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut res = Big::new();
        for ch in s.bytes() {
            if ch < b'0' || ch > b'9' { continue; }
            let digit = (ch - b'0') as u32;
            res = res.mul_small(10).add_small(digit);
        }
        res.normalize();
        Ok(res)
    }
}

impl Ord for Big {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.digits.len() != other.digits.len() {
            return self.digits.len().cmp(&other.digits.len());
        }
        for (a, b) in self.digits.iter().rev().zip(other.digits.iter().rev()) {
            if a != b { return a.cmp(b); }
        }
        Ordering::Equal
    }
}
impl PartialOrd for Big {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> { Some(self.cmp(other)) }
}

impl fmt::Debug for Big {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { write!(f, "{}", self.to_string_dec()) }
}

fn main(){
    let mut s = String::new();
    io::stdin().read_to_string(&mut s).unwrap();
    let s = s.trim();
    if s.is_empty() { return; }
    let A = Big::from_str(s).unwrap();
    let two = Big::from_u32(2);
    let thirteen = Big::from_u32(13);
    let twelve = Big::from_u32(12);

    let d1 = Big::from_u32(2);
    let d2 = Big::from_u32(13);

    let mut idx_map: HashMap<usize, HashSet<Big>> = HashMap::new();
    idx_map.entry(1).or_insert_with(HashSet::new).insert(d1.clone());
    idx_map.entry(2).or_insert_with(HashSet::new).insert(d2.clone());

    let mut visited: HashSet<(Big, Big)> = HashSet::new();
    let mut q: VecDeque<(Big, Big, usize)> = VecDeque::new();
    visited.insert((d1.clone(), d2.clone()));
    q.push_back((d1.clone(), d2.clone(), 2usize));

    while let Some((prev, cur, idx)) = q.pop_front() {
        let next1 = {
            let t = cur.mul_small(12);
            if t >= two {
                t.sub(&two).unwrap()
            } else {
                continue;
            }
        };
        if next1 <= A {
            let pair = (cur.clone(), next1.clone());
            if !visited.contains(&pair) {
                visited.insert(pair.clone());
                idx_map.entry(idx+1).or_insert_with(HashSet::new).insert(next1.clone());
                q.push_back((pair.0, pair.1, idx+1));
            }
        }
        let next2 = {
            let t1 = cur.mul_small(13);
            let t2 = prev.mul_small(12);
            if t1 >= t2 {
                t1.sub(&t2).unwrap()
            } else {
                continue;
            }
        };
        if next2 <= A {
            let pair = (cur.clone(), next2.clone());
            if !visited.contains(&pair) {
                visited.insert(pair.clone());
                idx_map.entry(idx+1).or_insert_with(HashSet::new).insert(next2.clone());
                q.push_back((pair.0, pair.1, idx+1));
            }
        }
    }

    let mut years: Vec<usize> = Vec::new();
    for (&idx, set) in &idx_map {
        if set.contains(&A) {
            years.push(idx);
        }
    }
    years.sort();

    if years.is_empty() {
        println!("NO");
        return;
    }
    println!("YES");
    let k = years.len();
    if k > 1000 {
        println!("1000");
        for i in 0..1000 {
            if i>0 { print!(" "); }
            print!("{}", years[i]);
        }
        println!();
    } else {
        println!("{}", k);
        for (i, y) in years.iter().enumerate() {
            if i>0 { print!(" "); }
            print!("{}", y);
        }
        println!();
    }

    let mut alts_set: HashSet<Big> = HashSet::new();
    for &y in &years {
        if let Some(set) = idx_map.get(&y) {
            for v in set {
                if v != &A {
                    alts_set.insert(v.clone());
                }
            }
        }
    }
    let mut alts: Vec<Big> = alts_set.into_iter().collect();
    alts.sort();
    let p = alts.len();
    if p > 1000 {
        println!("1000");
        for i in 0..1000 {
            println!("{}", alts[i].to_string_dec());
        }
    } else {
        println!("{}", p);
        for v in alts {
            println!("{}", v.to_string_dec());
        }
    }
}