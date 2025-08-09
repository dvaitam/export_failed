use std::io::{self, Read};

const MOD: i64 = 998244353;

fn distinct_prime_factors(mut x: i64) -> Vec<i64> {
    let mut res = Vec::new();
    if x % 2 == 0 {
        res.push(2);
        while x % 2 == 0 {
            x /= 2;
        }
    }
    let mut d = 3i64;
    while d * d <= x {
        if x % d == 0 {
            res.push(d);
            while x % d == 0 {
                x /= d;
            }
        }
        d += 2;
    }
    if x > 1 {
        res.push(x);
    }
    res
}

fn count_coprime(u: i64, ratio: i64, base_primes: &Vec<i64>) -> i64 {
    if u <= 0 {
        return 0;
    }
    if ratio == 1 {
        return u;
    }
    let mut primes = Vec::new();
    for &p in base_primes.iter() {
        if ratio % p == 0 {
            primes.push(p);
        }
    }
    let k = primes.len();
    let mut res = u;
    let mut mask: usize = 1;
    while mask < (1usize << k) {
        let mut prod: i64 = 1;
        let mut bits = 0;
        let mut j = 0usize;
        let mut m = mask;
        while m > 0 {
            if (m & 1) == 1 {
                prod *= primes[j];
                bits += 1;
            }
            j += 1;
            m >>= 1;
        }
        let add = u / prod;
        if bits % 2 == 1 {
            res -= add;
        } else {
            res += add;
        }
        mask += 1;
    }
    res
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let t: usize = it.next().unwrap().parse().unwrap();
    let mut outputs = Vec::with_capacity(t);

    for _ in 0..t {
        let n: usize = it.next().unwrap().parse().unwrap();
        let m: i64 = it.next().unwrap().parse().unwrap();
        let mut a = vec![0i64; n];
        for i in 0..n {
            a[i] = it.next().unwrap().parse().unwrap();
        }

        if a[0] > m {
            outputs.push("0".to_string());
            continue;
        }

        let base_primes = distinct_prime_factors(a[0]);
        let mut ans: i64 = 1;
        let mut ok = true;

        for i in 1..n {
            let prev = a[i - 1];
            let curr = a[i];
            if prev % curr != 0 {
                ok = false;
                break;
            }
            let ratio = prev / curr;
            let u = m / curr;
            let cnt = count_coprime(u, ratio, &base_primes);
            let cnt_mod = (cnt % MOD + MOD) % MOD;
            ans = ((ans as i128 * cnt_mod as i128) % MOD as i128) as i64;
        }

        if ok {
            outputs.push(ans.to_string());
        } else {
            outputs.push("0".to_string());
        }
    }

    println!("{}", outputs.join("\n"));
}