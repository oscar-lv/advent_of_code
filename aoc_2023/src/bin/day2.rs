use std::collections::HashMap;

fn main() {
    // read file
    let input = std::fs::read_to_string("day2_input.txt").unwrap();

    // count and print
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

// Part 1
fn part1(input: &str) -> i32 {
    let mut count: i32 = 0;
    let map: HashMap<String, i32> = [
        ("red".to_string(), 12),
        ("blue".to_string(), 14),
        ("green".to_string(), 13),
    ]
    .iter()
    .cloned()
    .collect();
    // Loop through games
    input.lines().for_each(|game| {
        let game_split: Vec<&str> = game.trim().split(":").collect();
        let game_id: i32 = game_split[0]
            .replace("Game ", "")
            .replace(":", "")
            .parse()
            .unwrap_or(0);

        let replaced = game_split[1].replace(";", ",");
        let colors: Vec<&str> = replaced.split(",").collect();
        let mut add_idx: bool = true;
        for color in colors {
            let color = color.trim();
            let elements: Vec<&str> = color.split(" ").collect();
            let number: i32 = elements[0].parse().unwrap();
            let c_string: &str = elements[1];
            if let Some(&map_value) = map.get(c_string) {
                if number > map_value {
                    add_idx = false;
                }
            }
        }
        if add_idx {
            count += game_id;
        }
    });
    count
}

// Part 2
fn part2(input: &str) -> i32 {
    let mut count: i32 = 0;
    // Loop through games
    input.lines().for_each(|game| {
        let mut map: HashMap<String, i32> = HashMap::new();
        map.insert("red".to_string(), 0);
        map.insert("blue".to_string(), 0);
        map.insert("green".to_string(), 0);

        let game_split: Vec<&str> = game.trim().split(":").collect();
        let replaced = game_split[1].replace(";", ",");
        let colors: Vec<&str> = replaced.split(",").collect();
        for color in colors {
            let color = color.trim();
            let elements: Vec<&str> = color.split(" ").collect();
            let number: i32 = elements[0].parse().unwrap();
            let c_string: &str = elements[1];
            if let Some(&map_value) = map.get(c_string) {
                if number > map_value {
                    map.insert(c_string.to_string(), number);
                }
            }
        }
        count += map.get("red").unwrap_or(&0)
            * map.get("green").unwrap_or(&0)
            * map.get("blue").unwrap_or(&0);
    });
    count
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part_1() {
        let test_input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
                                Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
                                Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
                                Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
                                Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"
            .to_string();
        let result = part1(&test_input);
        assert_eq!(result, 8);
    }
    #[test]
    fn test_part_2() {
        let test_input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
                                Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
                                Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
                                Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
                                Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"
            .to_string();
        let result = part2(&test_input);
        assert_eq!(result, 2286);
    }
}
