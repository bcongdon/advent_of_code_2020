DIRS_OFFSETS = {"N": (0, 1), "S": (0, -1), "E": (1, 0), "W": (-1, 0)}
DIRS = ["N", "E", "S", "W"]


def navigate(directions):
    x, y = 0, 0
    cd = "E"
    for d, amt in directions:
        if d == "L":
            d, amt = "R", 360 - amt
        if d == "R":
            cd = DIRS[(DIRS.index(cd) + (amt // 90)) % len(DIRS)]
        elif d == "F":
            dx, dy = DIRS_OFFSETS[cd]
            x, y = x + amt * dx, y + amt * dy
        elif d in "NSEW":
            dx, dy = DIRS_OFFSETS[d]
            x, y = x + amt * dx, y + amt * dy
    return x, y


def navigate_waypoint(directions):
    x, y, wpx, wpy = 0, 0, 10, 1
    for d, amt in directions:
        if d == "L":
            d, amt = "R", 360 - amt
        if d == "R":
            if amt == 90:
                wpx, wpy = wpy, -wpx
            if amt == 180:
                wpx, wpy = -wpx, -wpy
            if amt == 270:
                wpx, wpy = -wpy, wpx
        elif d == "F":
            x, y = x + amt * wpx, y + amt * wpy
        elif d in "NSEW":
            dx, dy = DIRS_OFFSETS[d]
            wpx, wpy = wpx + amt * dx, wpy + amt * dy
    return x, y


if __name__ == "__main__":
    with open("12.txt") as f:
        raw = f.readlines()
    directions = [(l[0], int(l[1:])) for l in raw]
    x, y = navigate(directions)
    print(f"Part 1: {abs(x) + abs(y)}")

    x, y = navigate_waypoint(directions)
    print(f"Part 2: {abs(x) + abs(y)}")
