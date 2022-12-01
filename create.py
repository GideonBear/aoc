import re
from pathlib import Path


num = re.compile(
    r'[0-9]+'
)
text = r'''
with open('{}.txt') as f:
    text = f.read().strip('\n')
'''.strip('\n') + '\n' * 4


here = Path()
years = (year for year in here.iterdir() if num.match(year.name))
new_yearnum = max(int(year.name) for year in years) + 1
new_year = here / str(new_yearnum)
new_year.mkdir()

for i in range(1, 26):
    (new_year / f'{i}a.py').write_text(text.format(i))
    (new_year / f'{i}b.py').write_text(text.format(i))
