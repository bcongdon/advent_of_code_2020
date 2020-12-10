if __name__ == "__main__":
    with open("10.txt") as f:
        nums = [int(l) for l in f.readlines()]
    s = sorted(nums)
    s = [0] + s + [s[-1] + 3]
    d1, d3 = 0, 0
    for i in range(1, len(s)):
        diff = s[i] - s[i - 1]
        if diff == 1:
            d1 += 1
        elif diff == 3:
            d3 += 1
    print(f"Part 1: {d1 * d3}")

    partial = [0 for _ in range(len(s))]
    partial[-1] = 1
    for i in range(len(s) - 2, -1, -1):
        tot = 0
        j = i + 1
        while j < len(s) and s[j] - s[i] <= 3:
            tot += partial[j]
            j += 1
        partial[i] = tot
    print(f"Part 2: {partial[0]}")
