class Package:
    def __init__(self, data):
        self.data = data

    def __eq__(self, other):
        return self.data == other.data

    def __lt__(self, other):
        return other >= self

    def __ge__(self, other):
        if type(self.data) != list and type(other.data) == list:
            return Package([self.data]) >= other
        elif type(self.data) == list and type(other.data) != list:
            return self >= Package([other.data])
        elif type(self.data) == int and type(other.data) == int:
            return self.data >= other.data
        else:
            len_diff = len(self.data) - len(other.data)
            it1 = iter(self.data)
            it2 = iter(other.data)

            while True:
                try:
                    right = Package(next(it2))
                except:
                    return True

                try:
                    left = Package(next(it1))
                except:
                    return len_diff == 0
                if right == left:
                    continue
                return left >= right

data = []
for line in open("13.in").read().split("\n"):
    if line == "":
        continue
    data.append(Package(eval(line)))


acc = 0
for i, [a,b] in enumerate(zip(data[::2], data[1::2])):
    if a <= b:
        acc += (i+1)
print("part1: ", acc)

p2 = Package([[2]])
p6 = Package([[6]])
data.extend([p2, p6])

ordered = sorted(data)
a = ordered.index(p2) + 1
b = ordered.index(p6) + 1
print("part2: ", a*b)
