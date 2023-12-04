from pprint import pprint

with open('4.txt') as f:
    text = f.read().strip('\n')


cards = text.split("\n")
cards = (card[card.find(":") + 2:] for card in cards)
cards = (card.split(" | ") for card in cards)
cards = list(tuple(map(str.split, card)) for card in cards)
assert all(len(card) == 2 for card in cards)

total = 0
for win_nums, my_nums in cards:
    num_matching_nums = len(list(filter(lambda my_num: my_num in win_nums, my_nums)))
    if num_matching_nums == 0:
        continue
    points = 2 ** (num_matching_nums - 1)
    total += points
print(total)
