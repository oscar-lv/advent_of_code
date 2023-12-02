use md5;

fn main() {
    // read file
    let input = std::fs::read_to_string("day4_input.txt").unwrap();

    // count and print
    println!("Part 1: {}", part1(&input));
}

// Part 1
fn part1(input: &str) -> i32 {
    let mut append: i32 = 0;
    loop {
        let digest = md5::compute(format!("{}{}", input, append));
        let result_str = format!("{:x}", digest);
        if result_str.starts_with("000000") {
            return append;
        }
        append += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part_1() {
        let test_input = "abcdef".to_string();
        let result = part1(&test_input);
        assert_eq!(result, 609043);
    }
}
