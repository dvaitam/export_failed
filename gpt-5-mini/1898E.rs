use std::io::{self, Read};
fn main() {
    let mut s = String::new();
    io::stdin().read_to_string(&mut s).unwrap();
    let mut it = s.split_whitespace();
    let tc: usize = it.next().unwrap().parse().unwrap();
    let mut out = String::new();
    for _ in 0..tc {
        let _n: usize = it.next().unwrap().parse().unwrap();
        let _m: usize = it.next().unwrap().parse().unwrap();
        let s_str = it.next().unwrap();
        let t_str = it.next().unwrap();
        let mut cnts = [0usize;26];
        for ch in s_str.bytes() { cnts[(ch-b'a') as usize]+=1; }
        let mut ok = true;
        for ch in t_str.bytes() {
            let idx = (ch-b'a') as usize;
            if cnts[idx]==0 { ok=false; break; }
            cnts[idx]-=1;
        }
        out.push_str(if ok {"YES\n"} else {"NO\n"});
    }
    print!("{}", out);
}