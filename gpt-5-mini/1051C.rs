use std::io::{self, Read};
fn main() {
    let mut s = String::new();
    io::stdin().read_to_string(&mut s).unwrap();
    let mut it = s.split_whitespace();
    let n: usize = it.next().unwrap().parse().unwrap();
    let mut a: Vec<usize> = Vec::with_capacity(n);
    for _ in 0..n {
        a.push(it.next().unwrap().parse().unwrap());
    }
    let mut pos: Vec<Vec<usize>> = vec![Vec::new(); 101];
    for (i, &v) in a.iter().enumerate() {
        pos[v].push(i);
    }
    let mut uniq = 0;
    for v in 1..=100 {
        if pos[v].len() == 1 { uniq += 1; }
    }
    if uniq % 2 == 1 {
        let mut found = false;
        for v in 1..=100 {
            if pos[v].len() >= 3 { found = true; break; }
        }
        if !found {
            println!("NO");
            return;
        }
    }
    let mut ans: Vec<char> = vec!['A'; n];
    let mut turn_a = true;
    for v in 1..=100 {
        if pos[v].len() == 1 {
            let p = pos[v][0];
            ans[p] = if turn_a { 'A' } else { 'B' };
            turn_a = !turn_a;
        }
    }
    if uniq % 2 == 1 {
        for v in 1..=100 {
            if pos[v].len() >= 3 {
                // move one occurrence to B
                let p = pos[v][0];
                ans[p] = 'B';
                break;
            }
        }
    }
    println!("YES");
    let s: String = ans.into_iter().collect();
    println!("{}", s);
}