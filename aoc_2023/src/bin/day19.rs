use std::time::Instant;
use std::{collections::VecDeque, ops::RangeInclusive};

use std::collections::HashMap;

fn main() {
    // read file
    let input = std::fs::read_to_string("src/bin/day19_input.txt").unwrap();

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

fn parse(input: &str) -> (Vec<HashMap<String, i32>>, HashMap<String, Vec<String>>) {
    let (rules, parts) = input.split_once("\n\n").unwrap();
    let part_map: Vec<HashMap<String, i32>> = parts
        .lines()
        .map(|part| {
            let elements = part.split(",").collect::<Vec<_>>();
            let mut map = HashMap::new();
            elements.iter().for_each(|&element| {
                let (var, num) = element.split_once("=").unwrap();
                map.insert(
                    var.to_string().replace("{", ""),
                    num.replace("}", "").parse::<i32>().unwrap(),
                );
            });
            map
        })
        .collect();
    let rule_map: HashMap<String, Vec<String>> = rules
        .lines()
        .map(|rule| {
            let (key, flow) = rule.split_once("{").unwrap();
            let flow = flow
                .replace("}", "")
                .split(",")
                .map(|s| s.to_string())
                .collect::<Vec<String>>();
            (key.to_string(), flow)
        })
        .collect();
    (part_map, rule_map)
}

fn outcome(
    part: &HashMap<String, i32>,
    rule_map: &HashMap<String, Vec<String>>,
    start: String,
) -> String {
    // check if part is valid
    let mut res = "R".to_string();
    match rule_map.get(&start) {
        Some(flow) => {
            // Handle the flow
            for rule in flow {
                if rule.contains(':') {
                    let (operation, outcome) = rule.split_once(":").unwrap();
                    if (operation.chars().nth(1).unwrap() == '>'
                        && part[&operation.chars().nth(0).unwrap().to_string()]
                            > operation[2..].parse::<i32>().unwrap())
                        || (operation.chars().nth(1).unwrap() == '<'
                            && part[&operation.chars().nth(0).unwrap().to_string()]
                                < operation[2..].parse::<i32>().unwrap())
                    {
                        res = outcome.to_string();
                        break;
                    }
                } else {
                    res = rule.to_string();
                }
            }
        }
        None => {
            panic!("Invalid flow")
        }
    }
    match res.as_str() {
        "A" => res,
        "R" => res,
        _ => outcome(part, rule_map, res),
    }
}

impl ItemRanges {
    pub fn possibilities(self) -> usize {
        let len_x = self.x.count();
        let len_m = self.m.count();
        let len_a = self.a.count();
        let len_s = self.s.count();
        len_x * len_m * len_a * len_s
    }
}

impl std::ops::Index<char> for ItemRanges {
    type Output = RangeInclusive<u32>;
    fn index(&self, index: char) -> &Self::Output {
        match index {
            'x' => &self.x,
            'm' => &self.m,
            'a' => &self.a,
            's' => &self.s,
            _ => panic!("Invalid variable {index}"),
        }
    }
}

impl std::ops::IndexMut<char> for ItemRanges {
    fn index_mut(&mut self, index: char) -> &mut Self::Output {
        match index {
            'x' => &mut self.x,
            'm' => &mut self.m,
            'a' => &mut self.a,
            's' => &mut self.s,
            _ => panic!("Invalid variable {index}"),
        }
    }
}

#[derive(Debug, Clone)]
struct ItemRanges {
    x: RangeInclusive<u32>,
    m: RangeInclusive<u32>,
    a: RangeInclusive<u32>,
    s: RangeInclusive<u32>,
}

fn outcome_2(rule_map: &HashMap<String, Vec<String>>) -> usize {
    // check if part is valid
    let start_ranges = ItemRanges {
        x: 1..=4000,
        m: 1..=4000,
        a: 1..=4000,
        s: 1..=4000,
    };
    let mut queue = VecDeque::new();
    queue.push_back(("in", start_ranges));
    let mut accepted_ranges = Vec::new();

    while let Some((outcome, mut ranges)) = queue.pop_front() {
        if outcome == "A" {
            accepted_ranges.push(ranges);
            continue;
        } else if outcome == "R" {
            continue;
        }
        let flow = rule_map.get(outcome).unwrap();
        for rule in flow {
            if rule.contains(':') {
                let mut branch_ranges = ranges.clone();
                let (operation, outcome) = rule.split_once(":").unwrap();
                let (target, operator, comparator) = (
                    operation
                        .chars()
                        .nth(0)
                        .unwrap()
                        .to_string()
                        .chars()
                        .nth(0)
                        .unwrap(),
                    operation.chars().nth(1).unwrap(),
                    operation[2..].parse::<u32>().unwrap(),
                );
                match operator {
                    '<' => {
                        branch_ranges[target] = (*branch_ranges[target].start())
                            ..=(*branch_ranges[target].end().min(&comparator) - 1);
                        ranges[target] =
                            (*ranges[target].start().max(&comparator))..=(*ranges[target].end());
                        queue.push_back((outcome, branch_ranges));
                    }
                    '>' => {
                        branch_ranges[target] = (*branch_ranges[target].start().max(&comparator)
                            + 1)
                            ..=(*branch_ranges[target].end());
                        ranges[target] =
                            (*ranges[target].start())..=(*ranges[target].end().min(&comparator));
                        queue.push_back((outcome, branch_ranges));
                    }
                    _ => {
                        panic!("Invalid operator")
                    }
                }
            } else {
                queue.push_back((rule, ranges));
                break;
            }
        }
    }
    accepted_ranges
        .into_iter()
        .map(ItemRanges::possibilities)
        .sum()
}

fn part1(input: &str) -> usize {
    let (part_map, rule_map) = parse(input);
    let mut total = 0;
    for part in part_map {
        if outcome(&part, &rule_map, "in".to_string()) == "A" {
            total += part.values().sum::<i32>() as usize
        }
    }
    total
}

fn part2(input: &str) -> usize {
    let (_, rule_map) = parse(input);
    outcome_2(&rule_map)
}

#[cfg(test)]
mod day19 {
    use super::*;
    #[test]
    fn test_part_1() {
        let test_input = std::fs::read_to_string("src/bin/day19_test.txt").unwrap();
        let result = part1(&test_input);
        assert_eq!(result, 19114);
    }
    #[test]
    fn test_part_2() {
        let test_input = std::fs::read_to_string("src/bin/day19_test.txt").unwrap();
        let result = part2(&test_input);
        assert_eq!(result, 167409079868000);
    }
}
