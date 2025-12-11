use std::{collections::HashMap, io};

use common::{Context as _, Input, solve};

struct Graph {
    nodes: HashMap<String, Vec<String>>,
}

impl Input for Graph {
    fn parse_reader<R: io::BufRead>(reader: R) -> common::Result<Self> {
        let mut nodes = HashMap::new();

        for line in reader.lines() {
            let line = line?;
            let (node, rest) = line.split_once(": ").context("expeced colon separator")?;
            nodes.insert(
                node.to_string(),
                rest.split(' ').map(str::to_string).collect(),
            );
        }

        Ok(Self { nodes })
    }
}

fn toposort(graph: &Graph) -> Vec<&String> {
    let mut incoming = HashMap::new();
    for (node, edges) in &graph.nodes {
        incoming.entry(node).or_insert(0);
        for edge in edges {
            *incoming.entry(edge).or_insert(0) += 1;
        }
    }
    let mut sorted = incoming
        .iter()
        .filter_map(|(node, incoming)| (*incoming == 0).then_some(*node))
        .collect::<Vec<_>>();
    let mut current = 0;
    while current < sorted.len() {
        if let Some(edges) = graph.nodes.get(sorted[current]) {
            for edge in edges {
                *incoming.get_mut(edge).unwrap() -= 1;
                if incoming[edge] == 0 {
                    sorted.push(edge);
                }
            }
        }

        current += 1;
    }

    sorted
}

fn main() -> common::Result<()> {
    solve(
        |input: &Graph| {
            let sorted = toposort(input);
            let mut routes_to_node = HashMap::new();
            routes_to_node.insert("you", 1);

            for &node in &sorted {
                let routes = *routes_to_node.entry(node).or_insert(0);

                if let Some(edges) = input.nodes.get(node) {
                    for edge in edges {
                        *routes_to_node.entry(edge).or_insert(0) += routes;
                    }
                }
            }

            routes_to_node["out"]
        },
        |input| {
            #[derive(Clone, Default)]
            struct Routes {
                counts_none: u64,
                counts_fft: u64,
                counts_dac: u64,
                counts_both: u64,
            }

            let sorted = toposort(input);
            let mut routes_to_node = HashMap::new();
            routes_to_node.insert(
                "svr",
                Routes {
                    counts_none: 1,
                    counts_fft: 0,
                    counts_dac: 0,
                    counts_both: 0,
                },
            );

            for &node in &sorted {
                let from_routes = routes_to_node
                    .entry(node)
                    .or_insert(Routes::default())
                    .clone();

                if let Some(edges) = input.nodes.get(node) {
                    for edge in edges {
                        let to_routes = routes_to_node.entry(edge).or_insert(Routes::default());
                        match node.as_str() {
                            "fft" => {
                                to_routes.counts_fft += from_routes.counts_none;
                                to_routes.counts_both += from_routes.counts_dac;
                            }
                            "dac" => {
                                to_routes.counts_dac += from_routes.counts_none;
                                to_routes.counts_both += from_routes.counts_fft;
                            }
                            _ => {
                                to_routes.counts_none += from_routes.counts_none;
                                to_routes.counts_fft += from_routes.counts_fft;
                                to_routes.counts_dac += from_routes.counts_dac;
                                to_routes.counts_both += from_routes.counts_both;
                            }
                        }
                    }
                }
            }

            routes_to_node["out"].counts_both
        },
    )
}
