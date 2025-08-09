use std::io::{self, Read};

fn main() {
    // Read input
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace().map(|s| s.parse::<i64>().unwrap());

    let t = it.next().unwrap();
    let mut out = String::new();

    for _ in 0..t {
        let n = it.next().unwrap();
        let mut b = Vec::with_capacity(n as usize);
        for _ in 0..n {
            b.push(it.next().unwrap());
        }

        if n <= 2 {
            out.push_str("YES\n");
            continue;
        }

        let mut low = i64::MIN; // allowed lower bound for current b[i]
        let mut high = i64::MAX; // allowed upper bound for current b[i]
        let mut ok = true;

        for i in 1..n as usize {
            let cur = b[i];
            if cur < low || cur > high {
                ok = false;
                break;
            }
            let prev = b[i - 1];
            if cur > prev {
                if prev > low { low = prev; }
                high = i64::MAX;
            } else if cur < prev {
                if prev < high { high = prev; }
                low = i64::MIN;
            } else {
                // equal: interval unchanged
            }
        }

        if ok {
            out.push_str("YES\n");
        } else {
            out.push_str("NO\n");
        }
    }

    print!("{}", out);
}