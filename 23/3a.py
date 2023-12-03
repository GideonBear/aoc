import contextlib

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

res = 0
for num, spaces in nums:
    num_done = False
    for space in spaces:
        for y, x in get_adj(space):
            with contextlib.suppress(IndexError):
                if is_symbol(lines[y][x]):
                    #print(f"Found {num} : {space} to be adjacent, adding to res")
                    res += num
                    num_done = True
                    break
        if num_done:
            #print(f"Found {num} : {space} to be non-adjacent, trying next space")
            break

print(res)
