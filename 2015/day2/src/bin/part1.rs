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
        // create array of numbers by splitting on 'x'
        let split: Vec<&str> = line.split('x').collect();
        let dim: Vec<i32> = split.iter().map(|x| x.parse::<i32>().unwrap()).collect();
        let areas: Vec<i32> = [dim[0] * dim[1], dim[1] * dim[2], dim[2] * dim[0]].to_vec();
        let min_area = areas.iter().min().unwrap();
        let total_area = 2 * areas.iter().sum::<i32>() + min_area;
        // concatenate first and last number into string,parse the combined string into i32, and add to sum
        sum += total_area;
    });
    sum
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part_1() {
        let test_input = "2x3x4".to_string();
        let result = part1(&test_input);
        assert_eq!(result, 58);
    }
}
