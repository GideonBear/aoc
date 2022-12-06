from more_itertools import windowed


with open('6.txt') as f:
    text = f.read().strip('\n')


for i, window in enumerate(windowed(text, 4), start=4):
    if len(window) == len(set(window)):
        print(i)
        break
