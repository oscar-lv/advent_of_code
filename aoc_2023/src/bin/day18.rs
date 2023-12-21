use std::time::Instant;

fn main() {
    // read file
    let input = std::fs::read_to_string("src/bin/day18_input.txt").unwrap();

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

fn area_from_instructions(instructions: Vec<(&str, isize)>) -> isize {
    // shoelace formula
    let (mut area, mut row, mut col) = (0, 0, 0);
    for (dir, steps) in instructions {
        let (nrow, ncol) = (row, col);
        match dir {
            "U" => row -= steps,
            "D" => row += steps,
            "L" => col -= steps,
            "R" => col += steps,
            _ => panic!("Invalid direction"),
        };
        area += (row - nrow) * (col + ncol) + steps;
    }
    area / 2 + 1
}

fn part1(input: &str) -> isize {
    let instructions: Vec<(&str, isize)> = input
        .lines()
        .map(|line| {
            let (dir, rest) = line.split_once(" ").unwrap();
            let (steps, _) = rest.split_once(" ").unwrap();
            (dir, steps.parse::<isize>().unwrap())
        })
        .collect();

    area_from_instructions(instructions)
}

fn part2(input: &str) -> isize {
    let instructions = input
        .lines()
        .map(|l| {
            let (_, color) = l.split_once("#").unwrap();
            let color_chars: Vec<char> = color.chars().collect();
            let dir = match color_chars[color_chars.len() - 2] {
                '0' => "R",
                '1' => "D",
                '2' => "L",
                '3' => "U",
                _ => panic!("Invalid direction"),
            };
            (
                dir,
                isize::from_str_radix(&color[..color.len() - 2], 16).unwrap(),
            )
        })
        .collect();

    area_from_instructions(instructions)
}

#[cfg(test)]
mod day18 {
    use super::*;
    #[test]
    fn test_part_1() {
        let test_input = std::fs::read_to_string("src/bin/day18_test.txt").unwrap();
        let result = part1(&test_input);
        assert_eq!(result, 62);
    }
    #[test]
    fn test_part_2() {
        let test_input = std::fs::read_to_string("src/bin/day18_test.txt").unwrap();
        let result = part2(&test_input);
        assert_eq!(result, 952408144115);
    }
}
