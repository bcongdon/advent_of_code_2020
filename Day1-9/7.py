import re
from collections import defaultdict


def parse_bag_req(req):
    bag, rest = req.split(" bags contain ")
    if "no other" in rest:
        return (bag, [])

    children = []
    for r in re.sub(r"\.|bags?", "", rest).split(","):
        w = r.strip().split(" ")
        children.append((w[0], " ".join(w[1:])))
    return (bag, children)


if __name__ == "__main__":
    with open("7.txt") as f:
        inputs = [l.strip() for l in f.readlines()]
    parsed = [parse_bag_req(r) for r in inputs]

    graph = defaultdict(set)
    for (bag, children) in parsed:
        for _, child in children:
            graph[child].add(bag)

    can_hold = set(["shiny gold"])
    while True:
        num_held = len(can_hold)
        for c in set(can_hold):
            can_hold |= graph[c]
        if num_held == len(can_hold):
            break
    print(f"Part 1: {len(can_hold)-1}")

    graph = {bag: children for (bag, children) in parsed}

    def required_bags(bag):
        return 1 + sum(int(cost) * required_bags(child) for (cost, child) in graph[bag])

    print(f"Part 2: {required_bags('shiny gold') - 1}")
