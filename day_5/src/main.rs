use std::{io, str::FromStr};

use common::{Context as _, Input, solve};

#[derive(Clone)]
struct Range {
    start: usize,
    end: usize,
}

impl FromStr for Range {
    type Err = common::Error;

    fn from_str(s: &str) -> common::Result<Self> {
        let (start, end) = s.split_once('-').context("expected range to contain '-'")?;
        Ok(Self {
            start: start.parse().context("invalid start index")?,
            end: end.parse().context("invalid end index")?,
        })
    }
}

struct Database {
    fresh: Vec<Range>,
    available: Vec<usize>,
}

impl Input for Database {
    fn parse_reader<R: io::BufRead>(reader: R) -> common::Result<Self> {
        let mut lines = reader.lines();

        let mut fresh = Vec::new();
        for line in lines.by_ref() {
            let line = line.context("unexpected end of input")?;
            if line.is_empty() {
                break;
            }

            fresh.push(line.parse().context("invalid range")?);
        }

        let available = lines
            .map(|s| {
                s.context("unexpected end of input")?
                    .parse()
                    .context("invalid ingredient")
            })
            .collect::<Result<_, _>>()?;

        Ok(Self { fresh, available })
    }
}

fn main() -> common::Result<()> {
    solve(
        |input: &Database| {
            input
                .available
                .iter()
                .filter(|&&a| input.fresh.iter().any(|f| f.start <= a && a <= f.end))
                .count()
        },
        |input| {
            let mut ranges = input.fresh.clone();
            ranges.sort_by_key(|r| r.start);

            let mut total = 0;
            let mut start = 1;
            let mut end = 0;
            for range in ranges {
                if range.start <= end {
                    end = usize::max(end, range.end);
                } else {
                    total += (end + 1) - start;
                    start = range.start;
                    end = range.end;
                }
            }

            total + end - start + 1
        },
    )
}
