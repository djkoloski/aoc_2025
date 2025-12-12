use std::io;

use common::{Context as _, Input, bail, solve};

struct Present {
    cells: [bool; 9],
}

struct TreeArea {
    width: u32,
    height: u32,
    counts: Vec<u32>,
}

struct Problem {
    presents: Vec<Present>,
    tree_areas: Vec<TreeArea>,
}

impl Input for Problem {
    fn parse_reader<R: io::BufRead>(reader: R) -> common::Result<Self> {
        let mut presents = Vec::new();
        let mut tree_areas = Vec::new();

        let mut lines = reader.lines();
        while let Some(line) = lines.next() {
            let line = line?;

            if !line.contains('x') {
                // Present
                let mut present = Present { cells: [false; 9] };
                for y in 0..3 {
                    let line = lines.next().context("expected present line")??;
                    for (i, c) in line.chars().enumerate() {
                        present.cells[i + y * 3] = c == '#';
                    }
                }

                if !lines
                    .next()
                    .context("expected newline after present")??
                    .is_empty()
                {
                    bail!("unexpected content after present definition");
                }

                presents.push(present);
            } else {
                // Tree area
                let (width, rest) = line.split_once('x').context("expected width")?;
                let (height, rest) = rest.split_once(": ").context("expected height")?;
                tree_areas.push(TreeArea {
                    width: width.parse().context("invalid tree area width")?,
                    height: height.parse().context("invalid tree area height")?,
                    counts: rest.split(' ').map(str::parse).collect::<Result<_, _>>()?,
                });
            }
        }

        Ok(Self {
            presents,
            tree_areas,
        })
    }
}

fn main() -> common::Result<()> {
    solve(
        |input: &Problem| {
            let mut possible = 0;
            for tree_area in &input.tree_areas {
                let space = tree_area.width * tree_area.height;
                let mut minimum_required = 0;
                for (count, present) in tree_area.counts.iter().zip(input.presents.iter()) {
                    minimum_required +=
                        *count * present.cells.iter().filter(|x| **x).count() as u32;
                }
                if minimum_required <= space {
                    // Eh, it's probably possible
                    possible += 1;
                }
            }
            possible
        },
        |_input| 0,
    )
}
