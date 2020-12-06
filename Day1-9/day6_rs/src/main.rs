#![feature(iterator_fold_self)]
use std::collections::HashSet;

fn main() {
    let groups = include_str!("6.txt")
        .split_terminator("\n\n")
        .collect::<Vec<_>>();

    let part1: usize = groups
        .iter()
        .map(|g| g.replace("\n", "").chars().collect::<HashSet<_>>().len())
        .sum();
    println!("Part 1: {}", part1);

    let part2: usize = groups
        .iter()
        .map(|g| {
            g.trim_end_matches("\n")
                .split("\n")
                .map(|s| s.chars().collect::<HashSet<_>>())
                .fold_first(|s1, s2| s1.intersection(&s2).cloned().collect())
                .unwrap()
                .len()
        })
        .sum();
    println!("Part 2: {}", part2);
}
