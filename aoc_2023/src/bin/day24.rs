use itertools::Itertools;
use std::time::Instant;
use z3::ast::{Ast, Int};

fn main() {
    // read file
    let input = std::fs::read_to_string("src/bin/day24_input.txt").unwrap();

    // start timer
    let start = Instant::now();

    // count and print
    println!(
        "Part 1: {}",
        part1(&input, 200000000000000.0, 400000000000000.0)
    );

    // print time taken by part1
    println!("Time taken by Part 1: {:?}", start.elapsed());

    // reset timer
    let start = Instant::now();

    println!("Part 2: {}", part2(&input));

    // print time taken by part2
    println!("Time taken by Part 2: {:?}", start.elapsed());
}

fn parse(input: &str) -> Vec<((f64, f64, f64), (f64, f64, f64))> {
    input
        .split('\n')
        .map(|l| {
            let (a, b) = l.split_once(" @ ").unwrap();
            let (x, y, z) = a
                .split(", ")
                .map(|w| w.parse::<f64>().unwrap())
                .collect_tuple()
                .unwrap();
            let (dx, dy, dz) = b
                .split(", ")
                .map(|w| w.trim().parse::<f64>().unwrap())
                .collect_tuple()
                .unwrap();
            ((x, y, z), (dx, dy, dz))
        })
        .collect::<Vec<_>>()
}

fn intersection(
    (x1, y1, dx1, dy1): (f64, f64, f64, f64),
    (x2, y2, dx2, dy2): (f64, f64, f64, f64),
) -> Option<(f64, f64)> {
    let slope_1 = dy1 / dx1;
    let slope_2 = dy2 / dx2;
    if (slope_2 - slope_1).abs() < 0.0001 {
        return None;
    }
    let x = (slope_1 * x1 - slope_2 * x2 + y2 - y1) / (slope_1 - slope_2);
    let y = slope_1 * (x - x1) + y1;
    Some((x, y))
}

fn part1(input: &str, range_lower: f64, range_upper: f64) -> usize {
    let lines = parse(input);
    let range = range_lower..range_upper;

    lines
        .iter()
        .tuple_combinations()
        .filter(
            |(&((x1, y1, _), (dx1, dy1, _)), &((x2, y2, _), (dx2, dy2, _)))| {
                let Some((x, y)) = intersection((x1, y1, dx1, dy1), (x2, y2, dx2, dy2)) else {
                    return false;
                };
                if dx1.signum() != (x - x1).signum() || dx2.signum() != (x - x2).signum() {
                    return false;
                }
                range.contains(&x) && range.contains(&y)
            },
        )
        .count()
}
fn part2(input: &str) -> i64 {
    let lines = parse(input);
    let ctx = z3::Context::new(&z3::Config::new());
    let s = z3::Solver::new(&ctx);
    let [fx, fy, fz, fdx, fdy, fdz] =
        ["fx", "fy", "fz", "fdx", "fdy", "fdz"].map(|v| Int::new_const(&ctx, v));

    let zero = Int::from_i64(&ctx, 0);
    for (i, &((x, y, z), (dx, dy, dz))) in lines.iter().enumerate() {
        if i == 3 {
            break;
        }
        let [x, y, z, dx, dy, dz] = [x, y, z, dx, dy, dz].map(|v| Int::from_i64(&ctx, v as _));
        let t = Int::new_const(&ctx, format!("t{i}"));
        s.assert(&t.ge(&zero));
        s.assert(&((&x + &dx * &t)._eq(&(&fx + &fdx * &t))));
        s.assert(&((&y + &dy * &t)._eq(&(&fy + &fdy * &t))));
        s.assert(&((&z + &dz * &t)._eq(&(&fz + &fdz * &t))));
    }
    assert_eq!(s.check(), z3::SatResult::Sat);
    let model = s.get_model().unwrap();
    let res = model.eval(&(&fx + &fy + &fz)).unwrap();
    res.as_i64().unwrap()
}

#[cfg(test)]
mod day24 {
    use super::*;
    #[test]
    fn test_part_1() {
        let test_input = std::fs::read_to_string("src/bin/day24_test.txt").unwrap();
        let result = part1(&test_input, 7.0, 27.0);
        assert_eq!(result, 2);
    }
    #[test]
    fn test_part_2() {
        let test_input = std::fs::read_to_string("src/bin/day24_test.txt").unwrap();
        let result = part2(&test_input);
        assert_eq!(result, 47);
    }
}
