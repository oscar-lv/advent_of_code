use cached::proc_macro::cached;
use std::time::Instant;
fn main() {
    // read file
    let input = std::fs::read_to_string("day12_input.txt").unwrap();

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

#[cached(
    key = "String",
    convert = r#"{format!("{:?}{:?}{:?}", s, in_group, cons)}"#
)]
fn solve(s: &[u8], in_group: Option<usize>, cons: &[usize]) -> usize {
    if s.is_empty() {
        return match in_group {
            Some(n) if cons == &[n] => 1,
            None if cons.is_empty() => 1,
            _ => 0,
        };
    }
    match (s[0], in_group, cons) {
        (b'.', None, _) | (b'?', None, []) => solve(&s[1..], None, cons),
        (b'.' | b'?', Some(n), [e, ..]) if n == *e => solve(&s[1..], None, &cons[1..]),
        (b'#' | b'?', Some(n), [e, ..]) if n < *e => solve(&s[1..], Some(n + 1), cons),
        (b'#', None, [_, ..]) => solve(&s[1..], Some(1), cons),
        (b'?', None, _) => solve(&s[1..], None, cons) + solve(&s[1..], Some(1), cons),
        _ => 0,
    }
}

fn parse(input: &str) -> Vec<(&[u8], Vec<usize>)> {
    input
        .lines()
        .map(|l| {
            let (sequence, results) = l.split_once(" ").unwrap();
            (
                sequence.as_bytes(),
                results
                    .split(',')
                    .map(|x| x.parse::<usize>().unwrap())
                    .collect::<Vec<usize>>(),
            )
        })
        .collect()
}

fn part1(input: &str) -> usize {
    parse(input)
        .into_iter()
        .map(|(s, ns)| solve(s, None, &ns))
        .sum()
}

fn part2(input: &str) -> usize {
    let new_input = input.lines().fold(String::new(), |mut acc, l| {
        let (s, n) = l.split_once(" ").unwrap();
        acc.push_str(&format!("{s}?{s}?{s}?{s}?{s} {n},{n},{n},{n},{n}\n"));
        acc
    });
    part1(&new_input)
}

#[cfg(test)]
mod day12 {
    use super::*;
    #[test]
    fn test_part_1() {
        let test_input = std::fs::read_to_string("src/bin/day12_test.txt").unwrap();
        let result = part1(&test_input);
        assert_eq!(result, 21);
    }
    #[test]
    fn test_part_2() {
        let test_input = std::fs::read_to_string("src/bin/day12_test.txt").unwrap();
        let result = part2(&test_input);
        assert_eq!(result, 525152);
    }
}
