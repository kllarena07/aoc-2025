in_file = open("test.in", "r")

lines = in_file.readlines()
lines = [line.strip() for line in lines]

source = 0
for i in range(len(lines[0])):
  if lines[0][i] == "S":
    source = i
    break

beams = [source]
total = 0

def calculate_beams(i):
  global beams, total
  if i == len(lines):
    return
  
  line = lines[i]
  new_beams = []
  
  for b in beams:
    if line[b] == "^":
      total += 1
      if b > 0:
        new_beams.append(b - 1)
      if b < len(line) - 1:
        new_beams.append(b + 1)
      
    else:
      new_beams.append(b)

  new_beams = set(new_beams)
  beams = new_beams.copy()
  calculate_beams(i + 1)

calculate_beams(1)

beams = set(beams)
print(beams)
print(total)