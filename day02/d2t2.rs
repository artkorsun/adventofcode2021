use std::io;
use std::io::prelude::*;
use std::fs::File;

fn main() -> io::Result<()> {
    let mut f = File::open("input.dat")?;
    let mut buffer = String::new();

    f.read_to_string(&mut buffer)?;

    let mut horizontal_pos:i32 = 0;
    let mut depth:i32 = 0;
    let mut aim:i32 = 0;

    for s in buffer.lines() {
        let mut iter = s.split_whitespace();

        let command = iter.next().unwrap();
        let val:i32 = iter.next().unwrap().parse().unwrap();

        match command {
            "forward" => {
                horizontal_pos += val;
                depth += aim * val;
            },
            "up" => aim -= val,
            "down" => aim += val,
            _ => {}
        }
    }

    println!("Result: {}", horizontal_pos * depth);
    return Ok(());
}

