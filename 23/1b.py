from collections.abc import Iterable

with open('1.txt') as f:
    text = f.read().strip('\n')

numbers = {
    "one": "1",
    "two": "2",
    "three": "3",
    "four": "4",
    "five": "5",
    "six": "6",
    "seven": "7",
    "eight": "8",
    "nine": "9",
}


def find_number(s: Iterable[str]) -> str:
    con_s = ""
    for char in s:
        if char.isdigit():
            return char
        con_s += char
        for k, v in numbers.items():
            if k in con_s:
                return v


lines = text.split("\n")
result = 0
for line in lines:
    n1 = find_number(line)
    n2 = find_number(reversed(line))
    result += int(n1 + n2)
print(result)
