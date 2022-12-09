from termcolor import cprint

from dataclasses import dataclass


def _diff(a: int, b: int) -> int:
    return abs(a - b)


def debug(strict: bool = False):
    if require_move():
        if strict:
            cprint('Violated below', 'red')
        cprint(f'{head=} {tail=}', 'red')
    else:
        cprint(f'{head=} {tail=}', 'green')


def require_move():
    return _diff(head.x, tail.x) > 1 or _diff(head.y, tail.y) > 1


@dataclass(frozen=True)
class Vector:
    x: int
    y: int

    def __add__(self, other):
        assert isinstance(other, Vector)
        return Vector(self.x + other.x, self.y + other.y)

    def __mul__(self, other):
        assert isinstance(other, int)
        return Vector(self.x * other, self.y * other)

    def __neg__(self):
        return Vector(-self.x, -self.y)

    def __bool__(self) -> bool:
        return bool(self.x or self.y)

    @classmethod
    def zero(cls):
        return Vector(0, 0)

    @classmethod
    def from_dir_size(cls, direction: str, size: str):
        size = int(size)
        return cls.from_dir(direction) * size

    @classmethod
    def from_dir(cls, direction: str):
        return Vector(*{
            'R': (1, 0),
            'L': (-1, 0),
            'U': (0, 1),
            'D': (0, -1),
        }[direction])

    def split(self):
        assert 0 in (self.x, self.y)
        if self.x == 0:
            count = abs(self.y)
            single = Vector(self.y // count, 0)
        else:
            count = abs(self.x)
            single = Vector(0, self.x // count)
        return (single for _ in range(count))

    def is_illegal(self):
        return self.x < 0 or self.y < 0

    def diff(self, other):
        x = self.x - other.x
        y = self.y - other.y
        return Vector(x, y)


with open('9.e.txt') as f:
    text = f.read().strip('\n')

moves = []
for move in text.split('\n'):
    moves += Vector.from_dir_size(*move.split(' ')).split()


visited = set()
head = Vector.zero()
tail = Vector.zero()

for move in moves:
    head += move
    debug()

    if require_move():
        diff = head.diff(tail)
        x = diff.x - 1 if head.x != tail.x else 0
        y = diff.y - 1 if head.y != tail.y else 0
        to_move = Vector(x, y)

        print(f'Applying {to_move} to {tail}')
        tail += to_move
        print(f'{tail=}')

    debug(True)
    visited.add(tail)

print(len(visited))
