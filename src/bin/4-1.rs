use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use anyhow::Result;

const FILE_NAME: &str = "inputs/4-input.txt";

#[derive(Debug)]
struct Board([[Option<u32>; 5]; 5]);

impl Board {
    fn new() -> Self {
        Self([[None; 5]; 5])
    }

    fn check_bingo(&self) -> bool {
        for i in 0..5 {
            if (0..5).all(|n| self.0[n][i] == None) {
                return true;
            }
            if (0..5).all(|n| self.0[i][n] == None) {
                return true;
            }
        }
        false
    }

    fn poke(&mut self, value: u32) -> bool {
        for i in 0..5 {
            for j in 0..5 {
                if let Some(v) = self.0[i][j] {
                    if v == value {
                        self.0[i][j] = None;
                        return true;
                    }
                }
            }
        }
        false
    }

    fn sum(&self) -> u32 {
        let mut sum = 0;
        for i in 0..5 {
            for j in 0..5 {
                if let Some(value) = self.0[i][j] {
                    sum += value;
                }
            }
        }
        sum
    }
}

fn main() -> Result<()> {
    let mut first_line = String::new();
    let mut reader = BufReader::new(File::open(FILE_NAME)?);
    reader.read_line(&mut first_line)?;

    let numbers: Vec<u32> = first_line
        .trim()
        .split(',')
        .map(|s| s.parse().unwrap())
        .collect();

    let mut boards = Vec::new();

    while let Ok(len) = reader.read_line(&mut String::new()) {
        if len == 0 {
            break;
        }
        let mut board = Board::new();
        for i in 0..5 {
            let mut line = String::new();
            reader.read_line(&mut line)?;
            dbg!(&line);
            let board_row_num = line
                .trim()
                .split_ascii_whitespace()
                .map(|s| s.parse().unwrap());
            for (j, num) in board_row_num.into_iter().enumerate() {
                board.0[i][j] = Some(num)
            }
        }
        boards.push(board);
    }

    'outer: for num in numbers {
        for board in boards.iter_mut() {
            if board.poke(num) && board.check_bingo() {
                dbg!("bingo!");
                dbg!(num);
                let sum = board.sum();
                dbg!(sum * num);
                break 'outer;
            }
        }
    }

    Ok(())
}
