import math

data = open("input.txt").read()[:-1]
ins="".join([m[0]*int(m[1]) for m in [[y for y in x.split()] for x in data.split('\n')]])

def move(H, T):
    NT = (T[0], T[1])
    if (math.sqrt((H[0] - T[0]) ** 2 + (H[1] - T[1]) ** 2)) < 2:
        return H,NT

    if H[0] == T[0]:
        if H[1] > T[1]:
            NT = (T[0], T[1] + 1)
        elif H[1] < T[1]:
            NT = (T[0], T[1] - 1)

    if H[1] == T[1]:
        if H[0] > T[0]:
            NT = (T[0] + 1, T[1])
        elif H[0] < T[0]:
            NT = (T[0] - 1, T[1])


    if H[0] > T[0] and H[1] > T[1]:
        NT = (T[0] + 1, T[1] + 1)
    if H[0] < T[0] and H[1] > T[1]:
        NT = (T[0] - 1, T[1] + 1)
    if H[0] > T[0] and H[1] < T[1]:
        NT = (T[0] + 1, T[1] - 1)
    if H[0] < T[0] and H[1] < T[1]:
        NT = (T[0] - 1, T[1] - 1)

    return H, NT


H = T = (0,0)
visited = set()
for dir in ins:
    if dir == 'R':
        H = (H[0]+1, H[1])
    elif dir == 'U':
        H = (H[0], H[1]+1)
    elif dir == 'L':
        H = (H[0]-1, H[1])
    elif dir == 'D':
        H = (H[0], H[1]-1)
    H,T = move(H, T)
    visited.add(T)
print("part1: ", len(set(visited)))


def pairwise(t):
    it = iter(t)
    it2 = iter(t)
    next(it2)
    return zip(it,it2)

rope = [(0,0)]*10
visited = set()
for dir in ins:
    if dir == 'R':
        rope[0] = (rope[0][0]+1, rope[0][1])
    elif dir == 'U':
        rope[0] = (rope[0][0], rope[0][1]+1)
    elif dir == 'L':
        rope[0] = (rope[0][0]-1, rope[0][1])
    elif dir == 'D':
        rope[0] = (rope[0][0], rope[0][1]-1)

    for i,j in pairwise(range(0,len(rope))):
        _, rope[j] = move(rope[i], rope[j])
        if j == len(rope)-1:
            visited.add(rope[j])

print("part2: ", len(visited))
