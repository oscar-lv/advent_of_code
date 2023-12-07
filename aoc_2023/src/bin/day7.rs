extern crate regex;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::io::Write;
use std::ops::BitXor;
use std::time::Instant;

fn main() {
    // read file
    let input = std::fs::read_to_string("day7_input.txt").unwrap();

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

struct Hand {
    cards: String,
    strength: u32,
    bid: i32,
}

impl Hand {
    fn build(input: &str) -> Hand {
        let (cards, bid) = input.split_once(" ").unwrap();
        let bid = bid.parse::<i32>().unwrap();

        let (mut ranks, mut power) = ([0u8; 13], 0);
        for i in 0..5 {
            let card = match cards.as_bytes()[i] {
                b'A' => 12,
                b'K' => 11,
                b'Q' => 10,
                b'J' => 9,
                b'T' => 8,
                n => n - b'0' - 2,
            };
            ranks[card as usize] += 1;
            power |= (card as u32) << 4 * (4 - i);
        }
        ranks.sort_unstable_by(|a, b| b.cmp(a));
        power |= match ranks[0] {
            5 => 6,
            4 => 5,
            3 if ranks[1] == 2 => 4,
            3 => 3,
            2 if ranks[1] == 2 => 2,
            2 => 1,
            _ => 0,
        } << 29;

        Hand {
            cards: cards.to_string(),
            strength: power,
            bid,
        }
    }
    fn build_part2(input: &str) -> Hand {
        let (cards, bid) = input.split_once(" ").unwrap();
        let bid = bid.parse::<i32>().unwrap();

        let (mut ranks, mut power, mut jokers) = ([0u8; 13], 0, 0);
        for i in 0..5 {
            let card = match cards.as_bytes()[i] {
                b'A' => 12,
                b'K' => 11,
                b'Q' => 10,
                b'J' => 0,
                b'T' => 9,
                n => n - b'0' - 1,
            };
            ranks[card as usize] += 1 * (card != 0) as u8;
            power |= (card as u32) << 4 * (4 - i);
            jokers += 1 * (card == 0) as u8;
        }
        ranks.sort_unstable_by(|a, b| b.cmp(a));
        power |= match ranks[0] + jokers {
            5 => 6,
            4 => 5,
            3 if ranks[1] == 2 => 4,
            3 => 3,
            2 if ranks[1] == 2 => 2,
            2 => 1,
            _ => 0,
        } << 29;

        Hand {
            cards: cards.to_string(),
            strength: power,
            bid,
        }
    }
}

// Process hands, establish rankings within strength groups and determine rank of each hand
fn part1(input: &str) -> usize {
    let mut hands: Vec<Hand> = input.lines().map(|l| Hand::build(l)).collect::<Vec<Hand>>();

    hands.sort_by_key(|a| a.strength);

    // enumerate the hands and multiply the rank by the bid
    hands
        .iter()
        .enumerate()
        .map(|(i, h)| (i + 1) * h.bid as usize)
        .sum()
}

fn part2(input: &str) -> usize {
    let mut hands: Vec<Hand> = input
        .lines()
        .map(|l| Hand::build_part2(l))
        .collect::<Vec<Hand>>();

    hands.sort_by_key(|a| a.strength);

    // enumerate the hands and multiply the rank by the bid
    hands
        .iter()
        .enumerate()
        .map(|(i, h)| (i + 1) * h.bid as usize)
        .sum()
}

#[cfg(test)]
mod day7 {
    use super::*;
    #[test]
    fn test_part_1() {
        let test_input = std::fs::read_to_string("src/bin/day7_test.txt").unwrap();
        let result = part1(&test_input);
        assert_eq!(result, 6440);
    }
    #[test]
    fn test_part_2() {
        let test_input = std::fs::read_to_string("src/bin/day7_test.txt").unwrap();
        let result = part2(&test_input);
        assert_eq!(result, 5905);
    }
}
