use std::time::Instant;

fn main() {
    // read file
    let input = std::fs::read_to_string("src/bin/day15_input.txt").unwrap();

    // start timer
    let start = Instant::now();

    // count and print
    println!("Part 1: {}", part1(&input));

    // print time taken by part1
    println!("Time taken by Part 1: {:?}", start.elapsed());

    // reset timer
    let start = Instant::now();

    println!("Part 2: {}", part2(&input));

    // print time taken by part2
    println!("Time taken by Part 2: {:?}", start.elapsed());
}

struct LensBox<'s> {
    num: usize,
    slots: Vec<(&'s str, u8)>,
}

impl<'s> LensBox<'s> {
    fn new(num: usize) -> Self {
        Self {
            num,
            slots: Vec::new(),
        }
    }
    fn update(&mut self, box_label: &'s str, instruction: &str) {
        if instruction.is_empty() {
            if let Some(index) = self.slots.iter().position(|(l, _)| *l == box_label) {
                self.slots.remove(index);
            }
        } else {
            let fl = instruction.parse().unwrap();

            if let Some(f) =
                self.slots
                    .iter_mut()
                    .find_map(|(l, f)| if *l == box_label { Some(f) } else { None })
            {
                *f = fl;
            } else {
                self.slots.push((box_label, fl));
            }
        }
    }

    fn power(&self) -> usize {
        self.slots
            .iter()
            .enumerate()
            .map(|(i, lens)| (self.num + 1) * (i + 1) * lens.1 as usize)
            .sum()
    }
}

fn hash(input: &str) -> usize {
    input
        .as_bytes()
        .iter()
        .fold(0, |acc, &c| (acc + c as usize) * 17 % 256)
}

fn part1(input: &str) -> usize {
    input.split(",").map(|x| hash(x)).sum()
}

fn part2(input: &str) -> usize {
    let mut boxes = (0..256).map(LensBox::new).collect::<Vec<LensBox>>();

    for step in input.trim().split(",") {
        let (box_label, instruction) = step.split_once(['=', '-']).unwrap();
        boxes[hash(box_label)].update(box_label, instruction);
    }

    boxes
        .iter()
        .filter_map(|b| {
            if b.slots.is_empty() {
                None
            } else {
                Some(b.power())
            }
        })
        .sum()
}

#[cfg(test)]
mod day15 {
    use super::*;
    #[test]
    fn test_part_1() {
        let test_input = std::fs::read_to_string("src/bin/day15_test.txt").unwrap();
        let result = part1("HASH");
        assert_eq!(result, 52);
    }
    #[test]
    fn test_part_1_1() {
        let test_input = std::fs::read_to_string("src/bin/day15_test.txt").unwrap();
        let result = part1(&test_input);
        assert_eq!(result, 1320);
    }
    #[test]
    fn test_part_2() {
        let test_input = std::fs::read_to_string("src/bin/day15_test.txt").unwrap();
        let result = part2(&test_input);
        assert_eq!(result, 145);
    }
}
