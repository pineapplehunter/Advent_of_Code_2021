use std::fs::read_to_string;

use anyhow::Result;

const FILE_NAME: &str = "inputs/7-input.txt";

fn main() -> Result<()> {
    let contents = read_to_string(FILE_NAME)?;
    let crabs: Vec<i32> = contents
        .trim()
        .split(',')
        .map(|s| s.parse().unwrap())
        .collect();

    let mut min_fuel = i32::MAX;
    let mut min_endpoint = 0;

    let minx = *crabs.iter().min().unwrap();
    let maxx = *crabs.iter().max().unwrap();

    for end_point in minx..=maxx {
        let fuel: i32 = crabs
            .iter()
            .map(|c| {
                let dist = (c - end_point).abs();
                (dist * (dist + 1)) / 2
            })
            .sum();
        if fuel < min_fuel {
            min_fuel = fuel;
            min_endpoint = end_point;
        }
    }

    dbg!(&min_endpoint);
    dbg!(&min_fuel);

    Ok(())
}
