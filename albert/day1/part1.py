in_file = open("test.in", "r")

lines = in_file.readlines()

total_sum = 50
count = 0
for line in lines:
  direction = line[0]
  amount = int(line[1:])
  
  if direction == "L":
    total_sum -= amount
  else:
    total_sum += amount
    
  if total_sum % 100 == 0:
    count += 1
  
print(count)