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
highest_score = 0

for i in range(size):
    for j in range(size):
        tree = Vector(i, j)
        score = 1
        for vector in vectors:
            direction_score = 0
            curr_looking = tree + vector
            while True:
                if curr_looking.is_illegal():
                    break
                try:
                    if curr_looking.size >= tree.size:
                        direction_score += 1
                        break
                except IndexError:
                    break
                direction_score += 1
                curr_looking += vector
            score *= direction_score

        highest_score = max(score, highest_score)
        score = 0


print(highest_score)
