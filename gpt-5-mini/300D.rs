use std::io::Read;

fn conv(a: &Vec<i64>, b: &Vec<i64>, kmax: usize, m: i64) -> Vec<i64> {
    let mut c = vec![0i64; kmax+1];
    for i in 0..=kmax {
        if a[i] == 0 { continue; }
        let ai = a[i];
        let lim = kmax - i;
        for j in 0..=lim {
            if b[j] == 0 { continue; }
            let idx = i + j;
            c[idx] += ai * b[j];
            if c[idx].abs() > (1i64<<62) { c[idx] %= m; }
        }
    }
    for v in &mut c { *v %= m; if *v < 0 { *v += m; } }
    c
}

fn depth_count(mut n: i64) -> i32 {
    if n < 3 || n % 2 == 0 { return -1; }
    let mut h = 0i32;
    while n >= 3 && n % 2 == 1 {
        // level h exists
        n = (n - 1) / 2;
        if n < 3 || n % 2 == 0 { break; }
        h += 1;
    }
    // Need count of levels: we counted while reducing; but this returns number of depths-1?
    // Simpler: recompute properly: count levels where size >=3 and odd
    // Let's do separate
    0
}

fn compute_levels(mut n: i64) -> i32 {
    if n < 3 || n % 2 == 0 { return -1; }
    let mut cnt = 0i32;
    while n >= 3 && n % 2 == 1 {
        cnt += 1;
        n = (n - 1) / 2;
    }
    return cnt - 1; // cnt levels means depths 0..cnt-1 so max depth = cnt-1
}

fn main() {
    const MOD: i64 = 7340033;
    let mut s = String::new();
    std::io::stdin().read_to_string(&mut s).unwrap();
    let mut it = s.split_whitespace();
    let q: usize = it.next().unwrap().parse().unwrap();
    let mut queries = Vec::with_capacity(q);
    let mut maxk = 0usize;
    let mut maxn = 0i64;
    for _ in 0..q {
        let n: i64 = it.next().unwrap().parse().unwrap();
        let k: usize = it.next().unwrap().parse().unwrap();
        if k > maxk { maxk = k; }
        if n > maxn { maxn = n; }
        queries.push((n,k));
    }
    // compute Hmax
    let mut temp = maxn;
    let mut cnt = 0i32;
    while temp >= 3 && temp % 2 == 1 {
        cnt += 1;
        temp = (temp - 1) / 2;
    }
    let hmax = if cnt==0 { -1 } else { cnt-1 }; // max depth index
    let mut G: Vec<Vec<i64>> = Vec::new();
    if hmax >= 0 {
        // kmax is maxk
        let kmax = maxk;
        // G^{(0)} = 1 + x
        let mut g0 = vec![0i64; kmax+1];
        g0[0] = 1;
        if kmax >= 1 { g0[1] = 1; }
        G.push(g0);
        for _h in 1..=hmax {
            let prev = &G[G.len()-1];
            let a = conv(prev, prev, kmax, MOD); // prev^2
            let b = conv(&a, &a, kmax, MOD); // prev^4
            let mut f = vec![0i64; kmax+1];
            for i in 1..=kmax {
                f[i] = b[i-1] % MOD;
            }
            let mut gnew = vec![0i64; kmax+1];
            gnew[0] = 1;
            for i in 1..=kmax {
                gnew[i] = f[i] % MOD;
            }
            G.push(gnew);
        }
    }
    let mut out = String::new();
    for (n,k) in queries {
        if k == 0 {
            out.push_str("1\n");
            continue;
        }
        if n < 3 || n % 2 == 0 {
            out.push_str("0\n");
            continue;
        }
        // compute depth H for this n
        let mut tmp = n;
        let mut cnt = 0i32;
        while tmp >= 3 && tmp % 2 == 1 {
            cnt += 1;
            tmp = (tmp - 1) / 2;
        }
        let h = cnt - 1;
        if h < 0 {
            out.push_str("0\n");
            continue;
        }
        if k > maxk {
            out.push_str("0\n");
            continue;
        }
        let ans = G[h as usize][k] % MOD;
        out.push_str(&format!("{}\n", ans));
    }
    print!("{}", out);
}