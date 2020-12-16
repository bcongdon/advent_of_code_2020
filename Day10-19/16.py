import re
from functools import reduce


def completely_invalid_sum(defs, ticket):
    invalid, s = False, 0
    for val in ticket:
        found = False
        for ((x1, x2), (y1, y2)) in defs:
            if (x1 <= val <= x2) or (y1 <= val <= y2):
                found = True
                break
        if not found:
            s += val
            invalid = True
    return invalid, s


def field_sets(defs, ticket):
    valid_fields = []
    for val in ticket:
        s = set(range(len(defs)))
        for (idx, ((x1, x2), (y1, y2))) in enumerate(defs):
            if (not (x1 <= val <= x2)) and (not (y1 <= val <= y2)):
                s.remove(idx)
        assert len(s) > 0
        valid_fields.append(s)
    return valid_fields


if __name__ == "__main__":
    with open("16.txt") as f:
        defs_raw, raw_ticket, nearby = f.read().split("\n\n")
    nearby = [[int(i) for i in l.split(",")] for l in nearby.strip().split("\n")[1:]]

    defs = []
    for f in defs_raw.strip().split("\n"):
        m = re.match(".*: (\d+)-(\d+) or (\d+)-(\d+)", f)
        if not m:
            continue
        defs.append(((int(m[1]), int(m[2])), (int(m[3]), int(m[4]))))
    print(f"Part 1: {sum(completely_invalid_sum(defs, t)[1] for t in nearby)}")

    nearby = [t for t in nearby if not completely_invalid_sum(defs, t)[0]]
    valid_fields = [set(range(len(defs))) for _ in range(len(defs))]
    for t in nearby:
        for idx, fields in enumerate(field_sets(defs, t)):
            valid_fields[idx] &= fields

    field_map = dict()
    while len(field_map) != len(defs):
        idx, singular = next(
            (idx, s) for idx, s in enumerate(valid_fields) if len(s) == 1
        )
        val = list(singular)[0]
        for s in valid_fields:
            s.discard(val)
        field_map[val] = idx

    ticket = [int(i) for i in raw_ticket.split("\n")[1].split(",")]
    part2 = reduce((lambda x, y: x * y), [ticket[field_map[i]] for i in range(6)])
    print(f"Part 2: {part2}")
