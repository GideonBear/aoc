import operator
from functools import reduce

with open('2.txt') as f:
    text = f.read().strip('\n')


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
    max_cubes = {
        "red": 0,
        "green": 0,
        "blue": 0,
    }
    for turn in turns:
        for color_hand in turn:
            n, c = color_hand.split(" ")
            n = int(n)
            max_cubes[c] = max(n, max_cubes[c])
    power = reduce(operator.mul, max_cubes.values())
    res += power

print(res)
