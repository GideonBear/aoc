from dataclasses import dataclass


@dataclass
class Vector:
    x: int
    y: int

    def __add__(self, other):
        assert isinstance(other, Vector)
        return Vector(self.x + other.x, self.y + other.y)

    @property
    def size(self):
        return forest[self.x][self.y]

    def is_illegal(self):
        return self.x < 0 or self.y < 0


vectors = [Vector(x, y) for x, y in ((0, 1), (0, -1), (1, 0), (-1, 0))]


with open('8.txt') as f:
    text = f.read().strip('\n')


forest = [list(line) for line in text.split('\n')]
size = len(forest)
count = 0

for i in range(size):
    for j in range(size):
        tree = Vector(i, j)
        tree_done = False
        for vector in vectors:
            if tree_done:
                break
            curr_looking = tree + vector
            while True:
                if curr_looking.is_illegal():
                    count += 1
                    tree_done = True
                    break
                try:
                    if curr_looking.size >= tree.size:
                        break
                except IndexError:
                    count += 1
                    tree_done = True
                    break
                curr_looking += vector


print(count)
