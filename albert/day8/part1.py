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

visited = set()
groups = []
for v in sorted_dists:
  p1, p2 = v[0], v[1]
  
  if p1 not in visited and p2 not in visited:
    groups.append([p1, p2])
    visited.add(p1)
    visited.add(p2)
  elif p1 not in visited: # this means p2 is in visited
    for arr in groups:
      if p2 in arr:
        arr.append(p1)
        visited.add(p1)
        break
  elif p2 not in visited: # p1 in visited
    for arr in groups:
      if p1 in arr:
        arr.append(p2)
        visited.add(p2)
        break
  else:  # both visited
    group1 = None
    group2 = None
    
    for arr in groups:
        if p1 in arr:
            group1 = arr
        if p2 in arr:
            group2 = arr
        if group1 is not None and group2 is not None:
            break
    
    if group1 is not group2:
        group1.extend(group2)
        groups.remove(group2) 
  
    
groups = sorted(groups, key=lambda x: len(x), reverse=True)

print(len(groups[0]) * len(groups[1]) * len(groups[2]))