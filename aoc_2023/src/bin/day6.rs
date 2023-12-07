extern crate regex;
use std::ops::BitXor;
use std::time::Instant;

fn main() {
    // read file
    let input = std::fs::read_to_string("day6_input.txt").unwrap();

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

struct Races {
    times: Vec<i32>,
    records: Vec<i32>,
}

impl Races {
    fn parse(input: &str) -> Races {
        let (times, records) = input.split_once("\n").unwrap();
        let (_, times) = times.split_once(":").unwrap();
        let (_, records) = records.split_once(":").unwrap();
        Races {
            times: times
                .split_whitespace()
                .filter(|l| !l.is_empty())
                .map(|l| l.parse::<i32>().unwrap())
                .collect::<Vec<i32>>(),
            records: records
                .split_whitespace()
                .filter(|l| !l.is_empty())
                .map(|l| l.parse::<i32>().unwrap())
                .collect::<Vec<i32>>(),
        }
    }
}

fn get_possible_ways_to_win(time: usize, distance: usize) -> usize {
    let d = time * time - 4 * distance;
    let sqrt_d = (d as f64).sqrt() as usize;

    if sqrt_d * sqrt_d == d {
        sqrt_d - 1
    } else {
        sqrt_d + 1 - (time & 1).bitxor(sqrt_d & 1)
    }
}

// Process races, find the possible combos of charging/racing to beat the record
fn part1(input: &str) -> usize {
    let races = Races::parse(input);
    let mut ways: Vec<usize> = Vec::new();
    for i in 0..races.times.len() {
        ways.push(get_possible_ways_to_win(
            races.times[i] as usize,
            races.records[i] as usize,
        ));
    }
    ways.iter().product()
}

// function to get the total number of cards won given each winning number on a card gives you then n+1th card as a reward
fn part2(input: &str) -> i64 {
    let races = Races::parse(input);
    // concatenate the races.times into a single i64
    let time = races
        .times
        .iter()
        .map(|x| x.to_string())
        .collect::<String>()
        .parse::<usize>()
        .unwrap();

    let record = races
        .records
        .iter()
        .map(|x| x.to_string())
        .collect::<String>()
        .parse::<usize>()
        .unwrap();

    get_possible_ways_to_win(time as usize, record as usize) as i64
}

#[cfg(test)]
mod day6 {
    use super::*;
    #[test]
    fn test_part_1() {
        let test_input = std::fs::read_to_string("src/bin/day6_test.txt").unwrap();
        let result = part1(&test_input);
        assert_eq!(result, 288);
    }
    #[test]
    fn test_part_2() {
        let test_input = std::fs::read_to_string("src/bin/day6_test.txt").unwrap();
        let result = part2(&test_input);
        assert_eq!(result, 71503);
    }
}
