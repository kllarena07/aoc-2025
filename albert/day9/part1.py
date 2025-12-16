in_file = open("test.in", "r")

lines = in_file.readlines()

coords = [line.strip().split(",") for line in lines]

for c in coords:
  c[0] = int(c[0])
  c[1] = int(c[1])

max_area = 0

for i in range(len(coords)):
  c1 = coords[i]
  for j in range(i + 1, len(coords)):
    c2 = coords[j]
    area = abs(c1[0] - c2[0] + 1) * abs(c1[1] - c2[1] + 1)
    
    max_area = max(max_area, area)
    
print(max_area)
    
    
