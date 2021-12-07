use std::io;
use std::collections::BTreeMap;
use std::cmp::min;

fn main() -> io::Result<()> {
    let input = include_str!("input.dat");

    let mut number_counts = BTreeMap::new();

    for n in input.lines().next().unwrap().split(",") {
        let val:i32 = n.parse().unwrap();
        let val_count = number_counts.entry(val).or_insert(0);
        *val_count += 1;
    }

    let mut min_steps = i32::MAX;

    for i in *number_counts.keys().next().unwrap()..=*number_counts.keys().last().unwrap() {

        let mut res = 0;
        for (k, v) in &number_counts {
            if *k == i {
                continue;
            }
            res += (i - k).abs() * v;
        }
        min_steps = min(min_steps, res);
    }

    println!("Result: {:?}", min_steps);

    return Ok(());
}

