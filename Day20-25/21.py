import re


def parse_list(raw):
    match = re.match(r'(.*) \(contains (.*)\)', raw)
    ingredients = match.group(1).split(' ')
    allergens = match.group(2).split(', ')
    return (ingredients, allergens)


if __name__ == "__main__":
    with open('21.txt') as f:
        ing_lists = [parse_list(l) for l in f.readlines()]

    foods = set()
    for fs, _ in ing_lists:
        foods.update(fs)

    allergens = dict()
    for _, alls in ing_lists:
        for a in alls:
            allergens[a] = set(foods)

    for fs, alls in ing_lists:
        fs = set(fs)
        for a in alls:
            allergens[a] &= fs

    non_allergen_foods = set(foods)
    for suspects in allergens.values():
        non_allergen_foods -= set(suspects)
    part1 = 0
    for fs, _ in ing_lists:
        part1 += sum(1 for f in fs if f in non_allergen_foods)
    print(f"Part 1: {part1}")

    allergen_map = dict()
    while allergens:
        a, (f,) = next((a, fs) for a, fs in allergens.items() if len(fs) == 1)
        for suspects in allergens.values():
            suspects.discard(f)
        allergens.pop(a)
        allergen_map[a] = f
    canonical = ','.join(ing for _, ing in sorted(allergen_map.items()))
    print(f"Part 2: {canonical}")
