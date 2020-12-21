from functools import lru_cache
import re


def parse_rule(rule):
    key, rest = rule.split(":")
    if '"' in rest:
        return key, rest.strip().replace('"', "")
    else:
        return key, [i.split() for i in rest.split("|")]


if __name__ == "__main__":
    with open("19.txt") as f:
        rules_str, received_str = f.read().strip().split("\n\n")
    received = received_str.split()
    rule_list = [parse_rule(r) for r in rules_str.split("\n")]
    rules = {key: rules for key, rules in rule_list}

    @lru_cache()
    def pattern(key):
        r = rules[key]
        if type(r) == str:
            return r
        subrules = []
        for sr in r:
            subrules.append("".join(pattern(v) for v in sr))
        return "(?:" + "|".join(subrules) + ")"

    p0 = pattern("0")
    print("Part 1: " + str(sum(1 for r in received if re.fullmatch(p0, r))))

    new_pattern = f"(({pattern('42')})+)(({pattern('31')})+)"
    part2 = 0
    for r in received:
        match = re.fullmatch(new_pattern, r)
        if not match:
            continue
        if len(match.group(1)) // len(match.group(2)) > len(match.group(3)) // len(
            match.group(4)
        ):
            part2 += 1
    print(f"Part 2: {part2}")
