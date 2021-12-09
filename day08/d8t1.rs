use std::io;

fn main() -> io::Result<()> {
    let input = include_str!("input.dat");

    let mut signal_patterns = vec![]; 
    let mut outputs = vec![]; 

    for l in input.lines() {
        let mut data = l.split(" | ");
        let signals = data.next();
        let output = data.next();

        signal_patterns.push(signals.unwrap().split_whitespace().collect::<Vec<_>>());
        outputs.push(output.unwrap().split_whitespace().collect::<Vec<_>>());
    }

    let res: usize = outputs.iter().map(|v| v.iter().filter(|s| s.len() == 2 || s.len() == 3 || s.len() == 4 || s.len() == 7).count()).sum();

    println!("Result: {:?}", res);

    return Ok(());
}

