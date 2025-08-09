use std::io::{self, Read};
fn is_pow2(x: i64) -> bool { x>0 && (x & (x-1))==0 }
fn main(){
    let mut s = String::new();
    io::stdin().read_to_string(&mut s).unwrap();
    let mut it = s.split_whitespace();
    let t: usize = it.next().unwrap().parse().unwrap();
    let mut out = String::new();
    for _ in 0..t {
        let n: usize = it.next().unwrap().parse().unwrap();
        let mut a = Vec::with_capacity(n);
        let mut sum: i64 = 0;
        for _ in 0..n {
            let v: i64 = it.next().unwrap().parse().unwrap();
            a.push(v);
            sum += v;
        }
        if sum % (n as i64) != 0 {
            out.push_str("No\n");
            continue;
        }
        let avg = sum / (n as i64);
        let mut cnt = [0i64; 35];
        let mut ok = true;
        for &v in &a {
            let delta = v - avg;
            if delta == 0 { continue; }
            let mut found = false;
            for p in 0..32 {
                let two_p = 1i64 << p;
                let tval = two_p - delta;
                if tval == 0 {
                    cnt[p] += 1;
                    found = true;
                    break;
                }
                if tval > 0 && is_pow2(tval) {
                    let q = tval.trailing_zeros() as usize;
                    cnt[p] += 1;
                    cnt[q] -= 1;
                    found = true;
                    break;
                }
            }
            if !found {
                ok = false;
                break;
            }
        }
        if !ok {
            out.push_str("No\n");
            continue;
        }
        if cnt.iter().all(|&x| x==0) {
            out.push_str("Yes\n");
        } else {
            out.push_str("No\n");
        }
    }
    print!("{}", out);
}