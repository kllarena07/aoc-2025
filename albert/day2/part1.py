in_file = open("test.in", "r")

lines = in_file.readlines()

def calculate_invalid(start, begin):
  total = 0
  
  # if odd num of digits, it won't work
  # if even, then check divisible 10^(num_digits / 2)  + 1
  
  for i in range(start, begin + 1):
    num_digits = len(str(i)) 
    if (num_digits % 2 == 1):
      continue
    elif (i % (10 ** (num_digits / 2) + 1) == 0):
      total += i
  
  return total

result = 0
for line in lines:
  line = line.strip(',\n').split(",")
  for r in line:
    rge = r.split("-")
    result += calculate_invalid(int(rge[0]), int(rge[1]))

print(result)
