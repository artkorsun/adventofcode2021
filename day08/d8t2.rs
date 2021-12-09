use std::io;
use std::collections::HashSet;
use std::collections::BTreeSet;
use std::collections::HashMap;

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

    let mut res = 0;

    for (i, sp) in signal_patterns.iter().enumerate() {
        let _1 = sp.iter().find(|pattern| pattern.len() == 2).unwrap();
        let _4 = sp.iter().find(|pattern| pattern.len() == 4).unwrap();
        let _7 = sp.iter().find(|pattern| pattern.len() == 3).unwrap();
        let _8 = sp.iter().find(|pattern| pattern.len() == 7).unwrap();

        let _0_and_6_and_9 = sp.iter().filter(|pattern| pattern.len() == 6).collect::<HashSet<_>>();
        let _2_and_3_and_5 = sp.iter().filter(|pattern| pattern.len() == 5).collect::<HashSet<_>>();

        let _4_chars = _4.chars().collect::<HashSet<_>>();

        let mut _9: &str = "";
        let mut _0_and_6 = HashSet::new();

        for pattern in &_0_and_6_and_9 {
            let chars = pattern.chars().collect::<HashSet<_>>();

            let diff = _4_chars.difference(&chars).collect::<HashSet<_>>();

            if diff.len() > 0 {
                _0_and_6.insert(pattern);
            }
            else {
                _9 = pattern;
            }
        }

        let _8_chars = _8.chars().collect::<HashSet<_>>();
        let _9_chars = _9.chars().collect::<HashSet<_>>();
        let _8_9_diff = _8_chars.difference(&_9_chars).collect::<HashSet<_>>();

        //  -
        // | |
        //  -
        // * |
        //  _
        let bottom_left = _8_9_diff.iter().next().unwrap();

        let mut _2: &str = "";
        let mut _3_and_5 = HashSet::new();

        for pattern in &_2_and_3_and_5 {
            let chars = pattern.chars().collect::<HashSet<_>>();

            if !chars.contains(bottom_left) {
                _3_and_5.insert(pattern);
            }
            else {
                _2 = pattern;
            }
        }

        let mut _3: &str = "";
        let mut _5: &str = "";

        let _7_chars = _7.chars().collect::<HashSet<_>>();

        for pattern in &_3_and_5 {
            let chars = pattern.chars().collect::<HashSet<_>>();

            let diff = _7_chars.difference(&chars).collect::<HashSet<_>>();

            if diff.len() > 0 {
                _5 = pattern;
            }
            else {
                _3 = pattern;
            }
        }

        let mut _6: &str = "";
        let mut _0: &str = "";

        for pattern in &_0_and_6 {
            let chars = pattern.chars().collect::<HashSet<_>>();

            let diff = _7_chars.difference(&chars).collect::<HashSet<_>>();

            if diff.len() > 0 {
                _6 = pattern;
            }
            else {
                _0 = pattern;
            }
        }

        let mut vals = HashMap::new();

        vals.insert(_0.chars().collect::<BTreeSet<_>>(), 0);
        vals.insert(_1.chars().collect::<BTreeSet<_>>(), 1);
        vals.insert(_2.chars().collect::<BTreeSet<_>>(), 2);
        vals.insert(_3.chars().collect::<BTreeSet<_>>(), 3);
        vals.insert(_4.chars().collect::<BTreeSet<_>>(), 4);
        vals.insert(_5.chars().collect::<BTreeSet<_>>(), 5);
        vals.insert(_6.chars().collect::<BTreeSet<_>>(), 6);
        vals.insert(_7.chars().collect::<BTreeSet<_>>(), 7);
        vals.insert(_8.chars().collect::<BTreeSet<_>>(), 8);
        vals.insert(_9.chars().collect::<BTreeSet<_>>(), 9);

        for (order, output_val) in outputs[i].iter().rev().enumerate() {
            res += vals.get(&output_val.chars().collect::<BTreeSet<_>>()).unwrap() * i32::pow(10, order as u32);
        }
    }

    println!("Result: {:?}", res);

    return Ok(());
}

