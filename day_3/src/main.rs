use std::str::FromStr;

use common::{Lines, solve};

struct Bank {
    joltages: Vec<u8>,
}

impl FromStr for Bank {
    type Err = common::Error;

    fn from_str(s: &str) -> common::Result<Self> {
        Ok(Self {
            joltages: s.chars().map(|c| c as u8 - b'0').collect(),
        })
    }
}

fn joltage(bank: &Bank, n: usize) -> usize {
    let mut result = 0;
    let mut pos = 0;
    for i in 0..n {
        let remaining = n - i - 1;
        let range = &bank.joltages[pos..bank.joltages.len() - remaining];
        let max = *range.iter().max().unwrap();
        pos += range.iter().position(|j| *j == max).unwrap() + 1;
        result = result * 10 + max as usize;
    }
    result
}

fn main() -> common::Result<()> {
    solve(
        |input: &Lines<Bank>| {
            input
                .lines
                .iter()
                .map(|bank| joltage(bank, 2))
                .sum::<usize>()
        },
        |input| {
            input
                .lines
                .iter()
                .map(|bank| joltage(bank, 12))
                .sum::<usize>()
        },
    )
}
