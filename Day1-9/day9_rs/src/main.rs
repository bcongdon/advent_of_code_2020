use itertools::Itertools;

fn first_invalid(nums: &Vec<u64>) -> u64 {
    let mut buffer = &nums[..25];
    for (i, n) in nums.iter().skip(25).enumerate() {
        if !buffer
            .iter()
            .cartesian_product(buffer.iter())
            .filter(|&(x, y)| x + y == *n)
            .next()
            .is_some()
        {
            return *n;
        }
        buffer = &nums[i + 1..i + 26];
    }
    0
}

fn subset_sum_endpoint_sum(nums: &Vec<u64>, target: u64) -> u64 {
    let (mut curr_sum, mut start) = (nums[0], 0);
    for i in 1..=nums.len() {
        while curr_sum > target && start < (i - 1) {
            curr_sum -= nums[start];
            start += 1;
        }
        if curr_sum == target {
            return nums[start] + nums[i - 1];
        } else if i < nums.len() {
            curr_sum += nums[i];
        }
    }
    0
}

fn main() {
    let input = include_str!("9.txt")
        .split_terminator("\n")
        .map(|l| l.parse::<u64>().unwrap())
        .collect::<Vec<_>>();
    let part1 = first_invalid(&input);
    println!("Part 1: {}", part1);
    let part2 = subset_sum_endpoint_sum(&input, part1);
    println!("Part 2: {}", part2);
}
