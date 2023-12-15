extern crate regex;
use std::time::Instant;

fn main() {
    // read file
    let input = std::fs::read_to_string("day11_input.txt").unwrap();

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

fn is_column_all_dots(matrix: &[Vec<char>], col_index: usize) -> bool {
    matrix
        .iter()
        .all(|row| col_index < row.len() && row[col_index] == '.')
}

struct Grid {
    x_offset: Vec<usize>,
    y_offset: Vec<usize>,
    positions: Vec<(usize, usize)>,
}

impl Grid {
    fn new(input: &str, shift_factor: usize) -> Grid {
        let grid: Vec<Vec<char>> = input
            .lines()
            .map(|line| line.chars().collect::<Vec<char>>())
            .collect();

        let mut positions: Vec<(usize, usize)> = Vec::new();
        let mut x_offset: Vec<usize> = vec![0];
        let mut y_offset: Vec<usize> = vec![0];

        for i in 0..grid.len() {
            if grid[i].iter().all(|c| *c == '.') {
                y_offset.push(y_offset.iter().last().unwrap() + (shift_factor - 1));
            } else {
                y_offset.push(*y_offset.iter().last().unwrap());
            }
            for j in 0..grid[0].len() {
                if grid[i][j] == '#' {
                    positions.push((i, j));
                }
            }
        }

        for col_index in 0..grid[0].len() {
            if is_column_all_dots(&grid, col_index) {
                x_offset.push(x_offset.iter().last().unwrap() + (shift_factor - 1));
            } else {
                x_offset.push(*x_offset.iter().last().unwrap());
            }
        }

        Grid {
            x_offset: x_offset[1..].to_vec(),
            y_offset: y_offset[1..].to_vec(),
            positions,
        }
    }

    fn get_total_distances(&self) -> i64 {
        let mut sum: i64 = 0;
        for i in 0..self.positions.len() {
            for j in i + 1..self.positions.len() {
                let (y1, x1) = self.positions[i];
                let (y2, x2) = self.positions[j];
                let x_dist = (x1 as i64 - x2 as i64).abs()
                    + (self.x_offset[x2] as i64 - self.x_offset[x1] as i64).abs();
                let y_dist = (y1 as i64 - y2 as i64).abs()
                    + (self.y_offset[y2] as i64 - self.y_offset[y1] as i64).abs();
                sum += x_dist + y_dist;
            }
        }
        sum
    }
}

fn part1(input: &str) -> i64 {
    let grid = Grid::new(input, 2);
    grid.get_total_distances()
}

fn part2(input: &str) -> i64 {
    let grid = Grid::new(input, 1_000_000);
    grid.get_total_distances()
}

#[cfg(test)]
mod day11 {
    use super::*;
    #[test]
    fn test_part_1() {
        let test_input = std::fs::read_to_string("src/bin/day11_test.txt").unwrap();
        let result = part1(&test_input);
        assert_eq!(result, 374);
    }
    #[test]
    fn test_part_2() {
        let test_input = std::fs::read_to_string("src/bin/day11_test.txt").unwrap();
        let result = part2(&test_input);
        assert_eq!(result, 82000210);
    }
}
