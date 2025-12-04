use common::{Grid, bail, solve};

#[derive(Clone, PartialEq)]
enum Tile {
    None,
    Paper,
}

impl TryFrom<char> for Tile {
    type Error = common::Error;

    fn try_from(value: char) -> common::Result<Self> {
        Ok(match value {
            '.' => Self::None,
            '@' => Self::Paper,
            _ => bail!("invalid tile '{value}'"),
        })
    }
}

fn is_accessible(grid: &Grid<Tile>, x: usize, y: usize) -> bool {
    *grid.get(x, y).unwrap() == Tile::Paper
        && grid
            .adjacent(x, y)
            .filter(|&(nx, ny)| *grid.get(nx, ny).unwrap() == Tile::Paper)
            .count()
            <= 4
}

fn main() -> common::Result<()> {
    solve(
        |input: &Grid<Tile>| {
            input
                .iter()
                .filter(|&(x, y)| is_accessible(input, x, y))
                .count()
        },
        |input| {
            let mut grid = input.clone();

            let mut removed = 0;
            let mut is_finished = false;
            while !is_finished {
                is_finished = true;

                for (x, y) in grid.iter() {
                    if is_accessible(&grid, x, y) {
                        grid.set(x, y, Tile::None);
                        removed += 1;
                        is_finished = false;
                    }
                }
            }

            removed
        },
    )
}
