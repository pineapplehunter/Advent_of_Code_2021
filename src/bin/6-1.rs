use std::fs::read_to_string;

use anyhow::Result;

const FILE_NAME: &str = "inputs/6-input.txt";

struct LanternFish(u8);

impl LanternFish {
    fn new(v: u8) -> Self {
        Self(v)
    }

    fn tick(&mut self) -> bool {
        if self.0 == 0 {
            self.0 = 6;
            true
        } else {
            self.0 -= 1;
            false
        }
    }
}

fn main() -> Result<()> {
    let mut fishes: Vec<LanternFish> = read_to_string(FILE_NAME)?
        .trim()
        .split(',')
        .map(|s| s.parse::<u8>().unwrap())
        .map(LanternFish::new)
        .collect();

    for _ in 0..80 {
        let mut new_fishes = 0;
        for f in fishes.iter_mut() {
            if f.tick() {
                new_fishes += 1;
            }
        }
        for _ in 0..new_fishes {
            fishes.push(LanternFish::new(8))
        }
        dbg!(fishes.len());
    }

    Ok(())
}
