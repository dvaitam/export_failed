use std::io::{self, Read};

struct Scanner {
    buf: Vec<u8>,
    idx: usize,
}
impl Scanner {
    fn new() -> Self {
        let mut input = String::new();
        io::stdin().read_to_string(&mut input).unwrap();
        Self { buf: input.into_bytes(), idx: 0 }
    }
    fn next_usize(&mut self) -> usize {
        while self.idx < self.buf.len() && self.buf[self.idx].is_ascii_whitespace() {
            self.idx += 1;
        }
        let mut x = 0usize;
        while self.idx < self.buf.len() && !self.buf[self.idx].is_ascii_whitespace() {
            x = x * 10 + (self.buf[self.idx] - b'0') as usize;
            self.idx += 1;
        }
        x
    }
}

fn main() {
    let mut sc = Scanner::new();
    let t = sc.next_usize();
    let mut out = String::new();
    for _ in 0..t {
        let n = sc.next_usize();
        let mut adj = vec![Vec::<(usize, usize)>::new(); n + 1];
        for i in 1..n {
            let u = sc.next_usize();
            let v = sc.next_usize();
            adj[u].push((v, i));
            adj[v].push((u, i));
        }
        if n % 3 != 0 {
            out.push_str("-1\n");
            continue;
        }
        let mut state = vec![3u8; n + 1]; // 0,1,2 valid; 3 = unset
        let mut cuts: Vec<usize> = Vec::new();
        let mut ok = true;
        let mut stack: Vec<(usize, usize, usize, bool)> = Vec::with_capacity(2 * n);
        stack.push((1, 0, 0, false));
        while let Some((v, p, pe, processed)) = stack.pop() {
            if !processed {
                stack.push((v, p, pe, true));
                for &(to, ei) in &adj[v] {
                    if to == p { continue; }
                    stack.push((to, v, ei, false));
                }
            } else {
                let mut c0 = 0usize;
                let mut c1 = 0usize;
                let mut c2 = 0usize;
                for &(to, ei) in &adj[v] {
                    if to == p { continue; }
                    match state[to] {
                        0 => {
                            c0 += 1;
                            cuts.push(ei);
                        }
                        1 => c1 += 1,
                        2 => c2 += 1,
                        _ => {}
                    }
                }
                if c2 > 1 {
                    ok = false;
                    state[v] = 0;
                    continue;
                }
                if c2 == 1 {
                    if c1 > 0 {
                        ok = false;
                        state[v] = 0;
                    } else {
                        state[v] = 0;
                    }
                } else {
                    if c1 == 0 {
                        state[v] = 1;
                    } else if c1 == 1 {
                        state[v] = 2;
                    } else if c1 == 2 {
                        state[v] = 0;
                    } else {
                        ok = false;
                        state[v] = 0;
                    }
                }
            }
        }
        if !ok || state[1] != 0 {
            out.push_str("-1\n");
            continue;
        }
        out.push_str(&format!("{}\n", cuts.len()));
        if !cuts.is_empty() {
            for (i, e) in cuts.iter().enumerate() {
                if i > 0 { out.push(' '); }
                out.push_str(&e.to_string());
            }
        }
        out.push('\n');
    }
    print!("{}", out);
}