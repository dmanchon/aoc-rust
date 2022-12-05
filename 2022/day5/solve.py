import copy

state, instructions = open("input.txt").read().split("\n\n")
_stacks = []
for i in [x*4 + 1 for x in range(9)]:
    s = [x[i] for x in state.split("\n")[:-1]]
    s.reverse()
    _stacks.append(list(filter(lambda c: c != " ", s)))

moves = [
    (x, y, z)
    for (_, x, _, y, _, z) in [
        tuple(x.split(" ")) for x in [x for x in instructions.split("\n")]
    ][:-1]
]

# part1
stacks = copy.deepcopy(_stacks)
for (n, _from, _to) in moves:
    for _ in range(int(n)):
        stacks[int(_to) - 1].extend(stacks[int(_from) - 1].pop())

print("part1: ", "".join([x[-1] for x in stacks]))

# part2
stacks = copy.deepcopy(_stacks)
for (n, _from, _to) in moves:
    N = int(n)
    elements = stacks[int(_from) - 1][-N:]
    del stacks[int(_from) - 1][-N:]
    stacks[int(_to) - 1].extend(elements)

print("part2: ", "".join([x[-1] for x in stacks]))
