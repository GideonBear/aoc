def range_overlaps(a: range, b: range) -> bool:
    return bool(range(max(a[0], b[0]), min(a[-1], b[-1])+1))


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
overlaps = (
    range_overlaps(a, b) or range_overlaps(b, a)
    for a, b
    in assignments
)
print(sum(overlaps))
