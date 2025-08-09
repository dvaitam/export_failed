use std::io::{self, Read};
fn gcd(mut a: i64, mut b: i64) -> i64 {
    while b != 0 {
        let t = a % b;
        a = b;
        b = t;
    }
    a.abs()
}
fn main() {
    let mut s = String::new();
    io::stdin().read_to_string(&mut s).unwrap();
    let mut it = s.split_whitespace();
    let k: usize = it.next().unwrap().parse().unwrap();
    let n: usize = it.next().unwrap().parse().unwrap();
    let mut p_list = Vec::with_capacity(k);
    for _ in 0..k { p_list.push(it.next().unwrap().parse::<usize>().unwrap()); }
    let mut x_list = Vec::with_capacity(n);
    for _ in 0..n { x_list.push(it.next().unwrap().parse::<usize>().unwrap()); }
    let maxp = *p_list.last().unwrap();
    let maxx = *x_list.last().unwrap();
    let mut is_period = vec![false; maxp+1];
    for &pp in &p_list { if pp<=maxp { is_period[pp]=true; } }
    let mut present = vec![false; maxx+1];
    for &x in &x_list { present[x]=true; }
    let mut visited = vec![0u32; maxx+1];
    let mut token: u32 = 1;
    for &p in &p_list {
        let r = x_list[0] % p;
        let start = if r==0 { p } else { r };
        let mut first_found = 0usize;
        // mark covered
        let mut v = start;
        while v <= maxx {
            if present[v] {
                visited[v]=token;
                if first_found==0 { first_found = v; }
            }
            v += p;
        }
        // compute gcd of remaining
        let mut base: i64 = -1;
        let mut g: i64 = 0;
        for &x in &x_list {
            if visited[x] != token {
                if base==-1 { base = x as i64; }
                else { g = gcd(g, (x as i64) - base); }
            }
        }
        if base==-1 {
            // all covered
            let s1 = if first_found!=0 { first_found } else { x_list[0] };
            let cp1 = p;
            let cp2 = p_list[0];
            let s2 = 1usize;
            println!("YES");
            println!("{} {}", s1, cp1);
            println!("{} {}", s2, cp2);
            return;
        } else {
            if g==0 {
                let cp2 = p_list[0];
                println!("YES");
                let s1 = if first_found!=0 { first_found } else { x_list[0] };
                println!("{} {}", s1, p);
                println!("{} {}", base as usize, cp2);
                return;
            } else {
                let mut found_cp2 = 0usize;
                let mut d = 1usize;
                while (d as i64) * (d as i64) <= g {
                    if g % (d as i64) == 0 {
                        let d2 = (g / (d as i64)) as usize;
                        if d <= maxp && is_period[d] { found_cp2 = d; break; }
                        if d2 <= maxp && is_period[d2] { found_cp2 = d2; break; }
                    }
                    d += 1;
                }
                if found_cp2!=0 {
                    println!("YES");
                    let s1 = if first_found!=0 { first_found } else { x_list[0] };
                    println!("{} {}", s1, p);
                    println!("{} {}", base as usize, found_cp2);
                    return;
                }
            }
        }
        token += 1;
    }
    println!("NO");
}