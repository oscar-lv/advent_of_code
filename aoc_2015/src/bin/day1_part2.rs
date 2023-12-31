// Part 2
fn count_to_basement(input: &str) -> i32 {
    let mut count = 0;
    let mut i = 0;
    for char in input.chars() {
        i += 1;
        if char == '(' {
            count += 1;
        } else {
            count -= 1;
        }
        if count == -1 {
            return i;
        }
    }
    0
}

// Hello world in rust
fn main() {
    // read file
    let input = std::fs::read_to_string("day1_input.txt").unwrap();

    println!("Part 2: {}", count_to_basement(&input));
}
