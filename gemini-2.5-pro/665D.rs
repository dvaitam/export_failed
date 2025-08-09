```rust
use std::io::{self, BufRead, Write};

fn main() {
    let stdout = io::stdout();
    let mut writer = io::BufWriter::new(stdout.lock());
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    const MAX_N: usize = 2_000_001;
    let mut is_prime = vec