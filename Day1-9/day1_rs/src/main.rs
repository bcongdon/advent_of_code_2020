use itertools::Itertools;
use std::collections::{HashMap, HashSet};

fn part1(nums: &[i32]) -> i32 {
    let lookup = nums.iter().map(|&x| 2020 - x).collect::<HashSet<_>>();
    nums.iter()
        .filter_map(|l| {
            if lookup.contains(l) {
                Some(l * (2020 - l))
            } else {
                None
            }
        })
        .next()
        .unwrap()
}

fn part2(nums: &[i32]) -> i32 {
    let lookup = nums
        .iter()
        .cartesian_product(nums.iter())
        .map(|(&x, &y)| (2020 - x - y, (x, y)))
        .collect::<HashMap<_, _>>();
    nums.iter()
        .filter_map(|&l| match lookup.get(&l) {
            Some((y, z)) => Some(l * y * z),
            _ => None,
        })
        .next()
        .unwrap()
}

fn main() {
    let input = include_str!("1.txt");

    let nums = input
        .split_terminator('\n')
        .map(|l| l.parse::<i32>().expect("Line should be a number"))
        .collect::<Vec<_>>();

    println!("Part 1: {}", part1(&nums));
    println!("Part 2: {}", part2(&nums));
}
