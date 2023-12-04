import itertools
from pprint import pprint

with open('4.txt') as f:
    text = f.read().strip('\n')


cards = text.split("\n")
cards = (card[card.find(":") + 2:] for card in cards)
cards = (card.split(" | ") for card in cards)
cards = dict(zip(
    itertools.count(),
    ((i,) + tuple(map(str.split, card)) for i, card in enumerate(cards))
))

total = 0
q = list(cards.values())
while q:
    print(f"{len(q)=}")
    total += 1
    i, win_nums, my_nums = q.pop()
    num_matching_nums = len(list(filter(lambda my_num: my_num in win_nums, my_nums)))
    #print(f"Found {num_matching_nums} matches in card {i}")
    for j in range(1, num_matching_nums + 1):
        q.append(cards[i + j])
print(total)
