extern crate regex;
use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::time::Instant;

fn main() {
    // read file
    let input = std::fs::read_to_string("day4_input.txt").unwrap();

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

// Function to process the input and return the count
fn part1(input: &str) -> i32 {
    let mut sum: i32 = 0;
    input.lines().for_each(|x| {
        // regex to remove "Card 1:" from Card 1: 41 48 83 86 17 | 83 86  6 31 17 9 48 53
        let splits: Vec<&str> = x.split(":").collect::<Vec<&str>>()[1].split('|').collect();

        let set_1: HashSet<_> = splits[0].trim().split_whitespace().collect();
        let set_2: HashSet<_> = splits[1].trim().split_whitespace().collect();

        let count = set_1.intersection(&set_2).count();
        // 2 to the power of count or if 0 : 0
        match count {
            0 => sum += 0,
            _ => sum += i32::pow(2, (count - 1) as u32),
        }
    });
    sum
}

// function to get the total number of cards won given each winning number on a card gives you then n+1th card as a reward
fn part2(input: &str) -> i32 {
    let mut cards: HashMap<i32, i32> = HashMap::new();
    let re = Regex::new(r"(\d+)").unwrap();

    input.lines().for_each(|x| {
        let card_split = x.split(":").collect::<Vec<&str>>();
        let caps = re.captures(card_split[0]).unwrap();
        let id = caps[1].parse::<i32>().unwrap();
        let _ = *cards.entry(id).or_insert(1);

        let line = card_split[1];
        let splits: Vec<&str> = line.split('|').collect();

        let set_1: HashSet<_> = splits[0].trim().split_whitespace().collect();
        let set_2: HashSet<_> = splits[1].trim().split_whitespace().collect();

        let count = set_1.intersection(&set_2).count();

        let card_count = *cards.get(&id).unwrap_or(&0); // Clone the value here

        for i in 1..count + 1 {
            let next_card = id + i as i32;
            *cards.entry(next_card).or_insert(1) += card_count;
        }
    });

    // Calculate the sum
    cards.values().sum()
}

#[cfg(test)]
mod day4 {
    use super::*;
    #[test]
    fn test_part_1() {
        let test_input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17 9 48 53
        Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
        Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
        Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
        Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
        Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"
            .to_string();
        let result = part1(&test_input);
        assert_eq!(result, 13);
    }
    #[test]
    fn test_part_2() {
        let test_input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
        Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
        Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
        Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
        Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
        Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"
            .to_string();

        let result = part2(&test_input);
        assert_eq!(result, 30);
    }
}
