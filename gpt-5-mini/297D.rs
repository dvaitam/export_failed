use std::io::{self, Read};
fn main(){
    let mut s = String::new();
    io::stdin().read_to_string(&mut s).unwrap();
    let mut it = s.split_whitespace();
    let h: usize = it.next().unwrap().parse().unwrap();
    let w: usize = it.next().unwrap().parse().unwrap();
    let k: usize = it.next().unwrap().parse().unwrap();
    let mut lines: Vec<String> = Vec::new();
    for _ in 0..(2*h-1) {
        lines.push(it.next().unwrap().to_string());
    }
    let mut numE = vec![0u64; 16];
    let mut numN = vec![0u64; 16];
    let mut total: u64 = 0;
    for i in 0..h {
        // horizontal between (i,j) and (i,j+1) from lines[2*i], length w-1
        let hor = lines[2*i].as_bytes();
        for j in 0..w-1 {
            let ch = hor[j] as char;
            let u = (i%2)*2 + (j%2);
            let v = (i%2)*2 + ((j+1)%2);
            let (a,b) = if u < v {(u,v)} else {(v,u)};
            let idx = a*4 + b;
            if ch == 'E' { numE[idx] += 1; } else { numN[idx] += 1; }
            total += 1;
        }
        if i+1 < h {
            let ver = lines[2*i+1].as_bytes();
            for j in 0..w {
                let ch = ver[j] as char;
                let u = (i%2)*2 + (j%2);
                let v = (((i+1)%2)*2) + (j%2);
                let (a,b) = if u < v {(u,v)} else {(v,u)};
                let idx = a*4 + b;
                if ch == 'E' { numE[idx] += 1; } else { numN[idx] += 1; }
                total += 1;
            }
        }
    }
    let mut best_sat: u64 = 0;
    let mut best_labels = vec![0usize;4];
    let mut labels = vec![0usize;4];
    fn gen(i: usize, maxl: usize, labels: &mut Vec<usize>, best_sat: &mut u64, best_labels: &mut Vec<usize>, numE: &Vec<u64>, numN: &Vec<u64>, total: u64, k: usize){
        if i==4 {
            let mut sat: u64 = 0;
            for u in 0..4 {
                for v in (u+1)..4 {
                    let idx = u*4+v;
                    if labels[u]==labels[v] { sat += numE[idx]; } else { sat += numN[idx]; }
                }
            }
            let groups = labels.iter().cloned().max().unwrap()+1;
            if groups <= k && sat > *best_sat {
                *best_sat = sat;
                best_labels.clone_from(labels);
                *best_labels = labels.clone();
            }
            return;
        }
        for lab in 0..=maxl+1 {
            labels[i]=lab;
            gen(i+1, if lab>maxl {lab} else {maxl}, labels, best_sat, best_labels, numE, numN, total, k);
        }
    }
    gen(1,0,&mut labels,&mut best_sat,&mut best_labels,&numE,&numN,total,k);
    if best_sat*4 >= 3*total {
        println!("YES");
        let mut color_map = vec![0usize;4];
        let mut used = 0;
        let mut map_assign = std::collections::HashMap::new();
        for c in 0..4 {
            let g = best_labels[c];
            if !map_assign.contains_key(&g) {
                used += 1;
                map_assign.insert(g, used);
            }
            color_map[c] = *map_assign.get(&g).unwrap();
        }
        for i in 0..h {
            let mut row = String::new();
            for j in 0..w {
                let c = (i%2)*2 + (j%2);
                let col = color_map[c];
                if j>0 { row.push(' '); }
                row.push_str(&col.to_string());
            }
            println!("{}", row);
        }
    } else {
        println!("NO");
    }
}