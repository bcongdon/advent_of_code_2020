use std::collections::HashMap;

fn nth_spoken_number(seed: &[u64], nth: usize) -> u64 {
    let mut last_spoken = seed[0..seed.len() - 1]
        .iter()
        .enumerate()
        .map(|(idx, val)| (*val, idx))
        .collect::<HashMap<u64, usize>>();
    let mut most_recent = seed[seed.len() - 1];
    for idx in (seed.len() - 1)..(nth - 1) {
        let spoken = if let Some(most_recent_time) = last_spoken.get(&most_recent) {
            idx - *most_recent_time
        } else {
            0
        };
        last_spoken.insert(most_recent, idx);
        most_recent = spoken as u64;
    }
    most_recent
}

fn main() {
    let seed = vec![8, 13, 1, 0, 18, 9];
    println!("Part 1: {}", nth_spoken_number(&seed, 2020));
    println!("Part 1: {}", nth_spoken_number(&seed, 30000000));
}
