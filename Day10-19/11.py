from itertools import product


def iterate1(grid):
    n, m = len(grid), len(grid[0])
    next_grid = [['' for _ in range(m)] for _ in range(n)]
    changed = False
    for (x, y) in product(range(n), range(m)):
        neighbors = 0
        for (nx, ny) in [(x - 1, y), (x - 1, y - 1), (x - 1, y + 1), (x + 1, y), (x + 1, y - 1), (x + 1, y + 1), (x, y - 1), (x, y + 1)]:
            if (0 <= nx < n) and (0 <= ny < m) and grid[nx][ny] == '#':
                neighbors += 1
        state = grid[x][y]
        if state == 'L' and neighbors == 0:
            state = '#'
            changed = True
        elif state == '#' and neighbors >= 4:
            state = 'L'
            changed = True
        next_grid[x][y] = state
    return next_grid, changed


def iterate2(grid):
    n, m = len(grid), len(grid[0])
    next_grid = [['' for _ in range(m)] for _ in range(n)]
    changed = False
    for (x, y) in product(range(n), range(m)):
        neighbors = 0
        for (dy, dx) in product(range(-1, 2), range(-1, 2)):
            cx, cy = x, y
            while (dx, dy) != (0, 0) and (0 <= (cx + dx) < n) and (0 <= (cy + dy) < m):
                cx, cy = cx + dx, cy + dy
                if grid[cx][cy] == '#':
                    neighbors += 1
                    break
                elif grid[cx][cy] == 'L':
                    break
        state = grid[x][y]
        if state == 'L' and neighbors == 0:
            state = '#'
            changed = True
        elif state == '#' and neighbors >= 5:
            state = 'L'
            changed = True
        next_grid[x][y] = state
    return next_grid, changed


if __name__ == "__main__":
    with open("11.txt") as f:
        orig = [l.strip() for l in f.readlines()]

    state = orig
    while True:
        state, changed = iterate1(state)
        if not changed:
            break
    print(f"Part 1: {sum(r.count('#') for r in state)}")

    state = orig
    while True:
        state, changed = iterate2(state)
        if not changed:
            break
    print(f"Part 2: {sum(r.count('#') for r in state)}")
