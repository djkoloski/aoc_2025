use core::{f64, fmt, str::FromStr};

use common::{Context, Lines, bail, solve};

struct Machine {
    lights: u32,
    buttons: Vec<u32>,
    joltages: Vec<u32>,
}

impl FromStr for Machine {
    type Err = common::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut pieces = s.split(' ');
        Ok(Self {
            lights: pieces
                .next()
                .context("missing indicator lights")?
                .strip_prefix('[')
                .context("indicators missing start bracket")?
                .strip_suffix(']')
                .context("indicators missing end bracket")?
                .chars()
                .enumerate()
                .map(|(i, c)| {
                    Ok(match c {
                        '.' => 0,
                        '#' => 1 << i,
                        _ => bail!("invalid indicator light `{c}`"),
                    })
                })
                .try_fold::<_, _, common::Result<_>>(0, |acc, x| Ok(acc | x?))?,
            joltages: pieces
                .next_back()
                .context("missing joltages")?
                .strip_prefix('{')
                .context("joltages missing start brace")?
                .strip_suffix('}')
                .context("joltages missing end brace")?
                .split(',')
                .map(|j| j.parse::<u32>().context("invalid joltage"))
                .collect::<Result<_, _>>()?,
            buttons: pieces
                .map(|p| {
                    p.strip_prefix('(')
                        .context("button missing start paren")?
                        .strip_suffix(')')
                        .context("button missing end paren")?
                        .split(',')
                        .map(|s| Ok(1 << s.parse::<u32>().context("invalid button target")?))
                        .try_fold(0, |acc, x: common::Result<_>| Ok(acc | x?))
                })
                .collect::<common::Result<_>>()?,
        })
    }
}

#[derive(Debug)]
struct Matrix {
    cols: usize,
    rows: usize,
    elements: Vec<f64>,
}

impl Matrix {
    fn new(rows: usize, cols: usize) -> Self {
        Self {
            cols,
            rows,
            elements: vec![0.0; rows * cols],
        }
    }

    fn get(&self, row: usize, col: usize) -> f64 {
        self.elements[col + row * self.cols]
    }

    fn set(&mut self, row: usize, col: usize, value: f64) {
        self.elements[col + row * self.cols] = value;
    }

    fn swap_rows(&mut self, a: usize, b: usize) {
        if a != b {
            for j in 0..self.cols {
                let tmp = self.get(a, j);
                self.set(a, j, self.get(b, j));
                self.set(b, j, tmp);
            }
        }
    }

    fn eliminate(&mut self, max_cols: usize) {
        let mut row = 0;
        let mut col = 0;

        while row < self.rows && col < max_cols {
            if let Some(swap_row) =
                (row..self.rows).find(|candidate_row| self.get(*candidate_row, col) != 0.0)
            {
                self.swap_rows(row, swap_row);

                let leader = self.get(row, col);
                for j in col..self.cols {
                    self.set(row, j, self.get(row, j) / leader);
                }

                for i in (0..self.rows).filter(|r| *r != row) {
                    let leader = self.get(i, col);
                    for j in col..self.cols {
                        self.set(i, j, self.get(i, j) - self.get(row, j) * leader);
                    }
                }

                row += 1;
            } else {
                col += 1;
            }
        }
    }
}

impl fmt::Display for Matrix {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for i in 0..self.rows {
            write!(f, "[")?;
            for j in 0..self.cols {
                write!(f, "{:>5.2} ", self.get(i, j))?;
            }
            writeln!(f, "]")?;
        }
        Ok(())
    }
}

fn main() -> common::Result<()> {
    solve(
        |input: &Lines<Machine>| {
            let mut total = 0;
            for machine in &input.lines {
                let mut min_presses = u32::MAX;
                for p in 0u32..1 << machine.buttons.len() {
                    if p.count_ones() >= min_presses {
                        continue;
                    }

                    let result = machine.buttons.iter().enumerate().fold(0, |acc, (i, b)| {
                        acc ^ if p & (1 << i) != 0 { *b } else { 0 }
                    });
                    if result == machine.lights {
                        min_presses = p.count_ones();
                    }
                }
                assert!(
                    min_presses < u32::MAX,
                    "failed to find a button combination which solves indicators"
                );
                total += min_presses;
            }
            total
        },
        |input| {
            let mut total = 0;
            for machine in &input.lines {
                let mut mat = Matrix::new(machine.joltages.len(), machine.buttons.len() + 1);
                for (i, joltage) in machine.joltages.iter().enumerate() {
                    for (j, button) in machine.buttons.iter().enumerate() {
                        let value = if button & 1 << i != 0 { 1.0 } else { 0.0 };
                        mat.set(i, j, value);
                    }
                    mat.set(i, machine.buttons.len(), *joltage as f64);
                }
                mat.eliminate(machine.buttons.len());

                #[derive(Debug)]
                struct FreeVariable {
                    index: usize,
                    base: u32,
                    range: u32,
                }

                impl FreeVariable {
                    fn value(&self, iteration: u32) -> u32 {
                        iteration / self.base % self.range
                    }
                }

                let mut base = 1;
                let mut free_variables = Vec::new();
                for j in 0..machine.buttons.len() {
                    if let Some(leader) = (0..machine.joltages.len())
                        .rev()
                        .find(|i| mat.get(*i, j) != 0.0)
                        && (0..j).any(|left| mat.get(leader, left) != 0.0)
                    {
                        let max = (0..machine.joltages.len())
                            .map(|i| {
                                if machine.buttons[j] & 1 << i != 0 {
                                    machine.joltages[i]
                                } else {
                                    0
                                }
                            })
                            .max()
                            .unwrap();

                        let range = max + 1;
                        free_variables.push(FreeVariable {
                            index: j,
                            base,
                            range,
                        });
                        base *= range;
                    }
                }

                let mut best = None;
                let mut presses = Vec::new();
                for iteration in 0..base {
                    presses.clear();
                    let mut total_presses = free_variables
                        .iter()
                        .map(|var| var.value(iteration))
                        .sum::<u32>();

                    let mut is_solution = true;
                    for i in 0..usize::min(
                        machine.joltages.len(),
                        machine.buttons.len() - free_variables.len(),
                    ) {
                        let mut p = mat.get(i, machine.buttons.len());
                        for var in free_variables.iter() {
                            p -= mat.get(i, var.index) * var.value(iteration) as f64;
                        }
                        let c = p.round();
                        if c < 0.0 || (c - p).abs() > 0.01 {
                            is_solution = false;
                            break;
                        } else {
                            presses.push(c as u32);
                            total_presses += c as u32;
                        }
                    }

                    if is_solution {
                        if best.is_none_or(|best| best > total_presses) {
                            best = Some(total_presses);
                        }

                        for var in free_variables.iter() {
                            presses.insert(var.index, var.value(iteration));
                        }
                    }
                }

                total += best.unwrap();
            }
            total
        },
    )
}
