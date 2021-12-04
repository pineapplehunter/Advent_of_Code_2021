use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use anyhow::Result;

const FILE_NAME: &str = "inputs/1-input.txt";

fn main() -> Result<()> {
    let mut last = None;
    let mut increase = 0;
    let mut lines = BufReader::new(File::open(FILE_NAME)?).lines();
    while let Some(Ok(line)) = lines.next() {
        dbg!(&line);
        let num: u32 = line.parse()?;
        if let Some(last) = last {
            if num > last {
                increase += 1;
            }
        }
        last = Some(num);
    }
    dbg!(increase);

    Ok(())
}
