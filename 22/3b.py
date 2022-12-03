from string import ascii_letters


priorities = ' ' + ascii_letters


def chunks(seq, size):
    return (seq[pos:pos + size] for pos in range(0, len(seq), size))


with open('3.txt') as f:
    text = f.read().strip('\n')


groups = chunks([set(elf) for elf in text.split('\n')], 3)
els = (a.intersection(b).intersection(c).pop() for a, b, c in groups)
pris = (priorities.index(el) for el in els)
print(sum(pris))
