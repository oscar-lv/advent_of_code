use std::collections::{HashMap, HashSet, VecDeque};

use itertools::Itertools;
use std::time::Instant;

fn main() {
    // read file
    let input = std::fs::read_to_string("src/bin/day22_input.txt").unwrap();

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

fn part2(input: &str) -> usize {
    let bricks = setup(input);
    bricks.keys().map(|&id| disintegrate(id, &bricks)).sum()
}
fn part1(input: &str) -> usize {
    let bricks = setup(input);
    bricks
        .values()
        .filter(|br| br.above.is_empty() || br.above.iter().all(|b| bricks[b].below.len() > 1))
        .count()
}

fn disintegrate(start: usize, bricks: &HashMap<usize, Brick>) -> usize {
    let mut seen = HashSet::new();
    let mut queue = VecDeque::from([start]);
    while let Some(id) = queue.pop_front() {
        if seen.contains(&id) {
            continue;
        }
        seen.insert(id);
        queue.extend(bricks[&id].above.iter().filter_map(|a| {
            if bricks[a].below.is_subset(&seen) {
                Some(*a)
            } else {
                None
            }
        }))
    }
    seen.len().saturating_sub(1)
}

fn setup(input: &str) -> HashMap<usize, Brick> {
    let mut grid = HashMap::new();
    let mut bricks = HashMap::new();

    for brick in parse(input).sorted_by_key(|b| b.zs) {
        brick.settle(&mut grid, &mut bricks)
    }
    bricks
}

fn parse(input: &str) -> impl Iterator<Item = Brick> + '_ {
    input
        .lines()
        .enumerate()
        .map(|(id, line)| Brick::new(id, line))
}

// Define a structure representing a brick in a 3D space.
struct Brick {
    id: usize,             // Unique identifier for the brick
    above: HashSet<usize>, // Set of bricks that are directly above this brick
    below: HashSet<usize>, // Set of bricks that are directly below this brick
    xs: (usize, usize),    // X-coordinate range (start, end)
    ys: (usize, usize),    // Y-coordinate range (start, end)
    zs: (usize, usize),    // Z-coordinate range (start, end)
}

// Implementation block for the Brick structure
impl Brick {
    // Constructor function for creating a new Brick instance
    // 'id' is the unique identifier of the brick
    // 'line' is a string containing the coordinate range information
    fn new(id: usize, line: &str) -> Self {
        // Split the line into two parts at the "~" character
        let (left, right) = line.split_once("~").unwrap();
        // Split each part into coordinates and collect them into vectors
        let (left, right) = (
            left.split(',').collect::<Vec<_>>(),
            right.split(',').collect::<Vec<_>>(),
        );
        // Create a new Brick instance with the parsed coordinate ranges
        Self {
            id,
            above: HashSet::new(),
            below: HashSet::new(),
            xs: (
                left[0].parse::<usize>().unwrap(),
                right[0].parse::<usize>().unwrap(),
            ),
            ys: (
                left[1].parse::<usize>().unwrap(),
                right[1].parse::<usize>().unwrap(),
            ),
            zs: (
                left[2].parse::<usize>().unwrap(),
                right[2].parse::<usize>().unwrap(),
            ),
        }
    }

    // Function to settle the brick into its position in a 3D grid
    // 'grid' is a mutable reference to a hash map representing the 3D grid
    // 'bricks' is a mutable reference to a hash map of all bricks
    fn settle(
        mut self,
        grid: &mut HashMap<usize, Vec<(usize, usize, usize)>>,
        bricks: &mut HashMap<usize, Brick>,
    ) {
        // Start from one layer below the bottom layer of the brick
        let mut z = self.zs.0 - 1;
        let mut below = HashSet::new();

        // Iterate downwards through the grid
        while z > 0 {
            if let Some(plain) = grid.get(&z) {
                // Find bricks in the current layer that are directly below the current brick
                below = plain
                    .iter()
                    .filter_map(|p| {
                        if (self.xs.0..=self.xs.1).contains(&p.0)
                            && (self.ys.0..=self.ys.1).contains(&p.1)
                        {
                            Some(p.2)
                        } else {
                            None
                        }
                    })
                    .collect();
                // Stop if any bricks are found below
                if !below.is_empty() {
                    break;
                }
            }
            z -= 1;
        }

        // Calculate the new Z-coordinate range for the brick
        let z1 = z + 1;
        let z2 = self.zs.1 - (self.zs.0 - z1);

        // Create a vector of points occupied by this brick
        let points = (self.xs.0..=self.xs.1)
            .cartesian_product(self.ys.0..=self.ys.1)
            .map(|(x, y)| (x, y, self.id))
            .collect::<Vec<_>>();

        // Add the brick's points to the grid
        for z in z1..=z2 {
            grid.entry(z)
                .and_modify(|v| v.extend(points.clone()))
                .or_insert(points.clone());
        }

        // Update the 'above' set of bricks that are below this brick
        for id in &below {
            if let Some(brick) = bricks.get_mut(id) {
                brick.above.insert(self.id);
            }
        }

        // Update the Z-coordinate range and below set of the brick
        self.zs = (z1, z2);
        self.below = below;
        // Insert the updated brick into the bricks hash map
        bricks.insert(self.id, self);
    }
}

#[cfg(test)]
mod day22 {
    use super::*;
    #[test]
    fn test_part_1() {
        let test_input = std::fs::read_to_string("src/bin/day22_test.txt").unwrap();
        let result = part1(&test_input);
        assert_eq!(result, 5);
    }
    #[test]
    fn test_part_2() {
        let test_input = std::fs::read_to_string("src/bin/day22_test.txt").unwrap();
        let result = part2(&test_input);
        assert_eq!(result, 16);
    }
}
