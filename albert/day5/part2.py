
in_file = open("test.in", "r")

lines = in_file.readlines()

total = 0
ranges = []
lines = [line.strip() for line in lines]

for i in range(len(lines)):
  if lines[i] == "":
    break
  
  r_min = int(lines[i].split("-")[0])
  r_max = int(lines[i].split("-")[1])
  
  ranges.append([r_min, r_max])

ranges = sorted(ranges) # sort originally by first element, tiebreak by second element
  
final_ranges = [ranges[0]] # first range

for i in range(1, len(ranges)): # iterate the rest of the ranges
  last_range = final_ranges[-1]
  current_range = ranges[i]
  # there will be 4 cases for the last element in final_ranges:
  # 1. same min, smaller or equal max -> do nothing 1-4 + 1-3 -> 1-4
  # 2. same min, larger max -> overwrite the max 1-4 + 1-5 -> 1-5
  # 3. larger min, current max <= last max, do nothing 1-4 + 2-3 -> 1-4
  # 4. larger min, current max > last max 
  # - 4a. if current min <= last max, overwrite the last max 1-4 + 3-5 -> 1-5
  # - 4b. if current min > last max, append 1-4 + 5-6 -> 1-4, 5-6

  if current_range[0] == last_range[0]: # if same min
    if current_range[1] > last_range[1]: # (2)
      last_range[1] = current_range[1]
    else: # (1)
      continue 
  
  if current_range[0] > last_range[0]: # if larger min 
    if current_range[1] <= last_range[1]: # (3)
      continue
    else: # (4)
      if current_range[0] < last_range[1]: # (4a)
        last_range[1] = current_range[1]
      else: # (4b)
        final_ranges.append(current_range)
        
for r in final_ranges:
  total += r[1] - r[0] + 1
  
print(total)
