use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use anyhow::Result;

const FILE_NAME: &str = "inputs/5-input.txt";

struct Field<const N: usize>([[u8; N]; N]);

impl<const N: usize> Field<N> {
    fn new() -> Self {
        Self([[0; N]; N])
    }

    fn draw(&mut self, pos1: (usize, usize), pos2: (usize, usize)) {
        if pos1.0 != pos2.0 && pos1.1 != pos2.1 {
            return;
        }

        if pos1.0 == pos2.0 {
            let min = usize::min(pos1.1, pos2.1);
            let max = usize::max(pos1.1, pos2.1);
            for i in min..=max {
                self.0[pos1.0][i] += 1;
            }
        } else if pos1.1 == pos2.1 {
            let min = usize::min(pos1.0, pos2.0);
            let max = usize::max(pos1.0, pos2.0);
            for i in min..=max {
                self.0[i][pos1.1] += 1;
            }
        }
    }

    fn danger_points(&self) -> usize {
        let mut sum: usize = 0;
        for i in 0..N {
            for j in 0..N {
                if self.0[i][j] >= 2 {
                    sum += 1;
                }
            }
        }
        sum
    }
}

fn main() -> Result<()> {
    let mut lines = BufReader::new(File::open(FILE_NAME)?).lines();
    let mut field = Field::<1024>::new();
    while let Some(Ok(line)) = lines.next() {
        // dbg!(&line);
        let (first, second) = line.split_once(" -> ").unwrap();

        let (val1, val2) = first.split_once(',').unwrap();
        let (val3, val4) = second.split_once(',').unwrap();
        let val1 = val1.parse().unwrap();
        let val2 = val2.parse().unwrap();
        let val3 = val3.parse().unwrap();
        let val4 = val4.parse().unwrap();

        field.draw((val1, val2), (val3, val4))
    }

    dbg!(field.danger_points());

    Ok(())
}
