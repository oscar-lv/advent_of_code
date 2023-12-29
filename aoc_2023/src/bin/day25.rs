use itertools::Itertools;
use std::collections::{HashMap, HashSet};

fn part1(input: &str) -> usize {
    let mut graph = HashMap::<_, HashSet<_>>::new();
    let mut edges = HashSet::new();
    for l in input.split('\n') {
        let (a, rest) = l.split_once(": ").unwrap();
        for b in rest.split(' ') {
            graph.entry(a).or_default().insert(b);
            graph.entry(b).or_default().insert(a);
            edges.insert(if a < b { (a, b) } else { (b, a) });
        }
    }

    let mut dot = String::from("graph {\n");
    for (a, b) in edges.iter().sorted() {
        dot += &format!("  {} -- {};\n", a, b);
    }
    dot += "}";
    // Run the following to visualize the graph:
    //   dot -Tsvg -Kneato out.dot > out.svg
    // Manually find the three edges.
    std::fs::write("out.dot", dot).unwrap();
    for (a, b) in [("zjm", "zcp"), ("rfg", "jks"), ("nsk", "rsg")] {
        graph.get_mut(a).unwrap().remove(b);
        graph.get_mut(b).unwrap().remove(a);
    }
    let size = component_size(&graph, "jks");
    println!("{}", (graph.len() - size) * size);
    (graph.len() - size) * size
}

fn component_size(graph: &HashMap<&str, HashSet<&str>>, a: &str) -> usize {
    let (mut seen, mut s) = (HashSet::new(), vec![a]);
    while let Some(x) = s.pop() {
        if seen.insert(x) {
            s.extend(&graph[x]);
        }
    }
    seen.len()
}

fn main() {
    let input = std::fs::read_to_string("src/bin/day25_input.txt").unwrap();
    part1(&input);
}
