extern crate regex;
use std::collections::HashMap;
use std::time::Instant;

extern crate rayon;
use rayon::prelude::*;

fn main() {
    // read file
    let input = std::fs::read_to_string("day9_input.txt").unwrap();

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

fn first_differences(line: Vec<i32>) -> Vec<i32> {
    let mut differences: Vec<i32> = Vec::new();
    let mut prev: i32 = line[0];
    for i in 1..line.len() {
        let current = line[i];
        differences.push(current - prev);
        prev = current;
    }
    differences
}

// Process hands, establish rankings within strength groups and determine rank of each hand
fn part1(input: &str) -> i32 {
    input
        .lines()
        .map(|line| {
            let mut line: Vec<i32> = line
                .split_whitespace()
                .map(|x| x.parse::<i32>().unwrap())
                .collect();
            let mut sum: i32 = *line.iter().last().unwrap();
            while !line.iter().all(|x| *x == 0) {
                line = first_differences(line);
                sum += line.iter().last().unwrap();
            }
            sum
        })
        .sum()
}

fn part2(input: &str) -> i32 {
    input
        .lines()
        .map(|line| {
            let mut line: Vec<i32> = line
                .split_whitespace()
                .map(|x| x.parse::<i32>().unwrap())
                .collect();
            let mut sum: i32 = 0;
            let mut line_vec: Vec<Vec<i32>> = Vec::new();
            while !line.iter().all(|x| *x == 0) {
                line_vec.push(line.clone());
                line = first_differences(line);
            }

            let mut diff = 0;
            for i in (0..line_vec.len() - 1).rev() {
                diff = line_vec[i][0] - line_vec[i + 1][0];
                line_vec[i].insert(0, diff)
            }

            diff
        })
        .sum()
}

#[cfg(test)]
mod day9 {
    use super::*;
    #[test]
    fn test_part_1() {
        let test_input = std::fs::read_to_string("src/bin/day9_test.txt").unwrap();
        let result = part1(&test_input);
        assert_eq!(result, 114);
    }
    #[test]
    fn test_part_2() {
        let test_input = std::fs::read_to_string("src/bin/day9_test.txt").unwrap();
        let result = part2(&test_input);
        assert_eq!(result, 2);
    }
}
