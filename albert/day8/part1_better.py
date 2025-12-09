import math

in_file = open("test.in", "r")

lines = in_file.readlines()
lines = [line.strip() for line in lines]

distances = {}

for i in range(len(lines)):
  p1 = tuple(int(k) for k in lines[i].split(","))
  for j in range(i + 1, len(lines)):
    p2 = tuple(int(k) for k in lines[j].split(","))
    distances[(p1, p2)] = math.dist(p1, p2)

sorted_dists = sorted(distances.items(), key=lambda item: item[1])[:1000]
sorted_dists = dict(sorted_dists)

groups = []

for i in range(len(lines)):
  p = tuple(int(k) for k in lines[i].split(","))
  groups.append({p})

for v in sorted_dists:
  p1, p2 = v[0], v[1]
  for i, group in enumerate(groups):
    if p1 in group:
        g1 = i
    if p2 in group:
        g2 = i
  
  if g1 != g2:
    groups[g1].update(groups[g2])
    groups.pop(g2)
    
groups = sorted(groups, key=lambda x: len(x), reverse=True)
print(len(groups[0]) * len(groups[1]) * len(groups[2]))