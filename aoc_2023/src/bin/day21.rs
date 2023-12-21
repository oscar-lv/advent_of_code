use std::collections::HashSet;
use std::time::Instant;

fn main() {
    // read file
    let input = std::fs::read_to_string("src/bin/day21_input.txt").unwrap();

    // start timer
    let start = Instant::now();

    // count and print
    println!("Part 1: {}", part1(&input, 64));

    // print time taken by part1
    println!("Time taken by Part 1: {:?}", start.elapsed());

    // reset timer
    let start = Instant::now();

    println!("Part 2: {}", part2(&input, 26501365));

    // print time taken by part2
    println!("Time taken by Part 2: {:?}", start.elapsed());
}

fn part2(map_str: &str, steps: usize) -> usize {
    let map: Vec<Vec<char>> = map_str
        .trim()
        .lines()
        .map(|line| line.chars().collect())
        .collect();
    let start_pos = find_start(&map).unwrap();

    let map_length = map.len();
    let grid_size = steps / map_length - 1;

    let even_maps_in_grid = ((grid_size + 1) / 2 * 2).pow(2);
    let odd_maps_in_grid = (grid_size / 2 * 2 + 1).pow(2);

    let odd_points_in_map = get_positions_after_steps(&map, start_pos, map_length * 2 + 1);
    let even_points_in_map = get_positions_after_steps(&map, start_pos, map_length * 2);

    let total_points_inside_map =
        odd_points_in_map.len() * odd_maps_in_grid + even_points_in_map.len() * even_maps_in_grid;

    let corner_top = get_positions_after_steps(&map, (map_length - 1, start_pos.1), map_length - 1);
    let corner_right = get_positions_after_steps(&map, (start_pos.0, 0), map_length - 1);
    let corner_bottom = get_positions_after_steps(&map, (0, start_pos.1), map_length - 1);
    let corner_left =
        get_positions_after_steps(&map, (start_pos.0, map_length - 1), map_length - 1);

    let total_points_in_corners =
        corner_top.len() + corner_bottom.len() + corner_left.len() + corner_right.len();

    let minor_diagonal_top_right =
        get_positions_after_steps(&map, (map_length - 1, 0), map_length / 2 - 1);
    let minor_diagonal_bottom_right = get_positions_after_steps(&map, (0, 0), map_length / 2 - 1);
    let minor_diagonal_bottom_left =
        get_positions_after_steps(&map, (0, map_length - 1), map_length / 2 - 1);
    let minor_diagonal_top_left =
        get_positions_after_steps(&map, (map_length - 1, map_length - 1), map_length / 2 - 1);

    let total_points_in_minor_diagonals = (grid_size + 1)
        * (minor_diagonal_top_right.len()
            + minor_diagonal_bottom_left.len()
            + minor_diagonal_top_left.len()
            + minor_diagonal_bottom_right.len());

    let major_diagonal_top_right =
        get_positions_after_steps(&map, (map_length - 1, 0), map_length * 3 / 2 - 1);
    let major_diagonal_bottom_right =
        get_positions_after_steps(&map, (0, 0), map_length * 3 / 2 - 1);
    let major_diagonal_bottom_left =
        get_positions_after_steps(&map, (0, map_length - 1), map_length * 3 / 2 - 1);
    let major_diagonal_top_left = get_positions_after_steps(
        &map,
        (map_length - 1, map_length - 1),
        map_length * 3 / 2 - 1,
    );

    let total_points_in_major_diagonals = grid_size
        * (major_diagonal_top_right.len()
            + major_diagonal_bottom_left.len()
            + major_diagonal_top_left.len()
            + major_diagonal_bottom_right.len());

    total_points_in_corners
        + total_points_in_minor_diagonals
        + total_points_in_major_diagonals
        + total_points_inside_map
}

fn part1(map_str: &str, steps: usize) -> usize {
    let map: Vec<Vec<char>> = map_str
        .trim()
        .lines()
        .map(|line| line.chars().collect())
        .collect();
    let start_pos = find_start(&map).unwrap();
    get_positions_after_steps(&map, start_pos, steps).len()
}

fn get_positions_after_steps(
    map: &[Vec<char>],
    start_pos: (usize, usize),
    steps: usize,
) -> HashSet<(usize, usize)> {
    let mut positions = HashSet::new();
    positions.insert(start_pos);

    for _ in 0..steps {
        let mut new_positions = HashSet::new();
        for pos in positions {
            let (x, y) = pos;
            if x > 0 && map[x - 1][y] != '#' {
                new_positions.insert((x - 1, y));
            }
            if x < map.len() - 1 && map[x + 1][y] != '#' {
                new_positions.insert((x + 1, y));
            }
            if y > 0 && map[x][y - 1] != '#' {
                new_positions.insert((x, y - 1));
            }
            if y < map[0].len() - 1 && map[x][y + 1] != '#' {
                new_positions.insert((x, y + 1));
            }
        }
        positions = new_positions;
    }
    positions
}

fn find_start(map: &[Vec<char>]) -> Option<(usize, usize)> {
    for (x, row) in map.iter().enumerate() {
        for (y, &c) in row.iter().enumerate() {
            if c == 'S' {
                return Some((x, y));
            }
        }
    }
    None
}

#[cfg(test)]
mod day21 {
    use super::*;
    #[test]
    fn test_part_1() {
        let test_input = std::fs::read_to_string("src/bin/day21_test.txt").unwrap();
        let result = part1(&test_input, 6);
        assert_eq!(result, 16);
    }
    #[test]
    fn test_part_2() {
        let test_input = std::fs::read_to_string("src/bin/day21_test.txt").unwrap();
        let result = part2(&test_input, 6);
        assert_eq!(result, 16);
    }
}
