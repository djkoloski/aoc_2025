use core::{iter, str::FromStr};

use common::{Context as _, Lines, solve};

struct Point {
    x: usize,
    y: usize,
}

impl FromStr for Point {
    type Err = common::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (x, y) = s.split_once(',').context("invalid coordinates")?;
        Ok(Self {
            x: x.parse().context("failed to parse X coordinate")?,
            y: y.parse().context("failed to parse Y coordinate")?,
        })
    }
}

fn main() -> common::Result<()> {
    solve(
        |input: &Lines<Point>| {
            let mut max = 0;
            for (i, a) in input.lines.iter().enumerate() {
                for b in &input.lines[i + 1..] {
                    let area = ((b.x as isize - a.x as isize).abs() + 1)
                        * ((b.y as isize - a.y as isize).abs() + 1);
                    max = isize::max(max, area);
                }
            }
            max
        },
        |input| {
            let mut x_coords = input.lines.iter().map(|p| p.x).collect::<Vec<_>>();
            x_coords.sort();
            x_coords.dedup();

            let mut y_coords = input.lines.iter().map(|p| p.y).collect::<Vec<_>>();
            y_coords.sort();
            y_coords.dedup();

            let small_points = input
                .lines
                .iter()
                .map(|p| Point {
                    x: x_coords.binary_search(&p.x).unwrap() * 2,
                    y: y_coords.binary_search(&p.y).unwrap() * 2,
                })
                .collect::<Vec<_>>();

            let width = x_coords.len() * 2;
            let height = y_coords.len() * 2;

            #[derive(Clone, Copy, PartialEq)]
            enum Tile {
                Unknown,
                Outside,
                Inside,
            }

            let mut bitmap = vec![Tile::Unknown; width * height];
            for (a, b) in small_points.iter().zip(
                small_points
                    .iter()
                    .skip(1)
                    .chain(iter::once(&small_points[0])),
            ) {
                let lx = usize::min(a.x, b.x);
                let ux = usize::max(a.x, b.x);
                for x in lx..=ux {
                    bitmap[x + a.y * width] = Tile::Inside;
                }
                let ly = usize::min(a.y, b.y);
                let uy = usize::max(a.y, b.y);
                for y in ly..=uy {
                    bitmap[a.x + y * width] = Tile::Inside;
                }
            }

            let mut frontier = Vec::new();
            for x in 0..width {
                frontier.push((x, 0));
                frontier.push((x, height - 1));
            }
            for y in 0..height {
                frontier.push((0, y));
                frontier.push((width - 1, y));
            }

            while let Some((x, y)) = frontier.pop() {
                if bitmap[x + y * width] == Tile::Unknown {
                    bitmap[x + y * width] = Tile::Outside;
                    if x > 0 {
                        frontier.push((x - 1, y));
                    }
                    if x < width - 1 {
                        frontier.push((x + 1, y));
                    }
                    if y > 0 {
                        frontier.push((x, y - 1));
                    }
                    if y < width - 1 {
                        frontier.push((x, y + 1));
                    }
                }
            }

            for tile in bitmap.iter_mut() {
                if *tile == Tile::Unknown {
                    *tile = Tile::Inside;
                }
            }

            let mut max_area = 0;
            for (i, a) in small_points.iter().enumerate() {
                'next_point: for b in &small_points[i + 1..] {
                    let lx = usize::min(a.x, b.x);
                    let ux = usize::max(a.x, b.x);
                    for x in lx..=ux {
                        if bitmap[x + a.y * width] == Tile::Outside {
                            continue 'next_point;
                        }
                        if bitmap[x + b.y * width] == Tile::Outside {
                            continue 'next_point;
                        }
                    }
                    let ly = usize::min(a.y, b.y);
                    let uy = usize::max(a.y, b.y);
                    for y in ly..=uy {
                        if bitmap[a.x + y * width] == Tile::Outside {
                            continue 'next_point;
                        }
                        if bitmap[b.x + y * width] == Tile::Outside {
                            continue 'next_point;
                        }
                    }

                    let rlx = x_coords[lx / 2];
                    let rux = x_coords[ux / 2];
                    let rly = y_coords[ly / 2];
                    let ruy = y_coords[uy / 2];

                    let area = (rux - rlx + 1) * (ruy - rly + 1);
                    max_area = usize::max(max_area, area);
                }
            }

            max_area
        },
    )
}
