import contextlib
import operator
from functools import reduce

with open('3.txt') as f:
    text = f.read().strip('\n')


def is_symbol(s):
    return not s.isdigit() and s != "."


def get_adj(space: tuple[int, int]):
    y, x = space
    for vy, vx in ((0, 1), (1, 1), (1, 0), (1, -1), (0, -1), (-1, -1), (-1, 0), (-1, 1)):
        yield y + vy, x + vx


lines = text.split("\n")

nums = []
for y, line in enumerate(lines):
    curr = ""
    curr_spaces = []
    for x, char in enumerate(line):
        if char.isdigit():
            curr += char
            curr_spaces.append((y, x))
        else:
            if curr:
                nums.append((int(curr), curr_spaces))
                curr = ""
                curr_spaces = []

    if curr:
        nums.append((int(curr), curr_spaces))

stars = []
for y, line in enumerate(lines):
    for x, char in enumerate(line):
        if char == "*":
            stars.append((y, x))

res = 0
for star in stars:
    #print(f"Trying star {star}")
    adj_nums = []
    done_nums = []
    for adj in get_adj(star):
        for num, num_spaces in nums:
            if num in done_nums:
                continue
            for num_space in num_spaces:
                if num_space == adj:
                    #print(f"Adj space {adj} found to be occupied by {num}")
                    adj_nums.append(num)
                    done_nums.append(num)
    if len(adj_nums) == 2:
        res += reduce(operator.mul, adj_nums)
    elif len(adj_nums) == 3:
        print("- 3 adj_nums, nothing done")

print(res)
