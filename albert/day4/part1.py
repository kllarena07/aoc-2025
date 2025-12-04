from collections import defaultdict

in_file = open("test.in", "r")

lines = in_file.readlines()

total = 0

roll_map = []
adjacent_count = defaultdict(int) # (i, j): number surrounding

for line in lines:
  roll_map.append(line.strip())

for i in range(len(lines)):
  for j in range(len(lines[i])):
    if lines[i][j] == "@":
      adjacent_count[(i-1, j-1)] += 1
      adjacent_count[(i-1, j)] += 1
      adjacent_count[(i-1, j+1)] += 1
      adjacent_count[(i, j-1)] += 1
      adjacent_count[(i, j+1)] += 1
      adjacent_count[(i+1, j-1)] += 1
      adjacent_count[(i+1, j)] += 1
      adjacent_count[(i+1, j+1)] += 1
    
for i in range(len(lines)):
  for j in range(len(lines[i])):
    if adjacent_count[(i, j)] < 4 and lines[i][j] == "@":
      total += 1

print(total)

  