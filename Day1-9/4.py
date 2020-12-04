import re


def num_range(s, lo, hi):
    try:
        val = int(re.match(r'\d+', s).group(0))
    except:
        return False
    return lo <= val <= hi


fields = {'byr': lambda f: bool(re.match(r'^\d{4}$', f)) and num_range(f, 1920, 2002),
          'iyr': lambda f: bool(re.match(r'^\d{4}$', f)) and num_range(f, 2010, 2020),
          'eyr': lambda f: bool(re.match(r'^\d{4}$', f)) and num_range(f, 2020, 2030),
          'hgt': lambda f: (bool(re.match(r'^\d+cm$', f)) and num_range(f, 150, 193)) or (bool(re.match(r'\d+in$', f) and num_range(f, 59, 76))),
          'hcl': lambda f: bool(re.match(r'^#[0-9a-f]{6}$', f)),
          'ecl': lambda f: f in ('amb', 'blu', 'brn', 'gry', 'grn', 'hzl', 'oth'),
          'pid': lambda f: bool(re.match(r'^\d{9}$', f))}


def is_valid(passport, part2=False):
    for f in fields:
        match = re.search(r'{}:(.+?)(\s|$)'.format(f), passport)
        if not match:
            return False
        if part2 and not fields[f](match.group(1)):
            return False
    return True


if __name__ == "__main__":
    with open("4.txt") as f:
        passports = f.read().split('\n\n')

    part1 = sum(1 for p in passports if is_valid(p))
    print(f"Part 1: {part1}")

    part2 = sum(1 for p in passports if is_valid(p, part2=True))
    print(f"Part 2: {part2}")
