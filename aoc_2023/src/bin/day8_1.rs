use std::time::Instant;

pub fn main() {
    let input = include_bytes!("day8_input.txt");
    // start timer
    let start = Instant::now();

    // print time taken by part1
    let split = input.iter().position(|&c| c == b'\n').unwrap();

    let mut map = [0u64; 174763];
    let enc = |n: &[u8]| {
        ((n[0] - b'0') as u64) << 12 | ((n[1] - b'0') as u64) << 6 | (n[2] - b'0') as u64
    };
    input[split + 2..].split(|&c| c == b'\n').for_each(|node| {
        map[enc(&node[0..3]) as usize] = enc(&node[7..10]) | enc(&node[12..15]) << 32;
    });

    println!(
        "{}",
        input[0..split]
            .iter()
            .cycle()
            .scan(enc(b"AAA"), |node, step| {
                *node = if step == &b'L' {
                    map[*node as usize] & u32::MAX as u64
                } else {
                    map[*node as usize] >> 32
                };
                Some(*node & 0b111111 == (b'Z' - b'0') as u64)
            })
            .position(|node| node)
            .unwrap()
            + 1
    );
    println!("Time taken by Part 1: {:?}", start.elapsed());
}

pub fn main1() {
    let input = include_bytes!("day8_input.txt");
    let start = Instant::now();

    let split = input.iter().position(|&c| c == b'\n').unwrap();

    let (mut map, mut starts) = ([0u64; 174763], Vec::with_capacity(6));
    let enc = |n: &[u8]| {
        ((n[0] - b'0') as u64) << 12 | ((n[1] - b'0') as u64) << 6 | (n[2] - b'0') as u64
    };
    input[split + 2..].split(|&c| c == b'\n').for_each(|node| {
        map[enc(&node[0..3]) as usize] = enc(&node[7..10]) | enc(&node[12..15]) << 32;
        if node[2] == b'A' {
            starts.push(enc(&node[0..3]));
        }
    });

    println!(
        "{}",
        starts
            .into_iter()
            .map(|node| {
                input[0..split]
                    .iter()
                    .cycle()
                    .scan(node, |node, step| {
                        *node = if step == &b'L' {
                            map[*node as usize] & u32::MAX as u64
                        } else {
                            map[*node as usize] >> 32
                        };
                        Some(*node & 0b111111 == (b'Z' - b'0') as u64)
                    })
                    .position(|node| node)
                    .unwrap()
                    + 1
            })
            .fold(1, num_integer::lcm)
    );
    println!("Time taken by Part 1: {:?}", start.elapsed());
}
