if __name__ == "__main__":
    with open("13.txt") as f:
        raw = f.readlines()
    ts = int(raw[0])
    busses = [int(b) if b != "x" else None for b in raw[1].split(",")]

    wait_times = sorted([(b - (ts % b), b) for b in busses if b])
    print(f"Part 1: {wait_times[0][0] * wait_times[0][1]}")

    busses = ((b, idx) for idx, b in enumerate(busses) if b)
    start_ts, jmp, ts = 0, 1, 0
    for (bus, offset) in sorted(busses, reverse=True):
        ts = start_ts
        while True:
            if (ts + offset) % bus == 0:
                start_ts = ts
                jmp *= bus
                break
            ts += jmp
    print(f"Part 2: {ts}")
