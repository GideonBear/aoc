from dataclasses import dataclass
from pprint import pprint

with open('5.txt') as f:
    text = f.read().strip('\n')


@dataclass
class Ma:
    from_cat: str
    to_cat: str
    ranges: list[tuple[int, int, int]]

    @classmethod
    def from_string(cls, from_cat, to_cat, s):
        self = cls(from_cat, to_cat, [])
        for ma_range in s.split("\n"):
            self.ranges.append(tuple(map(int, ma_range.split(" "))))
        return self

    def __getitem__(self, item):
        for dest_start, source_start, length in self.ranges:
            if source_start < item < source_start + length:
                return dest_start + (item - source_start)
        return item


seeds, *mas = text.split("\n\n")

seeds = seeds.removeprefix("seeds: ")
seeds = seeds.split(" ")
seeds = map(int, seeds)

old_mas = mas
mas = []
for ma in old_mas:
    dash_pos = ma.find("-")
    from_cat = ma[:dash_pos]
    ma = ma[dash_pos + 4:]

    space_pos = ma.find(" ")
    to_cat = ma[:space_pos]
    ma = ma[space_pos + 6:]

    ma = Ma.from_string(from_cat, to_cat, ma)

    mas.append(ma)

for ma in mas:
    seeds = map(ma.__getitem__, seeds)
print(min(seeds))
