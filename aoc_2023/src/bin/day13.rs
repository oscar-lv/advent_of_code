use std::time::Instant;

extern crate rayon;
use rayon::prelude::*;

fn main() {
    // read file
    let input = std::fs::read_to_string("src/bin/day13_input.txt").unwrap();

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

fn parse(input: &str) -> (Vec<String>, Vec<String>) {
    let lines: Vec<&str> = input.lines().collect();
    let rows: Vec<String> = lines
        .iter()
        .map(|line| line.replace(".", "0").replace("#", "1"))
        .collect();

    let max_length = lines[0].len();
    let mut columns: Vec<String> = Vec::new();

    for i in 0..max_length {
        let column: String = lines
            .iter()
            .map(|line| line.chars().nth(i).unwrap())
            .collect();
        columns.push(column.replace(".", "0").replace("#", "1"));
    }

    (rows, columns)
}

fn symmetric<T: std::cmp::PartialEq>(arr: Vec<T>) -> usize {
    for center in 0..arr.len() - 1 {
        if arr[center] == arr[center + 1] {
            let mut left: i32 = center as i32 - 1;
            let mut right: usize = center + 2;

            while left >= 0 && right < arr.len() && arr[left as usize] == arr[right] {
                left -= 1;
                right += 1;
            }

            if left < 0 || right >= arr.len() {
                return center + 1;
            }
        }
    }
    0
}

fn diff_in_strings(a: &String, b: &String) -> usize {
    a.chars()
        .zip(b.chars())
        .map(|(x, y)| (x as i32 - y as i32).abs() as usize)
        .sum()
}

fn symmetric2(arr: Vec<String>) -> usize {
    for center in 0..arr.len() - 1 {
        let mut diff_count = 0;
        let mut left: i32 = center as i32;
        let mut right: usize = center + 1;

        while left >= 0 && right < arr.len() && diff_count <= 1 {
            // increment diff count with bitwise differences
            diff_count += diff_in_strings(&arr[left as usize], &arr[right]);
            left -= 1;
            right += 1;
        }

        if (left < 0 || right >= arr.len()) && diff_count == 1 {
            return center + 1;
        }
    }
    0
}

fn part1(input: &str) -> usize {
    let inputs = input.split("\n\n").collect::<Vec<&str>>();
    inputs
        .par_iter()
        .map(|input| {
            let (rows, columns) = parse(input);
            symmetric(rows) * 100 + symmetric(columns)
        })
        .sum()
}

fn part2(input: &str) -> usize {
    let inputs = input.split("\n\n").collect::<Vec<&str>>();
    inputs
        .par_iter()
        .map(|input| {
            let (rows, columns) = parse(input);
            symmetric2(rows) * 100 + symmetric2(columns)
        })
        .sum()
}

#[cfg(test)]
mod day13 {
    use super::*;
    #[test]
    fn test_part_1() {
        let test_input = std::fs::read_to_string("src/bin/day13_test.txt").unwrap();
        let result = part1(&test_input);
        assert_eq!(result, 405);
    }
    #[test]
    fn test_part_2() {
        let test_input = std::fs::read_to_string("src/bin/day13_test.txt").unwrap();
        let result = part2(&test_input);
        assert_eq!(result, 400);
    }
}
