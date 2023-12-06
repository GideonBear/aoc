import sys
import re
from pathlib import Path


num = re.compile(
    r'[0-9]+'
)
text = r'''
use std::fs;

let text = fs::read_to_string("{}.txt").expect("Error while reading file");
'''.strip('\n') + '\n' * 3


here = Path()

if len(sys.argv) > 1:
    new_yearnum = int(sys.argv[1])
else:
    years = (year for year in here.iterdir() if num.match(year.name))
    new_yearnum = max(int(year.name) for year in years) + 1

new_year = here / str(new_yearnum)
new_year.mkdir()

for i in range(1, 26):
    (new_year / f'{i}a.rs').write_text(text.format(i))
    (new_year / f'{i}b.rs').write_text(text.format(i))

print(f'Created {new_year}')
