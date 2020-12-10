use std::iter;

fn part1(nums: &Vec<u64>) -> u64 {
    let (mut d1, mut d3) = (0, 0);
    for i in 1..nums.len() {
        match nums[i] - nums[i - 1] {
            1 => d1 += 1,
            3 => d3 += 1,
            _ => {}
        }
    }
    d1 * d3
}

fn part2(nums: &Vec<u64>) -> u64 {
    let mut partial = iter::repeat(0).take(nums.len()).collect::<Vec<_>>();
    let n = partial.len();
    partial[n - 1] = 1;
    for i in (0..=n - 2).rev() {
        let (mut tot, mut j) = (0, i + 1);
        while j < n && nums[j] - nums[i] <= 3 {
            tot += partial[j];
            j += 1;
        }
        partial[i] = tot;
    }
    partial[0]
}
fn main() {
    let mut input = include_str!("10.txt")
        .split_terminator("\n")
        .map(|l| l.parse::<u64>().unwrap())
        .collect::<Vec<_>>();
    input.sort();
    input.insert(0, 0);
    input.push(*input.last().unwrap() + 3);

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}
