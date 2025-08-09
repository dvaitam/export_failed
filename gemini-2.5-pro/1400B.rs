use std::io::{self, BufRead};
use std::cmp;
use std::mem::swap;

fn solve() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let line1 = lines.next().unwrap().unwrap();
    let mut iter = line1.split_whitespace().map(|x| x.parse::<u64>().unwrap());
    let p = iter.next().unwrap();
    let f = iter.next().unwrap();

    let line2 = lines.next().unwrap().unwrap();
    let mut iter = line2.split_whitespace().map(|x| x.parse::<u64>().unwrap());
    let mut cnt_s = iter.next().unwrap();
    let mut cnt_w = iter.next().unwrap();

    let line3 = lines.next().unwrap().unwrap();
    let mut iter = line3.split_whitespace().map(|x| x.parse::<u64>().unwrap());
    let mut s = iter.next().unwrap();
    let mut w = iter.next().unwrap();

    if s > w {
        swap(&mut s, &mut w);
        swap(&mut cnt_s, &mut cnt_w);
    }

    let mut max_items = 0;

    for num_s_me in 0..=cnt_s {
        if num_s_me * s > p {
            break;
        }

        let num_w_me = cmp::min(cnt_w, (p - num_s_me * s) / w);

        let rem_s = cnt_s - num_s_me;
        let rem_w = cnt_w - num_w_me;

        let mut num_s_follower = 0;
        let mut num_w_follower = 0;

        if s <= w {
            num_s_follower = cmp::min(rem_s, f / s);
            num_w_follower = cmp::min(rem_w, (f - num_s_follower * s) / w);
        } else {
            num_w_follower = cmp::min(rem_w, f / w);
            num_s_follower = cmp::min(rem_s, (f - num_w_follower * w) / s);
        }

        max_items = cmp::max(max_items, num_s_me + num_w_me + num_s_follower + num_w_follower);
    }

    println!("{}", max_items);
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    let t_str = lines.next().unwrap().unwrap();
    let t: i32 = t_str.parse().unwrap();
    for _ in 0..t {
        solve();
    }
}