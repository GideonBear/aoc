from string import digits

import numpy as np


def chunked_between(iterable, chunk_size: int, between_size: int):
    in_between = False
    curr_chunk_size = chunk_size
    chunk_step = 0
    chunk = []
    for item in iterable:
        chunk.append(item)
        chunk_step += 1
        if curr_chunk_size == chunk_step:
            if not in_between:
                yield chunk
            chunk = []
            in_between = not in_between
            curr_chunk_size = between_size if in_between else chunk_size
            chunk_step = 0


with open('5.txt') as f:
    text = f.read().strip('\n')


lineiter = iter(text.split('\n'))

lines = []

for line in lineiter:
    if line[:3] == ' 1 ':
        next(lineiter)
        break
    curr_line = []
    for chunk in chunked_between(line, 3, 1):
        char = chunk[1]
        if char == ' ':
            char = None
        curr_line.append(char)
    lines.append(curr_line)

stacks = [
    list(
        filter(
            lambda x: x is not None,
            reversed(stack)
        ))
    for stack
    in np.array(lines).transpose()
]

moves = []
for line in lineiter:
    move = []
    num = ''
    for char in line:
        if char == ' ' and num:
            move.append(int(num))
            num = ''
        elif char in digits:
            num += char
    if num:
        move.append(int(num))
    moves.append(move)

moves = ((num, orig - 1, dest - 1) for num, orig, dest in moves)

for num, orig, dest in moves:
    orig = stacks[orig]
    dest = stacks[dest]
    print(orig, dest)
    items = orig[-num:]
    del orig[-num:]
    dest.extend(items)
    print(orig, dest)

print(''.join(stack[-1] for stack in stacks))
