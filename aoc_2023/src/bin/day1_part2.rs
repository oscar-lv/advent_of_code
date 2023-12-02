fn main() {
    // read file
    let input = std::fs::read_to_string("src/bin/day1_input.txt").unwrap();

    // count and print
    println!("Part 1: {}", part2(&input));
}

// Helper function to encode numbers in text
fn encode_numbers(text: &str) -> String {
    text.replace("one", "o1e")
        .replace("two", "t2o")
        .replace("three", "t3e")
        .replace("four", "f4r")
        .replace("five", "f5e")
        .replace("six", "s6x")
        .replace("seven", "s7n")
        .replace("eight", "e8t")
        .replace("nine", "n9e")
        .replace("zero", "z0o")
}

// Improved Part 1
fn part2(input: &str) -> i32 {
    input
        .lines()
        .filter(|line| !line.is_empty()) // Filtering out empty lines
        .map(|line| {
            let line = line.trim();
            // Print line (Consider removing this in a production environment)
            println!("{}", line);

            // Replace words with encoded numbers directly
            let encoded_line = encode_numbers(line);

            // Collect only the digits
            let numbers: Vec<char> = encoded_line.chars().filter(|c| c.is_digit(10)).collect();

            // Concatenate first and last number, parse to i32, and handle potential parse errors
            match (numbers.first(), numbers.last()) {
                (Some(first), Some(last)) => {
                    format!("{}{}", first, last).parse::<i32>().unwrap_or(0)
                }
                _ => 0, // Handle cases where there are no digits
            }
        })
        .sum() // Sum the parsed numbers
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part_2() {
        let test_input = "two1nine
        eightwothree
        abcone2threexyz
        xtwone3four
        4nineeightseven2
        zoneight234
        7pqrstsixteen"
            .to_string();
        let result = part2(&test_input);
        assert_eq!(result, 281);
    }
}
