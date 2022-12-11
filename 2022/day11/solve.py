import copy

class Monkey:
    def __init__(self, id, init, fn, test, dest_true, dest_false):
        self.id = id
        self.items = init
        self.fn = fn
        self.test = test
        self.dest_true = dest_true
        self.inspected = 0
        self.dest_false = dest_false

    def move(self, div=None):
        LCM = 2 * 13 * 5 * 3 * 11 * 17 * 7 * 19

        for item in self.items:
            self.inspected += 1
            worry = self.fn(item)
            if div:
                worry = worry//3
            else:
                worry %= LCM
            if worry%self.test == 0:
                yield (worry, self.dest_true)
            else:
                yield (worry, self.dest_false)
        self.items = []
        return

sample = [
    Monkey(0, [79.0,98.0],lambda x: x*19, 23, 2, 3),
    Monkey(1, [54.0,65.0,75.0,74.0],lambda x: x+6, 19, 2, 0),
    Monkey(2, [79.0,60.0,97.0],lambda x: x*x, 13, 1, 3),
    Monkey(3, [74.0],lambda x: x+3, 17, 0, 1),
]

data = [
    Monkey(0, [92, 73, 86, 83, 65, 51, 55, 93],lambda x: x*5, 11, 3, 4),
    Monkey(1, [99, 67, 62, 61, 59, 98],lambda x: x*x, 2, 6, 7),
    Monkey(2, [81, 89, 56, 61, 99],lambda x: x*7, 5, 1, 5),
    Monkey(3, [97, 74, 68],lambda x: x+1, 17, 2, 5),
    Monkey(4, [78,73],lambda x: x+3, 19, 2, 3),
    Monkey(5, [50],lambda x: x+5, 7, 1, 6),
    Monkey(6, [95, 88, 53, 75],lambda x: x+8, 3, 0, 7),
    Monkey(7, [50, 77, 98, 85, 94, 56, 89],lambda x: x+2, 13, 4, 0),
]

def solve(n, monkeys, div=None):
    for _ in range(n):
        for monkey in monkeys:
            for (item, dest) in monkey.move(div=div):
                monkeys[dest].items.append(item)

    moves = list(reversed(sorted([x.inspected for x in monkeys])))
    return moves[0]*moves[1]

m1 = copy.deepcopy(data)
print("part1: ", solve(20, m1, 3))

m2 = copy.deepcopy(data)
print("part2: ", solve(10_000, m2))
