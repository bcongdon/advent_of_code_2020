from itertools import product


def first_invalid(nums):
    buffer = nums[:25]
    for n in nums[25:]:
        if not any((x, y)
                   for (x, y) in product(buffer, buffer) if x + y == n):
            return n
        buffer = buffer[1:] + [n]


def subset_sum(nums, target):
    curr_sum, start = nums[0], 0
    for i in range(1, len(nums) + 1):
        while curr_sum > target and start < (i - 1):
            curr_sum -= nums[start]
            start += 1
        if curr_sum == target:
            return (start, i - 1)
        if i < len(nums):
            curr_sum += nums[i]


if __name__ == "__main__":
    with open("9.txt") as f:
        nums = [int(l) for l in f.readlines()]
    part1 = first_invalid(nums)
    print(f"Part 1: {part1}")
    lo, hi = subset_sum(nums, part1)
    print(f"Part 2: {nums[hi] + nums[lo]}")
