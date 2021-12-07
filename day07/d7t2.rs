use std::io;
use std::collections::BTreeMap;
use std::collections::HashMap;
use std::cmp::min;

fn get_move_cost(len: i32, cache: &mut HashMap<i32, i32>) -> i32 {
    if cache.contains_key(&len) {
        return *cache.get(&len).unwrap();
    }

    if len == 1 {
        cache.insert(1, 1);
        return 1;
    }

    let res = get_move_cost(len - 1, cache) + len;
    cache.insert(len, res);

    return res;
}

fn main() -> io::Result<()> {
    let input = include_str!("input.dat");

    let mut number_counts = BTreeMap::new();

    for n in input.lines().next().unwrap().split(",") {
        let val:i32 = n.parse().unwrap();
        let val_count = number_counts.entry(val).or_insert(0);
        *val_count += 1;
    }

    let mut cache = HashMap::new();

    let mut min_steps = i32::MAX;

    for i in *number_counts.keys().next().unwrap()..=*number_counts.keys().last().unwrap() {

        let mut res = 0;
        for (k, v) in &number_counts {
            if *k == i {
                continue;
            }
            res += get_move_cost((i - k).abs(), &mut cache) * v;
        }
        min_steps = min(min_steps, res);
    }

    println!("Result: {:?}", min_steps);

    return Ok(());
}

