import operator
from collections import deque


def eval(line):
    acc = None
    op = None
    idx = 0
    while idx < len(line):
        val = None
        c = line[idx]
        idx += 1
        if c == ")":
            return acc, idx
        elif c == "(":
            val, used = eval(line[idx:])
            idx += used
        elif c == "+":
            op = operator.add
            continue
        elif c == "*":
            op = operator.mul
            continue
        else:
            val = int(c)

        if acc is None:
            acc = val
        else:
            assert op is not None
            acc = op(acc, val)
    return acc, idx


def eval2(line):
    out = []
    op_stack = []
    for c in line:
        if c == ")":
            while op_stack and op_stack[-1] != "(":
                out.append(op_stack.pop())
            if op_stack and op_stack[-1] == "(":
                op_stack.pop()
        elif c == "(":
            op_stack.append(c)
        elif c == "+":
            op_stack.append(c)
        elif c == "*":
            while op_stack and op_stack[-1] == "+":
                out.append(op_stack.pop())
            op_stack.append(c)
        else:
            val = int(c)
            out.append(val)
    while op_stack:
        out.append(op_stack.pop())
    stack = []
    for tok in out:
        if type(tok) == int:
            stack.append(tok)
        elif tok == "+":
            a, b = stack.pop(), stack.pop()
            stack.append(a + b)
        elif tok == "*":
            a, b = stack.pop(), stack.pop()
            stack.append(a * b)
    return stack[0]


if __name__ == "__main__":
    with open("18.txt") as f:
        lines = [l.replace(" ", "").strip() for l in f.readlines()]
    print(f"Part 1: {sum(eval(l)[0] for l in lines)}")
    print(f"Part 2: {sum(eval2(l) for l in lines)}")
