def nth_number(seed, nth):
    last_spoken = {n: idx for idx, n in enumerate(seed[:-1])}
    most_recent = seed[-1]
    for idx in range(len(nums) - 1, nth - 1):

        if most_recent in last_spoken:
            spoken = idx - last_spoken[most_recent]
        else:
            spoken = 0
        last_spoken[most_recent] = idx
        most_recent = spoken
    return most_recent


if __name__ == "__main__":
    with open("15.txt") as f:
        nums = [int(i) for i in f.read().split(',')]

    print(f"Part 1: {nth_number(nums, 2020)}")
    print(f"Part 2: {nth_number(nums, 30000000)}")
