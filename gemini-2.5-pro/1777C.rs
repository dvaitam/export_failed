```rust
use std::io::{self, BufWriter, Read, Write};
use std::str::SplitWhitespace;

struct Scanner<'a> {
    iter: SplitWhitespace<'a>,
}

impl<'a> Scanner<'a> {
    fn new(s: &'a str)