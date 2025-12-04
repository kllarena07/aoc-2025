from collections import defaultdict

in_file = open("test.in", "r")

lines = in_file.readlines()

total = 0

roll_map = []

for line in lines:
  roll_map.append(list(line.strip()))

while True:
  count = 0
  adjacent_count = defaultdict(int) # (i, j): number surrounding

  for i in range(len(roll_map)):
    for j in range(len(roll_map[i])):
      if roll_map[i][j] == "@":
        adjacent_count[(i-1, j-1)] += 1
        adjacent_count[(i-1, j)] += 1
        adjacent_count[(i-1, j+1)] += 1
        adjacent_count[(i, j-1)] += 1
        adjacent_count[(i, j+1)] += 1
        adjacent_count[(i+1, j-1)] += 1
        adjacent_count[(i+1, j)] += 1
        adjacent_count[(i+1, j+1)] += 1
      
  for i in range(len(roll_map)):
    for j in range(len(roll_map[i])):
      if adjacent_count[(i, j)] < 4 and roll_map[i][j] == "@":
        roll_map[i][j] = "."
        count += 1

  if count == 0:
    break
  
  total += count
  
print(total)

  