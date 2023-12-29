use itertools::Itertools;
use petgraph::prelude::*;
use rustworkx_core::connectivity::stoer_wagner_min_cut;
use std::collections::HashMap;
use std::collections::HashSet;
use std::time::Instant;

fn part1(input: &str) -> usize {
    let nodes = parse(input);
    let mut unique: HashSet<&str> = HashSet::new();

    // Adding all nodes to set
    nodes.iter().for_each(|(key, values)| {
        unique.insert(key);
        unique.extend(values.iter().unique()); // Add .iter() before .unique()
    });

    // Creating a graph
    let mut graph = UnGraph::<&str, u32>::default();

    let node_map: HashMap<&str, NodeIndex> = unique
        .iter()
        .map(|node| (*node, graph.add_node(&node)))
        .collect();

    for (key, values) in nodes.iter() {
        for node in values {
            graph.add_edge(node_map[key], node_map[node], 1);
        }
    }

    let min_cut_res: rustworkx_core::Result<Option<(usize, Vec<_>)>> =
        stoer_wagner_min_cut(&graph, |_| Ok(1));

    let (_, partition) = min_cut_res.unwrap().unwrap();
    (unique.len() - partition.len()) * partition.len()
}

fn parse(input: &str) -> Vec<(&str, Vec<&str>)> {
    input
        .lines()
        .map(|line| {
            let (a, rest) = line.split_once(": ").unwrap();
            let b = rest.split(' ').collect::<Vec<&str>>();
            (a, b)
        })
        .collect()
}

fn main() {
    // read file
    let input = std::fs::read_to_string("src/bin/day25_input.txt").unwrap();

    // start timer
    let start = Instant::now();

    // count and print
    println!("Part 1: {}", part1(&input));

    // print time taken by part1
    println!("Time taken by Part 1: {:?}", start.elapsed());
}

#[cfg(test)]
mod day25 {
    use super::*;
    #[test]
    fn test_part_1() {
        let test_input = std::fs::read_to_string("src/bin/day25_test.txt").unwrap();
        let result = part1(&test_input);
        assert_eq!(result, 54);
    }
}
