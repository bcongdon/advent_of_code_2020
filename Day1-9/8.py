def run(instructions, swp_index=-1):
    acc, pc = 0, 0
    ran = set()
    while pc < len(instructions):
        if pc in ran:
            return (acc, pc)
        ran.add(pc)
        cmd, offset = instructions[pc]
        if pc == swp_index:
            if cmd == 'nop':
                cmd = 'jmp'
            elif cmd == 'jmp':
                cmd = 'nop'
        if cmd == 'nop':
            pc += 1
        elif cmd == 'jmp':
            pc += offset
        elif cmd == 'acc':
            acc += offset
            pc += 1
    return (acc, pc)


if __name__ == "__main__":
    with open("8.txt") as f:
        inputs = [l.strip() for l in f.readlines()]

    instructions = []
    for (cmd, offset) in [i.split() for i in inputs]:
        instructions.append((cmd, int(offset)))
    part1, _ = run(instructions)
    print(f"Part 1: {part1}")

    for i in range(len(inputs)):
        part2, pc = run(instructions, i)
        if pc == len(inputs):
            print(f"Part 2: {part2}")
