from collections import defaultdict

DIRECTIONS = {
    'e': (1, 0),
    'w': (-1, 0),
    'ne': (1, -1),
    'nw': (0, -1),
    'se': (0, 1),
    'sw': (-1, 1)
}


def parse_directions(dir_str):
    idx = 0
    out = []
    while idx < len(dir_str):
        if dir_str[idx] in "ns":
            out.append(dir_str[idx:idx + 2])
            idx += 2
        else:
            out.append(dir_str[idx])
            idx += 1
    return out


def find_relative_coord(line):
    x, y = 0, 0
    for step in parse_directions(line):
        dx, dy = DIRECTIONS[step]
        x += dx
        y += dy
    return x, y


def hex_neighbors(coord):
    return set((coord[0] + dx, coord[1] + dy) for dx, dy in DIRECTIONS.values())


if __name__ == "__main__":
    with open('24.txt') as f:
        lines = [l.strip() for l in f.readlines()]

    coords = [find_relative_coord(l) for l in lines]
    tiles = defaultdict(bool)
    for x, y in coords:
        tiles[(x, y)] = not tiles[(x, y)]
    part1 = sum(1 for v in tiles.values() if v)
    print("Part 1: {}".format(part1))

    black = set([c for c, v in tiles.items() if v])
    for day in range(100):
        new = set()
        neighbors = set()
        for tile in black:
            curr_neighbors = hex_neighbors(tile)
            if len(curr_neighbors & black) in (1, 2):
                new.add(tile)
            neighbors |= curr_neighbors
        for n in (neighbors - black):
            if len(hex_neighbors(n) & black) == 2:
                new.add(n)
        black = new
    print("Part 2: {}".format(len(black)))
