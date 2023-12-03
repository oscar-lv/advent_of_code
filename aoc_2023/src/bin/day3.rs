extern crate regex;
use regex::Regex;
use std::collections::HashMap;
use std::time::Instant;

fn main() {
    // read file
    let input = std::fs::read_to_string("day3_input.txt").unwrap();

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

struct Input {
    grid: Vec<Vec<char>>,
}

impl Input {
    fn new(input: &str) -> Input {
        let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

        Input { grid }
    }
}

fn is_symbol(c: char) -> bool {
    !c.is_digit(10) && c != '.'
}

// Function to process the input and return the count
fn part1(inp: &str) -> i32 {
    let mut sum = 0;
    let input = Input::new(&inp.replace(" ", ""));
    let rows = input.grid.len();
    let cols = input.grid[0].len(); // Assuming all rows have equal length

    let re = Regex::new(r"\d+").unwrap();
    for (row_idx, row) in input.grid.iter().enumerate() {
        for cap in re.find_iter(&row.iter().collect::<String>()) {
            let num_value: i32 = cap.as_str().parse().unwrap();
            let num_start = cap.start();
            let num_end = cap.end();

            // check adjacent cells for symbols
            for r in row_idx.saturating_sub(1)..=(row_idx + 1).min(rows - 1) {
                for c in num_start.saturating_sub(1)..=num_end.min(cols - 1) {
                    if (r != row_idx || c < num_start || c >= num_end)
                        && is_symbol(input.grid[r][c])
                    {
                        sum += num_value;
                        break;
                    }
                }
            }
        }
    }
    sum
}

// Function to process sum the products of all numbers adjacent to * (limited to 2) in the input
fn part2(inp: &str) -> i32 {
    let mut sum = 0;
    let input = Input::new(&inp.replace(" ", ""));
    let mut numbers_and_positions: HashMap<i32, (usize, usize, usize)> = HashMap::new();
    let re = Regex::new(r"\d+").unwrap();
    for (row_idx, row) in input.grid.iter().enumerate() {
        for cap in re.find_iter(&row.iter().collect::<String>()) {
            let num_value: i32 = cap.as_str().parse().unwrap();
            let num_start = cap.start();
            let num_end = cap.end();
            numbers_and_positions.insert(num_value, (row_idx, num_start, num_end));
        }
    }
    for (row_idx, row) in input.grid.iter().enumerate() {
        for (col_idx, &c) in row.iter().enumerate() {
            if c == '*' {
                let mut adjacent_numbers: Vec<i32> = Vec::new();
                for (number, (num_row, num_start, num_end)) in &numbers_and_positions {
                    // Check if the number is in the same, above, or below row
                    if *num_row == row_idx
                        || *num_row == row_idx + 1
                        || *num_row == row_idx.saturating_sub(1)
                    {
                        for i in *num_start..=*num_end - 1 {
                            if i >= col_idx - 1 && i <= col_idx + 1 {
                                adjacent_numbers.push(*number);
                                break;
                            }
                        }
                    }
                }

                if adjacent_numbers.len() == 2 {
                    sum += adjacent_numbers[0] * adjacent_numbers[1];
                }
            }
        }
    }
    sum
}

#[cfg(test)]
mod day3 {
    use super::*;
    #[test]
    fn test_part_1() {
        let test_input = "467..114..
        ...*......
        ..35..633.
        ......#...
        617*......
        .....+.58.
        ..592.....
        ......755.
        ...$.*....
        .664.598.."
            .to_string();
        let result = part1(&test_input);
        println!("{}", result);
        assert_eq!(result, 4361);
    }
    #[test]
    fn test_part_2() {
        let test_input = "467..114..
        ...*......
        ..35..633.
        ......#...
        617*......
        .....+.58.
        ..592.....
        ......755.
        ...$.*....
        .664.598.."
            .to_string();

        let result = part2(&test_input);
        assert_eq!(result, 467835);
    }
}
