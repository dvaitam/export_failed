use std::io::{self, BufRead};

fn main() {
    solve();
}

fn solve() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    
    let n: usize = lines.next().unwrap().unwrap().parse().unwrap();
    
    let mut a: Vec<i64> = Vec::with_capacity(n);
    for _ in 0..n {
        a.push(lines.next().unwrap().unwrap().parse().unwrap());
    }
    
    a.sort_unstable();

    match n {
        0 => {
            println!("YES");
            println!("1");
            println!("1");
            println!("3");
            println!("3");
        }
        1 => {
            let val = a[0];
            println!("YES");
            println!("{}", 2 * val);
            println!("{}", 2 * val);
            println!("{}", 3 * val);
        }
        2 => {
            let a1 = a[0];
            let a2 = a[1];

            if a2 <= 2 * a1 {
                let y3 = 4 * a1 - a2;
                let y4 = 3 * a1;
                if y3 > 0 && a2 <= y3 {
                    println!("YES");
                    println!("{}", y3);
                    println!("{}", y4);
                    return;
                }
            }
            
            if a2 >= 2 * a1 && a2 <= 3 * a1 {
                let y2 = 4 * a1 - a2;
                let y4 = 3 * a1;
                if y2 > 0 && a1 <= y2 && y2 <= a2 {
                     println!("YES");
                     println!("{}", y2);
                     println!("{}", y4);
                     return;
                }
            }

            if a2 == 3 * a1 {
                let y2 = 2 * a1;
                if y2 > 0 {
                    println!("YES");
                    println!("{}", y2);
                    println!("{}", y2);
                    return;
                }
            }

            if (a1 + a2) % 4 == 0 {
                let y1 = (a1 + a2) / 4;
                if y1 > 0 && a2 <= 3 * y1 {
                   let y4 = 3 * y1;
                   if a2 <= y4 {
                        println!("YES");
                        println!("{}", y1);
                        println!("{}", y4);
                        return;
                   }
                }
            }
            
            if a2 % 3 == 0 {
                let y1 = a2 / 3;
                if y1 > 0 && y1 <= a1 && 3 * a1 <= 2 * a2 {
                    let y3 = 4 * y1 - a1;
                    if y3 > 0 && a1 <= y3 && y3 <= a2 {
                         println!("YES");
                         println!("{}", y1);
                         println!("{}", y3);
                         return;
                    }
                }
            }
            
            if a2 % 3 == 0 {
                let y1 = a2 / 3;
                if y1 > 0 && 3 * a1 >= 2 * a2 && a1 <= a2 {
                    let y2 = 4 * y1 - a1;
                    if y2 > 0 && y1 <= y2 && y2 <= a1 {
                        println!("YES");
                        println!("{}", y1);
                        println!("{}", y2);
                        return;
                    }
                }
            }
            
            println!("NO");
        }
        3 => {
            let a1 = a[0];
            let a2 = a[1];
            let a3 = a[2];

            if a3 % 3 == 0 {
                let x = a3 / 3;
                if x > 0 && a1 + a2 == 4 * x && x <= a1 {
                    println!("YES");
                    println!("{}", x);
                    return;
                }
            }

            if a2 + a3 == 4 * a1 {
                let x = 3 * a1;
                if x > 0 && a3 <= x && a2 <= 2 * a1 {
                    println!("YES");
                    println!("{}", x);
                    return;
                }
            }
            
            if a3 == 3 * a1 {
                let x = 4 * a1 - a2;
                if x > 0 && a2 <= x && x <= a3 && a2 <= 2 * a1 {
                    println!("YES");
                    println!("{}", x);
                    return;
                }
            }
            
            if a3 == 3 * a1 {
                let x = 4 * a1 - a2;
                if x > 0 && a1 <= x && x <= a2 {
                    println!("YES");
                    println!("{}", x);
                    return;
                }
            }

            println!("NO");
        }
        4 => {
            let a1 = a[0];
            let a2 = a[1];
            let a3 = a[2];
            let a4 = a[3];
            if a1 > 0 && a4 == 3 * a1 && a2 + a3 == 4 * a1 {
                println!("YES");
            } else {
                println!("NO");
            }
        }
        _ => {}
    }
}