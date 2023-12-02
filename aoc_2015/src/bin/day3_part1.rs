use std::collections::HashSet;

fn main() {
    // read file
    let input = std::fs::read_to_string("src/bin/input.txt").unwrap();

    // count and print
    println!("Part 1: {}", part1(&input));
}

// Part 1
fn part1(input: &str) -> i32 {
    let mut x = 0;
    let mut y = 0;

    let mut positions = HashSet::new();
    // for each line, read line
    input.chars().for_each(|char| {
        // move santa
        match char {
            '^' => y += 1,
            'v' => y -= 1,
            '>' => x += 1,
            '<' => x -= 1,
            _ => (),
        }
        // add position to set
        positions.insert((x, y));
    });
    // return length of set
    positions.len() as i32
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part_1() {
        let test_input = "^>v<".to_string();
        let result = part1(&test_input);
        assert_eq!(result, 4);
    }
}
