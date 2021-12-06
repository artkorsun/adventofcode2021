use std::io;
use std::cmp;
use std::iter::FromIterator;
use std::collections::HashMap;

#[derive(PartialEq, Eq, Hash, Debug)]
struct Point(i32, i32);

impl FromIterator<i32> for Point {
    fn from_iter<I: IntoIterator<Item=i32>>(iter: I) -> Self {
        let mut temp = vec![];

        for i in iter {
            temp.push(i)
        }

        Point(temp[0], temp[1])
    }
}

fn main() -> io::Result<()> {
    let input = include_str!("input.dat");

    let mut coord_count: HashMap<Point, i32> = HashMap::new();

    for line in input.lines() {
        let coords: Vec<Point> = line.split(" -> ").map(|s| s.split(",").map(|n| n.parse().unwrap()).collect::<Point>()).collect();

        let c0 = &coords[0];
        let c1 = &coords[1];

        if c0.0 == c1.0 { 
            for i in cmp::min(c0.1, c1.1)..cmp::max(c0.1, c1.1) + 1 {
                let count = coord_count.entry(Point(c0.0, i)).or_insert(0);
                *count += 1;
            }
        } 
        else if c0.1 == c1.1 { 
            for i in cmp::min(c0.0, c1.0)..cmp::max(c0.0, c1.0) + 1 {
                let count = coord_count.entry(Point(i, c0.1)).or_insert(0);
                *count += 1;
            }
        } 
        else {
            let x_step = if c0.0 < c1.0 { 1 } else { -1 };
            let y_step = if c0.1 < c1.1 { 1 } else { -1 };

            for i in 0..=(c1.0 - c0.0) * x_step {
                let count = coord_count.entry(Point(c0.0 + i * x_step, c0.1 + i * y_step)).or_insert(0);
                *count += 1;
            }
        }
    }

    println!("Result: {:?}", coord_count.iter().filter(|(_p, count)| **count > 1).count());

    return Ok(());
}

