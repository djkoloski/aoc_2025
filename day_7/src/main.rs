use std::cmp;

use common::{Context, Input, solve};

struct Manifold {
    start_pos: usize,
    rows: Vec<Vec<usize>>,
}

impl Input for Manifold {
    fn parse_reader<R: std::io::BufRead>(reader: R) -> common::Result<Self> {
        let mut lines = reader.lines();
        let start_pos = lines
            .next()
            .context("missing start line")??
            .chars()
            .position(|c| c == 'S')
            .context("missing start position")?;

        // burn an empty line
        lines.next().context("unexpected end of input")??;

        let mut rows = Vec::new();
        while let Some(line) = lines.next() {
            rows.push(
                line?
                    .chars()
                    .enumerate()
                    .filter_map(|(i, c)| (c == '^').then_some(i))
                    .collect(),
            );

            // burn an empty line
            lines.next().context("unexpected end of input")??;
        }

        Ok(Self { start_pos, rows })
    }
}

fn main() -> common::Result<()> {
    solve(
        |input: &Manifold| {
            let mut beams = vec![input.start_pos];
            let mut splits = 0;
            for row in &input.rows {
                let mut next_beams = Vec::new();

                let mut current_beam = 0;
                let mut current_split = 0;
                while current_beam < beams.len() && current_split < row.len() {
                    match beams[current_beam].cmp(&row[current_split]) {
                        cmp::Ordering::Less => {
                            next_beams.push(beams[current_beam]);
                            current_beam += 1;
                        }
                        cmp::Ordering::Equal => {
                            next_beams.push(beams[current_beam] - 1);
                            next_beams.push(beams[current_beam] + 1);
                            current_beam += 1;
                            current_split += 1;
                            splits += 1;
                        }
                        cmp::Ordering::Greater => {
                            current_split += 1;
                        }
                    }
                }
                // add remaining beams
                for beam in &beams[current_beam..] {
                    next_beams.push(*beam);
                }

                next_beams.dedup();
                beams = next_beams;
            }
            splits
        },
        |input| {
            let mut beams = vec![(input.start_pos, 1)];
            for row in &input.rows {
                let mut next_beams = Vec::new();

                let mut current_beam = 0;
                let mut current_split = 0;
                while current_beam < beams.len() && current_split < row.len() {
                    let beam = &beams[current_beam];
                    match beam.0.cmp(&row[current_split]) {
                        cmp::Ordering::Less => {
                            next_beams.push(*beam);
                            current_beam += 1;
                        }
                        cmp::Ordering::Equal => {
                            next_beams.push((beam.0 - 1, beam.1));
                            next_beams.push((beam.0 + 1, beam.1));
                            current_beam += 1;
                            current_split += 1;
                        }
                        cmp::Ordering::Greater => {
                            current_split += 1;
                        }
                    }
                }
                // add remaining beams
                for beam in &beams[current_beam..] {
                    next_beams.push(*beam);
                }

                let mut deduped = Vec::<(usize, usize)>::new();
                for beam in next_beams {
                    if let Some(last) = deduped.last_mut()
                        && last.0 == beam.0
                    {
                        last.1 += beam.1;
                    } else {
                        deduped.push(beam);
                    }
                }

                beams = deduped;
            }
            beams.iter().map(|(_, n)| n).sum::<usize>()
        },
    )
}
