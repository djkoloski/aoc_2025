use core::str::FromStr;
use std::collections::{HashMap, HashSet};

use common::{Context, Lines, solve};

struct Point {
    x: i64,
    y: i64,
    z: i64,
}

impl FromStr for Point {
    type Err = common::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut pieces = s.split(',');
        Ok(Self {
            x: pieces
                .next()
                .context("missing X coordinate")?
                .parse()
                .context("invalid X coordinate")?,
            y: pieces
                .next()
                .context("missing Y coordinate")?
                .parse()
                .context("invalid Y coordinate")?,
            z: pieces
                .next()
                .context("missing Z coordinate")?
                .parse()
                .context("invalid Z coordinate")?,
        })
    }
}

impl Point {
    fn sq_dist(a: &Point, b: &Point) -> i64 {
        let dx = b.x - a.x;
        let dy = b.y - a.y;
        let dz = b.z - a.z;
        dx * dx + dy * dy + dz * dz
    }
}

fn compute_sq_dists(points: &[Point]) -> Vec<(i64, usize, usize)> {
    let mut result = Vec::new();
    for (i, a) in points.iter().enumerate() {
        for (j, b) in points.iter().enumerate().skip(i + 1) {
            result.push((Point::sq_dist(a, b), i, j));
        }
    }
    result.sort();
    result
}

fn main() -> common::Result<()> {
    solve(
        |input: &Lines<Point>| {
            let connect_count = if input.lines.len() == 20 { 10 } else { 1000 };

            let sq_dists = compute_sq_dists(&input.lines);
            let mut edges = HashMap::new();
            for &(_, i, j) in sq_dists.iter().take(connect_count) {
                edges.entry(i).or_insert(Vec::new()).push(j);
                edges.entry(j).or_insert(Vec::new()).push(i);
            }
            let mut visited = HashSet::new();
            let mut component_sizes = Vec::new();
            for start in edges.keys() {
                if visited.contains(start) {
                    continue;
                }

                let mut component_size = 0;
                let mut frontier = vec![*start];
                while let Some(from) = frontier.pop() {
                    visited.insert(from);
                    component_size += 1;
                    for &to in &edges[&from] {
                        if !visited.contains(&to) {
                            visited.insert(to);
                            frontier.push(to);
                        }
                    }
                }
                component_sizes.push(component_size);
            }
            component_sizes.sort();

            component_sizes.pop().unwrap()
                * component_sizes.pop().unwrap()
                * component_sizes.pop().unwrap()
        },
        |input| {
            let mut sq_dists = compute_sq_dists(&input.lines).into_iter();
            let mut representatives = Vec::from_iter(0..input.lines.len());
            let mut connected = 0;
            loop {
                let (_, from, to) = sq_dists.next().unwrap();

                let from_rep = representatives[from];
                let to_rep = representatives[to];

                if from_rep == to_rep {
                    continue;
                }

                let old_rep = usize::max(from_rep, to_rep);
                let new_rep = usize::min(from_rep, to_rep);
                for rep in representatives.iter_mut() {
                    if *rep == old_rep {
                        *rep = new_rep;
                        if new_rep == 0 {
                            connected += 1;
                        }
                    }
                }

                if connected == representatives.len() - 1 {
                    break input.lines[from].x * input.lines[to].x;
                }
            }
        },
    )
}
