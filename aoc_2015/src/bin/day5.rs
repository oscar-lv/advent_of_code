fn main() {
    // read file
    let input = std::fs::read_to_string("day5_input.txt").unwrap();

    // count and print
    println!("Part 1: {}", part1(&input));
}

fn is_nice(input: &str) -> bool {
    if input.contains("ab") || input.contains("cd") || input.contains("pq") || input.contains("xy")
    {
        return false;
    }

    let mut vowels = 0;
    let mut twice = false;
    let mut prev_char = '\0';

    for c in input.chars() {
        if "aeiou".contains(c) {
            vowels += 1;
        }
        if c == prev_char {
            twice = true;
        }
        prev_char = c;
    }

    vowels >= 3 && twice
}
// Part 1
fn part1(input: &str) -> i32 {
    let mut nice: i32 = 0;
    input.lines().for_each(|line| {
        if is_nice(line) {
            nice += 1;
        }
    });
    nice
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_day5_part_1() {
        let test_input = "ugknbfddgicrmopn".to_string();
        let result = part1(&test_input);
        assert_eq!(result, 1);
    }
}
