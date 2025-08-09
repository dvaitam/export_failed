use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let t: usize = it.next().unwrap().parse().unwrap();
    let mut outputs = Vec::with_capacity(t);

    for _ in 0..t {
        let m: usize = it.next().unwrap().parse().unwrap();
        let k: usize = it.next().unwrap().parse().unwrap();

        let mut a = vec![0i64; k];
        for i in 0..k {
            a[i] = it.next().unwrap().parse::<i64>().unwrap();
        }

        let n = m - 1;
        let mut t_vec = vec![0i32; n];
        let mut r_vec = vec![0i32; n];

        let mut taken = vec![0i64; k];
        let mut last_occ = vec![usize::MAX; k];
        let mut total_unknown: i64 = 0;

        let mut first_dis_opt: Option<usize> = None;

        for j in 0..n {
            let tj: i32 = it.next().unwrap().parse().unwrap();
            let rj: i32 = it.next().unwrap().parse().unwrap();
            t_vec[j] = tj;
            r_vec[j] = rj;

            if tj == 0 {
                total_unknown += 1;
            } else {
                let idx = (tj - 1) as usize;
                taken[idx] += 1;
                last_occ[idx] = j;
            }

            if rj == 1 && first_dis_opt.is_none() {
                first_dis_opt = Some(j);
            }
        }

        let mut ans = String::with_capacity(k);

        if first_dis_opt.is_none() {
            for i in 0..k {
                let need = (a[i] - taken[i]).max(0);
                if need <= total_unknown {
                    ans.push('Y');
                } else {
                    ans.push('N');
                }
            }
            outputs.push(ans);
            continue;
        }

        let p = first_dis_opt.unwrap();

        let mut fpre = vec![0i64; k];
        let mut uprefix: i64 = 0;
        for j in 0..p {
            if t_vec[j] == 0 {
                uprefix += 1;
            } else {
                let idx = (t_vec[j] - 1) as usize;
                fpre[idx] += 1;
            }
        }

        let mut need_dmin: i64 = i64::MAX;
        for i in 0..k {
            let eli = last_occ[i] < p;
            if eli {
                let need_pref = (a[i] - fpre[i]).max(0);
                if need_pref < need_dmin {
                    need_dmin = need_pref;
                }
            }
        }
        if need_dmin == i64::MAX {
            need_dmin = 0; // Shouldn't happen for consistent input, but safe guard
        }

        for i in 0..k {
            let need = (a[i] - taken[i]).max(0);
            let eli = last_occ[i] < p;
            let need_pref = (a[i] - fpre[i]).max(0);

            let ok = if eli && uprefix >= need_pref {
                need <= total_unknown
            } else {
                need <= (total_unknown - need_dmin)
            };

            if ok {
                ans.push('Y');
            } else {
                ans.push('N');
            }
        }

        outputs.push(ans);
    }

    println!("{}", outputs.join("\n"));
}