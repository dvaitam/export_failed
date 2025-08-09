use std::collections::HashSet;
use std::io::{self, BufRead, BufWriter, Write};

fn solve(s: &str) -> (i64, i64) {
    let mut path_points = HashSet::new();
    let mut x: i64 = 0;
    let mut y: i64 = 0;

    for c in s.chars() {
        match c {
            'L' => x -= 1,
            'R' => x += 1,
            'D' => y -= 1,
            'U' => y += 1,
            _ => (),
        }
        path_points.insert((x, y));
    }

    for &(obs_x, obs_y) in &path_points {
        if obs_x == 0 && obs_y == 0 {
            continue;
        }

        let mut cur_x: i64 = 0;
        let mut cur_y: i64 = 0;

        for c in s.chars() {
            let (target_x, target_y) = match c {
                'L' => (cur_x - 1, cur_y),
                'R' => (cur_x + 1, cur_y),
                'D' => (cur_x, cur_y - 1),
                'U' => (cur_x, cur_y + 1),
                _ => (cur_x, cur_y),
            };

            if (target_x, target_y) != (obs_x, obs_y) {
                cur_x = target_x;
                cur_y = target_y;
            }
        }

        if cur_x == 0 && cur_y == 0 {
            return (obs_x, obs_y);
        }
    }

    (0, 0)
}

fn main() {
    let stdin = io::stdin();
    let mut reader = stdin.lock();
    let stdout = io::stdout();
    let mut writer = BufWriter::new(stdout.lock());

    let mut t_str = String::new();
    reader.read_line(&mut t_str).unwrap();
    let t: i32 = t_str.trim().parse().unwrap();

    let mut s_buf = String::new();
    for _ in 0..t {
        s_buf.clear();
        reader.read_line(&mut s_buf).unwrap();
        let s = s_buf.trim();
        let (x, y) = solve(s);
        writeln!(writer, "{} {}", x, y).unwrap();
    }
}