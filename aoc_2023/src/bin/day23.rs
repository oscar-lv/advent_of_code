use std::collections::{HashMap, HashSet, VecDeque};

use itertools::Itertools;
use std::time::Instant;

const NEIGHBOURS: &[(isize, isize)] = &[(-1, 0), (0, 1), (1, 0), (0, -1)];

fn main() {
    // read file
    let input = std::fs::read_to_string("src/bin/day23_input.txt").unwrap();

    // start timer
    let start = Instant::now();

    // count and print
    println!("Part 1: {}", part1(&input));

    // print time taken by part1
    println!("Time taken by Part 1: {:?}", start.elapsed());

    // reset timer
    let start = Instant::now();

    println!("Part 2: {}", part2(&input));

    // print time taken by part2
    println!("Time taken by Part 2: {:?}", start.elapsed());
}

fn dfs(
    graph: &HashMap<(usize, usize), Vec<(usize, usize, usize)>>,
    visited: &mut Vec<Vec<bool>>,
    (r, c): (usize, usize),
) -> Option<usize> {
    if r == visited.len() - 1 {
        return Some(0);
    };
    let mut max_dist = None;
    for &(nr, nc, dist) in &graph[&(r, c)] {
        if !visited[nr][nc] {
            visited[nr][nc] = true;
            if let Some(d) = dfs(graph, visited, (nr, nc)) {
                max_dist = Some(max_dist.unwrap_or(0).max(d + dist));
            }
            visited[nr][nc] = false;
        }
    }
    max_dist
}

fn longest_path(input: &str, corridors: bool) -> usize {
    let grid = input.split('\n').map(str::as_bytes).collect::<Vec<_>>();
    let mut graph: HashMap<_, Vec<_>> = HashMap::<_, Vec<_>>::new();
    for (r, c) in (0..grid.len()).cartesian_product(0..grid[0].len()) {
        let neighbours = match grid[r][c] {
            b'#' => continue,
            _ if corridors => NEIGHBOURS,
            b'.' => NEIGHBOURS,
            b'^' => &NEIGHBOURS[0..][..1],
            b'>' => &NEIGHBOURS[1..][..1],
            b'v' => &NEIGHBOURS[2..][..1],
            b'<' => &NEIGHBOURS[3..][..1],
            _ => unreachable!(),
        };
        let e = graph.entry((r, c)).or_default();
        for (dr, dc) in neighbours {
            let (nr, nc) = ((r as isize + dr) as usize, (c as isize + dc) as usize);
            if grid
                .get(nr)
                .and_then(|row| row.get(nc))
                .is_some_and(|&t| t != b'#')
            {
                e.push((nr, nc, 1));
            }
        }
    }
    let corridors = graph
        .iter()
        .filter(|(_, n)| n.len() == 2)
        .map(|(&node, _)| node)
        .collect::<Vec<_>>();
    for (r, c) in corridors {
        let neighbours = graph.remove(&(r, c)).unwrap();
        let (nr1, nc1, d1) = neighbours[0];
        let (nr2, nc2, d2) = neighbours[1];
        let n1 = graph.get_mut(&(nr1, nc1)).unwrap();
        if let Some(i) = n1.iter().position(|&(rr, cc, _)| (rr, cc) == (r, c)) {
            n1[i] = (nr2, nc2, d1 + d2);
        }
        let n2 = graph.get_mut(&(nr2, nc2)).unwrap();
        if let Some(i) = n2.iter().position(|&(rr, cc, _)| (rr, cc) == (r, c)) {
            n2[i] = (nr1, nc1, d1 + d2);
        }
    }
    dfs(
        &graph,
        &mut vec![vec![false; grid[0].len()]; grid.len()],
        (0, 1),
    )
    .unwrap()
}

fn part1(input: &str) -> usize {
    longest_path(input, false)
}
fn part2(input: &str) -> usize {
    longest_path(input, true)
}

#[cfg(test)]
mod day23 {
    use super::*;
    #[test]
    fn test_part_1() {
        let test_input = std::fs::read_to_string("src/bin/day23_test.txt").unwrap();
        let result = part1(&test_input);
        assert_eq!(result, 94);
    }
    #[test]
    fn test_part_2() {
        let test_input = std::fs::read_to_string("src/bin/day23_test.txt").unwrap();
        let result = part2(&test_input);
        assert_eq!(result, 164);
    }
}
