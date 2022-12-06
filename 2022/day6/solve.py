input = open("input.txt").read()

def solve(input, n):
    header = []
    for i, x in enumerate(input):
        header.append(x)
        if len(header) == n:
            if len(set(header)) == n:
                return i + 1
            header = header[1:]

print("part1: ", solve(input, 4))
print("part2: ", solve(input, 14))
