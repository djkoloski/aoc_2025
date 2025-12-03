use std::str::FromStr;

use common::{Context as _, List, solve};

struct ProductRange {
    start: usize,
    end: usize,
}

impl FromStr for ProductRange {
    type Err = common::Error;

    fn from_str(s: &str) -> common::Result<Self> {
        let (start, end) = s.split_once("-").context("invalid product id range")?;
        Ok(Self {
            start: start.parse().context("invalid start")?,
            end: end.parse().context("invalid end")?,
        })
    }
}

fn next_repetition(i: usize) -> usize {
    if i == 0 {
        return 11;
    }

    let digits = i.ilog10() + 1;
    let half_pow = 10usize.pow(digits.div_ceil(2));
    if !digits.is_multiple_of(2) {
        return (half_pow + 1) * 10usize.pow(digits / 2);
    }

    let upper = i / half_pow;
    let lower = i % half_pow;
    if lower < upper {
        (half_pow + 1) * upper
    } else if (upper + 1).ilog10() + 1 == digits.div_ceil(2) {
        (half_pow + 1) * (upper + 1)
    } else {
        (half_pow * 10 + 1) * 10usize.pow(digits / 2)
    }
}

fn is_repetition(i: usize) -> bool {
    let digits = i.ilog10() as usize + 1;
    for d in 1..=digits / 2 {
        if !digits.is_multiple_of(d) {
            continue;
        }

        let pow = 10usize.pow(d as u32);
        let mut x = i / pow;
        let lower = i % pow;
        while x != 0 {
            if x % pow != lower {
                break;
            }
            x /= pow;
        }

        if x == 0 {
            return true;
        }
    }

    false
}

fn main() -> common::Result<()> {
    solve(
        |input: &List<ProductRange>| {
            let mut total = 0;
            for range in &input.elements {
                let mut current = next_repetition(range.start - 1);
                while current <= range.end {
                    total += current;
                    current = next_repetition(current);
                }
            }
            total
        },
        |input| {
            let mut total = 0;
            for range in &input.elements {
                for i in range.start..=range.end {
                    if is_repetition(i) {
                        total += i;
                    }
                }
            }
            total
        },
    )
}
