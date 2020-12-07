from functools import reduce

if __name__ == "__main__":
    with open("6.txt") as f:
        groups = f.read().split("\n\n")

    part1 = sum(len(set(g.replace("\n", ""))) for g in groups)
    print(f"Part 1: {part1}")

    sets = [[set(s) for s in g.strip().split("\n")] for g in groups]
    part2 = sum(len(reduce(lambda x, y: x & y, s)) for s in sets)
    print(f"Part 2: {part2}")
