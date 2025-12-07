
in_file = open("test.in", "r")

lines = in_file.readlines()
lines = [line.strip('\n') for line in lines]
ops = lines[-1].strip(" ").split()

formatted_lines = []
total = 0

for j in range(len(lines) - 1):
  line = []
  str = ""

  for i in range(len(lines[j])):
    # print(lines[0][i])
    
    if lines[0][i] != " " or lines[1][i] != " " or lines[2][i] != " " or lines[3][i] != " ": # not a gap
      str += lines[j][i]
    else:
      line.append(str)
      str = ""
      i += 1
      
  line.append(str)
      
  formatted_lines.append(line)

# print(formatted_lines)

for i in range(len(formatted_lines[0])):    
  result = 0 if ops[i] == "+" else 1
  for j in range(len(formatted_lines[0][i])): 
    num = ""

    for k in range(len(formatted_lines)):
      num += formatted_lines[k][i][j]
    # print(num)
    
    if ops[i] == "+":
      result += int(num)
    else:
      result *= int(num)
  # print()

  total += result
  
print(total)