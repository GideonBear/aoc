from string import ascii_letters


priorities = ' ' + ascii_letters


with open('3.txt') as f:
    text = f.read().strip('\n')


rucksacks = ((set(rucksack[:(middle := len(rucksack) // 2)]), set(rucksack[middle:])) for rucksack in text.split('\n'))
els = (a.intersection(b).pop() for a, b in rucksacks)
pris = (priorities.index(el) for el in els)
print(sum(pris))
