#![feature(int_abs_diff)]

use std::collections::HashMap;
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use anyhow::Result;

const FILE_NAME: &str = "inputs/5-input.txt";

struct Field(HashMap<(u16, u16), u8>);

impl Field {
    fn new() -> Self {
        Self(HashMap::new())
    }

    fn draw(&mut self, pos1: (u16, u16), pos2: (u16, u16)) {
        if pos1.0 == pos2.0 {
            let min = u16::min(pos1.1, pos2.1);
            let max = u16::max(pos1.1, pos2.1);
            for i in min..=max {
                *self.0.entry((pos1.0, i)).or_insert(0) += 1;
            }
        } else if pos1.1 == pos2.1 {
            let min = u16::min(pos1.0, pos2.0);
            let max = u16::max(pos1.0, pos2.0);
            for i in min..=max {
                *self.0.entry((i, pos1.1)).or_insert(0) += 1;
            }
        } else if pos1.0.abs_diff(pos2.0) == pos1.1.abs_diff(pos2.1) {
            let mini = u16::min(pos1.1, pos2.1);
            let maxi = u16::max(pos1.1, pos2.1);
            let minj = u16::min(pos1.0, pos2.0);
            let maxj = u16::max(pos1.0, pos2.0);
            if (pos1.0 < pos2.0 && pos1.1 < pos2.1) || (pos1.0 > pos2.0 && pos1.1 > pos2.1) {
                for (i, j) in (mini..=maxi).zip(minj..=maxj) {
                    *self.0.entry((j, i)).or_insert(0) += 1;
                }
            } else {
                for (i, j) in (mini..=maxi).rev().zip(minj..=maxj) {
                    *self.0.entry((j, i)).or_insert(0) += 1;
                }
            }
        } else {
            unreachable!()
        }
    }

    fn danger_points(&self) -> usize {
        let mut sum: usize = 0;
        for value in self.0.values() {
            if *value >= 2 {
                sum += 1;
            }
        }
        sum
    }
}

fn main() -> Result<()> {
    let mut lines = BufReader::new(File::open(FILE_NAME)?).lines();
    let mut field = Field::new();
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
