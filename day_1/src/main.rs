use std::str::FromStr;

use common::{Context, Lines, bail, solve};

struct Rotation {
    amount: isize,
}

impl FromStr for Rotation {
    type Err = common::Error;

    fn from_str(s: &str) -> common::Result<Self> {
        let direction = match s.get(0..1).context("missing direction")? {
            "L" => -1,
            "R" => 1,
            c => bail!("invalid direction '{c}'"),
        };
        let distance = s
            .get(1..)
            .context("missing distance")?
            .parse::<isize>()
            .context("invalid distance")?;
        Ok(Self {
            amount: direction * distance,
        })
    }
}

fn main() -> common::Result<()> {
    solve(
        |input: &Lines<Rotation>| {
            let mut zeros = 0;
            let mut total = 50;
            for rotation in &input.lines {
                total = (total + rotation.amount) % 100;
                if total == 0 {
                    zeros += 1;
                }
            }
            zeros
        },
        |input| {
            let mut zeros = 0;
            let mut total = 50;
            for rotation in &input.lines {
                let next = total + rotation.amount;
                if (total > 0 && next <= 0) || (total < 0 && next >= 0) {
                    zeros += 1;
                }
                zeros += next.abs() / 100;
                total = next % 100;
            }
            zeros
        },
    )
}
