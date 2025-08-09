use std::io::{self, Read};
use std::collections::BTreeMap;
use std::ops::Bound::{Excluded, Unbounded};
fn main() {
    let mut s = String::new();
    io::stdin().read_to_string(&mut s).unwrap();
    let mut it = s.split_whitespace();
    let n: usize = it.next().unwrap().parse().unwrap();
    let mut a = Vec::with_capacity(n);
    for _ in 0..n {
        a.push(it.next().unwrap().parse::<i64>().unwrap());
    }
    let mut map: BTreeMap<i64, usize> = BTreeMap::new();
    map.insert(a[0], 0);
    let mut out = Vec::with_capacity(n - 1);
    for i in 1..n {
        let v = a[i];
        let pred = map.range(..v).next_back().map(|(k, idx)| (*k, *idx));
        let succ = map.range((Excluded(&v), Unbounded)).next().map(|(k, idx)| (*k, *idx));
        let p = match (pred, succ) {
            (None, Some((k, _))) => k,
            (Some((k, _)), None) => k,
            (Some((kp, ip)), Some((ks, is))) => if ip > is { kp } else { ks },
            _ => 0,
        };
        out.push(p.to_string());
        map.insert(v, i);
    }
    println!("{}", out.join(" "));
}