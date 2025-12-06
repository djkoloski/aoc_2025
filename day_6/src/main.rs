use std::{io, str::FromStr};

use common::{Context as _, Input, bail, solve};

enum Op {
    Add,
    Multiply,
}

impl FromStr for Op {
    type Err = common::Error;

    fn from_str(s: &str) -> common::Result<Self> {
        Ok(match s {
            "+" => Self::Add,
            "*" => Self::Multiply,
            _ => bail!("invalid operation '{s}'"),
        })
    }
}

struct Problems {
    operands: Vec<Vec<String>>,
    operators: Vec<Op>,
}

impl Input for Problems {
    fn parse_reader<R: io::BufRead>(reader: R) -> common::Result<Self> {
        let mut lines = reader.lines().collect::<Result<Vec<_>, _>>()?;
        let ops_line = lines.pop().context("expected operators line")?;

        let mut operands = vec![Vec::new(); lines.len()];
        let mut operators = Vec::new();

        let mut pos = 0;
        while pos < ops_line.len() {
            let next_pos = ops_line[pos + 1..]
                .find(['+', '*'])
                .map(|p| p + pos + 1)
                .unwrap_or(ops_line.len() + 1);
            for (i, line) in lines.iter().enumerate() {
                operands[i].push(line[pos..next_pos - 1].to_string());
            }
            operators.push(ops_line[pos..=pos].parse()?);
            pos = next_pos;
        }

        Ok(Self {
            operands,
            operators,
        })
    }
}

fn main() -> common::Result<()> {
    solve(
        |input: &Problems| {
            let mut total = 0;
            for (i, op) in input.operators.iter().enumerate() {
                let operands = input
                    .operands
                    .iter()
                    .map(|v| v[i].trim().parse::<usize>().unwrap());
                let result = match op {
                    Op::Add => operands.sum::<usize>(),
                    Op::Multiply => operands.product(),
                };
                total += result;
            }
            total
        },
        |input| {
            let mut total = 0;
            for i in 0..input.operators.len() {
                let mut result = match input.operators[i] {
                    Op::Add => 0,
                    Op::Multiply => 1,
                };
                for j in 0..input.operands[0][i].len() {
                    let mut operand = 0;
                    for k in 0..input.operands.len() {
                        let c = input.operands[k][i][j..=j].chars().next().unwrap();
                        if c != ' ' {
                            operand = operand * 10 + (c as u8 - b'0') as usize;
                        }
                    }

                    println!("operand: {operand}");

                    match input.operators[i] {
                        Op::Add => result += operand,
                        Op::Multiply => result *= operand,
                    }
                }
                total += result;
            }
            total
        },
    )
}
