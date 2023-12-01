with open('1.txt') as f:
    text = f.read().strip('\n')


lines = text.split("\n")
result = 0
for line in lines:
    digits = list(filter(str.isdigit, line))
    result += int(digits[0] + digits[-1])
print(result)
