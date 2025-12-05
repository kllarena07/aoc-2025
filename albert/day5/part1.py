
in_file = open("test.in", "r")

lines = in_file.readlines()

total = 0

ranges = [] 

lines = [line.strip() for line in lines]

start = 0

for i in range(len(lines)):
  if lines[i] == "":
    start = i + 1
    break
  
  ranges.append((int(lines[i].split("-")[0]), int(lines[i].split("-")[1]) + 1))
  
  
for i in range(start, len(lines)):
  for r in ranges:
    if int(lines[i]) in range(r[0], r[1]):
      total += 1
      break

    
print(total)
