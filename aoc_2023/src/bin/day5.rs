use std::ops::Range;
use std::time::Instant;

fn main() {
    // read file
    let input = std::fs::read_to_string("day5_input.txt").unwrap();

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

struct Map {
    ranges: Vec<MapRange>,
}

struct MapRange {
    range: Range<i64>,
    shift: i64,
}

impl Map {
    fn build(input: &str) -> Map {
        let map_ranges: &str = input.split_once("\n").unwrap().1;
        let ranges: Vec<MapRange> = map_ranges
            .split("\n")
            .filter(|l| !l.is_empty())
            .map(|l| {
                let splits: Vec<&str> = l.split(" ").collect();
                let destination = splits[0].parse::<i64>().unwrap();
                let source = splits[1].parse::<i64>().unwrap();
                let step = splits[2].parse::<i64>().unwrap();
                // build hashmap from {destination..destation+step :  source..source+step}
                MapRange {
                    range: source..(source + step),
                    shift: destination - source,
                }
            })
            .collect::<Vec<MapRange>>();
        Map { ranges }
    }

    fn get(&self, seed: i64) -> i64 {
        for r in &self.ranges {
            if r.range.contains(&seed) {
                return seed + r.shift;
            }
        }
        seed
    }
}
struct Almanac {
    seeds: Vec<i64>,
    maps: Vec<Map>,
}

impl Almanac {
    fn new(input: &str) -> Almanac {
        let (seeds, map_str) = input.split_once("\n\n").unwrap();
        let seeds = seeds.split(": ").collect::<Vec<&str>>()[1]
            .split_whitespace()
            .map(|x| x.parse::<i64>().unwrap())
            .collect::<Vec<i64>>();

        let maps: Vec<Map> = map_str
            .split("\n\n")
            .filter(|l| !l.is_empty())
            .map(|l| Map::build(l))
            .collect();
        Almanac {
            seeds: seeds,
            maps: maps,
        }
    }
}

// Function to process the input and return the count
fn part1(input: &str) -> i64 {
    let mut min: i64 = i64::MAX;
    let almanac = Almanac::new(input);
    for seed in almanac.seeds.iter() {
        let mut cur = *seed;
        for m in almanac.maps.iter() {
            cur = m.get(cur);
        }
        min = min.min(cur);
    }
    min
}

// function to get the total number of cards won given each winning number on a card gives you then n+1th card as a reward
fn part2(input: &str) -> i64 {
    let mut min: i64 = i64::MAX;
    let almanac = Almanac::new(input);
    let mut seed_ranges: Vec<Range<i64>> = Vec::new();
    // create ranges from seeds such as [1,12,2,13] becomes [1..1+12, 2..2+13]
    for i in (0..almanac.seeds.len()).step_by(2) {
        seed_ranges.push(almanac.seeds[i]..(almanac.seeds[i] + almanac.seeds[i + 1] - 1));
    }
    for seed_range in seed_ranges {
        println!("seed_range: {:?} ", seed_range);
        for seed in seed_range {
            // println!("seed: {}", seed);
            let mut cur = seed;
            for m in almanac.maps.iter() {
                cur = m.get(cur);
            }
            min = min.min(cur);
        }
    }
    min
}

#[cfg(test)]
mod day5 {
    use super::*;
    #[test]
    fn test_part_1() {
        let test_input = std::fs::read_to_string("src/bin/day5_test.txt").unwrap();
        let result = part1(&test_input);
        assert_eq!(result, 35);
    }
    #[test]
    fn test_part_2() {
        let test_input = std::fs::read_to_string("src/bin/day5_test.txt").unwrap();
        let result = part2(&test_input);
        assert_eq!(result, 46);
    }
}
