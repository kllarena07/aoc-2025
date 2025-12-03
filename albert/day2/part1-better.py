in_file = open("test.in", "r")

lines = in_file.readlines()

def calculate_invalid(start, begin):
  total = 0

  for i in range(start, begin + 1):
    string_i = str(i)
    num_digits = len(string_i) 
    if (num_digits % 2 == 1):
      continue
    else:
      half_digits = int(num_digits / 2)
      first_half = string_i[:half_digits]
      second_half = string_i[half_digits:]
      
      if (first_half == second_half):
        total += i
  
  return total

result = 0
for line in lines:
  line = line.strip(',\n').split(",")
  for r in line:
    rge = r.split("-")
    result += calculate_invalid(int(rge[0]), int(rge[1]))

print(result)
