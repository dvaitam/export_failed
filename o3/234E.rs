use std::io::{self, Read};

#[derive(Clone)]
struct Team {
    name: String,
    rating: i32,
}

fn next_rand(x: &mut usize, a: usize, b: usize, c: usize) -> usize {
    *x = (*x * a + b) % c;
    *x
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();
    let n: usize = it.next().unwrap().parse().unwrap();
    let mut x: usize = it.next().unwrap().parse().unwrap();
    let a: usize = it.next().unwrap().parse().unwrap();
    let b: usize = it.next().unwrap().parse().unwrap();
    let c: usize = it.next().unwrap().parse().unwrap();

    let mut teams: Vec<Team> = (0..n)
        .map(|_| Team {
            name: it.next().unwrap().to_string(),
            rating: it.next().unwrap().parse().unwrap(),
        })
        .collect();

    teams.sort_by(|p, q| q.rating.cmp(&p.rating));

    let m = n / 4;
    let mut baskets: Vec<Vec<Team>> = vec![Vec::with_capacity(m); 4];
    for (i, t) in teams.into_iter().enumerate() {
        baskets[i / m].push(t);
    }

    let mut groups: Vec<Vec<Team>> = Vec::with_capacity(m);

    for g in 0..m {
        let mut group: Vec<Team> = Vec::with_capacity(4);
        if g < m - 1 {
            for b_idx in 0..4 {
                let s = baskets[b_idx].len();
                let k = next_rand(&mut x, a, b, c) % s;
                group.push(baskets[b_idx].remove(k));
            }
        } else {
            for b_idx in 0..4 {
                group.push(baskets[b_idx].pop().unwrap());
            }
        }
        group.sort_by(|p, q| q.rating.cmp(&p.rating));
        groups.push(group);
    }

    for (idx, group) in groups.iter().enumerate() {
        let letter = (b'A' + idx as u8) as char;
        println!("Group {}", letter);
        for team in group {
            println!("{}", team.name);
        }
    }
}