use std::fs::read_to_string;

use anyhow::Result;

const FILE_NAME: &str = "inputs/6-input.txt";

struct LanternFishEnv {
    zero_index: usize,
    numbers: [usize; 9],
}

impl LanternFishEnv {
    fn new() -> Self {
        Self {
            zero_index: 0,
            numbers: [0; 9],
        }
    }

    fn add(&mut self, v: usize) {
        self.numbers[(self.zero_index + v) % 9] += 1;
    }

    fn tick(&mut self) {
        let reps = self.numbers[self.zero_index];
        self.numbers[(self.zero_index + 7) % 9] += reps;
        self.zero_index += 1;
        self.zero_index %= 9;
    }

    fn count(&self) -> usize {
        self.numbers.iter().sum()
    }
}

fn main() -> Result<()> {
    let contents = read_to_string(FILE_NAME)?;
    let fishes = contents
        .trim()
        .split(',')
        .map(|s| s.parse::<usize>().unwrap());

    let mut env = LanternFishEnv::new();

    for fish in fishes {
        env.add(fish);
    }

    for _ in 0..256 {
        env.tick();
        dbg!(env.count());
    }

    Ok(())
}
