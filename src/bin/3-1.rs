use anyhow::Result;
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

const FILE_NAME: &str = "inputs/3-input.txt";

fn main() -> Result<()> {
    let mut count0 = [0; 12];
    let mut count1 = [0; 12];

    let mut lines = BufReader::new(File::open(FILE_NAME)?).lines();
    while let Some(Ok(line)) = lines.next() {
        for (index, c) in line.chars().enumerate() {
            match c {
                '0' => count0[index] += 1,
                '1' => count1[index] += 1,
                _ => unreachable!(),
            }
        }
    }

    let mut gamma: u32 = 0;

    for (z, o) in count0.iter().zip(count1.iter()) {
        gamma <<= 1;
        if o > z {
            gamma |= 1
        }
    }

    dbg!(&gamma);

    let epsilon = !gamma & 0xFFF;

    dbg!(&epsilon);

    dbg!(gamma * epsilon);

    Ok(())
}
