def score(hand):
    score = 0
    for idx, card in enumerate(hand[::-1]):
        score += (idx + 1) * card
    return score


def play_game(p1, p2):
    round_cache = set()
    while p1 and p2:
        key = ','.join([str(c) for c in p1]) + '|' + \
            ','.join([str(c) for c in p1])
        if key in round_cache:
            return 1, p1, p2
        round_cache.add(key)

        a, b, p1, p2 = p1[0], p2[0], p1[1:], p2[1:]
        if a <= len(p1) and b <= len(p2):
            p1_c, p2_c = list(p1[:a]), list(p2[:b])
            round_winner, _, _ = play_game(p1_c, p2_c)
        else:
            round_winner = 1 if a > b else 2

        assert(round_winner in (1, 2))
        if round_winner == 1:
            p1.extend([a, b])
        else:
            p2.extend([b, a])
    return 1 if not p2 else 2, p1, p2


def part1(p1, p2):
    while p1 and p2:
        a, b, p1, p2 = p1[0], p2[0], p1[1:], p2[1:]
        if a > b:
            p1.extend([a, b])
        else:
            p2.extend([b, a])
    return score(p1 if p1 else p2)


if __name__ == "__main__":
    with open('22.txt') as f:
        players = f.read().strip().split('\n\n')
    p1 = [int(i) for i in players[0].split('\n')[1:]]
    p2 = [int(i) for i in players[1].split('\n')[1:]]

    print(f"Part 1: {part1(p1, p2)}")

    _, p1, p2 = play_game(p1, p2)
    print("Part 2: " + str(score(p1) if p1 else score(p2)))
