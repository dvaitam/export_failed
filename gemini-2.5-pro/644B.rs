use std::io::BufRead;

struct Scanner<R: BufRead> {
    reader: R,
    buf_str: Vec<u8>,
    buf_iter: std::str::SplitWhitespace<'static>,
}

impl<R: BufRead> Scanner<R> {
    fn new(reader: R) -> Self {
        Self {
            reader,
            buf_str: Vec::new(),
            buf_iter: "".split_whitespace(),
        }
    }
}

fn main() {
    let stdin = std::io::stdin();
    let handle = stdin.lock();
    let _scanner = Scanner::new(handle);
}