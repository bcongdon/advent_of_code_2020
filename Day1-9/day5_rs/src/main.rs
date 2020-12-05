use std::collections::BTreeSet;

fn determine_range(line: &str, mut hi: usize) -> usize {
    let mut lo = 0;
    let mut mid = (lo + hi) / 2;
    for c in line.chars() {
        mid = (lo + hi) / 2;
        match c {
            'F' | 'L' => hi = mid,
            'B' | 'R' => lo = mid,
            _ => panic!("Unknown char: {}", c),
        };
    }
    if mid > hi {
        mid
    } else {
        hi
    }
}

fn main() {
    let lines = include_str!("5.txt")
        .split_terminator("\n")
        .collect::<Vec<_>>();
    let seat_ids = lines
        .iter()
        .map(|l| {
            let row = determine_range(&l[0..7], 127);
            let col = determine_range(&l[7..], 7);
            row * 8 + col
        })
        .collect::<BTreeSet<_>>();
    let (min_id, max_id) = (
        seat_ids.iter().min().unwrap(),
        seat_ids.iter().max().unwrap(),
    );
    let my_id = ((min_id + 1)..=*max_id)
        .filter(|id| !seat_ids.contains(id))
        .next()
        .unwrap();

    println!("Part 1: {}", max_id);
    println!("Part 2: {}", my_id);
}
