use std::collections::HashSet;

fn main() {
    // read file
    let input = std::fs::read_to_string("src/bin/input.txt").unwrap();

    // count and print
    println!("Part 2: {}", part2(&input));
}

// Improved Part 1
fn part2(input: &str) -> i32 {
    let mut santa_x: i32 = 0;
    let mut santa_y: i32 = 0;
    let mut robot_x: i32 = 0;
    let mut robot_y: i32 = 0;
    let mut positions = HashSet::new();
    positions.insert((0, 0));
    let mut santa_turn: bool = true;
    input.chars().for_each(|char| {
        // move santa
        if santa_turn {
            match char {
                '^' => santa_y += 1,
                'v' => santa_y -= 1,
                '>' => santa_x += 1,
                '<' => santa_x -= 1,
                _ => (),
            }
            positions.insert((santa_x, santa_y));
        } else {
            match char {
                '^' => robot_y += 1,
                'v' => robot_y -= 1,
                '>' => robot_x += 1,
                '<' => robot_x -= 1,
                _ => (),
            }
            positions.insert((robot_x, robot_y));
        }
        santa_turn = !santa_turn;
    });
    positions.len() as i32
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part_2() {
        let test_input = "^v".to_string();
        let result = part2(&test_input);
        assert_eq!(result, 3);
    }
}
