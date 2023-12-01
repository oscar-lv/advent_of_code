fn main() {
    // read file
    let input = std::fs::read_to_string("src/bin/input.txt").unwrap();

    // count and print
    println!("Part 1: {}", part1(&input));
}

// Part 1
fn part1(input: &str) -> i32 {
    let mut sum: i32 = 0;
    // for each line, read line
    input.lines().for_each(|line| {
        // if line is empty, continue
        if line.is_empty() {
            return;
        }
        // trim whitespace
        let line = line.trim();
        // print line
        println!("{}", line);
        // create empty dynamic arrary of strings
        let mut numbers: Vec<String> = Vec::new();
        // for each character, check if it is a digit after trimming whitespace
        line.chars().for_each(|c| {
            if c.is_digit(10) {
                numbers.push(c.to_string());
            }
        });
        // concatenate first and last number into string,parse the combined string into i32, and add to sum
        sum += format!("{}{}", numbers[0], numbers[numbers.len() - 1])
            .parse::<i32>()
            .unwrap();
    });
    sum
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part_1() {
        let test_input = "1abc2
        pqr3stu8vwx
        a1b2c3d4e5f
        treb7uchet"
            .to_string();
        let result = part1(&test_input);
        assert_eq!(result, 142);
    }
}
