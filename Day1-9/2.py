def parse_policy(policy):
    a, b, c = policy.split()
    lo, hi = a.split("-")
    return (int(lo), int(hi), b[:-1], c)


def part1(pws):
    return sum(1 for lo, hi, c, pw in pws if lo <= pw.count(c) <= hi)


def part2(pws):
    return sum(1 for lo, hi, c, pw in pws if (pw[lo - 1] == c) != (pw[hi - 1] == c))


if __name__ == "__main__":
    with open("2.txt") as f:
        lines = f.readlines()
    pws = list(map(parse_policy, lines))
    print("Part 1: {}".format(part1(pws)))
    print("Part 2: {}".format(part2(pws)))
