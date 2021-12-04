use anyhow::Result;
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

const FILE_NAME: &str = "inputs/2-input.txt";

fn main() -> Result<()> {
    let mut position = (0, 0);
    let mut aim = 0;

    let mut lines = BufReader::new(File::open(FILE_NAME)?).lines();
    while let Some(Ok(line)) = lines.next() {
        let (command, value) = line.split_once(" ").unwrap();
        let value = value.parse::<u32>()?;

        match command {
            "forward" => position = (position.0 + value, position.1 + value * aim),
            "down" => aim += value,
            "up" => aim -= value,
            _ => unreachable!(),
        }
    }

    dbg!(&position);
    dbg!(position.0 * position.1);

    Ok(())
}
