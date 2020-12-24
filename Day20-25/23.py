def minus_one_with_wrap(i, N=9):
    if i == 1:
        return N
    else:
        return i - 1


def play(orig, num_moves):
    cups = list(orig)
    for _ in range(100):
        current, picked_up, rest = cups[0], cups[1:4], cups[4:]
        dest = minus_one_with_wrap(current)
        while dest in picked_up:
            dest = minus_one_with_wrap(dest)
        dest_idx = rest.index(dest)
        cups = rest[: dest_idx + 1] + picked_up + rest[dest_idx + 1 :] + [current]
    idx = cups.index(1)
    return cups[idx:] + cups[:idx]


def play_fast(orig, num_moves):
    cups = [None for _ in range(len(orig) + 1)]
    cups[0] = orig[0]
    for curr, nxt in zip(orig, orig[1:]):
        cups[curr] = nxt
    cups[nxt] = cups[0]

    for _ in range(num_moves):
        head = cups[cups[0]]
        tail = head
        picked_up = []
        for _ in range(3):
            picked_up.append(tail)
            tail = cups[tail]
        cups[cups[0]] = tail

        dest = minus_one_with_wrap(cups[0], len(cups) - 1)
        while dest in picked_up:
            dest = minus_one_with_wrap(dest, len(cups) - 1)

        cups[picked_up[-1]] = cups[dest]
        cups[dest] = head
        cups[0] = cups[cups[0]]

    out = []
    nxt = 1
    for _ in range(len(cups) - 1):
        out.append(nxt)
        nxt = cups[nxt]
    return out


if __name__ == "__main__":
    inp = "792845136"
    # inp = '389125467'
    orig = [int(i) for i in inp]
    N = len(orig)

    print("Part 1: " + "".join([str(c) for c in play(orig, 100)][1:]))

    p2_result = play_fast(orig + list(range(10, 1000001)), 10000000)
    print("Part 2: {}".format(p2_result[1] * p2_result[2]))
