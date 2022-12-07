input = open("input.txt").read()

fs = {}
dirs = [""]
current_dir = []
lines = input.split("\n")
i = 0

while True:
    line = lines[i]
    if line.startswith("$ cd"):
        _, folder = line.split("$ cd ")
        if folder == "/":
            current_dir = [""]
        elif folder == "..":
            current_dir.pop()
        else:
            current_dir.append(folder)
    elif line == "":
        break
    elif line.startswith("$ ls"):
        i+=1
        while True:
            line = lines[i]
            if line.startswith("$") or line == "":
                break
            size, name = line.split(" ")

            if size != "dir":
                fs["/".join(current_dir+[name])] = size
            else:
                dirs.append("/".join(current_dir+[name]))
            i+=1
        continue
    i+=1

sizes = {}
for d in dirs:
    for f,size in fs.items():
        if f.startswith(d+'/'):
            sizes[d] = sizes.get(d, 0) + int(size)

total = 0
for d,size in sizes.items():
    if size <= 100000:
        total += size

required = 30000000 - (70000000 - sizes[""])
ff = filter(lambda x: x>required,sorted(sizes.values()))

print("part1: ", total)
print("part2: ", next(ff))
