with open('6.txt') as f:
    text = f.read().strip('\n')


times, record_distances = (map(int, line.split()[1:]) for line in text.split("\n"))
races = [(time, record_distance) for time, record_distance in zip(times, record_distances)]

total = 1
for time, record_distance in races:
    ways_to_beat = 0
    for holding_time in range(time + 1):
        speed = holding_time
        time_left = time - holding_time
        distance = time_left * speed
        if distance > record_distance:
            ways_to_beat += 1
    total *= ways_to_beat

print(total)
