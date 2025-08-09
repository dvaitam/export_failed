use std::io::{self, Read};

fn main() {
    // Read input
    let mut s = String::new();
    io::stdin().read_to_string(&mut s).unwrap();
    let mut it = s.split_whitespace();
    let n: usize = it.next().unwrap().parse().unwrap();
    let mut a = vec![0i32; n];
    for i in 0..n {
        a[i] = it.next().unwrap().parse::<i32>().unwrap();
    }

    let mut res: Vec<(i32, i32)> = Vec::with_capacity(2 * n);
    let mut st1: Vec<i32> = Vec::new(); // rows with exactly one point (available for a==2)
    let mut avail: Vec<i32> = Vec::new(); // columns with a bottom point placed (available for a==3)
    let mut cur_row: i32 = n as i32;

    let mut ok = true;

    for idx in (0..n).rev() {
        match a[idx] {
            0 => {}
            1 => {
                if cur_row <= 0 {
                    ok = false;
                    break;
                }
                let col = (idx + 1) as i32;
                res.push((cur_row, col));
                st1.push(cur_row);
                avail.push(col);
                cur_row -= 1;
            }
            2 => {
                if let Some(r) = st1.pop() {
                    let col = (idx + 1) as i32;
                    res.push((r, col));
                    avail.push(col);
                } else {
                    ok = false;
                    break;
                }
            }
            3 => {
                if cur_row <= 0 || avail.is_empty() {
                    ok = false;
                    break;
                }
                let partner_col = avail.pop().unwrap();
                let col = (idx + 1) as i32;
                // Place at new row both (cur_row, col) and (cur_row, partner_col)
                res.push((cur_row, col));
                res.push((cur_row, partner_col));
                // Now col has a bottom point and can serve as partner later
                avail.push(col);
                cur_row -= 1;
            }
            _ => {
                ok = false;
                break;
            }
        }
    }

    if !ok {
        println!("-1");
        return;
    }

    println!("{}", res.len());
    for (r, c) in res {
        println!("{} {}", r, c);
    }
}