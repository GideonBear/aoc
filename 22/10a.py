class CPU:
    def __init__(self, counter: int = 0, x: int = 1):
        self.counter = counter
        self.x = x
        self.interesting = (20, 60, 100, 140, 180, 220)
        self.total = 0

    @property
    def strength(self) -> int:
        return self.counter * self.x

    def add(self, num: int) -> None:
        self.cycle(2)
        self.x += num

    def cycle(self, count: int = 1) -> None:
        for _ in range(count):
            self.counter += 1
            if self.counter in self.interesting:
                self.total += self.strength


with open('10.txt') as f:
    text = f.read().strip('\n')


cpu = CPU()

for op in text.split('\n'):
    if op == 'noop':
        cpu.cycle()
    else:
        cpu.add(int(op[5:]))

print(cpu.total)
