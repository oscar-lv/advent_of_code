// Part 1
fn count(input: &str) -> i32 {
    let mut count = 0;
    for char in input.chars() {
        if char == '(' {
            count += 1;
        } else if char == ')' {
            count -= 1;
        }
    }
    count
}

// Hello world in rust
fn main() {
    // read file
    let input = std::fs::read_to_string("day1_input.txt").unwrap();

    // count and print
    println!("Part 1: {}", count(&input));
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let test_input = "(())".to_string();
        let result = count(&test_input);
        assert_eq!(result, 0);
    }
}
