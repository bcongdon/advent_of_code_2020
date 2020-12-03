def toboggan(grid, slope):
    x, y = 0, 0
    dx, dy = slope
    trees = 0
    while y < len(grid):
        if grid[y][x % len(grid[0])] == '#':
            trees += 1
        y += dy
        x += dx
    return trees


if __name__ == "__main__":
    with open("3.txt") as f:
        lines = [l.strip() for l in f.readlines()]
    part1 = toboggan(lines, (3, 1))
    print(f"Part 1: {part1}")

    result = 1
    for slope in [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)]:
        result *= toboggan(lines, slope)
    print(f"Part 2: {result}")
