use std::{collections::BinaryHeap, time::Instant};

use hashbrown::HashMap;

fn main() {
    // read file
    let input = std::fs::read_to_string("src/bin/day17_input.txt").unwrap();

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

fn dijkstra(grid: &[&[u8]], min_step: isize, max_step: isize) -> i64 {
    // adjacency list of (row, col, direction) -> cost
    let mut adjacency: HashMap<(usize, usize, (isize, isize)), i64> = HashMap::new();
    // priority queue of (cost, (row, col, direction)), sorted by negative cost as BinaryHeap is a max heap
    let mut queue = BinaryHeap::from_iter([(0, (0, 0, (0, 0)))]);
    while let Some((cost, (row, col, d))) = queue.pop() {
        // if we have reached the end, return the negative cost
        if (row, col) == (grid.len() - 1, grid[0].len() - 1) {
            return -cost;
        }
        // if we have already visited this node with a lower cost, skip it
        if adjacency.get(&(row, col, d)).is_some_and(|&c| c < -cost) {
            continue;
        }
        // visit all neighbors
        for (dy, dx) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
            // skip if direction is opposite of current direction or the same as current direction
            if d == (dy, dx) || d == (-dy, -dx) {
                continue;
            }
            // calculate new cost
            let mut new_cost = -cost;
            for dist in 1..(max_step + 1) {
                let (new_row, new_col) = (
                    (row as isize + dy * dist) as usize,
                    (col as isize + dx * dist) as usize,
                );
                // check bounds
                if new_row >= grid.len() || new_col >= grid[0].len() {
                    continue;
                }
                new_cost += (grid[new_row][new_col] - b'0') as i64;
                if dist < min_step {
                    continue;
                }
                let key = (new_row, new_col, (dy, dx));
                if new_cost < *adjacency.get(&key).unwrap_or(&i64::MAX) {
                    adjacency.insert(key, new_cost);
                    queue.push((-new_cost, key));
                }
            }
        }
    }
    panic!("No path found");
}

fn part1(input: &str) -> usize {
    let grid = input.lines().map(|l| l.as_bytes()).collect::<Vec<&[u8]>>();
    dijkstra(&grid, 1, 3) as usize
}

fn part2(input: &str) -> usize {
    let grid = input.lines().map(|l| l.as_bytes()).collect::<Vec<&[u8]>>();
    dijkstra(&grid, 4, 10) as usize
}

#[cfg(test)]
mod day17 {
    use super::*;
    #[test]
    fn test_part_1() {
        let test_input = std::fs::read_to_string("src/bin/day17_test.txt").unwrap();
        let result = part1(&test_input);
        assert_eq!(result, 102);
    }
    #[test]
    fn test_part_2() {
        let test_input = std::fs::read_to_string("src/bin/day17_test.txt").unwrap();
        let result = part2(&test_input);
        assert_eq!(result, 94);
    }
}
