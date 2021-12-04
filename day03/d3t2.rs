use std::io;
use std::io::prelude::*;
use std::fs::File;

fn filter(source_lines: Vec<&str>, char_index: usize, prevail_majority: bool) -> Vec<&str> {
    let mut temp_zeros: Vec<&str> = vec![];
    let mut temp_ones: Vec<&str> = vec![];

    for line in source_lines {
        if line.chars().nth(char_index) == Some('0') {
            temp_zeros.push(line);
        }
        else {
            temp_ones.push(line);
        }
    }

    if prevail_majority {
        if temp_zeros.len() > temp_ones.len() {
            return temp_zeros;
        }

        if temp_ones.len() > temp_zeros.len() {
            return temp_ones;
        }
    }
    else {
        if temp_zeros.len() < temp_ones.len() {
            return temp_zeros;
        }

        if temp_ones.len() < temp_zeros.len() {
            return temp_ones;
        }
    }

    if prevail_majority {
        return temp_ones;
    }

    return temp_zeros;
}

fn main() -> io::Result<()> {
    let mut f = File::open("input.dat")?;
    let mut buffer = String::new();

    f.read_to_string(&mut buffer)?;

    let lines = buffer.lines().collect::<Vec<_>>();

    let mut oxygen = lines.clone();
    let mut co2 = lines.clone();

    for i in 0..lines[0].len() {
        if oxygen.len() != 1 {
            oxygen = filter(oxygen, i, false);
        }
        if co2.len() != 1 {
            co2 = filter(co2, i, true);
        }
    }

    println!("Oxygen: {:?}, CO2: {:?}, res: {:?}", oxygen, co2, isize::from_str_radix(&oxygen[0], 2).unwrap() * isize::from_str_radix(&co2[0], 2).unwrap());
    return Ok(());
}

