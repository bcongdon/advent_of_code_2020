#![feature(entry_insert)]
use std::collections::hash_map::Entry;
use std::collections::HashMap;

fn nth_spoken_number(seed: &[u64], nth: usize) -> u64 {
    let mut last_spoken = seed[0..seed.len() - 1]
        .iter()
        .enumerate()
        .map(|(idx, val)| (*val, idx))
        .collect::<HashMap<u64, usize>>();
    let mut most_recent = seed[seed.len() - 1];
    for idx in (seed.len() - 1)..(nth - 1) {
        let entry = last_spoken.entry(most_recent);
        let spoken = if let Entry::Occupied(ref most_recent_time) = entry {
            idx - most_recent_time.get()
        } else {
            0
        };
        entry.insert(idx);
        most_recent = spoken as u64;
    }
    most_recent
}

fn main() {
    let seed = vec![8, 13, 1, 0, 18, 9];
    println!("Part 1: {}", nth_spoken_number(&seed, 2020));
    println!("Part 1: {}", nth_spoken_number(&seed, 30000000));
}
