extern crate regex;
use std::collections::HashMap;
use std::time::Instant;

fn main() {
    // read file
    let input = std::fs::read_to_string("day8_input.txt").unwrap();

    // start timer
    let start = Instant::now();

    // count and print
    println!("Part 1: {}", part1(&input));

    // print time taken by part1
    println!("Time taken by Part 1: {:?}", start.elapsed());

    // reset timer
    let start = Instant::now();

    println!("Part 2: {}", part2_par(&input));

    // print time taken by part2
    println!("Time taken by Part 2: {:?}", start.elapsed());
}

fn parse(input: &str) -> (String, HashMap<String, (String, String)>) {
    let (directions, rest) = input.split_once("\n\n").unwrap();
    let mut map: HashMap<String, (String, String)> = HashMap::new();
    let rest = rest.split("\n").for_each(|l| {
        let (key, values) = l.split_once("=").unwrap();
        let (left, right) = values.split_once(",").unwrap();
        map.insert(
            key.trim().to_string(),
            (
                left.to_string().replace("(", "").trim().to_owned(),
                right.to_string().replace(")", "").trim().to_owned(),
            ),
        );
    });
    (directions.to_string(), map)
}

// Euclide's algorithm
fn gcd(a: usize, b: usize) -> usize {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn lcm(a: usize, b: usize) -> usize {
    a / gcd(a, b) * b
}

fn lcm_of_list(numbers: &[usize]) -> usize {
    numbers.iter().fold(1, |acc, &num| lcm(acc, num))
}

// Process hands, establish rankings within strength groups and determine rank of each hand
fn part1(input: &str) -> usize {
    let (directions, map) = parse(input);
    let mut current = "AAA";
    let mut i = 0;
    while current != "ZZZ" {
        let (left, right) = map.get(current).unwrap();
        if directions.as_bytes()[i % directions.len()] == b'R' {
            current = right;
        } else {
            current = left;
        }
        i += 1;
    }
    i
}

fn part2(input: &str) -> usize {
    let (directions, map) = parse(input);
    let mut current: Vec<String> = map.keys().filter(|&k| k.ends_with("A")).cloned().collect();
    let mut all_steps: Vec<usize> = Vec::new();
    for c in current {
        let mut step = 0;
        let mut c = c.to_string();
        while !c.ends_with("Z") {
            let (left, right) = map.get(&c).unwrap();
            if directions.as_bytes()[step % directions.len()] == b'R' {
                c = right.to_string();
            } else {
                c = left.to_string();
            }
            step += 1;
        }
        all_steps.push(step.try_into().unwrap());
    }
    lcm_of_list(&all_steps) as usize
}

extern crate rayon;
use rayon::prelude::*;

// parrallel version of part2
fn part2_par(input: &str) -> usize {
    let (directions, map) = parse(input);
    let current: Vec<String> = map.keys().filter(|&k| k.ends_with("A")).cloned().collect();

    // Process each element in `current` in parallel
    let all_steps: Vec<usize> = current
        .par_iter() // <-- Use a parallel iterator
        .map(|c| {
            let mut step = 0;
            let mut c = c.to_string();
            while !c.ends_with("Z") {
                let (left, right) = map.get(&c).unwrap();
                if directions.as_bytes()[step % directions.len()] == b'R' {
                    c = right.to_string();
                } else {
                    c = left.to_string();
                }
                step += 1;
            }
            step
        })
        .collect();

    lcm_of_list(&all_steps) as usize
}

#[cfg(test)]
mod day8 {
    use super::*;
    #[test]
    fn test_part_1() {
        let test_input = std::fs::read_to_string("src/bin/day8_test.txt").unwrap();
        let result = part1(&test_input);
        assert_eq!(result, 6);
    }
    #[test]
    fn test_part_2() {
        let test_input = std::fs::read_to_string("src/bin/day8_test2.txt").unwrap();
        let result = part2(&test_input);
        assert_eq!(result, 6);
    }
}
