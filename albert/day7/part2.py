from collections import defaultdict

in_file = open("test.in", "r")

lines = in_file.readlines()
lines = [line.strip() for line in lines]

source = 0
for i in range(len(lines[0])):
  if lines[0][i] == "S":
    source = i
    break

beams = {source: 1}
total = 0

def calculate_beams(i):
  global beams, total
  new_beams = defaultdict(int)
  if i == len(lines):
    return
  
  line = lines[i]
  
  for b in beams:
    if line[b] == "^":
      if b > 0:
        new_beams[b - 1] += beams[b]
      if b < len(line) - 1:
        new_beams[b + 1] += beams[b]
    else:
      new_beams[b] += beams[b]
      
  beams = new_beams.copy()
  calculate_beams(i + 1)

calculate_beams(1)

total = 0
for b in beams:
  total += beams[b]
  
print(total)