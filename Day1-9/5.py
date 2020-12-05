def determine_range(line, hi):
    lo = 0
    for c in line:
        mid = (lo + hi) // 2
        if c in ('F', 'L'):
            hi = mid
        elif c in ('B', 'R'):
            lo = mid
        else:
            raise Exception(f"unknown: {c}")
    return max(mid, hi)


if __name__ == "__main__":
    with open("5.txt") as f:
        inputs = [l.strip() for l in f.readlines()]

    seat_ids = set()
    for line in inputs:
        row = determine_range(line[:7], 127)
        col = determine_range(line[7:], 7)
        seat_id = row * 8 + col
        seat_ids.add(seat_id)
    min_seat_id, max_seat_id = min(seat_ids), max(seat_ids)
    lo, hi = min_seat_id, max_seat_id

    my_seat_id = None
    for s in range(lo + 1, hi):
        if s not in seat_ids and s - 1 in seat_ids and s + 1 in seat_ids:
            my_seat_id = s
            break

    print(f"Part 1: {max_seat_id}")
    print(f"Part 2: {my_seat_id}")
