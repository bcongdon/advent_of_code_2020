def determine_loop(subject, key):
    val = 1
    loop = 0
    while True:
        loop += 1
        val = (val * subject) % 20_201_227
        if val == key:
            return loop


def transform(subject, loop):
    val = 1
    for _ in range(loop):
        val = (val * subject) % 20_201_227
    return val


if __name__ == "__main__":
    door_pub = 12_578_151
    card_pub = 5_051_300

    card_loop = determine_loop(7, card_pub)
    print(f"Part 1: {transform(door_pub, card_loop)}")
