use std::io;
use std::io::prelude::*;
use std::fs::File;

fn main() -> io::Result<()> {
    let mut f = File::open("input.dat")?;
    let mut buffer = String::new();

    f.read_to_string(&mut buffer)?;

    let mut ones = vec![0; 12];
    let mut lines_count = 0;

    for s in buffer.lines() {
        for (i, c) in s.chars().enumerate() {
            if c == '1' {
                ones[i] += 1;
            }
        }

        lines_count += 1;
    }

    let gamma_str = ones.iter().map(|x| if x > &(lines_count / 2) { '1' } else { '0' } ).collect::<String>();
    let epsilon_str = ones.iter().map(|x| if x > &(lines_count / 2) { '0' } else { '1' } ).collect::<String>();
    
    let gamma = isize::from_str_radix(&gamma_str, 2).unwrap();
    let epsilon = isize::from_str_radix(&epsilon_str, 2).unwrap();

    println!("Result: {:?}", gamma * epsilon);
    return Ok(());
}

