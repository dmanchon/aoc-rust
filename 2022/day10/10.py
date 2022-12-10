import itertools

data = open("input.txt").read()
values = [1]


def parse(line):
    if line == "noop":
        return None
    else:
        _, x = line.split(" ")
        return int(x)


for instruction in map(parse, data.splitlines()):
    if instruction:
        values.extend([0, instruction])
    else:
        values.append(0)

acc = list(itertools.accumulate(values, lambda x, y: x + y))
print("part1: ", sum([acc[i - 1] * i for i in [20, 60, 100, 140, 180, 220]]))


grid = [abs(x % 40 - i % 40) < 2 for i, x in enumerate(acc)]
print("part2:", end="")
for i in range(40 * 6):
    if i % 40 == 0:
        print("\n", end="")
    print("#" if grid[i - 1] else ".", end="")
