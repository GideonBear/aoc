from heapq import nlargest

with open('1.txt') as f:
    text = f.read().strip('\n')

elves = text.split('\n\n')
elves = [sum(map(int, elve.split('\n'))) for elve in elves]
print(sum(nlargest(3, elves)))
