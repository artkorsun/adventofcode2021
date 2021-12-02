use std::io;
use std::io::prelude::*;
use std::fs::File;

fn main() -> io::Result<()> {
    let mut f = File::open("input.dat")?;
    let mut buffer = String::new();

    f.read_to_string(&mut buffer)?;

    let mut vals = Vec::new();

    for s in buffer.lines() {
        let n:i32 = s.parse().unwrap();

        vals.push(n);
    }

    let mut inc = 0;

    for i in 1..vals.len() {
        if vals[i] > vals[i - 1] {
            inc = inc + 1;
        }
    }

    println!("Result: {}", inc);
    return Ok(());
}

