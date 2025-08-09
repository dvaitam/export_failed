use std::io::{self, Read};
use std::collections::{HashMap, BinaryHeap};

struct BIT {
    n: usize,
    bit: Vec<i32>,
}
impl BIT {
    fn new(n: usize) -> Self { Self { n, bit: vec![0; n+1] } }
    fn add(&mut self, mut i: usize, v: i32) {
        while i <= self.n {
            self.bit[i] += v;
            i += i & (!i + 1);
        }
    }
    fn sum(&self, mut i: usize) -> i32 {
        let mut s = 0;
        while i > 0 {
            s += self.bit[i];
            i &= i - 1;
        }
        s
    }
}

enum Ev { Query(i64,i64), Id(i64) }

fn main() {
    let mut s = String::new();
    io::stdin().read_to_string(&mut s).unwrap();
    let mut it = s.split_whitespace();
    let n: i64 = it.next().unwrap().parse().unwrap();
    let q: usize = it.next().unwrap().parse().unwrap();
    let mut events: Vec<Ev> = Vec::with_capacity(q);
    for _ in 0..q {
        let tok = it.next().unwrap();
        if tok == "0" {
            let a: i64 = it.next().unwrap().parse().unwrap();
            let b: i64 = it.next().unwrap().parse().unwrap();
            events.push(Ev::Query(a,b));
        } else {
            let id: i64 = tok.parse().unwrap();
            events.push(Ev::Id(id));
        }
    }

    let mut mapL: HashMap<i64,i64> = HashMap::new();
    let mut mapR: HashMap<i64,i64> = HashMap::new();
    let mut heap: BinaryHeap<(i64,i64,i64)> = BinaryHeap::new();
    mapL.insert(1, n);
    mapR.insert(n, 1);
    heap.push((n, n, 1));

    let mut present: HashMap<i64,i64> = HashMap::new();
    let mut pos_for_event: Vec<Option<i64>> = vec![None; q];
    let mut is_arrival: Vec<bool> = vec![false; q];

    for (idx, ev) in events.iter().enumerate() {
        match *ev {
            Ev::Query(_,_) => {},
            Ev::Id(id) => {
                if let Some(&pos) = present.get(&id) {
                    pos_for_event[idx] = Some(pos);
                    is_arrival[idx] = false;
                    present.remove(&id);
                    let mut new_l = pos;
                    let mut new_r = pos;
                    if let Some(&l1) = mapR.get(&(pos-1)) {
                        let left_l = l1;
                        let left_r = pos-1;
                        mapR.remove(&left_r);
                        mapL.remove(&left_l);
                        new_l = left_l;
                    }
                    if let Some(&r1) = mapL.get(&(pos+1)) {
                        let right_r = r1;
                        let right_l = pos+1;
                        mapL.remove(&right_l);
                        mapR.remove(&right_r);
                        new_r = right_r;
                    }
                    mapL.insert(new_l, new_r);
                    mapR.insert(new_r, new_l);
                    let len = new_r - new_l + 1;
                    heap.push((len, new_r, new_l));
                } else {
                    // arrival
                    let mut chosen_l = 0i64;
                    let mut chosen_r = 0i64;
                    while let Some((len, r, l)) = heap.pop() {
                        if let Some(&cur_r) = mapL.get(&l) {
                            if cur_r == r {
                                chosen_l = l;
                                chosen_r = r;
                                break;
                            }
                        }
                    }
                    mapL.remove(&chosen_l);
                    mapR.remove(&chosen_r);
                    let mid = (chosen_l + chosen_r + 1) / 2;
                    pos_for_event[idx] = Some(mid);
                    is_arrival[idx] = true;
                    present.insert(id, mid);
                    if chosen_l <= mid-1 {
                        mapL.insert(chosen_l, mid-1);
                        mapR.insert(mid-1, chosen_l);
                        let len = (mid-1) - chosen_l + 1;
                        heap.push((len, mid-1, chosen_l));
                    }
                    if mid+1 <= chosen_r {
                        mapL.insert(mid+1, chosen_r);
                        mapR.insert(chosen_r, mid+1);
                        let len = chosen_r - (mid+1) + 1;
                        heap.push((len, chosen_r, mid+1));
                    }
                }
            }
        }
    }

    let mut used_pos: Vec<i64> = Vec::new();
    for p in pos_for_event.iter().flatten() { used_pos.push(*p); }
    used_pos.sort_unstable();
    used_pos.dedup();
    let m = used_pos.len();
    let mut bit = BIT::new(m);

    let mut out = String::new();
    for (idx, ev) in events.iter().enumerate() {
        match *ev {
            Ev::Query(a,b) => {
                let l = match used_pos.binary_search(&a) {
                    Ok(x) => x,
                    Err(x) => x,
                };
                let r = match used_pos.binary_search(&b) {
                    Ok(x) => x + 1,
                    Err(x) => x,
                };
                if l >= r {
                    out.push_str("0\n");
                } else {
                    let s = bit.sum(r) - bit.sum(l);
                    out.push_str(&format!("{}\n", s));
                }
            }
            Ev::Id(_) => {
                let pos = pos_for_event[idx].unwrap();
                let ci = used_pos.binary_search(&pos).unwrap() + 1;
                if is_arrival[idx] {
                    bit.add(ci, 1);
                } else {
                    bit.add(ci, -1);
                }
            }
        }
    }
    print!("{}", out);
}