def range_subset(a: range, b: range) -> bool:
    return a[0] in b and a[-1] in b


with open('4.txt') as f:
    text = f.read().strip('\n')


assignments = (
    (
        range(
            (
                assignment_range :=
                [int(num) for num in assignment.split('-')]
            )[0],
            assignment_range[1] + 1
        ) for assignment
        in pair.split(',')
    ) for pair
    in text.split('\n')
)
subsets = (
    range_subset(a, b) or range_subset(b, a)
    for a, b
    in assignments
)
print(sum(subsets))
