from itertools import product


def simulate(active, n, part2=True):
    for _ in range(n):
        next_active = set()
        to_visit = set()
        for x, y, z, w in active:
            neighbors = 0
            n_ranges = [
                range(x - 1, x + 2),
                range(y - 1, y + 2),
                range(z - 1, z + 2),
                (range(w - 1, w + 2) if part2 else [w]),
            ]
            for nx, ny, nz, nw in product(*n_ranges):
                if (nx, ny, nz, nw) == (x, y, z, w):
                    continue
                elif (nx, ny, nz, nw) in active:
                    neighbors += 1
                else:
                    to_visit.add((nx, ny, nz, nw))
            if neighbors in (2, 3):
                next_active.add((x, y, z, w))
        for x, y, z, w in to_visit:
            neighbors = 0
            n_ranges = [
                range(x - 1, x + 2),
                range(y - 1, y + 2),
                range(z - 1, z + 2),
                (range(w - 1, w + 2) if part2 else [w]),
            ]
            for nx, ny, nz, nw in product(*n_ranges):
                if (nx, ny, nz, nw) == (x, y, z, w):
                    continue
                elif (nx, ny, nz, nw) in active:
                    neighbors += 1
            if neighbors == 3:
                next_active.add((x, y, z, w))
        active = next_active
    return len(active)


if __name__ == "__main__":
    with open("17.txt") as f:
        grid_strs = [l.strip() for l in f.readlines()]

    active = set()
    for y, line in enumerate(grid_strs):
        for x, c in enumerate(line):
            if c == "#":
                active.add((x, y, 0, 0))
    print(f"Part 1: {simulate(active, 6, part2=False)}")
    print(f"Part 2: {simulate(active, 6, part2=True)}")
