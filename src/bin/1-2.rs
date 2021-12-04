use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use anyhow::Result;

const FILE_NAME: &str = "inputs/1-input.txt";

fn main() -> Result<()> {
    let mut nums = Vec::new();

    let mut lines = BufReader::new(File::open(FILE_NAME)?).lines();
    while let Some(Ok(line)) = lines.next() {
        let num = line.parse::<u32>()?;
        nums.push(num);
    }

    let mut last = None;
    let mut increase = 0;

    for i in 0..nums.len() - 2 {
        let sum = nums[i] + nums[i + 1] + nums[i + 2];
        if let Some(last) = last {
            if sum > last {
                increase += 1;
            }
        }
        last = Some(sum);
    }

    dbg!(increase);

    Ok(())
}
