with open('2.txt') as f:
    text = f.read().strip('\n')

cubes = {
    "red": 12,
    "green": 13,
    "blue": 14,
}


res = 0
lines = text.split("\n")


def flatten(l):
    for i in l:
        yield from i


for line in lines:
    line = line.removeprefix("Game ")
    gid = ""
    while line[0].isdigit():
        gid += line[0]
        line = line[1:]
    gid = int(gid)
    line = line.removeprefix(": ")

    turns = [turn.split(", ") for turn in line.split("; ")]
    for color_hand in flatten(turns):
        n, c = color_hand.split(" ")
        n = int(n)

        if n > cubes[c]:
            break
    else:
        res += gid

print(res)
