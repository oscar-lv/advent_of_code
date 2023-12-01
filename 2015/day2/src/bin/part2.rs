fn main() {
    // read file
    let input = std::fs::read_to_string("src/bin/input.txt").unwrap();

    // count and print
    println!("Part 2: {}", part2(&input));
}

// Improved Part 1
fn part2(input: &str) -> i32 {
    let mut sum = 0;
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
        // create array of numbers by splitting on 'x'
        let split: Vec<&str> = line.split('x').collect();
        let mut dim: Vec<i32> = split.iter().map(|x| x.parse::<i32>().unwrap()).collect();
        dim.sort();
        let shortest_perimeter = 2 * (dim[0] + dim[1]);

        let volume = dim[0] * dim[1] * dim[2];
        // concatenate first and last number into string,parse the combined string into i32, and add to sum
        sum += shortest_perimeter + volume;
    });
    sum
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part_2() {
        let test_input = "2x3x4".to_string();
        let result = part2(&test_input);
        assert_eq!(result, 34);
    }
}
