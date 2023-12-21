use std::collections::HashSet;
use std::time::Instant;

fn main() {
    // read file
    let input = std::fs::read_to_string("src/bin/day16_input.txt").unwrap();

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
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Clone, Copy)]
struct Beam {
    position: (usize, usize),
    direction: Direction,
}
impl Beam {
    fn step(&mut self) {
        self.position = match self.direction {
            Direction::Up => (self.position.0, self.position.1.saturating_sub(1)),
            Direction::Down => (self.position.0, self.position.1 + 1),
            Direction::Left => (self.position.0.saturating_sub(1), self.position.1),
            Direction::Right => (self.position.0 + 1, self.position.1),
        };
    }
}

struct Grid {
    grid: Vec<Vec<char>>,
    beams: Vec<Beam>,
    visited: HashSet<(usize, usize, Direction)>,
}

impl Grid {
    fn build(input: &str, beams: Vec<Beam>) -> Self {
        let grid = input.lines().map(|l| l.chars().collect()).collect();
        Self {
            grid,
            beams: beams,
            visited: HashSet::new(),
        }
    }

    fn direct_beam(&mut self, mut beam: Beam) -> (Beam, Vec<Beam>) {
        let mut new_beams = Vec::<Beam>::new();
        let cell = self.grid[beam.position.1][beam.position.0];
        match (beam.direction, cell) {
            (Direction::Right, '/') => beam.direction = Direction::Up,
            (Direction::Right, '\\') => beam.direction = Direction::Down,
            (Direction::Right, '|') => {
                beam.direction = Direction::Down;
                new_beams.push(Beam {
                    position: (beam.position.0, beam.position.1),
                    direction: Direction::Up,
                });
            }
            (Direction::Left, '/') => beam.direction = Direction::Down,
            (Direction::Left, '\\') => beam.direction = Direction::Up,
            (Direction::Left, '|') => {
                beam.direction = Direction::Up;
                new_beams.push(Beam {
                    position: (beam.position.0, beam.position.1),
                    direction: Direction::Down,
                });
            }
            (Direction::Up, '/') => beam.direction = Direction::Right,
            (Direction::Up, '\\') => beam.direction = Direction::Left,
            (Direction::Up, '-') => {
                beam.direction = Direction::Left;
                new_beams.push(Beam {
                    position: (beam.position.0, beam.position.1),
                    direction: Direction::Right,
                });
            }
            (Direction::Down, '/') => beam.direction = Direction::Left,
            (Direction::Down, '\\') => beam.direction = Direction::Right,
            (Direction::Down, '-') => {
                beam.direction = Direction::Right;
                new_beams.push(Beam {
                    position: (beam.position.0, beam.position.1),
                    direction: Direction::Left,
                });
            }
            _ => {}
        }
        (beam, new_beams)
    }

    fn traverse(&mut self) {
        while let Some(mut beam) = self.beams.pop() {
            while (0..self.grid[0].len()).contains(&beam.position.0)
                && (0..self.grid.len()).contains(&beam.position.1)
                && !self.visited.contains(&(
                    beam.position.0,
                    beam.position.1,
                    beam.direction.clone(),
                ))
            {
                self.visited
                    .insert((beam.position.0, beam.position.1, beam.direction.clone()));
                let (new_beam, new_beams) = self.direct_beam(beam);
                beam = new_beam;
                self.beams.extend(new_beams);
                beam.step();
            }
        }
    }
    fn count(&self) -> usize {
        // new set from only the first two elemts in visited
        let mut visited = self
            .visited
            .iter()
            .map(|(x, y, _)| (*x, *y))
            .collect::<HashSet<(usize, usize)>>();
        // return length of visited
        visited.len()
    }
}

fn part1(input: &str) -> usize {
    let mut grid = Grid::build(
        input,
        vec![Beam {
            position: (0, 0),
            direction: Direction::Right,
        }],
    );
    grid.traverse();
    grid.count()
}

fn part2(input: &str) -> usize {
    let grid_size = input.lines().next().unwrap().len();
    let grid_height = input.lines().count();

    let mut all_start = Vec::new();
    all_start.extend(create_beams(0..grid_size, 0, Direction::Down));
    all_start.extend(create_beams(0..grid_size, grid_height - 1, Direction::Up));
    all_start.extend(create_beams(0..grid_height, 0, Direction::Right));
    all_start.extend(create_beams(0..grid_height, grid_size - 1, Direction::Left));

    all_start
        .iter()
        .map(|start| {
            let mut grid = Grid::build(input, vec![*start]);
            grid.traverse();
            grid.count()
        })
        .max()
        .unwrap()
}

fn create_beams<T>(range: T, fixed_pos: usize, direction: Direction) -> Vec<Beam>
where
    T: Iterator<Item = usize>,
{
    range
        .map(|pos| {
            let position = match direction {
                Direction::Down | Direction::Up => (pos, fixed_pos),
                Direction::Left | Direction::Right => (fixed_pos, pos),
            };
            Beam {
                position,
                direction,
            }
        })
        .collect()
}

#[cfg(test)]
mod day16 {
    use super::*;
    #[test]
    fn test_part_1() {
        let test_input = std::fs::read_to_string("src/bin/day16_test.txt").unwrap();
        let result = part1(&test_input);
        assert_eq!(result, 46);
    }
    #[test]
    fn test_part_2() {
        let test_input = std::fs::read_to_string("src/bin/day16_test.txt").unwrap();
        let result = part2(&test_input);
        assert_eq!(result, 51);
    }
}
