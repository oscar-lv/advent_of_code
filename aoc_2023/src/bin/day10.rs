use std::time::Instant;

fn main() {
    // read file
    let input = std::fs::read_to_string("day10_input.txt").unwrap();

    // start timer
    let start = Instant::now();

    // count and print
    println!("Part 1: {}", part1(&input));

    // print time taken by part1
    println!("Time taken by Part 1: {:?}", start.elapsed());

    // reset timer
    let start = Instant::now();

    println!("Part 2: {}", part2());

    // print time taken by part2
    println!("Time taken by Part 2: {:?}", start.elapsed());
}

struct Grid {
    grid: Vec<Vec<char>>,
    visited: Vec<Vec<u8>>,
    start: (i32, i32),
}

impl Grid {
    fn find_start(&self) -> (i32, i32) {
        println!("{:?}", self.grid.len());
        for i in 0..self.grid.len() {
            for j in 0..self.grid[0].len() {
                if self.grid[i as usize][j as usize] == 'S' {
                    return (i as i32, j as i32);
                }
            }
        }
        panic!("No start found");
    }

    fn parse(input: &str) -> Vec<Vec<char>> {
        input
            .lines()
            .map(|line| line.chars().collect::<Vec<char>>())
            .collect()
    }

    fn build(input: &str) -> Grid {
        let grid = Self::parse(input);
        let mut g = Grid {
            grid: grid.clone(),
            visited: vec![vec![0; grid[0].len()]; grid.len()],
            start: (0, 0),
        };
        g.start = g.find_start();
        g
    }

    fn get_directions(c: char) -> Vec<(i32, i32)> {
        match c {
            'S' => vec![(1, 0), (-1, 0), (0, -1), (0, 1)],
            'L' => vec![(-1, 0), (0, 1)],
            'J' => vec![(-1, 0), (0, -1)],
            '7' => vec![(1, 0), (0, -1)],
            'F' => vec![(1, 0), (0, 1)],
            '|' => vec![(1, 0), (-1, 0)],
            '-' => vec![(0, -1), (0, 1)],
            _ => vec![],
        }
    }

    fn dfs(&mut self, y: i32, x: i32, distance: i32) -> i32 {
        if y < 0 || x < 0 || y as usize >= self.grid.len() || x as usize >= self.grid[0].len() {
            return 0;
        }
        if self.grid[y as usize][x as usize] == '.' {
            return 0;
        }
        if self.visited[y as usize][x as usize] == 1 {
            return 0;
        }
        let c = self.grid[y as usize][x as usize];
        // println!("{}", c);
        self.visited[y as usize][x as usize] = 1;

        let directions = Grid::get_directions(c);
        let mut max_distance = distance;

        for (dy, dx) in directions {
            let new_distance = self.dfs(y + dy, x + dx, distance + 1);
            if new_distance > max_distance {
                max_distance = new_distance;
            }
        }
        max_distance
    }
}

// // Process hands, establish rankings within strength groups and determine rank of each hand
fn part1(input: &str) -> i32 {
    let mut grid = Grid::build(input);
    grid.dfs(grid.start.0, grid.start.1, 0) / 2 + 1
}

fn part2() -> usize {
    let map = include_bytes!("day10_input.txt");
    let width = map.iter().position(|&b| b == b'\n').unwrap();
    let start = map.iter().position(|&b| b == b'S').unwrap();

    let mut covered = vec![false; (width + 1) * width];
    let (mut pos, mut dir) = {
        if matches!(map[start - width - 1], b'|' | b'7' | b'F') {
            (start - width - 1, 0)
        } else if matches!(map[start + width + 1], b'|' | b'L' | b'J') {
            (start + width + 1, 2)
        } else {
            (start - 1, 3)
        }
    };

    std::iter::repeat(())
        .position(|_| unsafe {
            *covered.get_unchecked_mut(pos) = true;
            match (map.get_unchecked(pos), dir) {
                (b'|', 2) => pos += width + 1,
                (b'|', 0) => pos -= width + 1,
                (b'-', 3) => pos -= 1,
                (b'-', 1) => pos += 1,
                (b'L', 2) | (b'F', 0) => {
                    pos += 1;
                    dir = 1;
                }
                (b'L', 3) | (b'J', 1) => {
                    pos -= width + 1;
                    dir = 0;
                }
                (b'7', 0) | (b'J', 2) => {
                    pos -= 1;
                    dir = 3;
                }
                (b'7', 1) | (b'F', 3) => {
                    pos += width + 1;
                    dir = 2;
                }
                (b'S', _) => return true,
                (_, _) => unreachable!(),
            }
            false
        })
        .unwrap();

    let mut inside = false;

    map.iter()
        .enumerate()
        .filter(|(pos, cell)| {
            let pipe = unsafe { *covered.get_unchecked(*pos) };
            inside &= pos % (width + 1) != 0;
            inside ^= pipe && matches!(*cell, b'|' | b'F' | b'7');
            inside && (!pipe || **cell == b'.') && (pos % (width + 1) != width)
        })
        .count()
}

#[cfg(test)]
mod day10 {
    use super::*;
    #[test]
    fn test_part_1() {
        let test_input = std::fs::read_to_string("src/bin/day10_test.txt").unwrap();
        let result = part1(&test_input);
        assert_eq!(result, 8);
    }
    #[test]
    fn test_part_2() {
        let result = part2();
        assert_eq!(result, 453);
    }
}
