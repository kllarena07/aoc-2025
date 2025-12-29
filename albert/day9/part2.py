in_file = open("example.in", "r")

lines = in_file.readlines()

coords = [line.strip().split(",") for line in lines]

for c in coords:
  c[0] = int(c[0])
  c[1] = int(c[1])
  
horizontal = []
vertical = []

for i in range(len(lines)):
  first = lines[i].strip().split(",")
  
  if i == len(lines) - 1:
    second = lines[0].strip().split(",")
  else:
    second = lines[i + 1].strip().split(",")
  
  first = (int(first[0]), int(first[1]))
  second = (int(second[0]), int(second[1]))
  
  if first[0] == second[0]: # vertical line
    vertical.append(sorted((first, second)))
  else:
    horizontal.append(sorted((first, second)))
  
def check_horizontal(point):
  for r in horizontal:
    if point[1] == int(r[0][1]) and point[0] in range(int(r[0][0]), int(r[1][0]) + 1):
      return True
  
  return False

def check_vertical(point):
  for r in vertical:
    # print(r)
    if point[0] == int(r[0][0]) and point[1] in range(int(r[0][1]), int(r[1][1]) + 1):
      return True
    
  return False

def check_point_on_line(point):
  return check_horizontal(point) or check_vertical(point)

def check_ray(point):
  # go from 0 to point
  num_lines = 0
  on_vertical = False
  for i in range(0, point[1]):
    # print(i)
    # print(point[0], i)
    if check_vertical((point[0], i)):
      on_vertical = True
      # print('hi')
      continue
      
    if on_vertical:
      num_lines += 1
      on_vertical = False
    
    if check_horizontal((point[0], i)):
      # print('here')
      num_lines += 1
      
  return num_lines % 2 == 1

def check_ray(point):
    px, py = point
    num_crossings = 0
    

    for seg in horizontal:
        (x1, y1), (x2, y2) = seg 
        
        if y1 < py:
            if x1 <= px < x2:
                num_crossings += 1
                
    return num_crossings % 2 == 1

def check_point(point):
  if check_point_on_line(point):
    return True
  
  return check_ray(point)

def check_bounds(p1, p2):
  point1 = (p1[0], p2[1])
  point2 = (p2[0], p1[1])
  
  # print(p1, p2, point1, point2)
  
  return check_point(point1) and check_point(point2)

max_area = 0
for i in range(len(coords)):
  c1 = coords[i]
  for j in range(i + 1, len(coords)):
    c2 = coords[j]
    
    if check_bounds(c1, c2):
      area = abs(c1[0] - c2[0] + 1) * abs(c1[1] - c2[1] + 1)
      
      max_area = max(max_area, area)
      # print(max_area)

# print(horizontal)
# print()
# print(vertical)
print(max_area)

# print(check_ray((7, 7)))
