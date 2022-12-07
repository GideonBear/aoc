class Dir:
    def __init__(self, parent=None):
        self.parent = parent
        self.children: dict[str, int] = {}

    @property
    def size(self) -> int:
        return sum(
            child
            if isinstance(child, int)
            else child.size
            for child
            in self.children.values())

    def add_dir(self, name: str):
        if name in self.children:
            raise ValueError('Child already exists')
        dir = Dir(self)
        self.add(name, dir)
        return dir

    def add(self, name: str, node):
        self.children[name] = node


with open('7.txt') as f:
    text = f.read().strip('\n')


root = Dir()
curr = None
dirs = []

for line in text.split('\n'):
    if line == '$ ls':
        continue
    elif line == '$ cd /':
        curr = root
    elif line == '$ cd ..':
        curr = curr.parent
    elif line.startswith('$ cd '):
        name = line[5:]
        curr = curr.children[name]
    elif line.startswith('dir '):
        name = line[4:]
        dir = curr.add_dir(name)
        dirs.append(dir)
    else:
        size, name = line.split(' ')
        curr.add(name, int(size))

total = 70_000_000
needed = 30_000_000

needed -= total - root.size

print(min(
    dir.size
    for dir
    in dirs
    if dir.size >= needed
))
