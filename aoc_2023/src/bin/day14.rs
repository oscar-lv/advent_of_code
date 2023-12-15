use std::time::Instant;

use rayon::result;

fn main() {
    // read file
    let input = std::fs::read_to_string("src/bin/day14_input.txt").unwrap();

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

fn parse_input(input: &str) -> Vec<String> {
    let rows: Vec<String> = input.lines().map(|line| line.to_string()).collect();
    // return columns

    (0..rows[0].len())
        .map(|i| rows.iter().map(|row| row.chars().nth(i).unwrap()).collect())
        .collect()
}

fn score(line: String) -> usize {
    let mut score = 0;
    let mut shift = 0;
    for (i, c) in line.chars().enumerate() {
        if c == 'O' {
            score += line.len() - i + shift;
        } else if c == '.' {
            shift += 1;
        } else {
            shift = 0;
        }
    }
    score
}

fn map_grid(grid: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut result = grid.clone();
    for col in 0..grid[0].len() {
        let mut next = 0;
        for row in 0..grid.len() {
            match grid[row][col] {
                'O' => {
                    result[next][col] = 'O';
                    if row != next {
                        result[row][col] = '.';
                    }
                    next += 1;
                }
                '#' => {
                    next = row + 1;
                }
                _ => {}
            }
        }
    }
    result
}

fn rotate_grid(grid: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut result: Vec<Vec<char>> = vec![vec!['.'; grid.len()]; grid[0].len()];
    for col in 0..grid[0].len() {
        for row in 0..grid.len() {
            result[col][grid[0].len() - 1 - row] = grid[row][col];
        }
    }
    result
}

fn count_rocks_on_north_beams(grid: &mut Vec<Vec<char>>) -> usize {
    grid.iter()
        .enumerate()
        .map(|(i, line)| line.iter().filter(|c| **c == 'O').count() * (grid.len() - i))
        .sum()
}

fn part1(input: &str) -> usize {
    let lines = parse_input(input);
    lines.iter().map(|line| score(line.to_string())).sum()
}

fn parse_grid(input_str: &str) -> Vec<Vec<char>> {
    input_str
        .lines()
        .map(|line| line.trim())
        .filter(|line| !line.is_empty())
        .map(|line| line.chars().collect())
        .collect()
}

pub fn part2(input: &str) -> usize {
    let mut grid: Vec<Vec<char>> = parse_grid(input);
    let mut results = Vec::new();
    // assumes the cycle is reached within 500 iters
    loop {
        let total = count_rocks_on_north_beams(&mut grid);
        results.push(total);
        for _ in 0..4 {
            grid = map_grid(&grid);
            grid = rotate_grid(&grid);
        }
        if results.len() > 500 {
            break;
        }
    }
    // assumes the last number only occurs once in the cycle
    let mut cycle_length = 0;
    for i in (0..results.len() - 2).rev() {
        if results[i] == results[results.len() - 1] {
            cycle_length = (results.len() - 1) - i;
            break;
        }
    }
    let rem = 1000000000 % cycle_length;
    for i in (0..results.len() - 1).rev() {
        if i % cycle_length == rem {
            return results[i];
        }
    }
    0
}

#[cfg(test)]
mod day14 {
    use super::*;
    #[test]
    fn test_part_1() {
        let test_input = std::fs::read_to_string("src/bin/day14_test.txt").unwrap();
        let result = part1(&test_input);
        assert_eq!(result, 136);
    }
    #[test]
    fn test_part_2() {
        let test_input = std::fs::read_to_string("src/bin/day14_test.txt").unwrap();
        let result = part2(&test_input);
        assert_eq!(result, 64);
    }
}
