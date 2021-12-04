use std::io;

#[derive(Debug)]
struct Board {
    pub rows: Vec<Vec<i32>>,
    pub cols: Vec<Vec<i32>>,
    pub bingo: bool
}

impl Board {
    fn new() -> Self {
        return Board {
            rows: vec![],
            cols: vec![],
            bingo: false
        }
    }

    fn read(&mut self, line: &str) {
        let numbers: Vec<i32> = line.split_ascii_whitespace().map(|s| s.parse().unwrap()).collect();
        self.rows.push(numbers.clone());

        for (i, n) in numbers.iter().enumerate() {
            if self.cols.len() < i + 1 { 
                self.cols.push(vec![]);
            }
            self.cols[i].push(*n);
        }
    }

    fn mark_number(&mut self, number: i32) {
        for r in &mut self.rows {
            let pos = r.iter().position(|x| *x == number);

            if pos.is_some() {
                r.swap_remove(pos.unwrap());

                if r.is_empty() {
                    self.bingo = true;
                }
            }
        }

        for c in &mut self.cols {
            let pos = c.iter().position(|x| *x == number);

            if pos.is_some() {
                c.swap_remove(pos.unwrap());

                if c.is_empty() {
                    self.bingo = true;
                }
            }
        }
    }

    fn get_sum_of_unmarked(&self) -> i32 {
        let mut res: i32 = 0;
        for r in &self.rows {
            res += r.iter().sum::<i32>();
        }
        
        return res;
    }
}

fn main() -> io::Result<()> {
    let mut lines = include_str!("input.dat").lines();

    let numbers: Vec<i32> = lines.next().unwrap().split(",").map(|s| s.parse().unwrap()).collect();

    let mut boards: Vec<Board> = vec![];
    for line in lines {
        if line.is_empty() {
            boards.push(Board::new());
            continue;
        }

        boards.last_mut().unwrap().read(line);
    }

    for n in numbers {
        for b in &mut boards {
            b.mark_number(n);

            if b.bingo {
                println!("Result: {:?}", b.get_sum_of_unmarked() * n);
                return Ok(());
            }
        }
    }

    return Ok(());
}

