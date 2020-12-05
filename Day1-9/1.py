def part1(nums):
    d = {}
    for n in lines:
        d[n] = True

    for n in lines:
        if 2020 - n not in d:
            continue
        return n * (2020 - n)


def part2(nums):
    d = {}
    for n1 in nums:
        for n2 in nums:
            d[2020 - (n1 + n2)] = (n1, n2)

    for n in nums:
        if n not in d:
            continue
        n1, n2 = d[n]
        return n1 * n2 * n


if __name__ == "__main__":
    with open("1.txt") as f:
        lines = [int(l) for l in f.readlines()]
    p1 = part1(lines)
    print("Part 1: {}".format(p1))
    p2 = part2(lines)
    print("Part 2: {}".format(p2))
